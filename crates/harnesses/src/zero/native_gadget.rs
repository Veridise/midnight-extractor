use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ZeroInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("zero/assert_zero/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_zero_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[entry("zero/assert_non_zero/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_non_zero_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[entry("zero/is_zero/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn is_zero_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
