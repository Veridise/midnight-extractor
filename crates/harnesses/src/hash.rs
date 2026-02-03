use crate::utils::{lookup_mux, plain_spread_lookup, plain_spread_lookup_ripemd160, range_lookup};
use mdnt_extractor_core::{chips::{NG, ripemd160::RipeMD160Adaptor, sha256::Sha256Adaptor}, entry};
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    hash::poseidon::PoseidonChip,
    instructions::HashInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedByte, AssignedNative},
};

pub type F = mdnt_extractor_core::fields::Blstrs;

entry!("hash/hash_1/sha256/byte", hash_sha256::<1>);
entry!("hash/hash_10/sha256/byte", hash_sha256::<10>);
#[harness(lookup_mux::<F>()
            .with("pow2range column check", range_lookup(8))
            .with("plain-spreaded lookup",plain_spread_lookup("Spread", "Unspread"))
)]
pub fn hash_sha256<const N: usize>(
    chip: &Sha256Adaptor<F, NG<F>>,
    layouter: &mut impl Layouter<F>,
    buf: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; 32], Error> {
    chip.hash(layouter, &buf)
}

entry!("hash/hash_1/ripemd160/byte", hash_ripemd160::<1>);
entry!("hash/hash_10/ripemd160/byte", hash_ripemd160::<10>);
#[harness(lookup_mux::<F>()
            .with("pow2range column check", range_lookup(8))
            .with("plain-spreaded lookup",plain_spread_lookup_ripemd160("Spread", "Unspread"))
)]
pub fn hash_ripemd160<const N: usize>(
    chip: &RipeMD160Adaptor<F, NG<F>>,
    layouter: &mut impl Layouter<F>,
    buf: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; 20], Error> {
    chip.hash(layouter, &buf)
}

entry!("hash/hash_1/poseidon/native", hash_poseidon::<1>);
entry!("hash/hash_10/poseidon/native", hash_poseidon::<10>);
#[harness(range_lookup(8))]
pub fn hash_poseidon<const N: usize>(
    chip: &PoseidonChip<F>,
    layouter: &mut impl Layouter<F>,
    buf: [AssignedNative<F>; N],
) -> Result<AssignedNative<F>, Error> {
    chip.hash(layouter, &buf)
}
