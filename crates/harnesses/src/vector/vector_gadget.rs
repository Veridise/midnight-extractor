use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_core::{chips::Vga, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::VectorInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
    vec::{vector_gadget::VectorGadget, AssignedVector},
};
use midnight_proofs::circuit::Value;

entry!("vector/resize/vector/native", resize_native_10_5);
entry!(
    "vector/assign/vector/native",
    assign_with_filler_native_10_5
);
entry!(
    "vector/assign_without_filler/vector/native",
    assign_without_filler_native_10_5
);
entry!(
    "vector/trim_beginning/vector/native",
    trim_beginning_native_10_5
);
entry!(
    "vector/padding_flag/vector/native",
    padding_flag_native_10_5
);
entry!("vector/get_limits/vector/native", get_limits_native_10_5);

entry!("vector/resize/vector/byte", resize_byte_10_5);
entry!("vector/assign/vector/byte", assign_with_filler_byte_10_5);
entry!(
    "vector/assign_without_filler/vector/byte",
    assign_without_filler_byte_10_5
);
entry!(
    "vector/trim_beginnging/vector/byte",
    trim_beginning_byte_10_5
);
entry!("vector/padding_flag/vector/byte", padding_flag_byte_10_5);
entry!("vector/get_limits/vector/byte", get_limits_byte_10_5);

pub type F = mdnt_extractor_core::fields::Blstrs;

#[harness(range_lookup(8))]
pub fn resize_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<AssignedVector<F, AssignedNative<F>, 20, 5>, Error> {
    chip.resize(layouter, x)
}

#[harness(range_lookup(8))]
pub fn assign_with_filler_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    (value, filler): ([L<F>; 10], L<F>),
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.assign_with_filler(
        layouter,
        Value::known(value.map(|f| f.0).to_vec()),
        Some(filler.0),
    )
}

#[harness(range_lookup(8))]
pub fn assign_without_filler_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    value: [L<F>; 10],
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.assign_with_filler(layouter, Value::known(value.map(|f| f.0).to_vec()), None)
}

#[harness(range_lookup(8))]
pub fn trim_beginning_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    (x, n_elems): (AssignedVector<F, AssignedNative<F>, 10, 5>, usize),
) -> Result<AssignedVector<F, AssignedNative<F>, 10, 5>, Error> {
    chip.trim_beginning(layouter, &x, n_elems)
}

#[harness(range_lookup(8))]
pub fn padding_flag_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<[AssignedBit<F>; 10], Error> {
    chip.padding_flag(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn get_limits_native_10_5(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, 10, 5>,
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.get_limits(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn resize_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<AssignedVector<F, AssignedByte<F>, 20, 5>, Error> {
    chip.0.resize(layouter, x)
}

#[harness(range_lookup(8))]
pub fn assign_with_filler_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    (value, filler): ([u8; 10], u8),
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.0.assign_with_filler(layouter, Value::known(value.to_vec()), Some(filler))
}

#[harness(range_lookup(8))]
pub fn assign_without_filler_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    value: [u8; 10],
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.0.assign_with_filler(layouter, Value::known(value.to_vec()), None)
}

#[harness(range_lookup(8))]
pub fn trim_beginning_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    (x, n_elems): (AssignedVector<F, AssignedByte<F>, 10, 5>, usize),
) -> Result<AssignedVector<F, AssignedByte<F>, 10, 5>, Error> {
    chip.0.trim_beginning(layouter, &x, n_elems)
}

#[harness(range_lookup(8))]
pub fn padding_flag_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<[AssignedBit<F>; 10], Error> {
    chip.0.padding_flag(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn get_limits_byte_10_5(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, 10, 5>,
) -> Result<(AssignedNative<F>, AssignedNative<F>), Error> {
    chip.0.get_limits(layouter, &x)
}
