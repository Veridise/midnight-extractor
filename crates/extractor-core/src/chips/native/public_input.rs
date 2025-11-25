use ff::PrimeField;
use midnight_circuits::{
    instructions::PublicInputInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter, Value},
        plonk::Error,
    },
    types::{AssignedNative, InnerValue},
};

use super::NativeGadgetAdaptor;

impl<F, N> PublicInputInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: PublicInputInstructions<F, AssignedCell<F, F>>,
{
    fn as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        assigned: &AssignedCell<F, F>,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        self.inner.as_public_input(layouter, assigned)
    }

    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        assigned: &AssignedCell<F, F>,
    ) -> Result<(), Error> {
        self.inner.constrain_as_public_input(layouter, assigned)
    }

    fn assign_as_public_input(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<<AssignedCell<F, F> as InnerValue>::Element>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.assign_as_public_input(layouter, value)
    }
}
