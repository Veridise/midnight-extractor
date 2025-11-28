use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    parsing::{automaton_chip::AutomatonChip, StdLibParser},
    types::{AssignedByte, AssignedNative},
};
use midnight_proofs::plonk::Error;

use crate::utils::{range_lookup, vec2array};
use mdnt_extractor_core::entry;

entry!("automaton/parse_5_1/automaton/byte", parse::<5, 1>);
#[harness(range_lookup(8))]
pub fn parse<const N: usize, const M: usize>(
    chip: &AutomatonChip<StdLibParser, F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedNative<F>; M], Error> {
    chip.parse(layouter, &StdLibParser::Jwt, &input).and_then(vec2array)
}
