use ff::PrimeField;
use midnight_circuits::{
    instructions::DecompositionInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::{AssignedBit, AssignedByte, AssignedNative},
};

use super::NativeGadgetAdaptor;

impl<F, N> DecompositionInstructions<F, AssignedNative<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: DecompositionInstructions<F, AssignedNative<F>>,
{
    fn assigned_to_le_bits(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bits: Option<usize>,
        enforce_canonical: bool,
    ) -> Result<Vec<AssignedBit<F>>, Error> {
        self.inner.assigned_to_le_bits(layouter, x, nb_bits, enforce_canonical)
    }

    fn assigned_to_le_bytes(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bytes: Option<usize>,
    ) -> Result<Vec<AssignedByte<F>>, Error> {
        self.inner.assigned_to_le_bytes(layouter, x, nb_bytes)
    }

    fn assigned_to_le_chunks(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        nb_bits_per_chunk: usize,
        nb_chunks: Option<usize>,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        self.inner.assigned_to_le_chunks(layouter, x, nb_bits_per_chunk, nb_chunks)
    }
}
