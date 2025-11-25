use ff::PrimeField;
use midnight_circuits::{
    instructions::BinaryInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::AssignedBit,
};

use super::NativeGadgetAdaptor;

impl<F, N> BinaryInstructions<F> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: BinaryInstructions<F>,
{
    fn and(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.and(layouter, bits)
    }

    fn or(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.or(layouter, bits)
    }

    fn xor(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.xor(layouter, bits)
    }

    fn not(
        &self,
        layouter: &mut impl Layouter<F>,
        bit: &AssignedBit<F>,
    ) -> Result<AssignedBit<F>, Error> {
        self.inner.not(layouter, bit)
    }
}
