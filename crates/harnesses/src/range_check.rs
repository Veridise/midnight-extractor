use crate::utils::range_lookup;
use mdnt_extractor_core::{
    cells::{load::NonZero, store::FreshVar},
    chips::NG,
};
use mdnt_extractor_macros::{entry, harness, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib, instructions::RangeCheckInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedNative,
};
use num_bigint::BigUint;

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("range-check/assign_lower_than_fixed/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assign_lower_than_fixed_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cell, bound): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<FreshVar, Error> {
    let value = cell.value();
    let assigned = chip.assign_lower_than_fixed(layouter, value.copied(), &bound.0)?;
    layouter.assign_region(
        || "assign_lower_than_fixed_native_gadget eq constraint",
        |mut region| region.constrain_equal(cell.cell(), assigned.cell()),
    )?;

    Ok(FreshVar)
}

#[entry("range-check/assert_lower_than_fixed/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assert_lower_than_fixed_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (value, bound): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<FreshVar, Error> {
    chip.assert_lower_than_fixed(layouter, &value, &bound.0)?;

    Ok(FreshVar)
}

#[usize_args(8)]
#[entry("range-check/assign_lower_than_fixed/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_lower_than_fixed_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cell, bound): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<FreshVar, Error> {
    let value = cell.value();
    let assigned = chip.assign_lower_than_fixed(layouter, value.copied(), &bound.0)?;
    layouter.assign_region(
        || "assign_lower_than_fixed_stdlib eq constraint",
        |mut region| region.constrain_equal(cell.cell(), assigned.cell()),
    )?;

    Ok(FreshVar)
}

#[usize_args(8)]
#[entry("range-check/assert_lower_than_fixed/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assert_lower_than_fixed_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (value, bound): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<FreshVar, Error> {
    chip.assert_lower_than_fixed(layouter, &value, &bound.0)?;

    Ok(FreshVar)
}
