use mdnt_extractor_core::fields::{Blstrs as F, Jubjub as C};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::HashToCurveInstructions as _,
    types::{AssignedNative, AssignedNativePoint},
};
use midnight_proofs::plonk::Error;

use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::hash_to_curve::HtcAdaptor, entry};

entry!(
    "hash-to-curve/hash_to_curve_1/hash-to-curve/native",
    hash_to_curve_native::<1>
);
entry!(
    "hash-to-curve/hash_to_curve_10/hash-to-curve/native",
    hash_to_curve_native::<10>
);
#[harness(range_lookup(8))]
pub fn hash_to_curve_native<const N: usize>(
    chip: &HtcAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    inputs: [AssignedNative<F>; N],
) -> Result<AssignedNativePoint<C>, Error> {
    chip.htc().hash_to_curve(layouter, &inputs)
}
