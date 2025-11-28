use crate::utils::range_lookup;
use mdnt_extractor_core::{
    chips::{FC, NG},
    entry,
};
use mdnt_extractor_macros::{harness, harness_with_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib, field::NativeChip, instructions::CanonicityInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedBit,
};
use num_bigint::BigUint;

type F = mdnt_extractor_core::fields::Blstrs;
type K = mdnt_extractor_core::fields::MidnightFp;

entry!(
    "canonicity/le_bits_lower_than_1/native/native",
    le_bits_lower_than_native::<1>
);
entry!(
    "canonicity/le_bits_lower_than_64/native/native",
    le_bits_lower_than_native::<64>
);
entry!(
    "canonicity/le_bits_lower_than_255/native/native",
    le_bits_lower_than_native::<255>
);
#[harness]
pub fn le_bits_lower_than_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_lower_than(layouter, &bits, bound)
}

entry!(
    "canonicity/le_bits_geq_than_1/native/native",
    le_bits_geq_than_native::<1>
);
entry!(
    "canonicity/le_bits_geq_than_64/native/native",
    le_bits_geq_than_native::<64>
);
entry!(
    "canonicity/le_bits_geq_than_255/native/native",
    le_bits_geq_than_native::<255>
);
#[harness]
pub fn le_bits_geq_than_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_geq_than(layouter, &bits, bound)
}

entry!(
    "canonicity/is_canonical_1/native/native",
    is_canonical_native::<1>
);
entry!(
    "canonicity/is_canonical_64/native/native",
    is_canonical_native::<64>
);
entry!(
    "canonicity/is_canonical_255/native/native",
    is_canonical_native::<255>
);
#[harness]
pub fn is_canonical_native<const N: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.is_canonical(layouter, &bits)
}

entry!(
    "canonicity/le_bits_lower_than_1/native-gadget/native",
    le_bits_lower_than_native_gadget::<1>
);
entry!(
    "canonicity/le_bits_lower_than_64/native-gadget/native",
    le_bits_lower_than_native_gadget::<64>
);
entry!(
    "canonicity/le_bits_lower_than_255/native-gadget/native",
    le_bits_lower_than_native_gadget::<255>
);
#[harness(range_lookup(8))]
pub fn le_bits_lower_than_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_lower_than(layouter, &bits, bound)
}

entry!(
    "canonicity/le_bits_geq_than_1/native-gadget/native",
    le_bits_geq_than_native_gadget::<1>
);
entry!(
    "canonicity/le_bits_geq_than_64/native-gadget/native",
    le_bits_geq_than_native_gadget::<64>
);
entry!(
    "canonicity/le_bits_geq_than_255/native-gadget/native",
    le_bits_geq_than_native_gadget::<255>
);
#[harness(range_lookup(8))]
pub fn le_bits_geq_than_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_geq_than(layouter, &bits, bound)
}

entry!(
    "canonicity/is_canonical_1/native-gadget/native",
    is_canonical_native_gadget::<1>
);
entry!(
    "canonicity/is_canonical_64/native-gadget/native",
    is_canonical_native_gadget::<64>
);
entry!(
    "canonicity/is_canonical_255/native-gadget/native",
    is_canonical_native_gadget::<255>
);
#[harness(range_lookup(8))]
pub fn is_canonical_native_gadget<const N: usize>(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.is_canonical(layouter, &bits)
}

entry!(
    "canonicity/le_bits_lower_than_1/field/field",
    le_bits_lower_than_field::<1>
);
entry!(
    "canonicity/le_bits_lower_than_64/field/field",
    le_bits_lower_than_field::<64>
);
entry!(
    "canonicity/le_bits_lower_than_255/field/field",
    le_bits_lower_than_field::<255>
);
#[harness(range_lookup(8))]
pub fn le_bits_lower_than_field<const N: usize>(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_lower_than(layouter, &bits, bound)
}

entry!(
    "canonicity/le_bits_geq_than_1/field/field",
    le_bits_geq_than_field::<1>
);
entry!(
    "canonicity/le_bits_geq_than_64/field/field",
    le_bits_geq_than_field::<64>
);
entry!(
    "canonicity/le_bits_geq_than_255/field/field",
    le_bits_geq_than_field::<255>
);
#[harness(range_lookup(8))]
pub fn le_bits_geq_than_field<const N: usize>(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_geq_than(layouter, &bits, bound)
}

entry!(
    "canonicity/is_canonical_1/field/field",
    is_canonical_field::<1>
);
entry!(
    "canonicity/is_canonical_64/field/field",
    is_canonical_field::<64>
);
entry!(
    "canonicity/is_canonical_255/field/field",
    is_canonical_field::<255>
);
#[harness(range_lookup(8))]
pub fn is_canonical_field<const N: usize>(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.is_canonical(layouter, &bits)
}

fn le_bits_lower_than_stdlib_args() -> usize {
    8
}
entry!(
    "canonicity/le_bits_lower_than_1/stdlib/native",
    le_bits_lower_than_stdlib::<1>
);
entry!(
    "canonicity/le_bits_lower_than_64/stdlib/native",
    le_bits_lower_than_stdlib::<64>
);
entry!(
    "canonicity/le_bits_lower_than_255/stdlib/native",
    le_bits_lower_than_stdlib::<255>
);
#[harness_with_args(usize, range_lookup(8))]
pub fn le_bits_lower_than_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_lower_than(layouter, &bits, bound)
}

fn le_bits_geq_than_stdlib_args() -> usize {
    8
}
entry!(
    "canonicity/le_bits_geq_than_1/stdlib/native",
    le_bits_geq_than_stdlib::<1>
);
entry!(
    "canonicity/le_bits_geq_than_64/stdlib/native",
    le_bits_geq_than_stdlib::<64>
);
entry!(
    "canonicity/le_bits_geq_than_255/stdlib/native",
    le_bits_geq_than_stdlib::<255>
);
#[harness_with_args(usize, range_lookup(8))]
pub fn le_bits_geq_than_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (bits, bound): ([AssignedBit<F>; N], BigUint),
) -> Result<AssignedBit<F>, Error> {
    chip.le_bits_geq_than(layouter, &bits, bound)
}

fn is_canonical_stdlib_args() -> usize {
    8
}
entry!(
    "canonicity/is_canonical_1/stdlib/native",
    is_canonical_stdlib::<1>
);
entry!(
    "canonicity/is_canonical_64/stdlib/native",
    is_canonical_stdlib::<64>
);
entry!(
    "canonicity/is_canonical_255/stdlib/native",
    is_canonical_stdlib::<255>
);
#[harness_with_args(usize, range_lookup(8))]
pub fn is_canonical_stdlib<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    bits: [AssignedBit<F>; N],
) -> Result<AssignedBit<F>, Error> {
    chip.is_canonical(layouter, &bits)
}
