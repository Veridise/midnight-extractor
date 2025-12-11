use haloumi::LookupCallbacks;
use mdnt_extractor_core::{entry as add_entry, fields::Blstrs as F};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::types::{AssignedByte, InnerValue as _};
use midnight_proofs::{
    circuit::AssignedCell,
    plonk::{Error, Expression},
};
use sha3_circuit::{
    instructions::Keccackf1600Instructions,
    packed_chip::{AbsorbedBlock, PackedChip},
    sha3_256_gadget::Sha3_256,
};

use crate::utils::vec2array;

type AssignedDenseBits = <PackedChip<F> as Keccackf1600Instructions<F>>::AssignedByte;

fn lookups() -> impl LookupCallbacks<F, Expression<F>> {
    //use crate::utils::{
    //    lookup_mux, plain_spread_lookup4, range_lookup, range_lookup_no_tag,
    //};
    crate::utils::ignore_lookup()
    //lookup_mux()
    //    .with(
    //        |n: &str| n.starts_with("spread byte lookup"),
    //        plain_spread_lookup4("Spread", "Unspread"),
    //    )
    //    .with(
    //        |n: &str| n.starts_with("decomposition lookup"),
    //        lookup_mux()
    //            .with(
    //                |n: &str| {
    //                    n.ends_with("limb 3")
    //                        || n.ends_with("limb 1")
    //                        || n.ends_with("limb 0")
    //                        || n.ends_with("limb 2")
    //                },
    //                range_lookup_no_tag(8),
    //            )
    //            .fallback(range_lookup(13)),
    //    )
}

add_entry!("sha3/digest_1/sha3/byte", sha3_digest::<1>);
#[harness(lookups())]
fn sha3_digest<const N: usize>(
    chip: &Sha3_256<F, PackedChip<F>>,
    layouter: &mut impl Layouter<F>,
    hash_input: [AssignedByte<F>; N],
) -> Result<([AssignedDenseBits; N], [AssignedDenseBits; 32]), Error> {
    let (bytes, digest) = chip.digest(layouter, &hash_input.clone().map(|b| b.value()))?;
    let bytes: [AssignedDenseBits; N] = vec2array(bytes)?;
    layouter.assign_region(
        || "link inputs",
        |mut region| {
            bytes
                .iter()
                .zip(hash_input.iter().cloned().map(AssignedCell::from).map(|c| c.cell()))
                .try_for_each(|(b, i)| region.constrain_equal(b.cell(), i))
        },
    )?;
    Ok((bytes, digest))
}

#[entry("keccakf/assign_message_block/packed/byte")]
#[harness(lookups())]
fn assign_message_block(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    block: [AssignedDenseBits; 136],
) -> Result<AbsorbedBlock<F>, Error> {
    let absorbed_block =
        chip.assign_message_block(layouter, &block.clone().map(|b| b.value().cloned()))?;
    let assigned_dense: [AssignedDenseBits; 136] = vec2array(absorbed_block.clone().into())?;
    layouter.assign_region(
        || "link inputs",
        |mut region| {
            assigned_dense
                .iter()
                .zip(block.iter().cloned().map(AssignedCell::from).map(|c| c.cell()))
                .try_for_each(|(b, i)| region.constrain_equal(b.cell(), i))
        },
    )?;
    Ok(absorbed_block)
}

type State = <PackedChip<F> as Keccackf1600Instructions<F>>::State;

#[entry("keccakf/initialize_and_absorb/packed/byte")]
#[harness(lookups())]
fn initialize_and_absorb(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    absorbed_block: AbsorbedBlock<F>,
) -> Result<State, Error> {
    chip.initialize_and_absorb(layouter, &absorbed_block)
}

#[entry("keccakf/keccakf/packed/byte")]
#[harness(lookups())]
fn keccakf(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    state: State,
) -> Result<State, Error> {
    chip.keccakf(layouter, &state)
}

#[entry("keccakf/keccakf_and_absorb_none/packed/byte")]
#[harness(lookups())]
fn keccakf_and_absorb_none(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    state: State,
) -> Result<State, Error> {
    chip.keccakf_and_absorb(layouter, &state, None)
}

#[entry("keccakf/keccakf_and_absorb_some/packed/byte")]
#[harness(lookups())]
fn keccakf_and_absorb_some(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    (state, block): (State, AbsorbedBlock<F>),
) -> Result<State, Error> {
    chip.keccakf_and_absorb(layouter, &state, Some(&block))
}

#[entry("keccakf/squeeze/packed/byte")]
#[harness(lookups())]
fn squeeze(
    chip: &PackedChip<F>,
    layouter: &mut impl Layouter<F>,
    state: State,
) -> Result<[AssignedDenseBits; 32], Error> {
    chip.squeeze(layouter, &state)
}
