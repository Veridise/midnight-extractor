use std::{borrow::Cow, collections::BTreeMap};

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::{expr::IRBexpr, stmt::IRStmt};
use midnight_circuits::parsing::{automaton_chip::NativeAutomaton, StdLibParser};
use midnight_proofs::plonk::Expression;

/// (word, next, marker)
type S<F> = (F, F, F);

type T<F> = (F, Vec<S<F>>);

fn flatten_transitions<F>(
    automata: impl Iterator<Item = (StdLibParser, NativeAutomaton<F>)>,
    invalid_word: u64,
) -> impl IntoIterator<Item = T<F>>
where
    F: PrimeField + Ord,
{
    let transitions = automata.flat_map(|(_, automaton)| {
        let zero_transition = [(F::ZERO, (F::ZERO, F::ZERO, F::ZERO))];
        let transitions = automaton.transitions.into_iter().map(|((s, w), (n, m))| (s, (w, n, m)));
        let final_states = automaton
            .final_states
            .into_iter()
            .map(|s| (s, (F::from(invalid_word), F::ZERO, F::ZERO)));

        zero_transition.into_iter().chain(transitions).chain(final_states)
    });

    let mut grouped: BTreeMap<F, Vec<(F, F, F)>> = BTreeMap::new();
    for (s, data) in transitions {
        grouped.entry(s).or_default().push(data);
    }

    grouped
}

#[derive(Clone, Debug)]
enum Placeholder<F> {
    State,
    Word,
    Next,
    Marker,
    Const(F),
}

impl<F: Clone + Copy> Placeholder<F> {
    fn into_expression<'l>(
        &self,
        lookup: &'l Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'l, Expression<F>>> {
        ExprOrTemp::Expr(match self {
            Placeholder::State => Cow::Borrowed(&lookup.inputs()[0]),
            Placeholder::Word => Cow::Borrowed(&lookup.inputs()[1]),
            Placeholder::Next => Cow::Borrowed(&lookup.inputs()[2]),
            Placeholder::Marker => Cow::Borrowed(&lookup.inputs()[3]),
            Placeholder::Const(f) => Cow::Owned(Expression::Constant(*f)),
        })
    }
}

fn create_placeholder_expr<F: PrimeField>((s, t): T<F>) -> IRBexpr<Placeholder<F>> {
    let s = IRBexpr::eq(Placeholder::State, Placeholder::Const(s));
    let t = IRBexpr::Or(
        t.into_iter()
            .map(|(w, n, m)| {
                let w = IRBexpr::eq(Placeholder::Word, Placeholder::Const(w));
                let n = IRBexpr::eq(Placeholder::Next, Placeholder::Const(n));
                let m = IRBexpr::eq(Placeholder::Marker, Placeholder::Const(m));
                IRBexpr::And(vec![w, n, m])
            })
            .collect(),
    );
    IRBexpr::implies(s, t)
}

fn create_placeholder_stmt<F: PrimeField>(
    transitions: impl Iterator<Item = T<F>>,

    invalid_word: u64,
) -> IRStmt<Placeholder<F>> {
    let assert_stmt = IRStmt::assert(IRBexpr::Or(
        transitions.map(create_placeholder_expr).collect(),
    ));
    let range_check = IRStmt::assert(IRBexpr::le(
        Placeholder::Word,
        Placeholder::Const(F::from(invalid_word)),
    ));
    IRStmt::seq([assert_stmt, range_check])
}

/// Lookup callback that handles parsing automata.
///
/// The handler will create a disjunction of implications.
/// Each implication's lhs checks the current state while the rhs
/// constraints each of the other cells of the lookup to be one of the
/// possible transitions:
///
/// ```text
/// state = S -> (
///     (word = W_0 /\ next = N_0 /\ marker = M_0) \/
///     (word = W_1 /\ next = N_1 /\ marker = M_1) \/ ...
/// )
/// ```
#[derive(Debug, Clone)]
pub struct AutomatonLookup<F> {
    stmt: IRStmt<Placeholder<F>>,
}

impl<F> AutomatonLookup<F>
where
    F: PrimeField + Ord,
{
    /// Creates a new lookup handler from the automata collection.
    ///
    /// Each automaton in the collection must have unique states to the other automata. No two
    /// automata can have a state with the same value.
    pub fn new(
        automata: impl Iterator<Item = (StdLibParser, NativeAutomaton<F>)>,
        bitsize: u64,
    ) -> Self {
        let invalid_word = (1 << bitsize) + 1;
        let transitions = flatten_transitions(automata, invalid_word);
        let stmt = create_placeholder_stmt(transitions.into_iter(), invalid_word);
        Self { stmt }
    }
}

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for AutomatonLookup<F> {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        _table: &dyn LookupTableGenerator<F>,
        _temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        Ok(self.stmt.map_into(&|p| p.into_expression(lookup)))
    }
}
