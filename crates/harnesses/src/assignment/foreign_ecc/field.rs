use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Loaded as L;
use mdnt_extractor_core::{
    cells::load::LoadedSecp256k1,
    chips::{Afp, Fecf, AF},
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{instructions::AssignmentInstructions, midnight_proofs::plonk::Error};
use midnight_proofs::circuit::Value;

type S = mdnt_extractor_core::fields::Secp256k1;
type F = mdnt_extractor_core::fields::Blstrs;
type K = mdnt_extractor_core::fields::Secp256k1Fq;

#[entry("assignment/assign/foreign-ecc-field/point")]
#[harness(range_lookup(8))]
pub fn assign_field_ecc(
    chip: &Fecf<F, S>,
    layouter: &mut impl Layouter<F>,
    x: LoadedSecp256k1,
) -> Result<Afp<F, S>, Error> {
    chip.assign(layouter, Value::known(x.into()))
}

#[entry("assignment/assign_fixed/foreign-ecc-field/point")]
#[harness(range_lookup(8))]
pub fn assign_fixed_field_ecc(
    chip: &Fecf<F, S>,
    layouter: &mut impl Layouter<F>,
    x: LoadedSecp256k1,
) -> Result<Afp<F, S>, Error> {
    chip.assign_fixed(layouter, x.into())
}

#[entry("assignment/assign/foreign-ecc-field/field")]
#[harness(range_lookup(8))]
pub fn assign_field_foreign_ecc(
    chip: &Fecf<F, S>,
    layouter: &mut impl Layouter<F>,
    x: L<K>,
) -> Result<AF<F, K>, Error> {
    chip.assign(layouter, Value::known(x.0))
}

#[entry("assignment/assign_fixed/foreign-ecc-field/field")]
#[harness(range_lookup(8))]
pub fn assign_fixed_field_foreign_ecc(
    chip: &Fecf<F, S>,
    layouter: &mut impl Layouter<F>,
    x: L<K>,
) -> Result<AF<F, K>, Error> {
    chip.assign_fixed(layouter, x.0)
}
