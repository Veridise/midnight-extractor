use crate::{utils::range_lookup, utils::vec2array};
use mdnt_extractor_macros::{entry, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::PublicInputInstructions as _,
    types::{AssignedBit, AssignedByte, AssignedNative},
};
use midnight_proofs::{circuit::Value, plonk::Error};

type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("public-input/as_public_input/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn as_public_input_native(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[usize_args(8)]
#[entry("public-input/constrain_as_public_input/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_public_input_native(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[usize_args(8)]
#[entry("public-input/assign_as_public_input/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_as_public_input_native(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_as_public_input(layouter, assigned.value().copied())
}

#[usize_args(8)]
#[entry("public-input/as_public_input/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn as_public_input_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedBit<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[usize_args(8)]
#[entry("public-input/constrain_as_public_input/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_public_input_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedBit<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[usize_args(8)]
#[entry("public-input/assign_as_public_input/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_as_public_input_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_as_public_input(layouter, Value::known(assigned))
}

#[usize_args(8)]
#[entry("public-input/as_public_input/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn as_public_input_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedByte<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[usize_args(8)]
#[entry("public-input/constrain_as_public_input/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_public_input_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedByte<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[usize_args(8)]
#[entry("public-input/assign_as_public_input/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_as_public_input_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    assigned: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign_as_public_input(layouter, Value::known(assigned))
}
