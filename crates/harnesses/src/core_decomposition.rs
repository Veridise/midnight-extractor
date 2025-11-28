use crate::utils::range_lookup;
use mdnt_extractor_core::circuit::to_plonk_error;
use mdnt_extractor_macros::{entry, harness_with_args};
use midnight_circuits::{
    field::decomposition::{
        chip::P2RDecompositionChip, instructions::CoreDecompositionInstructions,
    },
    midnight_proofs::plonk::Error,
    types::AssignedNative,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

fn decompose_fixed_limb_size_32_4_args() -> usize {
    32
}

#[entry("core-decomposition/decompose_fixed_limb_size_32_4/p2r-decomposition/native")]
#[harness_with_args(usize, range_lookup(32))]
pub fn decompose_fixed_limb_size_32_4(
    chip: &P2RDecompositionChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<[AssignedNative<F>; 4], Error> {
    let v = chip.decompose_fixed_limb_size(layouter, &x, 32, 32 / 4)?;

    v.try_into().map_err(|v: Vec<AssignedNative<F>>| {
        to_plonk_error(format!("Was expecting {} elements but got {}", 4, v.len()))
    })
}
