use crate::utils::range_lookup;
use mdnt_extractor_macros::{entry, harness_with_args, unit_harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::ZeroInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("zero/assert_zero/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_zero_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[usize_args(8)]
#[entry("zero/assert_non_zero/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_non_zero_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[usize_args(8)]
#[entry("zero/is_zero/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_zero_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
