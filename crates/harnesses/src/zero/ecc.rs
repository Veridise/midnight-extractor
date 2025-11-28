use crate::utils::range_lookup;
use mdnt_extractor_core::chips::ecc::EccChipAdaptor;
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ZeroInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNativePoint},
};

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("zero/assert_zero/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_zero_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[entry("zero/assert_non_zero/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_non_zero_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[entry("zero/is_zero/ecc/point")]
#[harness(range_lookup(8))]
pub fn is_zero_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNativePoint<C>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
