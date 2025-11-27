use mdnt_extractor_macros::unit_harness;
use midnight_circuits::{
    instructions::AssertionInstructions as _,
    types::{AssignedByte, AssignedNative},
    vec::{vector_gadget::VectorGadget, AssignedVector},
};

use crate::utils::range_lookup;
use mdnt_extractor_core::chips::Vga;
use mdnt_extractor_core::entry;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};

entry!(
    "assertion/assert_equal/vector/native",
    assert_equal_native_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_equal_native_vector<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, M, A>,
    y: AssignedVector<F, AssignedNative<F>, M, A>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x, &y)
}

entry!(
    "assertion/assert_not_equal/vector/native",
    assert_not_equal_native_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_native_vector<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, M, A>,
    y: AssignedVector<F, AssignedNative<F>, M, A>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x, &y)
}

entry!(
    "assertion/assert_equal_to_fixed/vector/native",
    assert_equal_to_fixed_native_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_native_vector<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    f: [L<F>; M],
    y: AssignedVector<F, AssignedNative<F>, M, A>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y, f.map(|f| f.0).to_vec())
}

entry!(
    "assertion/assert_not_equal_to_fixed/vector/native",
    assert_not_equal_to_fixed_native_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_native_vector<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    f: [L<F>; M],
    y: AssignedVector<F, AssignedNative<F>, M, A>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y, f.map(|f| f.0).to_vec())
}

entry!(
    "assertion/assert_equal/vector/byte",
    assert_equal_byte_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_equal_byte_vector<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, M, A>,
    y: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<(), Error> {
    chip.0.assert_equal(layouter, &x, &y)
}

entry!(
    "assertion/assert_not_equal/vector/byte",
    assert_not_equal_byte_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_byte_vector<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, M, A>,
    y: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<(), Error> {
    chip.0.assert_not_equal(layouter, &x, &y)
}

entry!(
    "assertion/assert_equal_to_fixed/vector/byte",
    assert_equal_to_fixed_byte_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_byte_vector<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    f: [u8; M],
    y: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<(), Error> {
    chip.0.assert_equal_to_fixed(layouter, &y, f.to_vec())
}

entry!(
    "assertion/assert_not_equal_to_fixed/vector/byte",
    assert_not_equal_to_fixed_byte_vector::<10, 5>
);
#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_byte_vector<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    f: [u8; M],
    y: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<(), Error> {
    chip.0.assert_not_equal_to_fixed(layouter, &y, f.to_vec())
}
