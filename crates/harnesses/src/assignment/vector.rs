use crate::utils::range_lookup;
use mdnt_extractor_core::chips::Vga;
use mdnt_extractor_core::entry as add_entry;
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedByte, AssignedNative, InnerValue},
    vec::{vector_gadget::VectorGadget, AssignedVector},
};

add_entry!(
    "assignment/assign/vector/native",
    assign_native_vector::<10, 5>
);
#[harness(range_lookup(8))]
pub fn assign_native_vector<const M: usize, const A: usize>(
    chip: &VectorGadget<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedNative<F>, M, A>,
) -> Result<AssignedVector<F, AssignedNative<F>, M, A>, Error> {
    chip.assign(layouter, x.value())
}

add_entry!("assignment/assign/vector/byte", assign_byte_vector::<10, 5>);
#[harness(range_lookup(8))]
pub fn assign_byte_vector<const M: usize, const A: usize>(
    chip: &Vga<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<AssignedVector<F, AssignedByte<F>, M, A>, Error> {
    chip.0.assign(layouter, x.value())
}
