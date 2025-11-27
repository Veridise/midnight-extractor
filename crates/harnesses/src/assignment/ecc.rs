use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Jubjub as C, JubjubFr, Loaded as L};
use mdnt_extractor_core::{cells::load::LoadedJubjubSubgroup, chips::ecc::EccChipAdaptor};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::native::{AssignedScalarOfNativeCurve as ScalarVar, EccChip},
    instructions::AssignmentInstructions,
    midnight_proofs::plonk::Error,
    types::{AssignedNativePoint, InnerValue},
};
use midnight_proofs::circuit::Value;

#[entry("assignment/assign/ecc/point")]
#[harness(range_lookup(8))]
pub fn assign_native_ecc(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.assign(layouter, x.value())
}

#[entry("assignment/assign_fixed/ecc/point")]
#[harness(range_lookup(8))]
pub fn assign_fixed_native_ecc(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    x: LoadedJubjubSubgroup,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.assign_fixed(layouter, x.into())
}

#[entry("assignment/assign/ecc/scalar")]
#[harness(range_lookup(8))]
pub fn assign_scalar_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: L<JubjubFr>,
) -> Result<ScalarVar<C>, Error> {
    chip.assign(layouter, Value::known(x.0))
}

#[entry("assignment/assign_fixed/ecc/scalar")]
#[harness(range_lookup(8))]
pub fn assign_fixed_scalar_ecc(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    x: L<JubjubFr>,
) -> Result<ScalarVar<C>, Error> {
    chip.assign_fixed(layouter, x.0)
}
