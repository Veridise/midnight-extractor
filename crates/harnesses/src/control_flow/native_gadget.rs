use crate::utils::range_lookup;
use mdnt_extractor_core::chips::NG;
use mdnt_extractor_macros::{entry, harness, unit_harness};
use midnight_circuits::{
    instructions::ControlFlowInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

type F = mdnt_extractor_core::fields::Blstrs;

#[entry("control-flow/select/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn select_native_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_assert_equal/native-gadget/native")]
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_native_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedNative<F>),
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn cond_swap_native_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}

#[entry("control-flow/select/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn select_bit_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_assert_equal/native-gadget/bit")]
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_bit_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedBit<F>),
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/native-gadget/bit")]
#[harness(range_lookup(8))]
pub fn cond_swap_bit_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<(AssignedBit<F>, AssignedBit<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}

#[entry("control-flow/select/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn select_byte_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedByte<F>, AssignedByte<F>),
) -> Result<AssignedByte<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[entry("control-flow/cond_assert_equal/native-gadget/byte")]
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_byte_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedByte<F>),
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[entry("control-flow/cond_swap/native-gadget/byte")]
#[harness(range_lookup(8))]
pub fn cond_swap_byte_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedByte<F>, AssignedByte<F>),
) -> Result<(AssignedByte<F>, AssignedByte<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}
