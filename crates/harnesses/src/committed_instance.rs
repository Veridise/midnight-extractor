use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_macros::{entry, harness, harness_with_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    field::NativeChip,
    instructions::public_input::CommittedInstanceInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("committed-instance/constrain_as_committed_public_input/native/native")]
#[harness]
pub fn constrain_as_committed_public_input_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

#[entry("committed-instance/constrain_as_committed_public_input/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn constrain_as_committed_public_input_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

#[entry("committed-instance/constrain_as_committed_public_input/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn constrain_as_committed_public_input_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

#[entry("committed-instance/constrain_as_committed_public_input/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn constrain_as_committed_public_input_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

fn constrain_as_committed_public_input_stdlib_args() -> usize {
    8
}
#[entry("committed-instance/constrain_as_committed_public_input/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_committed_public_input_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

fn constrain_as_committed_public_input_stdlib_bit_args() -> usize {
    8
}
#[entry("committed-instance/constrain_as_committed_public_input/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_committed_public_input_stdlib_bit(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}

fn constrain_as_committed_public_input_stdlib_byte_args() -> usize {
    8
}
#[entry("committed-instance/constrain_as_committed_public_input/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn constrain_as_committed_public_input_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
) -> Result<(), Error> {
    chip.constrain_as_committed_public_input(layouter, &x)
}
