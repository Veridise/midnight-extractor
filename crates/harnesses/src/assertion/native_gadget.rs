use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};
use mdnt_extractor_macros::{entry, unit_harness};
use midnight_circuits::{
    instructions::AssertionInstructions as _,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

#[entry("assertion/assert_equal/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_not_equal_to_fixed/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_equal/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: u8,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_not_equal_to_fixed/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: u8,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_equal/native-gadget/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/native-gadget/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/native-gadget/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_not_equal_to_fixed/native-gadget/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_equal_array_of_32/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_gadget_byte_array_of_32(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: [AssignedByte<F>; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal_array_of_32/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_gadget_byte_array_of_32(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: [AssignedByte<F>; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed_array_of_32/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_gadget_byte_array_of_32(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: [u8; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_not_equal_to_fixed_array_of_32/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_gadget_byte_array_of_32(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    f: [u8; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}
