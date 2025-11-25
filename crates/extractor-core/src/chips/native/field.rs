use ff::PrimeField;
use midnight_circuits::{instructions::FieldInstructions, types::AssignedNative};
use num_bigint::BigUint;

use super::NativeGadgetAdaptor;

impl<F, N> FieldInstructions<F, AssignedNative<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: FieldInstructions<F, AssignedNative<F>>,
{
    fn order(&self) -> BigUint {
        self.inner.order()
    }
}
