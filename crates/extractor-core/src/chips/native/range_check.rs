use ff::PrimeField;
use midnight_circuits::{
    instructions::{AssignmentInstructions, RangeCheckInstructions},
    midnight_proofs::{
        circuit::{Layouter, Value},
        plonk::Error,
    },
    types::{AssignedNative, InnerValue},
};
use num_bigint::BigUint;

use crate::utils::injectable_less_than;
use mdnt_support::big_to_fe;

use super::NativeGadgetAdaptor;

impl<F, N> RangeCheckInstructions<F, AssignedNative<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: RangeCheckInstructions<F, AssignedNative<F>> + AssignmentInstructions<F, AssignedNative<F>>,
{
    fn assign_lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        value: Value<<AssignedNative<F> as InnerValue>::Element>,
        bound: &BigUint,
    ) -> Result<AssignedNative<F>, Error> {
        let inner = self.inner.assign_lower_than_fixed(layouter, value, bound)?;
        let (region, stmt) = injectable_less_than(inner.cell(), big_to_fe::<F>(bound.clone()))?;
        log::info!("Injected stmt {stmt:?} in region {}", *region);
        self.inject(region, stmt);
        Ok(inner)
    }

    fn assert_lower_than_fixed(
        &self,
        _layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
        bound: &BigUint,
    ) -> Result<(), Error> {
        let (region, stmt) = injectable_less_than(x.cell(), big_to_fe::<F>(bound.clone()))?;
        log::info!("Injected stmt {stmt:?} in region {}", *region);
        self.inject(region, stmt);
        Ok(())
    }
}
