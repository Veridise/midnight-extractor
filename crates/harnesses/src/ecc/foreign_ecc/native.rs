use crate::utils::range_lookup;
use ff::Field as _;
use mdnt_extractor_core::circuit::to_plonk_error;
use mdnt_extractor_core::{
    cells::load::{BoundedNative, Gt1},
    chips::{Afp, Fecn, AF},
};
use mdnt_extractor_macros::{entry, harness};
use midnight_circuits::{
    instructions::EccInstructions as _, midnight_proofs::plonk::Error, types::AssignedNative,
};

use mdnt_extractor_core::fields::{Loaded as L, MidnightFp as K, G1 as G};

pub type F = mdnt_extractor_core::fields::Blstrs;

use mdnt_extractor_core::entry as add_entry;

#[entry("ecc/add/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn add(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (p, q): (Afp<F, G>, Afp<F, G>),
) -> Result<Afp<F, G>, Error> {
    chip.add(layouter, &p, &q)
}

#[entry("ecc/double/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn double(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    p: Afp<F, G>,
) -> Result<Afp<F, G>, Error> {
    chip.double(layouter, &p)
}

#[entry("ecc/negate/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn negate(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    p: Afp<F, G>,
) -> Result<Afp<F, G>, Error> {
    chip.negate(layouter, &p)
}

add_entry!("ecc/msm_1/foreign-ecc-native/point", msm::<1>);
add_entry!("ecc/msm_5/foreign-ecc-native/point", msm::<5>);
#[harness(range_lookup(8))]
pub fn msm<const N: usize>(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (scalars, bases): ([AssignedNative<F>; N], [Afp<F, G>; N]),
) -> Result<Afp<F, G>, Error> {
    chip.msm(layouter, &scalars, &bases)
}

add_entry!(
    "ecc/msm_by_bounded_scalars_1_8bit/foreign-ecc-native/point",
    msm_by_bounded_scalars::<1, 8>
);
add_entry!(
    "ecc/msm_by_bounded_scalars_5_8bit/foreign-ecc-native/point",
    msm_by_bounded_scalars::<5, 8>
);
#[harness(range_lookup(8))]
pub fn msm_by_bounded_scalars<const N: usize, const BITS: usize>(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (scalars, bases): ([BoundedNative<F, BITS>; N], [Afp<F, G>; N]),
) -> Result<Afp<F, G>, Error> {
    let scalars = scalars.into_iter().map(|s| (s.into(), BITS)).collect::<Vec<_>>();
    chip.msm_by_bounded_scalars(layouter, &scalars, &bases)
}

#[entry("ecc/mul_by_constant/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn mul_by_constant(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (scalar, base): (Gt1<L<F>>, Afp<F, G>),
) -> Result<Afp<F, G>, Error> {
    if scalar == F::ZERO {
        return Err(to_plonk_error(
            "Don't execute 'mul_by_constant' with a constant zero. Use 'mul_by_zero' instead.",
        ));
    }
    if scalar == F::ONE {
        return Err(to_plonk_error(
            "Don't execute 'mul_by_constant' with a constant one. Use 'mul_by_one' instead.",
        ));
    }
    chip.mul_by_constant(layouter, scalar.0 .0, &base)
}

#[entry("ecc/mul_by_zero/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn mul_by_zero(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    base: Afp<F, G>,
) -> Result<Afp<F, G>, Error> {
    chip.mul_by_constant(layouter, F::ZERO, &base)
}

#[entry("ecc/mul_by_one/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn mul_by_one(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    base: Afp<F, G>,
) -> Result<Afp<F, G>, Error> {
    chip.mul_by_constant(layouter, F::ONE, &base)
}

#[entry("ecc/point_from_coordinates/foreign-ecc-native/point")]
#[harness(range_lookup(8))]
pub fn point_from_coordinates(
    chip: &Fecn<F, G>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<Afp<F, G>, Error> {
    chip.point_from_coordinates(layouter, &x, &y)
}
