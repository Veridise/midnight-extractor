use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_macros::{entry, harness};
#[allow(clippy::unsafe_removed_from_name)]
use midnight_circuits::{
    field::NativeChip,
    instructions::UnsafeConversionInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("unsafe-conversion/unsafe_convert_to_bit/native/native")]
#[harness]
pub fn convert_unsafe_to_bit_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.convert_unsafe(layouter, &x)
}

#[entry("unsafe-conversion/unsafe_convert_to_byte/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn convert_to_byte_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedByte<F>, Error> {
    chip.convert_unsafe(layouter, &x)
}
