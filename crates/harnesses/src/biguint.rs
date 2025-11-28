use crate::utils::range_lookup;
use mdnt_extractor_core::{chips::BG, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    biguint::{extraction::LoadedBigUint, AssignedBigUint},
    midnight_proofs::plonk::Error,
    types::{AssignedBit, InnerValue as _},
};
use num_bigint::BigUint;

type F = mdnt_extractor_core::fields::Blstrs;

entry!(
    "biguint/assign_biguint_64bits/biguint/biguint",
    assign_biguint::<64>
);
entry!(
    "biguint/assign_biguint_300bits/biguint/biguint",
    assign_biguint::<300>
);
#[harness(range_lookup(8))]
pub fn assign_biguint<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    value: LoadedBigUint<F, BITS>,
) -> Result<LoadedBigUint<F, BITS>, Error> {
    let value: AssignedBigUint<F> = value.into();
    Ok(chip
        .assign_biguint(layouter, value.value(), BITS.try_into().unwrap())?
        .try_into()?)
}

entry!(
    "biguint/assign_fixed_biguint_64bits/biguint/biguint",
    assign_fixed_biguint::<64>
);
entry!(
    "biguint/assign_fixed_biguint_300bits/biguint/biguint",
    assign_fixed_biguint::<300>
);
#[harness(range_lookup(8))]
pub fn assign_fixed_biguint<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    value: BigUint,
) -> Result<LoadedBigUint<F, BITS>, Error> {
    Ok(chip.assign_fixed_biguint(layouter, value)?.try_into()?)
}

entry!(
    "biguint/constrain_as_public_input_64bits/biguint/biguint",
    constrain_as_public_input::<64, 64>
);
entry!(
    "biguint/constrain_as_public_input_300bits/biguint/biguint",
    constrain_as_public_input::<300, 300>
);
// BITS and NBITS should have the same value
#[harness(range_lookup(8))]
pub fn constrain_as_public_input<const BITS: usize, const NBITS: u32>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    value: LoadedBigUint<F, BITS>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &value.into(), NBITS)
}

entry!("biguint/add_64bits/biguint/biguint", add::<64>);
entry!("biguint/add_300bits/biguint/biguint", add::<300>);
#[harness(range_lookup(8))]
pub fn add<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<LoadedBigUint<F, BITS>, Error> {
    Ok(chip.add(layouter, &x.into(), &y.into())?.try_into()?)
}

entry!("biguint/sub_64bits/biguint/biguint", sub::<64>);
entry!("biguint/sub_300bits/biguint/biguint", sub::<300>);
#[harness(range_lookup(8))]
pub fn sub<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<LoadedBigUint<F, BITS>, Error> {
    // TODO: Add a check that ensures that x < y. Because if that condition doesn't hold then the
    // circuit is unsat.
    Ok(chip.sub(layouter, &x.into(), &y.into())?.try_into()?)
}

entry!("biguint/mul_64bits/biguint/biguint", mul::<64, { 64 * 2 }>);
entry!(
    "biguint/mul_300bits/biguint/biguint",
    mul::<300, { 300 * 2 }>
);
#[harness(range_lookup(8))]
pub fn mul<const BITS: usize, const OBITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<LoadedBigUint<F, OBITS>, Error> {
    Ok(chip.mul(layouter, &x.into(), &y.into())?.try_into()?)
}

entry!("biguint/div_rem_64bits/biguint/biguint", div_rem::<64>);
entry!("biguint/div_rem_300bits/biguint/biguint", div_rem::<300>);
#[harness(range_lookup(8))]
pub fn div_rem<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<(LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>), Error> {
    let (lhs, rhs) = chip.div_rem(layouter, &x.into(), &y.into())?;
    Ok((lhs.try_into()?, rhs.try_into()?))
}

//entry!("biguint/mod_exp_64bits_0/biguint/biguint", mod_exp::<64, 0>);
//entry!(
//    "biguint/mod_exp_300bits_0/biguint/biguint",
//    mod_exp::<300, 0>
//);
entry!("biguint/mod_exp_64bits_1/biguint/biguint", mod_exp::<64, 1>);
entry!(
    "biguint/mod_exp_300bits_1/biguint/biguint",
    mod_exp::<300, 1>
);
entry!("biguint/mod_exp_64bits_2/biguint/biguint", mod_exp::<64, 2>);
entry!(
    "biguint/mod_exp_300bits_2/biguint/biguint",
    mod_exp::<300, 2>
);
#[harness(range_lookup(8))]
pub fn mod_exp<const BITS: usize, const N: u64>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, m): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<LoadedBigUint<F, BITS>, Error> {
    Ok(chip.mod_exp(layouter, &x.into(), N, &m.into())?.try_into()?)
}

const BOUND: usize = 96; // Taken from midnight2/circuits/src/biguint/types.rh

// The upper bound is computed 'by hand' because the compiler is not happy if it is computed as the
// size of the output array
entry!(
    "biguint/to_le_bits_64bits/biguint/biguint",
    to_le_bits::<64, { (64usize.div_ceil(BOUND)) * BOUND }>
);
entry!(
    "biguint/to_le_bits_300bits/biguint/biguint",
    to_le_bits::<300, { (300usize.div_ceil(BOUND)) * BOUND }>
);
#[harness(range_lookup(8))]
pub fn to_le_bits<const BITS: usize, const BITS_UB: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, BITS>,
) -> Result<[AssignedBit<F>; BITS_UB], Error> {
    Ok(
        chip.to_le_bits(layouter, &x.into())?.try_into().map_err(|v: Vec<_>| {
            mdnt_support::error::Error::UnexpectedElements {
                header: "In to_le_bits harness: ".to_string(),
                expected: BITS_UB,
                actual: v.len(),
            }
        })?,
    )
}

entry!(
    "biguint/from_le_bits_64bits/biguint/biguint",
    from_le_bits::<64>
);
entry!(
    "biguint/from_le_bits_300bits/biguint/biguint",
    from_le_bits::<300>
);
#[harness(range_lookup(8))]
pub fn from_le_bits<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: [AssignedBit<F>; BITS],
) -> Result<LoadedBigUint<F, BITS>, Error> {
    Ok(chip.from_le_bits(layouter, &x)?.try_into()?)
}

entry!(
    "biguint/lower_than_64bits/biguint/biguint",
    lower_than::<64>
);
entry!(
    "biguint/lower_than_300bits/biguint/biguint",
    lower_than::<300>
);
#[harness(range_lookup(8))]
pub fn lower_than<const BITS: usize>(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (LoadedBigUint<F, BITS>, LoadedBigUint<F, BITS>),
) -> Result<AssignedBit<F>, Error> {
    chip.lower_than(layouter, &x.into(), &y.into())
}
