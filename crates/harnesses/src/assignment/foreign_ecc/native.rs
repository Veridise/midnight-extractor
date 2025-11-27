use crate::utils::range_lookup;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L, G1 as G};
use mdnt_extractor_core::{
    cells::load::LoadedG1,
    chips::{Afp, Fecn},
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::AssignmentInstructions, midnight_proofs::plonk::Error, types::AssignedNative,
};
use midnight_proofs::circuit::Value;

#[entry("assignment/assign/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn assign_field_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    x: LoadedG1,
) -> Result<Afp<F, G>, Error> {
    chip.assign(layouter, Value::known(x.into()))
}

#[entry("assignment/assign_fixed/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn assign_fixed_field_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    x: LoadedG1,
) -> Result<Afp<F, G>, Error> {
    chip.assign_fixed(layouter, x.into())
}

#[entry("assignment/assign/foreign-ecc-native/native")]
#[harness(range_lookup(8))]
pub fn assign_native_foreign_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    x: L<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign(layouter, Value::known(x.0))
}

#[entry("assignment/assign_fixed/foreign-ecc-native/native")]
#[harness(range_lookup(8))]
pub fn assign_fixed_native_foreign_ecc(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    x: L<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.assign_fixed(layouter, x.0)
}
