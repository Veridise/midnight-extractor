use crate::utils::range_lookup;
use mdnt_extractor_macros::{entry, harness_with_args, unit_harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::ControlFlowInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("control-flow/select/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn select_native_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[usize_args(8)]
#[entry("control-flow/cond_assert_equal/stdlib/native")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn cond_assert_equal_native_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedNative<F>),
    y: AssignedNative<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[usize_args(8)]
#[entry("control-flow/cond_swap/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn cond_swap_native_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}

#[usize_args(8)]
#[entry("control-flow/select/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn select_bit_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[usize_args(8)]
#[entry("control-flow/cond_assert_equal/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn cond_assert_equal_bit_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedBit<F>),
    y: AssignedBit<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[usize_args(8)]
#[entry("control-flow/cond_swap/stdlib/bit")]
#[harness_with_args(usize, range_lookup(8))]
pub fn cond_swap_bit_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedBit<F>, AssignedBit<F>),
) -> Result<(AssignedBit<F>, AssignedBit<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}

#[usize_args(8)]
#[entry("control-flow/select/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn select_byte_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedByte<F>, AssignedByte<F>),
) -> Result<AssignedByte<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}

#[usize_args(8)]
#[entry("control-flow/cond_assert_equal/stdlib/byte")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn cond_assert_equal_byte_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, AssignedByte<F>),
    y: AssignedByte<F>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x, &y)
}

#[usize_args(8)]
#[entry("control-flow/cond_swap/stdlib/byte")]
#[harness_with_args(usize, range_lookup(8))]
pub fn cond_swap_byte_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedByte<F>, AssignedByte<F>),
) -> Result<(AssignedByte<F>, AssignedByte<F>), Error> {
    chip.cond_swap(layouter, &cond, &a, &b)
}
