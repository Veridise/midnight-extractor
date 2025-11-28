use crate::utils::range_lookup;
use mdnt_extractor_core::fields::G1 as G;
use mdnt_extractor_core::{
    cells::load::LoadedG1,
    chips::{Afp, Fecn},
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::EqualityInstructions as _, midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("equality/is_equal/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn is_equal_native_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (x, y): (Afp<F, G>, Afp<F, G>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal_to_fixed/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (x, y): (Afp<F, G>, LoadedG1),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.into())
}
