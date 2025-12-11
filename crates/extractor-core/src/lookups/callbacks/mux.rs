use std::{borrow::Cow, collections::HashMap};

use ff::PrimeField;
use haloumi::{
    lookups::{table::LookupTableGenerator, Lookup},
    temps::{ExprOrTemp, Temps},
    LookupCallbacks,
};
use haloumi_ir::stmt::IRStmt;
use midnight_proofs::plonk::Expression;

pub trait LookupName: sealed::LookupNameSealed {
    fn check(&self, name: &str) -> bool;
}

mod sealed {
    pub trait LookupNameSealed {}

    impl LookupNameSealed for &str {}
    impl<T: Fn(&str) -> bool> LookupNameSealed for T {}
}

impl LookupName for &str {
    fn check(&self, name: &str) -> bool {
        *self == name
    }
}

impl<T: Fn(&str) -> bool> LookupName for T {
    fn check(&self, name: &str) -> bool {
        self(name)
    }
}

/// Stores several lookup callbacks and dispatches them based on the name of the lookup.
#[derive(Default)]
pub struct LookupMux<'a, F: PrimeField> {
    handlers: Vec<(
        Box<dyn LookupName + 'static>,
        Box<dyn LookupCallbacks<F, Expression<F>> + 'a>,
    )>,
    fallback: Option<Box<dyn LookupCallbacks<F, Expression<F>> + 'a>>,
}

impl<'a, F: PrimeField> LookupMux<'a, F> {
    pub fn with(
        mut self,
        name: impl LookupName + 'static,
        handler: impl LookupCallbacks<F, Expression<F>> + 'a,
    ) -> Self {
        self.handlers.push((Box::new(name), Box::new(handler)));
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
            .iter()
            .find_map(|(n, h)| n.check(name).then_some(h))
            .or(self.fallback.as_ref())
            .map(|b| b.as_ref())
            .ok_or_else(move || anyhow::anyhow!("Missing handler for lookup '{name}'"))
    }

    fn all_handlers(
        &self,
    ) -> impl Iterator<
        Item = (
            &(dyn LookupName + 'static),
            &(dyn LookupCallbacks<F, Expression<F>> + 'a),
        ),
    > {
        self.handlers
            .iter()
            .map(|(n, h)| (n.as_ref(), h.as_ref()))
            .chain(self.fallback.as_deref().map(|h| ((&|_: &str| true) as &dyn LookupName, h)))
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
        for l in lookups {
            log::info!("Lookup: '{}'", l.name());
        }
        let mut lookups = std::iter::zip(lookups.iter().copied(), tables.iter().copied())
            .map(Some)
            .collect::<Vec<_>>();
        let ir = self
            .all_handlers()
            .map(|(name, handler)| {
                let (selected_lookups, selected_tables): (Vec<_>, Vec<_>) = lookups
                    .iter_mut()
                    .filter_map(|l| l.take_if(|(l, _)| name.check(l.name())))
                    .unzip();

                handler.on_lookups(&selected_lookups, &selected_tables, temps)
            })
            .collect::<Result<IRStmt<_>, _>>()?;
        if lookups.iter().any(Option::is_some) {
            let names = lookups.iter().copied().flatten().map(|(l, _)| l.name()).fold(
                String::new(),
                |mut s, n| {
                    s.push_str(", ");
                    s.push_str(n);
                    s
                },
            );
            anyhow::bail!("Lookups {names} did not match any handler!");
        }
        Ok(ir)
    }
}

impl<F: PrimeField> std::fmt::Debug for LookupMux<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LookupMux handlers: {} (fallback: {})",
            self.handlers.len(),
            self.fallback.is_some()
        )
    }
}
