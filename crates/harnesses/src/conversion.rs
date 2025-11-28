use crate::utils::range_lookup;
use mdnt_extractor_core::{
    chips::ecc::EccChipAdaptor,
    chips::{AF, FC, NG},
};
use mdnt_extractor_macros::{entry, harness, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    ecc::native::AssignedScalarOfNativeCurve as ScalarVar,
    field::NativeChip,
    instructions::ConversionInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type F = mdnt_extractor_core::fields::Blstrs;
pub type K = mdnt_extractor_core::fields::MidnightFp;

#[entry("conversion/convert_to_bit/native/native")]
#[harness]
pub fn convert_to_bit_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_native/native/bit")]
#[harness]
pub fn convert_to_native_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_bit/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn convert_to_bit_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_byte/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn convert_to_byte_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedByte<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_native/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn convert_to_native_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_native/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn convert_to_native_native_gadget_byte(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_field/field/bit")]
#[harness(range_lookup(8))]
pub fn convert_to_field_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AF<F, K>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_field/field/byte")]
#[harness(range_lookup(8))]
pub fn convert_to_field_byte(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
) -> Result<AF<F, K>, Error> {
    chip.convert(layouter, &x)
}

#[entry("conversion/convert_to_scalar/ecc/point")]
#[harness(range_lookup(8))]
pub fn convert_to_scalar(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<ScalarVar<C>, Error> {
    chip.ecc().convert(layouter, &x)
}

#[usize_args(8)]
#[entry("conversion/convert_to_bit/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn convert_to_bit_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.convert(layouter, &x)
}

#[usize_args(8)]
#[entry("conversion/convert_to_byte/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn convert_to_byte_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedByte<F>, Error> {
    chip.convert(layouter, &x)
}

#[usize_args(8)]
#[entry("conversion/convert_to_native/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn convert_to_native_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.convert(layouter, &x)
}

#[usize_args(8)]
#[entry("conversion/convert_to_native/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn convert_to_native_stdlib_byte(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedByte<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.convert(layouter, &x)
}
