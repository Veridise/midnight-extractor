use crate::utils::{range_lookup, vec2array};
use ff::PrimeField;
use mdnt_extractor_core::chips::{AF, FC, NG};
use mdnt_extractor_macros::{entry, harness, harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::DecompositionInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
};

type F = mdnt_extractor_core::fields::Blstrs;
type K = mdnt_extractor_core::fields::MidnightFp;

const fn num_bytes<P: PrimeField>() -> usize {
    num_chunks::<P>(8)
}

const fn num_chunks<P: PrimeField>(bits: usize) -> usize {
    (P::NUM_BITS as usize).div_ceil(bits)
}

#[entry("decomposition/assigned_from_be_bits/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_from_be_bits_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; F::NUM_BITS as usize],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bits(layouter, &bits)
}

#[entry("decomposition/assigned_from_be_bytes/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_from_be_bytes_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<F>()],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bytes(layouter, &bytes)
}

#[entry("decomposition/assigned_from_le_bits/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_from_le_bits_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; F::NUM_BITS as usize],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_le_bits(layouter, &bits)
}

#[entry("decomposition/assigned_from_le_bytes/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_from_le_bytes_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<F>()],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bytes(layouter, &bytes)
}

#[entry("decomposition/assigned_to_be_bits/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_to_be_bits_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedBit<F>; F::NUM_BITS as usize], Error> {
    chip.assigned_to_be_bits(layouter, &x, None, true).and_then(vec2array)
}

#[entry("decomposition/assigned_to_be_bytes/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_to_be_bytes_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedByte<F>; num_bytes::<F>()], Error> {
    chip.assigned_to_be_bytes(layouter, &x, None).and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_bits/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_bits_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedBit<F>; F::NUM_BITS as usize], Error> {
    chip.assigned_to_le_bits(layouter, &x, None, true).and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_bytes/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_bytes_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedByte<F>; num_bytes::<F>()], Error> {
    chip.assigned_to_le_bytes(layouter, &x, None).and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_chunks_128/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_chunks_128_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedNative<F>; num_chunks::<F>(128)], Error> {
    chip.assigned_to_le_chunks(layouter, &x, 128, None).and_then(vec2array)
}

#[entry("decomposition/sgn0/native-gadget/native")]
#[harness(range_lookup(8))]
pub fn sgn0_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.sgn0(layouter, &x)
}

#[entry("decomposition/assigned_from_be_bits/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_from_be_bits_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; K::NUM_BITS as usize],
) -> Result<AF<F, K>, Error> {
    chip.assigned_from_be_bits(layouter, &bits)
}

#[entry("decomposition/assigned_from_be_bytes/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_from_be_bytes_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<K>()],
) -> Result<AF<F, K>, Error> {
    chip.assigned_from_be_bytes(layouter, &bytes)
}

#[entry("decomposition/assigned_from_le_bits/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_from_le_bits_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; K::NUM_BITS as usize],
) -> Result<AF<F, K>, Error> {
    chip.assigned_from_le_bits(layouter, &bits)
}

#[entry("decomposition/assigned_from_le_bytes/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_from_le_bytes_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<K>()],
) -> Result<AF<F, K>, Error> {
    chip.assigned_from_le_bytes(layouter, &bytes)
}

#[entry("decomposition/assigned_to_be_bits/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_to_be_bits_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<[AssignedBit<F>; K::NUM_BITS as usize], Error> {
    chip.assigned_to_be_bits(layouter, &x, None, true).and_then(vec2array)
}

#[entry("decomposition/assigned_to_be_bytes/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_to_be_bytes_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<[AssignedByte<F>; (K::NUM_BITS / 8) as usize], Error> {
    chip.assigned_to_be_bytes(layouter, &x, Some((K::NUM_BITS / 8) as usize))
        .and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_bits/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_bits_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<[AssignedBit<F>; K::NUM_BITS as usize], Error> {
    chip.assigned_to_le_bits(layouter, &x, None, true).and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_bytes/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_bytes_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<[AssignedByte<F>; (K::NUM_BITS / 8) as usize], Error> {
    chip.assigned_to_le_bytes(layouter, &x, Some((K::NUM_BITS / 8) as usize))
        .and_then(vec2array)
}

#[entry("decomposition/assigned_to_le_chunks_128/field/field")]
#[harness(range_lookup(8))]
pub fn assigned_to_le_chunks_128_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<[AssignedNative<F>; num_chunks::<K>(128)], Error> {
    chip.assigned_to_le_chunks(layouter, &x, 128, None).and_then(vec2array)
}

#[entry("decomposition/sgn0/field/field")]
#[harness(range_lookup(8))]
pub fn sgn0_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AssignedBit<F>, Error> {
    chip.sgn0(layouter, &x)
}

#[usize_args(8)]
#[entry("decomposition/assigned_from_be_bits/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_from_be_bits_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; F::NUM_BITS as usize],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bits(layouter, &bits)
}

#[usize_args(8)]
#[entry("decomposition/assigned_from_be_bytes/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_from_be_bytes_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<F>()],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bytes(layouter, &bytes)
}

#[usize_args(8)]
#[entry("decomposition/assigned_from_le_bits/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_from_le_bits_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; F::NUM_BITS as usize],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_le_bits(layouter, &bits)
}

#[usize_args(8)]
#[entry("decomposition/assigned_from_le_bytes/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_from_le_bytes_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; num_bytes::<F>()],
) -> Result<AssignedNative<F>, Error> {
    chip.assigned_from_be_bytes(layouter, &bytes)
}

#[usize_args(8)]
#[entry("decomposition/assigned_to_be_bits/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_to_be_bits_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedBit<F>; F::NUM_BITS as usize], Error> {
    chip.assigned_to_be_bits(layouter, &x, None, true).and_then(vec2array)
}

#[usize_args(8)]
#[entry("decomposition/assigned_to_be_bytes/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_to_be_bytes_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedByte<F>; num_bytes::<F>()], Error> {
    chip.assigned_to_be_bytes(layouter, &x, None).and_then(vec2array)
}

#[usize_args(8)]
#[entry("decomposition/assigned_to_le_bits/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_to_le_bits_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedBit<F>; F::NUM_BITS as usize], Error> {
    chip.assigned_to_le_bits(layouter, &x, None, true).and_then(vec2array)
}

#[usize_args(8)]
#[entry("decomposition/assigned_to_le_bytes/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_to_le_bytes_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedByte<F>; num_bytes::<F>()], Error> {
    chip.assigned_to_le_bytes(layouter, &x, None).and_then(vec2array)
}

#[usize_args(8)]
#[entry("decomposition/assigned_to_le_chunks_128/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assigned_to_le_chunks_128_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedNative<F>; num_chunks::<F>(128)], Error> {
    chip.assigned_to_le_chunks(layouter, &x, 128, None).and_then(vec2array)
}

#[usize_args(8)]
#[entry("decomposition/sgn0/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn sgn0_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedBit<F>, Error> {
    chip.sgn0(layouter, &x)
}
