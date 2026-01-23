use std::borrow::Cow;

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::{expr::IRBexpr, stmt::IRStmt};
use midnight_proofs::plonk::Expression;

/// Lookup callback that handles parsing automata.
#[derive(Debug, Clone)]
pub struct AutomatonLookup {
    automaton_module: &'static str,
    invalid_word: u64,
}

impl AutomatonLookup {
    /// Creates a new lookup handler from the automata collection.
    ///
    /// Each automaton in the collection must have unique states to the other automata. No two
    /// automata can have a state with the same value.
    pub fn new(automaton_module: &'static str, bitsize: u64) -> Self {
        let invalid_word = (1 << bitsize) + 1;
        Self {
            automaton_module,
            invalid_word,
        }
    }
}

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for AutomatonLookup {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        _table: &dyn LookupTableGenerator<F>,
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        let call_outs = temps.take(3).collect::<Vec<_>>();
        assert_eq!(call_outs.len(), 3);
        let automaton_call = IRStmt::call(
            self.automaton_module,
            [ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[0]))],
            call_outs.iter().copied().map(Into::into),
        );
        let copy_constraints = IRStmt::seq(
            call_outs
                .into_iter()
                .zip(&lookup.inputs()[1..4])
                .map(|(o, i)| IRStmt::eq(ExprOrTemp::Expr(Cow::Borrowed(i)), ExprOrTemp::Temp(o))),
        );
        let range_check = IRStmt::assert(IRBexpr::le(
            ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[1])),
            ExprOrTemp::Expr(Cow::Owned(Expression::from(self.invalid_word))),
        ));

        let mut stmt = IRStmt::seq([range_check, automaton_call, copy_constraints]);
        use haloumi_ir::meta::HasMeta as _;
        stmt.meta_mut().at_lookup(lookup.name(), lookup.idx(), None);
        stmt.propagate_meta();
        Ok(stmt)
    }
}
