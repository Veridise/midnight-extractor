use crate::utils::range_lookup;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::{hash_to_curve::MapToCurveInstructions as _, native::EccChip},
    types::{AssignedNative, AssignedNativePoint},
};
use midnight_proofs::plonk::Error;

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("map-to-curve/map_to_curve/ecc/point")]
#[harness(range_lookup(8))]
pub fn map_to_curve(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNative<F>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.map_to_curve(layouter, &assigned)
}
