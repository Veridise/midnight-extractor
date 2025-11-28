use crate::utils::range_lookup;
use mdnt_extractor_core::chips::ecc::EccChipAdaptor;
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ControlFlowInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNativePoint},
};

type C = mdnt_extractor_core::fields::Jubjub;
type F = mdnt_extractor_core::fields::Blstrs;

#[entry("control-flow/select/ecc/point")]
#[harness(range_lookup(8))]
pub fn select_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (
        AssignedBit<F>,
        AssignedNativePoint<C>,
        AssignedNativePoint<C>,
    ),
) -> Result<AssignedNativePoint<C>, Error> {
    chip.ecc().select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_assert_equal/ecc/point")]
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedNativePoint<C>),
    y: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.ecc().cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/ecc/point")]
#[harness(range_lookup(8))]
pub fn cond_swap_native_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (
        AssignedBit<F>,
        AssignedNativePoint<C>,
        AssignedNativePoint<C>,
    ),
) -> Result<(AssignedNativePoint<C>, AssignedNativePoint<C>), Error> {
    chip.ecc().cond_swap(layouter, &cond, &a, &b)
}
