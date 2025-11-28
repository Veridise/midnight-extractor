use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{AF, FC, NG};
use mdnt_extractor_macros::{
    entry, harness, harness_with_args, unit_harness, unit_harness_with_args, usize_args,
};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    field::NativeChip,
    instructions::FieldInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;
pub type K = mdnt_extractor_core::fields::MidnightFp;

#[entry("field/assert_qr/native/native")]
#[unit_harness]
pub fn assert_qr_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_qr(layouter, &x)
}

#[entry("field/is_square/native/native")]
#[harness]
pub fn is_square_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_square(layouter, &x)
}

#[entry("field/assert_qr/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn assert_qr_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_qr(layouter, &x)
}

#[entry("field/is_square/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn is_square_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_square(layouter, &x)
}

#[entry("field/assert_qr/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_qr_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_qr(layouter, &x)
}

#[entry("field/is_square/field/field")]
#[harness(range_lookup(8))]
pub fn is_square_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_square(layouter, &x)
}

#[usize_args(8)]
#[entry("field/assert_qr/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_qr_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNative<F>,
) -> Result<(), Error> {
    chip.assert_qr(layouter, &x)
}

#[usize_args(8)]
#[entry("field/is_square/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn is_square_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_square(layouter, &x)
}
