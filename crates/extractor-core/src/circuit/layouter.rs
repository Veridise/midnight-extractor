use ff::Field;
use haloumi::Synthesizer;
use mdnt_support::cells::ctx::LayoutAdaptor;
use midnight_proofs::{
    circuit::{groups, AssignedCell, Cell, Layouter, Region, Table, Value},
    plonk::{Challenge, Column, Error, Instance},
    ExtractionSupport,
};

pub struct ExtractionLayouter<'a, F: Field> {
    synthesizer: &'a mut Synthesizer<F>,
}

impl<'a, F: Field> ExtractionLayouter<'a, F> {
    pub fn new(synthesizer: &'a mut Synthesizer<F>) -> Self {
        Self { synthesizer }
    }
}

impl<F: Field> Layouter<F> for ExtractionLayouter<'_, F> {
    type Root = Self;

    fn assign_region<A, AR, N, NR>(&mut self, name: N, assignment: A) -> Result<AR, Error>
    where
        A: FnMut(Region<'_, F>) -> Result<AR, Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        todo!()
    }

    fn assign_table<A, N, NR>(&mut self, name: N, assignment: A) -> Result<(), Error>
    where
        A: FnMut(Table<'_, F>) -> Result<(), Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        todo!()
    }

    fn constrain_instance(
        &mut self,
        cell: Cell,
        column: Column<Instance>,
        row: usize,
    ) -> Result<(), Error> {
        todo!()
    }

    fn get_challenge(&self, challenge: Challenge) -> Value<F> {
        Value::unknown()
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        self.synthetizer.push_namespace(name_fn().into());
    }

    fn pop_namespace(&mut self, gadget_name: Option<String>) {
        self.synthetizer.pop_namespace(gadget_name);
    }

    fn push_group<N, NR, K>(&mut self, name: N, key: K)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
        K: groups::GroupKey,
    {
        self.synthetizer.enter_group(name().into(), key);
    }

    fn pop_group(&mut self, meta: groups::RegionsGroup) {
        self.synthetizer.exit_group(meta)
    }
}

/// Wrapper over [`Layouter`] that implements
/// [`LayoutAdaptor`](extractor_support::cells::ctx::LayoutAdaptor).
#[derive(Debug)]
pub struct AdaptsLayouter<'l, L> {
    layouter: &'l mut L,
}

impl<'l, L> AdaptsLayouter<'l, L> {
    /// Constructs a new wrapper.
    pub fn new(layouter: &'l mut L) -> Self {
        Self { layouter }
    }
}

impl<F: Field, L: Layouter<F>> LayoutAdaptor<F, ExtractionSupport> for AdaptsLayouter<L> {
    type Adaptee = L;

    fn adaptee_ref(&self) -> &L {
        self.layouter
    }

    fn adaptee_ref_mut(&mut self) -> &mut L {
        self.layouter
    }

    fn constrain_instance(
        &mut self,
        cell: Cell,
        instance_col: Column<Instance>,
        instance_row: usize,
    ) -> Result<(), Error> {
        self.layouter.constrain_instance(cell, instance_col, instance_row)
    }

    fn constrain_advice_constant(
        &mut self,
        advice_col: Column<Advice>,
        advice_row: usize,
        constant: F,
    ) -> Result<Cell, Error> {
        Ok(self
            .layouter
            .assign_region(
                || format!("Adv[{}, {advice_row}] == 0", advice_col.index()),
                |mut region| {
                    region.assign_advice_from_constant(
                        || format!("Adv[{}, {advice_row}]", advice_col.index()),
                        advice_col,
                        advice_row,
                        constant,
                    )
                },
            )?
            .cell())
    }

    fn assign_advice_from_instance(
        &mut self,
        advice_col: Column<Advice>,
        advice_row: usize,
        instance_col: Column<Instance>,
        instance_row: usize,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.layouter.assign_region(
            || "ins",
            |mut region| {
                region.assign_advice_from_instance(
                    || {
                        format!(
                            "Adv[{}, +{advice_row}] == Ins[{}, {instance_row}]",
                            advice_col.index(),
                            instance_col.index()
                        )
                    },
                    instance_col,
                    instance_row,
                    advice_col,
                    advice_row,
                )
            },
        )
    }

    fn copy_advice(
        &mut self,
        ac: &AssignedCell<F, F>,
        region: &mut Region<'_, F>,
        advice_col: Column<Advice>,
        advice_row: usize,
    ) -> Result<AssignedCell<F, F>, Error> {
        ac.copy_advice(|| "", region, advice_col, advice_row)
    }

    fn region<A, AR, N, NR>(&mut self, name: N, assignment: A) -> Result<AR, Error>
    where
        A: FnMut(Region<'_, F>) -> Result<AR, Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        self.layouter.assign_region(name, assignment)
    }
}
