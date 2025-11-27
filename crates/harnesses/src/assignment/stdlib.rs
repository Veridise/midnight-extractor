use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};
use mdnt_extractor_macros::{entry, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};
use midnight_proofs::circuit::Value;

#[usize_args(8)]
#[entry("assignment/assign/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign(layouter, x.value().copied())
}

#[usize_args(8)]
#[entry("assignment/assign/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[usize_args(8)]
#[entry("assignment/assign/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[usize_args(8)]
#[entry("assignment/assign_fixed/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_fixed_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: L<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_fixed(layouter, x.0)
}

#[usize_args(8)]
#[entry("assignment/assign_fixed/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_fixed_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_fixed(layouter, x)
}

#[usize_args(8)]
#[entry("assignment/assign_fixed/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_fixed_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign_fixed(layouter, x)
}
