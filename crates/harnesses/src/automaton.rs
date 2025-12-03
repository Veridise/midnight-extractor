use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    parsing::{automaton_chip::AutomatonChip, StdLibParser},
    types::{AssignedByte, AssignedNative},
};
use midnight_proofs::plonk::Error;

use crate::utils::{automaton, lookup_mux, range_lookup, vec2array};
use mdnt_extractor_core::entry;

entry!("automaton/parse_5/automaton/byte", parse::<5>);
#[harness(lookup_mux().with("pow2range column check", range_lookup(8)).with("automaton transition check", automaton("Automaton", 8)))]
pub fn parse<const N: usize>(
    chip: &AutomatonChip<StdLibParser, F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedNative<F>; N], Error> {
    chip.parse(layouter, &StdLibParser::Jwt, &input).and_then(vec2array)
}
