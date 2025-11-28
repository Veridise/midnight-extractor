use crate::utils::range_lookup;
use mdnt_extractor_core::{cells::load::NonZero, chips::NG};
use mdnt_extractor_macros::{entry, harness, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib, instructions::DivisionInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedNative,
};
use num_bigint::BigUint;

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("division/div_rem/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn div_rem_with_bound_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (dividend, divisor, dividend_bound): (AssignedNative<F>, NonZero<BigUint>, BigUint),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.div_rem(layouter, &dividend, divisor.0, Some(dividend_bound))
}

#[entry("division/div_rem_without_bound/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn div_rem_without_bound_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (dividend, divisor): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.div_rem(layouter, &dividend, divisor.0, None)
}

#[entry("division/rem/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn rem_with_bound_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (input, modulus, input_bound): (AssignedNative<F>, NonZero<BigUint>, BigUint),
) -> Result<AssignedNative<F>, Error> {
    chip.rem(layouter, &input, modulus.0, Some(input_bound))
}

#[entry("division/rem_without_bound/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn rem_without_bound_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (input, modulus): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<AssignedNative<F>, Error> {
    chip.rem(layouter, &input, modulus.0, None)
}

#[usize_args(8)]
#[entry("division/div_rem/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn div_rem_with_bound_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (dividend, divisor, dividend_bound): (AssignedNative<F>, NonZero<BigUint>, BigUint),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.div_rem(layouter, &dividend, divisor.0, Some(dividend_bound))
}

#[usize_args(8)]
#[entry("division/div_rem_without_bound/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn div_rem_without_bound_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (dividend, divisor): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.div_rem(layouter, &dividend, divisor.0, None)
}

#[usize_args(8)]
#[entry("division/rem/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn rem_with_bound_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (input, modulus, input_bound): (AssignedNative<F>, NonZero<BigUint>, BigUint),
) -> Result<AssignedNative<F>, Error> {
    chip.rem(layouter, &input, modulus.0, Some(input_bound))
}

#[usize_args(8)]
#[entry("division/rem_without_bound/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn rem_without_bound_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (input, modulus): (AssignedNative<F>, NonZero<BigUint>),
) -> Result<AssignedNative<F>, Error> {
    chip.rem(layouter, &input, modulus.0, None)
}
