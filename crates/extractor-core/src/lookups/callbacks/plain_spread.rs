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
pub struct PlainSpreadLookup<F: Field> {
    range_check: TagRangeLookup<F, 1, 2>,
    spread_module: &'static str,
    unspread_module: &'static str,
}

impl<F: PrimeField> PlainSpreadLookup<F> {
    pub fn new(spread_module: &'static str, unspread_module: &'static str) -> Self {
        // Lengths taken from the Sha256Chip.
        let lengths: [u32; 10] = [2, 3, 4, 5, 6, 7, 9, 10, 11, 12];
        let table =
            std::iter::once(([F::ZERO], [F::ONE, F::ONE])).chain(lengths.into_iter().map(|n| {
                (
                    [F::from(n as u64)],
                    [F::from(1 << n as u64), F::from(spread_bound(n))],
                )
            }));

        Self {
            spread_module,
            unspread_module,
            range_check: TagRangeLookup::new([0], [1, 2], table),
        }
    }
}

fn spread_bound(n: u32) -> u64 {
    ((1 << (2 * n)) - 1) / 3 + 1
}

#[cfg(test)]
mod tests {
    //! Tests to double check the formula for the spread bound is correct.

    #[test]
    fn test_2_to_the_11th() {
        let n = 11u32;
        assert_eq!(2048, 1 << n);
        assert_eq!(1398102, super::spread_bound(n));
    }

    #[test]
    fn test_2_to_the_10th() {
        let n = 10u32;
        assert_eq!(1024, 1 << n);
        assert_eq!(349526, super::spread_bound(n));
    }
}

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for PlainSpreadLookup<F> {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        table: &dyn LookupTableGenerator<F>,
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        let [plain, spread] = self.range_check.value_exprs(&lookup);
        let range_check_ir = self.range_check.on_lookup(lookup, table, temps)?;
        let temp = temps.next().ok_or_else(|| unreachable!())?;
        let spread_call = IRStmt::call(
            self.spread_module,
            [ExprOrTemp::Expr(Cow::Borrowed(plain))],
            [temp.into()],
        );
        let spread_out_constr = IRStmt::constraint(
            CmpOp::Eq,
            ExprOrTemp::Expr(Cow::Borrowed(spread)),
            ExprOrTemp::Temp(temp),
        );
        let temp = temps.next().ok_or_else(|| unreachable!())?;
        let unspread_call = IRStmt::call(
            self.unspread_module,
            [ExprOrTemp::Expr(Cow::Borrowed(spread))],
            [temp.into()],
        );
        let unspread_out_constr = IRStmt::constraint(
            CmpOp::Eq,
            ExprOrTemp::Expr(Cow::Borrowed(plain)),
            ExprOrTemp::Temp(temp),
        );

        Ok(IRStmt::seq([
            range_check_ir,
            spread_call,
            spread_out_constr,
            unspread_call,
            unspread_out_constr,
        ]))
    }

    fn on_lookups<'syn>(
        &self,
        lookups: &[&'syn Lookup<Expression<F>>],
        tables: &[&dyn LookupTableGenerator<F>],
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        if lookups.len() != 2 {
            anyhow::bail!(
                "Unexpected input. Was expecting two lookups but got {}",
                lookups.len()
            );
        }
        let per_lookup_ir = lookups
            .iter()
            .zip(tables.iter())
            .map(|(lookup, table)| self.on_lookup(lookup, *table, temps))
            .collect::<Result<IRStmt<_>, _>>()?;
        let fst_spread = self.range_check.value_exprs(&lookups[0])[1];
        let snd_spread = self.range_check.value_exprs(&lookups[1])[1];

        use IRBexpr::*;
        let det_axioms1 = IRStmt::assert(Implies(
            Box::new(Det(Cow::Owned(
                snd_spread + Expression::Constant(F::from(2u64)) * fst_spread,
            ))),
            Box::new(And(vec![
                Det(Cow::Borrowed(snd_spread)),
                Det(Cow::Borrowed(fst_spread)),
            ])),
        ))
        .map(&ExprOrTemp::Expr);
        let det_axioms2 = IRStmt::assert(Implies(
            Box::new(Det(Cow::Owned(
                fst_spread + Expression::Constant(F::from(2u64)) * snd_spread,
            ))),
            Box::new(And(vec![
                Det(Cow::Borrowed(snd_spread)),
                Det(Cow::Borrowed(fst_spread)),
            ])),
        ))
        .map(&ExprOrTemp::Expr);
        Ok(IRStmt::seq([per_lookup_ir, det_axioms1, det_axioms2]))
    }
}
