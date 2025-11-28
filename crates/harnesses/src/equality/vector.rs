use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_core::{chips::Vga, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::EqualityInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedBit, AssignedByte, AssignedNative},
    vec::{vector_gadget::VectorGadget, AssignedVector},
};
pub type F = mdnt_extractor_core::fields::Blstrs;

entry!(
    "equality/is_equal/vector/native",
    is_equal_vector_native::<10, 5>
);
#[harness(range_lookup(8))]
pub fn is_equal_vector_native<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (
        AssignedVector<F, AssignedNative<F>, M, A>,
        AssignedVector<F, AssignedNative<F>, M, A>,
    ),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

entry!(
    "equality/is_equal_to_fixed/vector/native",
    is_equal_to_fixed_vector_native::<10, 5>
);
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_vector_native<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedVector<F, AssignedNative<F>, M, A>, [L<F>; M]),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.map(|f| f.0).to_vec())
}

entry!(
    "equality/is_equal/vector/byte",
    is_equal_vector_byte::<10, 5>
);
#[harness(range_lookup(8))]
pub fn is_equal_vector_byte<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (
        AssignedVector<F, AssignedByte<F>, M, A>,
        AssignedVector<F, AssignedByte<F>, M, A>,
    ),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal(layouter, &x, &y)
}

entry!(
    "equality/is_equal_to_fixed/vector/byte",
    is_equal_to_fixed_vector_byte::<10, 5>
);
#[harness(range_lookup(8))]
pub fn is_equal_to_fixed_vector_byte<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedVector<F, AssignedByte<F>, M, A>, [u8; M]),
) -> Result<AssignedBit<F>, Error> {
    chip.is_equal_to_fixed(layouter, &x, y.to_vec())
}
