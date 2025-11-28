use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("equality/is_equal/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn is_equal_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_native_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn is_equal_native_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedByte<F>, AssignedByte<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal_to_fixed/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, L<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.0)
}

#[entry("equality/is_equal_to_fixed/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedByte<F>, u8),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y)
}

#[entry("equality/is_equal_to_true/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_true(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, true)
}

#[entry("equality/is_equal_to_false/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_bit_false(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, false)
}
