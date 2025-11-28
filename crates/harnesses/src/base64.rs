use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::Base64Instructions as _, parsing::Base64Chip, types::AssignedByte,
};
use midnight_proofs::plonk::Error;

use crate::utils::{range_lookup, vec2array};
use mdnt_extractor_core::entry;

entry!(
    "base64/decode_base64url_padded_16_12/base64/byte",
    decode_base64url_padded::<16, 12>
);
#[harness(range_lookup(8))]
pub fn decode_base64url_padded<const N: usize, const M: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; M], Error> {
    chip.decode_base64url(layouter, &input, true).and_then(vec2array)
}

entry!(
    "base64/decode_base64url_no_pad_15_12/base64/byte",
    decode_base64url_no_pad::<15, 12>
);
#[harness(range_lookup(8))]
pub fn decode_base64url_no_pad<const N: usize, const M: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; M], Error> {
    chip.decode_base64url(layouter, &input, false).and_then(vec2array)
}

entry!(
    "base64/decode_base64_padded_16_12/base64/byte",
    decode_base64_padded::<16, 12>
);
#[harness(range_lookup(8))]
pub fn decode_base64_padded<const N: usize, const M: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; M], Error> {
    chip.decode_base64(layouter, &input, true).and_then(vec2array)
}

entry!(
    "base64/decode_base64_no_pad_15_12/base64/byte",
    decode_base64_no_pad::<15, 12>
);
#[harness(range_lookup(8))]
pub fn decode_base64_no_pad<const N: usize, const M: usize>(
    chip: &Base64Chip<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; M], Error> {
    chip.decode_base64(layouter, &input, false).and_then(vec2array)
}
