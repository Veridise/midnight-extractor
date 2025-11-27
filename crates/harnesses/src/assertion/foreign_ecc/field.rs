use mdnt_extractor_core::{
    cells::load::LoadedSecp256k1,
    chips::{Afp, Fecf},
};

use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Secp256k1 as G};
use mdnt_extractor_macros::{entry, unit_harness};
use midnight_circuits::instructions::AssertionInstructions as _;

#[entry("assertion/assert_equal/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    x: Afp<F, G>,
    y: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    x: Afp<F, G>,
    y: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    f: LoadedSecp256k1,
    y: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.into())
}

#[entry("assertion/assert_not_equal_to_fixed/foreign-ecc-field/point")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_field_ecc(
    chip: &Fecf<F, G>,
    layouter: &mut impl Layouter<F>,
    f: LoadedSecp256k1,
    y: Afp<F, G>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.into())
}
