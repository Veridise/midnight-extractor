use crate::{utils::range_lookup, utils::vec2array};
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::PublicInputInstructions as _,
    types::{AssignedBit, AssignedByte, AssignedNative},
};
use midnight_proofs::{circuit::Value, plonk::Error};

type F = mdnt_extractor_core::fields::Blstrs;

#[entry("public-input/as_public_input/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn as_public_input_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_as_public_input(layouter, assigned.value().copied())
}

#[entry("public-input/as_public_input/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn as_public_input_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedBit<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedBit<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_as_public_input(layouter, Value::known(assigned))
}

#[entry("public-input/as_public_input/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn as_public_input_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedByte<F>,
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedByte<F>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    assigned: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign_as_public_input(layouter, Value::known(assigned))
}
