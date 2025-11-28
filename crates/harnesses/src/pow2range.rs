use crate::utils::range_lookup;
use mdnt_extractor_core::cells::store::FreshVar;
use mdnt_extractor_macros::{entry, harness_with_args, usize_args};
use midnight_circuits::{
    field::decomposition::pow2range::Pow2RangeChip,
    instructions::decomposition::Pow2RangeInstructions as _, midnight_proofs::plonk::Error,
    types::AssignedNative,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("pow2range/assert_values_lower_than_2_pow_8_1/pow2range/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn assert_values_lower_than_2_pow_8_1(
    chip: &Pow2RangeChip<F>,
    layouter: &mut impl Layouter<F>,
    values: [AssignedNative<F>; 1],
) -> Result<FreshVar, Error> {
    chip.assert_values_lower_than_2_pow_n(layouter, &values, 8)?;

    //todo!("Add postconditions");
    Ok(FreshVar)
}
