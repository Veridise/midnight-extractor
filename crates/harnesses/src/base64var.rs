use crate::utils::range_lookup;
use mdnt_extractor_core::entry;
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::base64::{Base64VarInstructions as _, Base64Vec},
    parsing::Base64Chip,
    types::AssignedByte,
    vec::AssignedVector,
};
use midnight_proofs::{circuit::Value, plonk::Error};

entry!(
    "base64var/assign_var_base64_16_5/base64/byte",
    assign_var_base64::<16, 5>
);
#[harness(range_lookup(8))]
pub fn assign_var_base64<const M: usize, const A: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: [u8; M],
) -> Result<Base64Vec<F, M, A>, Error> {
    chip.assign_var_base64(layouter, input.into_iter().map(Value::known).collect())
}

entry!(
    "base64var/base64_from_vec_16_5/base64/byte",
    base64_from_vec::<16, 5>
);
#[harness(range_lookup(8))]
pub fn base64_from_vec<const M: usize, const A: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: AssignedVector<F, AssignedByte<F>, M, A>,
) -> Result<Base64Vec<F, M, A>, Error> {
    chip.base64_from_vec(layouter, &input)
}

entry!(
    "base64var/var_decode_base64url_16_5_4_1/base64/byte",
    var_decode_base64url::<16, 5, 4, 1>
);
#[harness(range_lookup(8))]
pub fn var_decode_base64url<
    const M: usize,
    const A: usize,
    const MOUT: usize,
    const AOUT: usize,
>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: Base64Vec<F, M, A>,
) -> Result<AssignedVector<F, AssignedByte<F>, MOUT, AOUT>, Error> {
    chip.var_decode_base64url(layouter, &input)
}

entry!(
    "base64var/var_decode_base64_16_5_4_1/base64/byte",
    var_decode_base64::<16, 5, 4, 1>
);
#[harness(range_lookup(8))]
pub fn var_decode_base64<const M: usize, const A: usize, const MOUT: usize, const AOUT: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: Base64Vec<F, M, A>,
) -> Result<AssignedVector<F, AssignedByte<F>, MOUT, AOUT>, Error> {
    chip.var_decode_base64(layouter, &input)
}
