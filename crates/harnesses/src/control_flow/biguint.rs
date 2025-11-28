use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::BG, entry as add_entry};
use mdnt_extractor_macros::{harness, unit_harness};
use midnight_circuits::{
    biguint::extraction::LoadedBigUint, instructions::ControlFlowInstructions,
    midnight_proofs::plonk::Error, types::AssignedBit,
};

type F = mdnt_extractor_core::fields::Blstrs;

add_entry!(
    "control-flow/select_64bits/biguint/biguint",
    select_biguint_gadget::<64>
);
add_entry!(
    "control-flow/select_300bits/biguint/biguint",
    select_biguint_gadget::<300>
);
#[harness(range_lookup(8))]
pub fn select_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (
        AssignedBit<F>,
        LoadedBigUint<F, BITS>,
        LoadedBigUint<F, BITS>,
    ),
) -> Result<LoadedBigUint<F, BITS>, Error> {
    Ok(chip.select(layouter, &cond, &a.into(), &b.into())?.try_into()?)
}

add_entry!(
    "control-flow/cond_assert_equal_64bits/biguint/biguint",
    cond_assert_equal_biguint_gadget::<64>
);
add_entry!(
    "control-flow/cond_assert_equal_300bits/biguint/biguint",
    cond_assert_equal_biguint_gadget::<300>
);
#[unit_harness(range_lookup(8))]
pub fn cond_assert_equal_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, x): (AssignedBit<F>, LoadedBigUint<F, BITS>),
    y: LoadedBigUint<F, BITS>,
) -> Result<(), Error> {
    chip.cond_assert_equal(layouter, &cond, &x.into(), &y.into())
}

add_entry!(
    "control-flow/cond_swap_64bits/biguint/biguint",
    cond_swap_biguint_gadget::<64>
);
add_entry!(
    "control-flow/cond_swap_300bits/biguint/biguint",
    cond_swap_biguint_gadget::<300>
);
#[harness(range_lookup(8))]
pub fn cond_swap_biguint_gadget<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (
        AssignedBit<F>,
        LoadedBigUint<F, BITS>,
        LoadedBigUint<F, BITS>,
    ),
) -> Result<(LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>), Error> {
    let (lhs, rhs) = chip.cond_swap(layouter, &cond, &a.into(), &b.into())?;
    Ok((lhs.try_into()?, rhs.try_into()?))
}
