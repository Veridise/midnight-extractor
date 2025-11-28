use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::BG, entry as add_entry};
use mdnt_extractor_macros::{harness, unit_harness};
use midnight_circuits::{
    biguint::extraction::LoadedBigUint, instructions::ZeroInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedBit,
};

pub type F = mdnt_extractor_core::fields::Blstrs;

add_entry!(
    "zero/assert_zero_64bits/biguint/biguint",
    assert_zero_biguint_gadget::<64>
);
add_entry!(
    "zero/assert_zero_300bits/biguint/biguint",
    assert_zero_biguint_gadget::<300>
);
#[unit_harness(range_lookup(8))]
pub fn assert_zero_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: LoadedBigUint<F, BITS>,
) -> Result<(), Error> {
    chip.assert_zero(layouter, &x.into())
}

add_entry!(
    "zero/assert_non_zero_64bits/biguint/biguint",
    assert_non_zero_biguint_gadget::<64>
);
add_entry!(
    "zero/assert_non_zero_300bits/biguint/biguint",
    assert_non_zero_biguint_gadget::<300>
);
#[unit_harness(range_lookup(8))]
pub fn assert_non_zero_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
    x: LoadedBigUint<F, BITS>,
) -> Result<(), Error> {
    chip.assert_non_zero(layouter, &x.into())
}

add_entry!(
    "zero/is_zero_64bits/biguint/biguint",
    is_zero_biguint_gadget::<64>
);
add_entry!(
    "zero/is_zero_300bits/biguint/biguint",
    is_zero_biguint_gadget::<300>
);
#[harness(range_lookup(8))]
pub fn is_zero_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, BITS>,
) -> Result<AssignedBit<F>, Error> {
    chip.is_zero(layouter, &x.into())
}
