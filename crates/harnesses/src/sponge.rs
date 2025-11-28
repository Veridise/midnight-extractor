use crate::utils::range_lookup;
use mdnt_extractor_core::entry as add_entry;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    hash::poseidon::{extraction::LoadedPoseidonState, AssignedPoseidonState, PoseidonChip},
    instructions::SpongeInstructions as _,
    types::AssignedNative,
};
use midnight_proofs::plonk::Error;

pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("sponge/init_with_len/poseidon/native")]
#[harness(range_lookup(8))]
pub fn init(
    chip: &PoseidonChip<F>,
    layouter: &mut impl Layouter<F>,
    len: usize,
) -> Result<LoadedPoseidonState<F, 0>, Error> {
    Ok(chip.init(layouter, Some(len))?.try_into()?)
}

#[entry("sponge/init_without_len/poseidon/native")]
#[harness(range_lookup(8))]
pub fn init_without_len(
    chip: &PoseidonChip<F>,
    layouter: &mut impl Layouter<F>,
    _: (),
) -> Result<LoadedPoseidonState<F, 0>, Error> {
    Ok(chip.init(layouter, None)?.try_into()?)
}

add_entry!("sponge/absorb_0_0/poseidon/native", absorb::<0, 0, 0>);
add_entry!("sponge/absorb_1_0/poseidon/native", absorb::<1, 0, 1>);
add_entry!("sponge/absorb_5_0/poseidon/native", absorb::<5, 0, 5>);
add_entry!("sponge/absorb_10_0/poseidon/native", absorb::<10, 0, 10>);
add_entry!("sponge/absorb_0_1/poseidon/native", absorb::<0, 1, 1>);
add_entry!("sponge/absorb_1_1/poseidon/native", absorb::<1, 1, 2>);
add_entry!("sponge/absorb_5_1/poseidon/native", absorb::<5, 1, 6>);
add_entry!("sponge/absorb_10_1/poseidon/native", absorb::<10, 1, 11>);
add_entry!("sponge/absorb_0_5/poseidon/native", absorb::<0, 5, 5>);
add_entry!("sponge/absorb_1_5/poseidon/native", absorb::<1, 5, 6>);
add_entry!("sponge/absorb_5_5/poseidon/native", absorb::<5, 5, 10>);
add_entry!("sponge/absorb_10_5/poseidon/native", absorb::<10, 5, 15>);
add_entry!("sponge/absorb_0_10/poseidon/native", absorb::<0, 10, 10>);
add_entry!("sponge/absorb_1_10/poseidon/native", absorb::<1, 10, 11>);
add_entry!("sponge/absorb_5_10/poseidon/native", absorb::<5, 10, 15>);
add_entry!("sponge/absorb_10_10/poseidon/native", absorb::<10, 10, 20>);
#[harness(range_lookup(8))]
// NQ = N + Q
pub fn absorb<const Q: usize, const N: usize, const NQ: usize>(
    chip: &PoseidonChip<F>,
    layouter: &mut impl Layouter<F>,
    (state, inputs): (LoadedPoseidonState<F, Q>, [AssignedNative<F>; N]),
) -> Result<LoadedPoseidonState<F, NQ>, Error> {
    let mut state: AssignedPoseidonState<F> = state.into();
    chip.absorb(layouter, &mut state, &inputs)?;
    Ok(state.try_into()?)
}

add_entry!("sponge/squeeze_0/poseidon/native", squeeze::<0>);
add_entry!("sponge/squeeze_1/poseidon/native", squeeze::<1>);
add_entry!("sponge/squeeze_5/poseidon/native", squeeze::<5>);
add_entry!("sponge/squeeze_10/poseidon/native", squeeze::<10>);
#[harness(range_lookup(8))]
pub fn squeeze<const N: usize>(
    chip: &PoseidonChip<F>,
    layouter: &mut impl Layouter<F>,
    state: LoadedPoseidonState<F, N>,
) -> Result<AssignedNative<F>, Error> {
    let mut state: AssignedPoseidonState<F> = state.into();
    chip.squeeze(layouter, &mut state)
}
