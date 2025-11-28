use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_core::{cells::load::AssignedBoundedLoad, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    field::{decomposition::chip::P2RDecompositionChip, NativeChip, NativeGadget},
    instructions::ComparisonInstructions as _,
    midnight_proofs::plonk::Error,
    types::AssignedBit,
};

entry!(
    "comparison/geq/native-gadget/native",
    geq_native_bounded_byte
);
entry!(
    "comparison/leq/native-gadget/native",
    leq_native_bounded_byte
);
entry!(
    "comparison/lower_than/native-gadget/native",
    lower_than_native_bounded_byte
);
entry!(
    "comparison/lower_than_fixed/native-gadget/native",
    lower_than_fixed_native_bounded_byte
);

pub type F = mdnt_extractor_core::fields::Blstrs;

#[harness(range_lookup(8))]
pub fn lower_than_native_bounded_byte(
    chip: &NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBoundedLoad<F, 8>, AssignedBoundedLoad<F, 8>),
) -> Result<AssignedBit<F>, Error> {
    chip.lower_than(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn lower_than_fixed_native_bounded_byte(
    chip: &NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBoundedLoad<F, 8>, L<F>),
) -> Result<AssignedBit<F>, Error> {
    chip.lower_than_fixed(layouter, &x, y.0)
}

#[harness(range_lookup(8))]
pub fn leq_native_bounded_byte(
    chip: &NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBoundedLoad<F, 8>, AssignedBoundedLoad<F, 8>),
) -> Result<AssignedBit<F>, Error> {
    chip.leq(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn geq_native_bounded_byte(
    chip: &NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedBoundedLoad<F, 8>, AssignedBoundedLoad<F, 8>),
) -> Result<AssignedBit<F>, Error> {
    chip.geq(layouter, &x, &y)
}
