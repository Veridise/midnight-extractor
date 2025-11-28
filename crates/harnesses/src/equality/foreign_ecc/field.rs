use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Secp256k1 as G;
use mdnt_extractor_core::{
    cells::load::LoadedSecp256k1,
    chips::{Afp, Fecf},
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::EqualityInstructions as _, midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("equality/is_equal/foreign-ecc-field/point")]
#[harness(range_lookup(8))]
pub fn is_equal_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    (x, y): (Afp<F, G>, Afp<F, G>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal_to_fixed/foreign-ecc-field/point")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    (x, y): (Afp<F, G>, LoadedSecp256k1),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.into())
}
