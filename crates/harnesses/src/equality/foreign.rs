use crate::utils::range_lookup;
use mdnt_extractor_core::chips::{AF, FC};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::EqualityInstructions as _, midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;
pub type K = mdnt_extractor_core::fields::MidnightFp;

#[entry("equality/is_equal/field/field")]
#[harness(range_lookup(8))]
pub fn is_equal_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
    injected_ir: &mut InjectedIR<F>,
) -> Result<AssignedBit<F>, Error> {
    let result = chip.is_equal(layouter, &x, &y)?;
    let ir = chip.native_gadget().take_injected_ir();
    injected_ir.combine_ir(ir);
    Ok(result)
}
