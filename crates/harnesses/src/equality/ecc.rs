use crate::utils::range_lookup;
use mdnt_extractor_core::cells::load::LoadedJubjubSubgroup;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::native::EccChip,
    instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNativePoint},
};

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type CS = LoadedJubjubSubgroup;
pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("equality/is_equal/ecc/point")]
#[harness(range_lookup(8))]
pub fn is_equal_native_ecc(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNativePoint<C>, AssignedNativePoint<C>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

#[entry("equality/is_equal_to_fixed/ecc/point")]
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_native_ecc(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNativePoint<C>, CS),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.into())
}
