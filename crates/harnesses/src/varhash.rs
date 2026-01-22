use crate::utils::{lookup_mux, plain_spread_lookup, range_lookup};
use mdnt_extractor_core::entry;
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    hash::{poseidon::VarLenPoseidonGadget, sha256::VarLenSha256Gadget},
    instructions::hash::VarHashInstructions as _,
    types::{AssignedByte, AssignedNative},
    vec::AssignedVector,
};
use midnight_proofs::plonk::Error;

entry!("varhash/varhash_64/sha256/byte", varhash_sha256::<64>);
entry!("varhash/varhash_100/sha256/byte", varhash_sha256::<100>);
#[harness(lookup_mux::<F>()
            .with("pow2range column check", range_lookup(8))
            .with("plain-spreaded lookup", plain_spread_lookup("Spread", "Unspread"))
)]
pub fn varhash_sha256<const N: usize>(
    chip: &VarLenSha256Gadget<F>,
    layouter: &mut impl Layouter<F>,
    buf: AssignedVector<F, AssignedByte<F>, N, 64>,
) -> Result<[AssignedByte<F>; 32], Error> {
    chip.varhash(layouter, &buf)
}

entry!("varhash/varhash_2/poseidon/native", varhash_poseidon::<2>);
entry!("varhash/varhash_10/poseidon/native", varhash_poseidon::<10>);
#[harness(range_lookup(8))]
pub fn varhash_poseidon<const N: usize>(
    chip: &VarLenPoseidonGadget<F>,
    layouter: &mut impl Layouter<F>,
    buf: AssignedVector<F, AssignedNative<F>, N, 2>,
) -> Result<AssignedNative<F>, Error> {
    chip.varhash(layouter, &buf)
}
