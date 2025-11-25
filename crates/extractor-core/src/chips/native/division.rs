use ff::PrimeField;
use midnight_circuits::{
    instructions::DivisionInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::AssignedNative,
};
use num_bigint::BigUint;

use super::NativeGadgetAdaptor;

impl<F, N> DivisionInstructions<F, AssignedNative<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: DivisionInstructions<F, AssignedNative<F>>,
{
    fn div_rem(
        &self,
        layouter: &mut impl Layouter<F>,
        dividend: &AssignedNative<F>,
        bound: BigUint,
        divisor: Option<BigUint>,
    ) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
        self.inner.div_rem(layouter, dividend, bound, divisor)
    }
}
