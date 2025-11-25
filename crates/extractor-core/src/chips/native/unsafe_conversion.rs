use ff::PrimeField;
use midnight_circuits::{
    instructions::UnsafeConversionInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::{AssignedByte, AssignedNative},
};

use super::NativeGadgetAdaptor;

impl<F, N> UnsafeConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
    for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: UnsafeConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>,
{
    fn convert_unsafe(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
    ) -> Result<AssignedByte<F>, Error> {
        self.inner.convert_unsafe(layouter, x)
    }
}
