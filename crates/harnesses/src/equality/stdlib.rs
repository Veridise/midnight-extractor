use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_macros::{entry, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};
pub type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("equality/is_equal/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_native(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[usize_args(8)]
#[entry("equality/is_equal/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_native_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[usize_args(8)]
#[entry("equality/is_equal/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_native_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedByte<F>, AssignedByte<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[usize_args(8)]
#[entry("equality/is_equal_to_fixed/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_to_fixed_native(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, L<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.0)
}

#[usize_args(8)]
#[entry("equality/is_equal_to_fixed/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_to_fixed_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedByte<F>, u8),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y)
}

#[usize_args(8)]
#[entry("equality/is_equal_to_true/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_true(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, true)
}

#[usize_args(8)]
#[entry("equality/is_equal_to_false/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_false(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, false)
}
