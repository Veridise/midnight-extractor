use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{Afp, Fecn};
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ControlFlowInstructions, midnight_proofs::plonk::Error, types::AssignedBit,
};

use mdnt_extractor_core::fields::G1 as G;
type F = mdnt_extractor_core::fields::Blstrs;

#[entry("control-flow/select/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn select_native_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, Afp<F, G>, Afp<F, G>),
) -> Result<Afp<F, G>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_assert_equal/foreign-ecc-native/point")]
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_native_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, Afp<F, G>),
    y: Afp<F, G>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn cond_swap_native_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, Afp<F, G>, Afp<F, G>),
) -> Result<(Afp<F, G>, Afp<F, G>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}
