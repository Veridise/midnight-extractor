use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Jubjub as C};
use mdnt_extractor_core::{cells::load::LoadedJubjubSubgroup, chips::ecc::EccChipAdaptor};
use mdnt_extractor_macros::{entry, unit_harness};
use midnight_circuits::{instructions::AssertionInstructions as _, types::AssignedNativePoint};

#[entry("assertion/assert_equal/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNativePoint<C>,
    y: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.ecc().assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNativePoint<C>,
    y: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    f: LoadedJubjubSubgroup,
    y: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.into())
}

#[entry("assertion/assert_not_equal_to_fixed/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    f: LoadedJubjubSubgroup,
    y: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.into())
}
