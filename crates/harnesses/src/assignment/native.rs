use mdnt_extractor_core::entry as add_entry;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    field::NativeChip,
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedNative},
};
use midnight_proofs::circuit::Value;

use crate::utils::vec_len_err;

#[entry("assignment/assign/native/native")]
#[harness]
pub fn assign_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign(layouter, x.value().copied())
}

#[entry("assignment/assign/native/bit")]
#[harness]
pub fn assign_native_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign(layouter, Value::known(x))
}

#[entry("assignment/assign_fixed/native/native")]
#[harness]
pub fn assign_fixed_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: L<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_fixed(layouter, x.0)
}

#[entry("assignment/assign_fixed/native/bit")]
#[harness]
pub fn assign_fixed_native_bit(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: bool,
) -> Result<AssignedBit<F>, Error> {
    chip.assign_fixed(layouter, x)
}

add_entry!(
    "assignment/assign_many_1/native/native",
    assign_many_native::<1>
);
add_entry!(
    "assignment/assign_many_2/native/native",
    assign_many_native::<2>
);
add_entry!(
    "assignment/assign_many_3/native/native",
    assign_many_native::<3>
);
add_entry!(
    "assignment/assign_many_5/native/native",
    assign_many_native::<5>
);
add_entry!(
    "assignment/assign_many_8/native/native",
    assign_many_native::<8>
);
#[harness]
pub fn assign_many_native<const SIZE: usize>(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: [AssignedNative<F>; SIZE],
) -> Result<[AssignedNative<F>; SIZE], Error> {
    let input = x.iter().map(|x| x.value().copied()).collect::<Vec<_>>();
    chip.assign_many(layouter, &input)
        .and_then(|v| v.try_into().map_err(vec_len_err::<SIZE, _>))
}
