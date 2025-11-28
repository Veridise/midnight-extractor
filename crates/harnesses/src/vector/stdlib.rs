use crate::utils::range_lookup;
use mdnt_extractor_core::entry;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_macros::{harness_with_args, usize_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    instructions::VectorInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
    vec::AssignedVector,
};
use midnight_proofs::circuit::Value;

entry!("vector/resize/stdlib/native", resize_native::<10, 5>);
entry!(
    "vector/assign/stdlib/native",
    assign_with_filler_native::<10, 5>
);
entry!(
    "vector/assign_without_filler/stdlib/native",
    assign_without_filler_native::<10, 5>
);
entry!(
    "vector/trim_beginning/stdlib/native",
    trim_beginning_native::<10, 5>
);
entry!(
    "vector/padding_flag/stdlib/native",
    padding_flag_native::<10, 5>
);
entry!(
    "vector/get_limits/stdlib/native",
    get_limits_native::<10, 5>
);

entry!("vector/resize/stdlib/byte", resize_byte::<10, 5>);
entry!(
    "vector/assign/stdlib/byte",
    assign_with_filler_byte::<10, 5>
);
entry!(
    "vector/assign_without_filler/stdlib/byte",
    assign_without_filler_byte::<10, 5>
);
entry!(
    "vector/trim_beginnging/stdlib/byte",
    trim_beginning_byte::<10, 5>
);
entry!(
    "vector/padding_flag/stdlib/byte",
    padding_flag_byte::<10, 5>
);
entry!("vector/get_limits/stdlib/byte", get_limits_byte::<10, 5>);

pub type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn resize_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<AssignedVector<F, AssignedNative<F>, 20, 5>, Error> {
    chip.resize(layouter, x)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_with_filler_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (value, filler): ([L<F>; 10], L<F>),
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.assign_with_filler(
        layouter,
        Value::known(value.map(|f| f.0).to_vec()),
        Some(filler.0),
    )
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_without_filler_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    value: [L<F>; 10],
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.assign_with_filler(layouter, Value::known(value.map(|f| f.0).to_vec()), None)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn trim_beginning_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, n_elems): (AssignedVector<F, AssignedNative<F>, 10, 5>, usize),
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.trim_beginning(layouter, &x, n_elems)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn padding_flag_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<[AssignedBit<F>; 10], Error> {
    chip.padding_flag(layouter, &x)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn get_limits_native<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.get_limits(layouter, &x)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn resize_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<AssignedVector<F, AssignedByte<F>, 20, 5>, Error> {
    chip.resize(layouter, x)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_with_filler_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (value, filler): ([u8; 10], u8),
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.assign_with_filler(layouter, Value::known(value.to_vec()), Some(filler))
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn assign_without_filler_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    value: [u8; 10],
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.assign_with_filler(layouter, Value::known(value.to_vec()), None)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn trim_beginning_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, n_elems): (AssignedVector<F, AssignedByte<F>, 10, 5>, usize),
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.trim_beginning(layouter, &x, n_elems)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn padding_flag_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<[AssignedBit<F>; 10], Error> {
    chip.padding_flag(layouter, &x)
}

#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn get_limits_byte<const M: usize, const A: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.get_limits(layouter, &x)
}
