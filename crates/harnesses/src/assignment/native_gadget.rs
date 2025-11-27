use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};
use midnight_proofs::circuit::Value;

#[entry("assignment/assign/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assign_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign(layouter, x.value().copied())
}

#[entry("assignment/assign/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn assign_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[entry("assignment/assign/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn assign_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[entry("assignment/assign_fixed/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assign_fixed_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: L<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_fixed(layouter, x.0)
}

#[entry("assignment/assign_fixed/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn assign_fixed_native_gadget_bit(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_fixed(layouter, x)
}

#[entry("assignment/assign_fixed/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn assign_fixed_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: u8,
) -> Result<AssignedByte<F>, Error> {
    chip.assign_fixed(layouter, x)
}
