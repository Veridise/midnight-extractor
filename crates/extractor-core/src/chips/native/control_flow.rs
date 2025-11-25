use ff::PrimeField;
use midnight_circuits::{
    instructions::ControlFlowInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::Error,
    },
    types::AssignedBit,
};

use super::NativeGadgetAdaptor;

impl<F, N> ControlFlowInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ControlFlowInstructions<F, AssignedCell<F, F>>,
{
    fn select(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &AssignedCell<F, F>,
        y: &AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.select(layouter, cond, x, y)
    }
}

impl<F, N> ControlFlowInstructions<F, AssignedBit<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ControlFlowInstructions<F, AssignedBit<F>>,
{
    fn select(
        &self,
        layouter: &mut impl Layouter<F>,
        cond: &AssignedBit<F>,
        x: &AssignedBit<F>,
        y: &AssignedBit<F>,
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.select(layouter, cond, x, y)
    }
}
