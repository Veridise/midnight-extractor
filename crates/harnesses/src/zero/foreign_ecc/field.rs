use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{Afp, Fecf};
use mdnt_extractor_core::fields::Secp256k1 as G;
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ZeroInstructions as _, midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("zero/assert_zero/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_zero_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x)
}

#[entry("zero/assert_non_zero/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_non_zero_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x)
}

#[entry("zero/is_zero/foreign-ecc-field/point")]
#[harness(range_lookup(8))]
pub fn is_zero_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    x: Afp<F, G>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x)
}
