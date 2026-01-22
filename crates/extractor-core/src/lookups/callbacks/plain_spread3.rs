use std::borrow::Cow;

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temp, Temps},
    LookupCallbacks,
};
use haloumi_ir::{expr::IRBexpr, stmt::IRStmt};
use midnight_proofs::plonk::Expression;

/// Lookup handler that adds a range check for a plain-spread pair and
/// calls a module that declares that the latter is a functional dependency of the former.
///
/// For a lookup in the form `(8, x, ~x)` generates
///
/// ```text
/// x < 2^8
/// ~x <= (8^8 - 1)/7
/// ~x = Spread(x)
/// x = Unspread(~x)
/// ```
pub struct PlainSpreadLookup3<M: PlainSpreadLookup3Mode> {
    spread_module: &'static str,
    unspread_module: &'static str,
    mode: M,
}

/// Strategy pattern that implements the different ways this lookup is queried.
///
/// This is just a marker trait and the implementation is defined somewhere else.
pub trait PlainSpreadLookup3Mode: PlainSpreadLookup3ModeImpl {}

/// Actual implementation of the mode.
trait PlainSpreadLookup3ModeImpl {
    fn tag<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>>;

    fn spread<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>>;

    fn dense<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>>;

    fn validate_lookup<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> anyhow::Result<()>;
}

/// The lookup queries an spread value that is valid on the whole table.
///
/// Generates a less than based on the largest tag value (13):
///
/// ```text
/// x < 2^13
/// ~x <= (8^13 - 1)/7
/// ~x = Spread(x)
/// x = Unspread(~x)
/// ```
///
/// Where `x` is a temporary.
pub struct AnySpread;

impl PlainSpreadLookup3ModeImpl for AnySpread {
    fn tag<'syn, F: PrimeField>(
        &self,
        _: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Owned(13.into()))
    }

    fn spread<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[0]))
    }

    fn dense<'syn, F: PrimeField>(
        &self,
        _: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Temp(temps.next().unwrap())
    }

    fn validate_lookup<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> anyhow::Result<()> {
        ensure_lookup_size(lookup, 1)
    }
}

impl PlainSpreadLookup3Mode for AnySpread {}

/// The lookup queries an spread value that is valid for tag 12.
///
/// Generates a less than based on that tag value:
///
/// ```text
/// x < 2^12
/// ~x <= (8^12 - 1)/7
/// ~x = Spread(x)
/// x = Unspread(~x)
/// ```
///
/// Where `x` is a temporary.
pub struct Spread12;

impl PlainSpreadLookup3ModeImpl for Spread12 {
    fn tag<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[0]))
    }

    fn spread<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[1]))
    }

    fn dense<'syn, F: PrimeField>(
        &self,
        _: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Temp(temps.next().unwrap())
    }

    fn validate_lookup<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> anyhow::Result<()> {
        ensure_lookup_size(lookup, 2)?;
        ensure_tag_is_constant_value(&lookup.inputs()[0], 12)
    }
}

impl PlainSpreadLookup3Mode for Spread12 {}
/// The lookup queries an spread value that is valid for any tag.
///
/// Generates a sequence of less than expressions for each tag:
///
/// ```text
/// (tag = 0 && x = 0 && ~x = 0) ||
/// (tag = 1 && x < 2^1 && ~x <= (8^1 - 1)/7) ||
/// (tag = 2 && x < 2^2 && ~x <= (8^2 - 1)/7) ||
/// ... ||
/// (tag = 13 && x < 2^13 && ~x <= (8^13 - 1)/7) ||
/// ~x <= (8^12 - 1)/7
/// ~x = Spread(x)
/// x = Unspread(~x)
/// ```
///
/// Where `x` is a temporary.
pub struct SpreadByTag;
impl PlainSpreadLookup3ModeImpl for SpreadByTag {
    fn tag<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[0]))
    }

    fn spread<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[1]))
    }

    fn dense<'syn, F: PrimeField>(
        &self,
        _: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Temp(temps.next().unwrap())
    }

    fn validate_lookup<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> anyhow::Result<()> {
        ensure_lookup_size(lookup, 2)
    }
}

impl PlainSpreadLookup3Mode for SpreadByTag {}

/// The lookup queries an spread value that is valid for tag 8.
///
/// ```text
/// x < 2^8
/// ~x <= (8^8 - 1)/7
/// ~x = Spread(x)
/// x = Unspread(~x)
/// ```
pub struct SpreadByteLookup;
impl PlainSpreadLookup3ModeImpl for SpreadByteLookup {
    fn tag<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[0]))
    }

    fn spread<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[2]))
    }

    fn dense<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        _: &mut Temps,
    ) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
        ExprOrTemp::Expr(Cow::Borrowed(&lookup.inputs()[1]))
    }

    fn validate_lookup<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> anyhow::Result<()> {
        ensure_lookup_size(lookup, 3)?;
        ensure_tag_is_constant_value(&lookup.inputs()[0], 8)
    }
}

impl PlainSpreadLookup3Mode for SpreadByteLookup {}

impl<M: PlainSpreadLookup3Mode> PlainSpreadLookup3<M> {
    pub fn new(spread_module: &'static str, unspread_module: &'static str, mode: M) -> Self {
        Self {
            spread_module,
            unspread_module,
            mode,
        }
    }

    fn create_call<'syn, F: PrimeField>(
        &self,
        name: &str,
        input: ExprOrTemp<Cow<'syn, Expression<F>>>,
        output: ExprOrTemp<Cow<'syn, Expression<F>>>,
        temps: &mut Temps,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        let (temp, reused) = reuse_temp_or_create(&output, temps);
        let call = IRStmt::call(name, [input], [temp.clone().into()]);

        let out_constr = if !reused {
            IRStmt::eq(output, ExprOrTemp::Temp(temp))
        } else {
            IRStmt::empty()
        };

        IRStmt::seq([call, out_constr])
    }

    fn spread_call<'syn, F: PrimeField>(
        &self,
        spread: ExprOrTemp<Cow<'syn, Expression<F>>>,
        dense: ExprOrTemp<Cow<'syn, Expression<F>>>,
        temps: &mut Temps,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        self.create_call(self.spread_module, dense, spread, temps)
    }

    fn unspread_call<'syn, F: PrimeField>(
        &self,
        spread: ExprOrTemp<Cow<'syn, Expression<F>>>,
        dense: ExprOrTemp<Cow<'syn, Expression<F>>>,
        temps: &mut Temps,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        self.create_call(self.unspread_module, spread, dense, temps)
    }

    /// Generates the inequalities based on the mode.
    fn inequalities<'syn, F: PrimeField>(
        &self,
        tag_expr: ExprOrTemp<Cow<'syn, Expression<F>>>,
        dense: ExprOrTemp<Cow<'syn, Expression<F>>>,
        spread: ExprOrTemp<Cow<'syn, Expression<F>>>,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        fn mk_expr<'syn, F: PrimeField>(value: u64) -> ExprOrTemp<Cow<'syn, Expression<F>>> {
            ExprOrTemp::Expr(Cow::Owned(value.into()))
        }
        fn eq_zero<'syn, F: PrimeField>(
            input: ExprOrTemp<Cow<'syn, Expression<F>>>,
        ) -> IRBexpr<ExprOrTemp<Cow<'syn, Expression<F>>>> {
            IRBexpr::eq(input, mk_expr::<F>(0))
        }

        let checks = (1_u32..14).map(|tag| {
            [
                IRBexpr::eq(tag_expr.clone(), mk_expr::<F>(tag.into())),
                IRBexpr::lt(dense.clone(), mk_expr::<F>(2_u64.pow(tag))),
                IRBexpr::le(spread.clone(), mk_expr::<F>((8_u64.pow(tag) - 1) / 7)),
            ]
        });

        IRStmt::assert(IRBexpr::or_many(
            std::iter::once([
                eq_zero(tag_expr.clone()),
                eq_zero(dense.clone()),
                eq_zero(spread.clone()),
            ])
            .chain(checks)
            .map(|checks| IRBexpr::and_many(checks)),
        ))
    }
}

impl<F: PrimeField, M: PlainSpreadLookup3Mode> LookupCallbacks<F, Expression<F>>
    for PlainSpreadLookup3<M>
{
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        _table: &dyn LookupTableGenerator<F>,
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        self.mode.validate_lookup(lookup)?;
        let tag = self.mode.tag(lookup);
        let spread = self.mode.spread(lookup);
        let dense = self.mode.dense(lookup, temps);
        let spread_call = self.spread_call(spread.clone(), dense.clone(), temps);
        let unspread_call = self.unspread_call(spread.clone(), dense.clone(), temps);
        let inequalities = self.inequalities(tag, dense, spread);

        Ok(IRStmt::seq([inequalities, spread_call, unspread_call]))
    }
}

fn ensure_tag_is_constant_value<F: PrimeField>(
    expr: &Expression<F>,
    value: u64,
) -> anyhow::Result<()> {
    // Evaluate the expression assuming selectors are on.
    let folded = expr.evaluate(
        &|f| Some(f),
        &|_| Some(F::ONE),
        &|_| None,
        &|_| None,
        &|_| None,
        &|_| None,
        &|e| e.map(|f| -f),
        &|lhs, rhs| lhs.zip(rhs).map(|(lhs, rhs)| lhs + rhs),
        &|lhs, rhs| lhs.zip(rhs).map(|(lhs, rhs)| lhs * rhs),
        &|lhs, rhs| lhs.map(|lhs| lhs * rhs),
    );
    folded
        .ok_or_else(|| anyhow::anyhow!("Was expecting a constant expression but got {expr:?}"))
        .and_then(|f| {
            (f == F::from(value)).then_some(()).ok_or_else(|| {
                anyhow::anyhow!("Was expecting tag to be equal to {value} but got {f:?}")
            })
        })
}

fn ensure_lookup_size<E>(lookup: &Lookup<E>, size: usize) -> anyhow::Result<()> {
    if lookup.inputs().len() != size {
        anyhow::bail!(
            "Lookup mode expects {size} inputs but got {}",
            lookup.inputs().len()
        );
    }
    Ok(())
}

fn reuse_temp_or_create<E>(expr: &ExprOrTemp<E>, temps: &mut Temps) -> (Temp, bool) {
    match expr {
        ExprOrTemp::Temp(temp) => (temp.clone(), true),
        ExprOrTemp::Expr(_) => (temps.next().unwrap(), false),
    }
}
