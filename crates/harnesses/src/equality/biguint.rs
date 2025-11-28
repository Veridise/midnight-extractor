use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::BG, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    biguint::extraction::LoadedBigUint, instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedBit,
};
use num_bigint::BigUint;

pub type F = mdnt_extractor_core::fields::Blstrs;

entry!(
    "equality/is_equal_64bits/biguint/biguint",
    is_equal_biguint::<64>
);
entry!(
    "equality/is_equal_300bits/biguint/biguint",
    is_equal_biguint::<300>
);
#[harness(range_lookup(8))]
pub fn is_equal_biguint<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x.into(), &y.into())
}

entry!(
    "equality/is_equal_to_fixed_64bits/biguint/biguint",
    is_equal_to_fixed_biguint::<64>
);
entry!(
    "equality/is_equal_to_fixed_300bits/biguint/biguint",
    is_equal_to_fixed_biguint::<300>
);
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_biguint<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x.into(), y)
}
