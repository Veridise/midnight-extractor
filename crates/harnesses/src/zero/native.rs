use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    field::NativeChip,
    instructions::ZeroInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("zero/assert_zero/native/native")]
#[unit_harness]
pub fn assert_zero_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[entry("zero/assert_non_zero/native/native")]
#[unit_harness]
pub fn assert_non_zero_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[entry("zero/is_zero/native/native")]
#[harness]
pub fn is_zero_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
