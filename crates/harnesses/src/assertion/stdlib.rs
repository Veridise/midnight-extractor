use mdnt_extractor_macros::{entry, unit_harness_with_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::AssertionInstructions as _,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};

fn assert_equal_stdlib_args() -> usize {
    8
}
#[entry("assertion/assert_equal/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

fn assert_not_equal_stdlib_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

fn assert_equal_to_fixed_stdlib_args() -> usize {
    8
}
#[entry("assertion/assert_equal_to_fixed/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_to_fixed_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.0)
}

fn assert_not_equal_to_fixed_stdlib_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal_to_fixed/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_to_fixed_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: L<F>,
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.0)
}

fn assert_equal_stdlib_byte_args() -> usize {
    8
}
#[entry("assertion/assert_equal/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

fn assert_not_equal_stdlib_byte_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

fn assert_equal_to_fixed_stdlib_byte_args() -> usize {
    8
}
#[entry("assertion/assert_equal_to_fixed/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_to_fixed_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: u8,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

fn assert_not_equal_to_fixed_stdlib_byte_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal_to_fixed/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_to_fixed_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: u8,
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}

fn assert_equal_stdlib_bit_args() -> usize {
    8
}
#[entry("assertion/assert_equal/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

fn assert_not_equal_stdlib_bit_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

fn assert_equal_to_fixed_stdlib_bit_args() -> usize {
    8
}
#[entry("assertion/assert_equal_to_fixed/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_to_fixed_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

fn assert_not_equal_to_fixed_stdlib_bit_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal_to_fixed/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_to_fixed_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}

fn assert_equal_stdlib_byte_array_of_32_args() -> usize {
    8
}
#[entry("assertion/assert_equal_array_of_32/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_stdlib_byte_array_of_32(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: [AssignedByte<F>; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

fn assert_not_equal_stdlib_byte_array_of_32_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal_array_of_32/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_stdlib_byte_array_of_32(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: [AssignedByte<F>; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

fn assert_equal_to_fixed_stdlib_byte_array_of_32_args() -> usize {
    8
}
#[entry("assertion/assert_equal_to_fixed_array_of_32/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_equal_to_fixed_stdlib_byte_array_of_32(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: [u8; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

fn assert_not_equal_to_fixed_stdlib_byte_array_of_32_args() -> usize {
    8
}
#[entry("assertion/assert_not_equal_to_fixed_array_of_32/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_not_equal_to_fixed_stdlib_byte_array_of_32(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    f: [u8; 32],
    y: [AssignedByte<F>; 32],
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}
