use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    field::NativeChip,
    instructions::ControlFlowInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};

type F = mdnt_extractor_core::fields::Blstrs;

#[entry("control-flow/select/native/native")]
#[harness]
pub fn select_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}
#[entry("control-flow/cond_assert_equal/native/native")]
#[unit_harness]
pub fn cond_assert_equal_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedNative<F>),
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/native/native")]
#[harness]
pub fn cond_swap_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}

#[entry("control-flow/select/native/bit")]
#[harness]
pub fn select_bit_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_swap/native/bit")]
#[harness]
pub fn cond_swap_bit_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<(AssignedBit<F>, AssignedBit<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}
#[entry("control-flow/cond_assert_equal/native/bit")]
#[unit_harness]
pub fn cond_assert_equal_bit_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedBit<F>),
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}
