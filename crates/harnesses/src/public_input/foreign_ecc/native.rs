use crate::{utils::range_lookup, utils::vec2array};
use mdnt_extractor_core::chips::{Afp, Fecn};
use mdnt_extractor_core::fields::G1 as G;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::curves::CircuitCurve,
    field::foreign::params::{FieldEmulationParams, MultiEmulationParams},
    instructions::PublicInputInstructions as _,
    types::{AssignedNative, InnerValue as _},
};
use midnight_proofs::plonk::Error;

type F = mdnt_extractor_core::fields::Blstrs;
const NB_LIMBS: usize =
    <MultiEmulationParams as FieldEmulationParams<F, <G as CircuitCurve>::Base>>::NB_LIMBS as usize;

#[entry("public-input/as_public_input/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn as_public_input_native(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    assigned: Afp<F, G>,
) -> Result<[AssignedNative<F>; NB_LIMBS + 1], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_native(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    assigned: Afp<F, G>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_native(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    assigned: Afp<F, G>,
) -> Result<Afp<F, G>, Error> {
    chip.assign_as_public_input(layouter, assigned.value())
}
