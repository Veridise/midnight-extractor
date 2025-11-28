use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{AF, FC};
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ZeroInstructions as _, midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;
pub type K = mdnt_extractor_core::fields::MidnightFp;

#[entry("zero/assert_zero/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_zero_field_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[entry("zero/assert_non_zero/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_non_zero_field_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[entry("zero/is_zero/field/field")]
#[harness(range_lookup(8))]
pub fn is_zero_field_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
