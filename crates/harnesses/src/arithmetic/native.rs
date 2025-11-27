use mdnt_extractor_macros::harness;
use midnight_circuits::{
    field::NativeChip, instructions::ArithInstructions as _, midnight_proofs::plonk::Error,
    types::AssignedNative,
};

use mdnt_extractor_core::entry;
use mdnt_extractor_core::fields::{Blstrs as F, Loaded as L};

use crate::utils::vec_len_err;

entry!("arithmetic/add/native/native", add_native);
#[harness]
pub fn add_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add(layouter, &x, &y)
}

entry!("arithmetic/add_and_mul/native/native", add_and_mul_native);
#[harness]
pub fn add_and_mul_native(
    chip: &NativeChip<F>,
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

entry!("arithmetic/add_constant/native/native", add_constant_native);
#[harness]
pub fn add_constant_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add_constant(layouter, &x, c.0)
}

entry!(
    "arithmetic/add_constants_1/native/native",
    add_constants_1_native
);
#[harness]
pub fn add_constants_1_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 1], [L<F>; 1]),
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<1, _>)
}

entry!(
    "arithmetic/add_constants_2/native/native",
    add_constants_2_native
);
#[harness]
pub fn add_constants_2_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 2], [L<F>; 2]),
) -> Result<[AssignedNative<F>; 2], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<2, _>)
}

entry!(
    "arithmetic/add_constants_5/native/native",
    add_constants_5_native
);
#[harness]
pub fn add_constants_5_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 5], [L<F>; 5]),
) -> Result<[AssignedNative<F>; 5], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<5, _>)
}

entry!("arithmetic/div/native/native", div_native);
#[harness]
pub fn div_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.div(layouter, &x, &y)
}

entry!("arithmetic/inv/native/native", inv_native);
#[harness]
pub fn inv_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv(layouter, &x)
}

entry!("arithmetic/inv0/native/native", inv0_native);
#[harness]
pub fn inv0_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv0(layouter, &x)
}

entry!(
    "arithmetic/mul_with_const/native/native",
    mul_with_const_native
);
#[harness]
pub fn mul_with_const_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y, c): (AssignedNative<F>, AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, Some(c.0))
}

entry!(
    "arithmetic/mul_by_constant/native/native",
    mul_by_constant_native
);
#[harness]
pub fn mul_by_constant_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul_by_constant(layouter, &x, c.0)
}

entry!("arithmetic/mul_no_const/native/native", mul_no_const_native);
#[harness]
pub fn mul_no_const_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, None)
}

entry!("arithmetic/neg/native/native", neg_native);
#[harness]
pub fn neg_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.neg(layouter, &x)
}

entry!("arithmetic/pow0/native/native", pow0_native);
#[harness]
pub fn pow0_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 0)
}

entry!("arithmetic/pow1/native/native", pow1_native);
#[harness]
pub fn pow1_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 1)
}

entry!("arithmetic/pow2/native/native", pow2_native);
#[harness]
pub fn pow2_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 2)
}

entry!("arithmetic/square/native/native", square_native);
#[harness]
pub fn square_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.square(layouter, &x)
}

entry!("arithmetic/sub/native/native", sub_native);
#[harness]
pub fn sub_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.sub(layouter, &x, &y)
}
