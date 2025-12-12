use std::borrow::Cow;

use ff::{Field, PrimeField};
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::{cmp::CmpOp, expr::IRBexpr, stmt::IRStmt};
use midnight_proofs::plonk::Expression;

use crate::lookups::callbacks::range::TagRangeLookup;

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
pub struct PlainSpreadLookup3 {
    spread_module: &'static str,
    unspread_module: &'static str,
}

impl PlainSpreadLookup3 {
    pub fn new(spread_module: &'static str, unspread_module: &'static str) -> Self {
        Self {
            spread_module,
            unspread_module,
        }
    }

    fn spread<'syn, F>(&self, lookup: &'syn Lookup<Expression<F>>) -> &'syn Expression<F> {
        &lookup.inputs()[2]
    }

    fn dense<'syn, F>(&self, lookup: &'syn Lookup<Expression<F>>) -> &'syn Expression<F> {
        &lookup.inputs()[1]
    }

    fn spread_call<'syn, F: Clone>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        let temp = temps.next().unwrap();
        let spread_call = IRStmt::call(
            self.spread_module,
            [ExprOrTemp::Expr(Cow::Borrowed(self.dense(lookup)))],
            [temp.into()],
        );
        let spread_out_constr = IRStmt::eq(
            ExprOrTemp::Expr(Cow::Borrowed(self.spread(lookup))),
            ExprOrTemp::Temp(temp),
        );

        IRStmt::seq([spread_call, spread_out_constr])
    }

    fn unspread_call<'syn, F: Clone>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        temps: &mut Temps,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        let temp = temps.next().unwrap();
        let unspread_call = IRStmt::call(
            self.unspread_module,
            [ExprOrTemp::Expr(Cow::Borrowed(self.spread(lookup)))],
            [temp.into()],
        );
        let unspread_out_constr = IRStmt::eq(
            ExprOrTemp::Expr(Cow::Borrowed(self.dense(lookup))),
            ExprOrTemp::Temp(temp),
        );

        IRStmt::seq([unspread_call, unspread_out_constr])
    }

    fn ensure_tag_is_8<F: PrimeField>(&self, lookup: &Lookup<Expression<F>>) -> anyhow::Result<()> {
        // Evaluate the expression assuming selectors are on.
        let folded = lookup.inputs()[0].evaluate(
            &|f| Some(f),
            &|selector| Some(F::ONE),
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
            .ok_or_else(|| anyhow::anyhow!("Was expecting a constant expression"))
            .and_then(|f| {
                (f == F::from(8)).then_some(()).ok_or_else(|| {
                    anyhow::anyhow!("Was expecting tag to be equal to 8 but got {f:?}")
                })
            })
    }

    fn inequalities<'syn, F: PrimeField>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
    ) -> IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>> {
        IRStmt::seq([
            // x < 2^8
            IRStmt::lt(
                ExprOrTemp::Expr(Cow::Borrowed(self.dense(lookup))),
                ExprOrTemp::Expr(Cow::Owned(Expression::from(256))),
            ),
            // ~x <= (8^8 - 1)/7
            IRStmt::le(
                ExprOrTemp::Expr(Cow::Borrowed(self.spread(lookup))),
                ExprOrTemp::Expr(Cow::Owned(Expression::from((16_777_216 - 1) / 7))),
            ),
        ])
    }
}

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for PlainSpreadLookup3 {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        table: &dyn LookupTableGenerator<F>,
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        self.ensure_tag_is_8(lookup)?;
        let spread_call = self.spread_call(lookup, temps);
        let unspread_call = self.unspread_call(lookup, temps);
        let inequalities = self.inequalities(lookup);

        Ok(IRStmt::seq([inequalities, spread_call, unspread_call]))
    }
}
