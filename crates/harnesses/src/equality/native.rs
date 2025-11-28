use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    field::NativeChip,
    instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("equality/is_equal/native/native")]
#[harness(range_lookup(8))]
pub fn is_equal_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal/native/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_native_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal_to_fixed/native/native")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, L<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.0)
}

#[entry("equality/is_equal_to_true/native/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_true(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, true)
}

#[entry("equality/is_equal_to_false/native/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_false(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, false)
}
