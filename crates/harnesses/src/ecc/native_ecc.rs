use crate::utils::range_lookup;
use ff::Field as _;
use mdnt_extractor_core::circuit::to_plonk_error;
use mdnt_extractor_core::fields::Loaded;
use mdnt_extractor_core::{
    cells::load::{BoundedScalarVar, Gt1},
    chips::ecc::EccChipAdaptor,
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    ecc::native::{AssignedScalarOfNativeCurve as ScalarVar, EccChip},
    instructions::EccInstructions as _,
    midnight_proofs::plonk::Error,
    types::{AssignedByte, AssignedNative, AssignedNativePoint},
};

pub type C = mdnt_extractor_core::fields::Jubjub;
pub type F = mdnt_extractor_core::fields::Blstrs;
pub type S = mdnt_extractor_core::fields::JubjubFr;

use mdnt_extractor_core::entry as add_entry;

add_entry!(
    "ecc/scalar_from_le_bytes_4/ecc/scalar",
    scalar_from_le_bytes::<4>
);
#[harness(range_lookup(8))]
pub fn scalar_from_le_bytes<const N: usize>(
    chip: &EccChipAdaptor<C>,
    layouter: &mut impl Layouter<F>,
    bytes: [AssignedByte<F>; 4],
) -> Result<ScalarVar<C>, Error> {
    chip.ecc().scalar_from_le_bytes(layouter, &bytes)
}

#[entry("ecc/mul/ecc/point")]
#[harness(range_lookup(8))]
pub fn mul(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (scalar, base): (ScalarVar<C>, AssignedNativePoint<C>),
) -> Result<AssignedNativePoint<C>, Error> {
    chip.mul(layouter, &scalar, &base)
}

#[entry("ecc/add/ecc/point")]
#[harness(range_lookup(8))]
pub fn add(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (p, q): (AssignedNativePoint<C>, AssignedNativePoint<C>),
) -> Result<AssignedNativePoint<C>, Error> {
    chip.add(layouter, &p, &q)
}

#[entry("ecc/double/ecc/point")]
#[harness(range_lookup(8))]
pub fn double(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    p: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.double(layouter, &p)
}

#[entry("ecc/negate/ecc/point")]
#[harness(range_lookup(8))]
pub fn negate(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    p: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.negate(layouter, &p)
}

add_entry!("ecc/msm_1/ecc/point", msm::<1>);
add_entry!("ecc/msm_5/ecc/point", msm::<5>);
#[harness(range_lookup(8))]
pub fn msm<const N: usize>(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (scalars, bases): ([ScalarVar<C>; N], [AssignedNativePoint<C>; N]),
) -> Result<AssignedNativePoint<C>, Error> {
    chip.msm(layouter, &scalars, &bases)
}

add_entry!(
    "ecc/msm_by_bounded_scalars_1_8bit/ecc/point",
    msm_by_bounded_scalars::<1, 8>
);
add_entry!(
    "ecc/msm_by_bounded_scalars_5_8bit/ecc/point",
    msm_by_bounded_scalars::<5, 8>
);
#[harness(range_lookup(8))]
pub fn msm_by_bounded_scalars<const N: usize, const BITS: usize>(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (scalars, bases): ([BoundedScalarVar<C, BITS>; N], [AssignedNativePoint<C>; N]),
) -> Result<AssignedNativePoint<C>, Error> {
    let scalars = scalars.into_iter().map(|s| (s.into(), BITS)).collect::<Vec<_>>();
    chip.msm_by_bounded_scalars(layouter, &scalars, &bases)
}

#[entry("ecc/mul_by_constant/ecc/point")]
#[harness(range_lookup(8))]
pub fn mul_by_constant(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (scalar, base): (Gt1<Loaded<S>>, AssignedNativePoint<C>),
) -> Result<AssignedNativePoint<C>, Error> {
    if scalar == S::ZERO {
        return Err(to_plonk_error(
            "Don't execute 'mul_by_constant' with a constant zero. Use 'mul_by_zero' instead.",
        ));
    }
    if scalar == S::ONE {
        return Err(to_plonk_error(
            "Don't execute 'mul_by_constant' with a constant one. Use 'mul_by_one' instead.",
        ));
    }
    chip.mul_by_constant(layouter, scalar.0 .0, &base)
}

#[entry("ecc/mul_by_zero/ecc/point")]
#[harness(range_lookup(8))]
pub fn mul_by_zero(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    base: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.mul_by_constant(layouter, S::ZERO, &base)
}

#[entry("ecc/mul_by_one/ecc/point")]
#[harness(range_lookup(8))]
pub fn mul_by_one(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    base: AssignedNativePoint<C>,
) -> Result<AssignedNativePoint<C>, Error> {
    chip.mul_by_constant(layouter, S::ONE, &base)
}

#[entry("ecc/point_from_coordinates/ecc/point")]
#[harness(range_lookup(8))]
pub fn point_from_coordinates(
    chip: &EccChip<C>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNativePoint<C>, Error> {
    chip.point_from_coordinates(layouter, &x, &y)
}
