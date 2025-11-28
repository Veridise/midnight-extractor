use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::NG, entry};
use mdnt_extractor_macros::{harness, harness_with_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib, field::NativeChip, instructions::BinaryInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedBit,
};

entry!("binary/not/native-gadget/bit", not_native_gadget);
entry!("binary/and_1/native-gadget/bit", and_native_gadget::<1>);
entry!("binary/or_1/native-gadget/bit", or_native_gadget::<1>);
entry!("binary/xor_1/native-gadget/bit", xor_native_gadget::<1>);
entry!("binary/and_2/native-gadget/bit", and_native_gadget::<2>);
entry!("binary/or_2/native-gadget/bit", or_native_gadget::<2>);
entry!("binary/xor_2/native-gadget/bit", xor_native_gadget::<2>);
entry!("binary/and_5/native-gadget/bit", and_native_gadget::<5>);
entry!("binary/or_5/native-gadget/bit", or_native_gadget::<5>);
entry!("binary/xor_5/native-gadget/bit", xor_native_gadget::<5>);

entry!("binary/not/native/bit", not_native);
entry!("binary/and_1/native/bit", and_native::<1>);
entry!("binary/or_1/native/bit", or_native::<1>);
entry!("binary/xor_1/native/bit", xor_native::<1>);
entry!("binary/and_2/native/bit", and_native::<2>);
entry!("binary/or_2/native/bit", or_native::<2>);
entry!("binary/xor_2/native/bit", xor_native::<2>);
entry!("binary/and_5/native/bit", and_native::<5>);
entry!("binary/or_5/native/bit", or_native::<5>);
entry!("binary/xor_5/native/bit", xor_native::<5>);

entry!("binary/not/stdlib/bit", not_stdlib);
entry!("binary/and_1/stdlib/bit", and_stdlib::<1>);
entry!("binary/or_1/stdlib/bit", or_stdlib::<1>);
entry!("binary/xor_1/stdlib/bit", xor_stdlib::<1>);
entry!("binary/and_2/stdlib/bit", and_stdlib::<2>);
entry!("binary/or_2/stdlib/bit", or_stdlib::<2>);
entry!("binary/xor_2/stdlib/bit", xor_stdlib::<2>);
entry!("binary/and_5/stdlib/bit", and_stdlib::<5>);
entry!("binary/or_5/stdlib/bit", or_stdlib::<5>);
entry!("binary/xor_5/stdlib/bit", xor_stdlib::<5>);

type F = mdnt_extractor_core::fields::Blstrs;

#[harness]
pub fn not_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.not(layouter, &x)
}

#[harness]
pub fn and_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.and(layouter, &bits)
}
#[harness]
pub fn or_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.or(layouter, &bits)
}
#[harness]
pub fn xor_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.xor(layouter, &bits)
}

#[harness(range_lookup(8))]
pub fn not_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.not(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn and_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.and(layouter, &bits)
}
#[harness(range_lookup(8))]
pub fn or_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.or(layouter, &bits)
}
#[harness(range_lookup(8))]
pub fn xor_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.xor(layouter, &bits)
}

fn not_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn not_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedBit<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.not(layouter, &x)
}

fn and_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn and_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.and(layouter, &bits)
}
fn or_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn or_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.or(layouter, &bits)
}
fn xor_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn xor_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.xor(layouter, &bits)
}
