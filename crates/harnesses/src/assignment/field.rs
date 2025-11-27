use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{AF, FC};
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L, MidnightFp as K};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, InnerValue},
};
use midnight_proofs::circuit::Value;

#[entry("assignment/assign/field/field")]
#[harness(range_lookup(8))]
pub fn assign_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.assign(layouter, x.value())
}

#[entry("assignment/assign/field/bit")]
#[harness(range_lookup(8))]
pub fn assign_field_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[entry("assignment/assign_fixed/field/field")]
#[harness(range_lookup(8))]
pub fn assign_fixed_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: L<K>,
) -> Result<AF<F, K>, Error> {
    chip.assign_fixed(layouter, x.0)
}

#[entry("assignment/assign_fixed/field/bit")]
#[harness(range_lookup(8))]
pub fn assign_fixed_field_bit(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_fixed(layouter, x)
}
