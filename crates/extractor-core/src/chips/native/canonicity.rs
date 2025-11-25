use ff::PrimeField;
use midnight_circuits::{
    instructions::CanonicityInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::Error,
    },
    types::AssignedBit,
};
use num_bigint::BigUint;

use super::NativeGadgetAdaptor;

impl<F, N> CanonicityInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: CanonicityInstructions<F, AssignedCell<F, F>>,
{
    fn le_bits_lower_than(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
        bound: BigUint,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[le_bits_lower_than] Possible hijack!");
        self.inner.le_bits_lower_than(layouter, bits, bound)
    }

    fn le_bits_geq_than(
        &self,
        layouter: &mut impl Layouter<F>,
        bits: &[AssignedBit<F>],
        bound: BigUint,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[le_bits_geq_than] Possible hijack!");
        self.inner.le_bits_geq_than(layouter, bits, bound)
    }
}
