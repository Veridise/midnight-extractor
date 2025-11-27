use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};
use mdnt_extractor_macros::{entry, unit_harness};
use midnight_circuits::{
    field::NativeChip,
    instructions::AssertionInstructions as _,
    types::{AssignedBit, AssignedNative},
};

#[entry("assertion/assert_equal/native/native")]
#[unit_harness]
pub fn assert_equal_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/native/native")]
#[unit_harness]
pub fn assert_not_equal_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/native/native")]
#[unit_harness]
pub fn assert_equal_to_fixed_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_not_equal_to_fixed/native/native")]
#[unit_harness]
pub fn assert_not_equal_to_fixed_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_equal/native/bit")]
#[unit_harness]
pub fn assert_equal_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/native/bit")]
#[unit_harness]
pub fn assert_not_equal_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/native/bit")]
#[unit_harness]
pub fn assert_equal_to_fixed_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_not_equal_to_fixed/native/bit")]
#[unit_harness]
pub fn assert_not_equal_to_fixed_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}
