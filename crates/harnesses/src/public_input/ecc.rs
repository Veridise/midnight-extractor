use crate::{utils::range_lookup, utils::vec2array};
use mdnt_extractor_core::chips::ecc::EccChipAdaptor;
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::native::AssignedScalarOfNativeCurve as ScalarVar,
    instructions::PublicInputInstructions as _,
    types::{AssignedNative, AssignedNativePoint, InnerValue as _},
};
use midnight_proofs::plonk::Error;

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type F = mdnt_extractor_core::fields::Blstrs;

#[entry("public-input/as_public_input/ecc/point")]
#[harness(range_lookup(8))]
pub fn as_public_input_native(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNativePoint<C>,
) -> Result<[AssignedNative<F>; 2], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/ecc/point")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_native(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNativePoint<C>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/ecc/point")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_native(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.assign_as_public_input(layouter, assigned.value())
}

#[entry("public-input/as_public_input/ecc/scalar")]
#[harness(range_lookup(8))]
pub fn as_public_input_scalar(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: ScalarVar<C>,
) -> Result<[AssignedNative<F>; 2], Error> {
    chip.as_public_input(layouter, &assigned).and_then(vec2array)
}

#[entry("public-input/constrain_as_public_input/ecc/scalar")]
#[harness(range_lookup(8))]
pub fn constrain_as_public_input_scalar(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: ScalarVar<C>,
) -> Result<(), Error> {
    chip.constrain_as_public_input(layouter, &assigned)
}

#[entry("public-input/assign_as_public_input/ecc/scalar")]
#[harness(range_lookup(8))]
pub fn assign_as_public_input_scalar(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    assigned: ScalarVar<C>,
) -> Result<ScalarVar<C>, Error> {
    chip.assign_as_public_input(layouter, assigned.value())
}
