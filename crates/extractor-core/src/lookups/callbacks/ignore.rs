use std::borrow::Cow;

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::stmt::IRStmt;
use midnight_proofs::plonk::Expression;

/// Lookup callback that emits an empty statement.
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct IgnoreLookup;

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for IgnoreLookup {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        _table: &dyn LookupTableGenerator<F>,
        _temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        log::debug!("Lookup {} \"{}\" was ignored.", lookup.idx(), lookup.name());
        Ok(IRStmt::empty())
    }
}
