use std::{borrow::Cow, collections::HashMap};

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::stmt::IRStmt;
use midnight_proofs::plonk::Expression;

/// Stores several lookup callbacks and dispatches them based on the name of the lookup.
#[derive(Default)]
pub struct LookupMux<'a, F: PrimeField> {
    handlers: HashMap<&'static str, Box<dyn LookupCallbacks<F, Expression<F>> + 'a>>,
    fallback: Option<Box<dyn LookupCallbacks<F, Expression<F>> + 'a>>,
}

impl<'a, F: PrimeField> LookupMux<'a, F> {
    pub fn with(
        mut self,
        name: &'static str,
        handler: impl LookupCallbacks<F, Expression<F>> + 'a,
    ) -> Self {
        self.handlers.insert(name, Box::new(handler));
        self
    }

    pub fn fallback(mut self, handler: impl LookupCallbacks<F, Expression<F>> + 'a) -> Self {
        self.fallback = Some(Box::new(handler));
        self
    }

    fn handler_for<'s, 'n: 's>(
        &'s self,
        name: &'n str,
    ) -> anyhow::Result<&'s (dyn LookupCallbacks<F, Expression<F>> + 'a)> {
        self.handlers
            .get(&name)
            .or(self.fallback.as_ref())
            .map(|b| b.as_ref())
            .ok_or_else(move || anyhow::anyhow!("Missing handler for lookup '{name}'"))
    }
}

impl<F: PrimeField> LookupCallbacks<F, Expression<F>> for LookupMux<'_, F> {
    fn on_lookup<'syn>(
        &self,
        lookup: &'syn Lookup<Expression<F>>,
        table: &dyn LookupTableGenerator<F>,
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        self.handler_for(lookup.name())?.on_lookup(lookup, table, temps)
    }

    fn on_lookups<'syn>(
        &self,
        lookups: &[&'syn Lookup<Expression<F>>],
        tables: &[&dyn LookupTableGenerator<F>],
        temps: &mut Temps,
    ) -> anyhow::Result<IRStmt<ExprOrTemp<Cow<'syn, Expression<F>>>>> {
        let mut lookups_by_name: HashMap<&str, (Vec<_>, Vec<_>)> = HashMap::new();
        for (n, lookup) in lookups.iter().enumerate() {
            let e = lookups_by_name.entry(lookup.name()).or_default();
            e.0.push(lookups[n]);
            e.1.push(tables[n]);
        }

        lookups_by_name
            .into_iter()
            .map(|(name, (l, t))| {
                let handler = self.handler_for(name)?;
                handler.on_lookups(&l, &t, temps)
            })
            .collect()
    }
}

impl<F: PrimeField> std::fmt::Debug for LookupMux<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LookupMux handles {{ ")?;
        for name in self.handlers.keys() {
            write!(f, "\"{name}\" ")?;
        }
        write!(f, ", fallback: {} }}", self.fallback.is_some())
    }
}
