use ff::PrimeField;
use midnight_circuits::{
    instructions::AssignmentInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter, Value},
        plonk::Error,
    },
    types::{AssignedBit, InnerValue},
};

use super::NativeGadgetAdaptor;

impl<F, N> AssignmentInstructions<F, AssignedBit<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: AssignmentInstructions<F, AssignedBit<F>>,
{
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<<AssignedBit<F> as InnerValue>::Element>,
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        constant: <AssignedBit<F> as InnerValue>::Element,
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.assign_fixed(layouter, constant)
    }
}

impl<F, N> AssignmentInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: AssignmentInstructions<F, AssignedCell<F, F>>,
{
    fn assign(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<<AssignedCell<F, F> as InnerValue>::Element>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        constant: <AssignedCell<F, F> as InnerValue>::Element,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.assign_fixed(layouter, constant)
    }
}
