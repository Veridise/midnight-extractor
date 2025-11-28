use crate::{utils::range_lookup, utils::vec2array};
use mdnt_extractor_core::chips::{AF, FC};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    field::foreign::params::{FieldEmulationParams, MultiEmulationParams},
    instructions::PublicInputInstructions as _,
    types::{AssignedNative, InnerValue as _},
};
use midnight_proofs::plonk::Error;

type F = mdnt_extractor_core::fields::Blstrs;
type K = mdnt_extractor_core::fields::MidnightFp;
const NB_LIMBS: usize = <MultiEmulationParams as FieldEmulationParams<F, K>>::NB_LIMBS as usize;

#[entry("public-input/as_public_input/field/field")]
#[harness(range_lookup(8))]
pub fn as_public_input_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    assigned: AF<F, K>,
) -> Result<[AssignedNative<F>; NB_LIMBS], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/field/field")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    assigned: AF<F, K>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/field/field")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_field(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    assigned: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.assign_as_public_input(layouter, assigned.value())
}
