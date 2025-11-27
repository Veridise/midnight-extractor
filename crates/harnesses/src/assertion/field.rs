use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{AF, FC};
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L, MidnightFp as K};
use mdnt_extractor_macros::{entry, unit_harness};
use midnight_circuits::{instructions::AssertionInstructions as _, types::AssignedBit};

#[entry("assertion/assert_equal/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
    y: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
    y: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    f: L<K>,
    y: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_not_equal_to_fixed/field/field")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    f: L<K>,
    y: AF<F, K>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.0)
}

#[entry("assertion/assert_equal/field/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

#[entry("assertion/assert_not_equal/field/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

#[entry("assertion/assert_equal_to_fixed/field/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f)
}

#[entry("assertion/assert_not_equal_to_fixed/field/bit")]
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    f: bool,
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f)
}
