use crate::{utils::range_lookup, utils::vec_len_err};
use mdnt_extractor_core::entry;
use mdnt_extractor_macros::harness_with_args;
use midnight_circuits::{
    compact_std_lib::ZkStdLib, instructions::ArithInstructions as _, midnight_proofs::plonk::Error,
    types::AssignedNative,
};

use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};

entry!("arithmetic/add/stdlib/native", add_stdlib);
entry!("arithmetic/add_and_mul/stdlib/native", add_and_mul_stdlib);
entry!("arithmetic/add_constant/stdlib/native", add_constant_stdlib);
entry!(
    "arithmetic/add_constants_1/stdlib/native",
    add_constants_1_stdlib
);
entry!(
    "arithmetic/add_constants_2/stdlib/native",
    add_constants_2_stdlib
);
entry!(
    "arithmetic/add_constants_5/stdlib/native",
    add_constants_5_stdlib
);
entry!("arithmetic/div/stdlib/native", div_stdlib);
entry!("arithmetic/inv/stdlib/native", inv_stdlib);
entry!("arithmetic/inv0/stdlib/native", inv0_stdlib);
entry!(
    "arithmetic/mul_by_constant/stdlib/native",
    mul_by_constant_stdlib
);
entry!("arithmetic/mul_no_const/stdlib/native", mul_no_const_stdlib);
entry!(
    "arithmetic/mul_with_const/stdlib/native",
    mul_with_const_stdlib
);
entry!("arithmetic/neg/stdlib/native", neg_stdlib);
entry!("arithmetic/pow0/stdlib/native", pow0_stdlib);
entry!("arithmetic/pow1/stdlib/native", pow1_stdlib);
entry!("arithmetic/pow2/stdlib/native", pow2_stdlib);
entry!("arithmetic/square/stdlib/native", square_stdlib);
entry!("arithmetic/sub/stdlib/native", sub_stdlib);

fn add_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add(layouter, &x, &y)
}

fn add_and_mul_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_and_mul_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    ((a, x), (b, y), (c, z), k, m): (
        (L<F>, AssignedNative<F>),
        (L<F>, AssignedNative<F>),
        (L<F>, AssignedNative<F>),
        L<F>,
        L<F>,
    ),
) -> Result<AssignedNative<F>, Error> {
    chip.add_and_mul(layouter, (a.0, &x), (b.0, &y), (c.0, &z), k.0, m.0)
}

fn add_constant_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_constant_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add_constant(layouter, &x, c.0)
}

fn add_constants_1_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_constants_1_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 1], [L<F>; 1]),
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<1, _>)
}

fn add_constants_2_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_constants_2_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 2], [L<F>; 2]),
) -> Result<[AssignedNative<F>; 2], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<2, _>)
}

fn add_constants_5_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn add_constants_5_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 5], [L<F>; 5]),
) -> Result<[AssignedNative<F>; 5], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<5, _>)
}

fn div_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn div_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.div(layouter, &x, &y)
}

fn inv_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn inv_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv(layouter, &x)
}

fn inv0_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn inv0_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv0(layouter, &x)
}

fn mul_by_constant_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn mul_by_constant_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul_by_constant(layouter, &x, c.0)
}

fn mul_no_const_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn mul_no_const_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, None)
}

fn neg_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn neg_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.neg(layouter, &x)
}

fn pow0_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn pow0_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 0)
}

fn mul_with_const_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn mul_with_const_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y, c): (AssignedNative<F>, AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, Some(c.0))
}

fn pow1_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn pow1_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 1)
}

fn pow2_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn pow2_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 2)
}

fn square_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn square_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.square(layouter, &x)
}

fn sub_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn sub_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.sub(layouter, &x, &y)
}
