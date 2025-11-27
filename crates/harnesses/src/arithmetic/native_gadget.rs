use crate::arithmetic::vec_len_err;
use crate::utils::range_lookup;
use mdnt_extractor_core::entry;
use mdnt_extractor_macros::harness;
use midnight_circuits::{
    instructions::ArithInstructions as _, midnight_proofs::plonk::Error, types::AssignedNative,
};

use mdnt_extractor_core::chips::NG;
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_core::fields::Loaded as L;

entry!("arithmetic/add/native-gadget/native", add_native_gadget);
entry!(
    "arithmetic/add_and_mul/native-gadget/native",
    add_and_mul_native_gadget
);
entry!(
    "arithmetic/add_constant/native-gadget/native",
    add_constant_native_gadget
);
entry!(
    "arithmetic/add_constants_1/native-gadget/native",
    add_constants_1_native_gadget
);
entry!(
    "arithmetic/add_constants_2/native-gadget/native",
    add_constants_2_native_gadget
);
entry!(
    "arithmetic/add_constants_5/native-gadget/native",
    add_constants_5_native_gadget
);
entry!("arithmetic/div/native-gadget/native", div_native_gadget);
entry!("arithmetic/inv/native-gadget/native", inv_native_gadget);
entry!("arithmetic/inv0/native-gadget/native", inv0_native_gadget);
entry!(
    "arithmetic/mul_by_constant/native-gadget/native",
    mul_by_constant_native_gadget
);
entry!(
    "arithmetic/mul_no_const/native-gadget/native",
    mul_no_const_native_gadget
);
entry!(
    "arithmetic/mul_with_const/native-gadget/native",
    mul_with_const_native_gadget
);
entry!("arithmetic/neg/native-gadget/native", neg_native_gadget);
entry!("arithmetic/pow0/native-gadget/native", pow0_native_gadget);
entry!("arithmetic/pow1/native-gadget/native", pow1_native_gadget);
entry!("arithmetic/pow2/native-gadget/native", pow2_native_gadget);
entry!(
    "arithmetic/square/native-gadget/native",
    square_native_gadget
);
entry!("arithmetic/sub/native-gadget/native", sub_native_gadget);

#[harness(range_lookup(8))]
pub fn add_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn add_and_mul_native_gadget(
    chip: &NG<F>,
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

#[harness(range_lookup(8))]
pub fn add_constant_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.add_constant(layouter, &x, c.0)
}

#[harness(range_lookup(8))]
pub fn add_constants_1_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 1], [L<F>; 1]),
) -> Result<[AssignedNative<F>; 1], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<1, _>)
}

#[harness(range_lookup(8))]
pub fn add_constants_2_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 2], [L<F>; 2]),
) -> Result<[AssignedNative<F>; 2], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<2, _>)
}

#[harness(range_lookup(8))]
pub fn add_constants_5_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (xs, cs): ([AssignedNative<F>; 5], [L<F>; 5]),
) -> Result<[AssignedNative<F>; 5], Error> {
    chip.add_constants(layouter, &xs, &cs.map(|f| f.0))?
        .try_into()
        .map_err(vec_len_err::<5, _>)
}

#[harness(range_lookup(8))]
pub fn div_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.div(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn inv_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn inv0_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.inv0(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn mul_by_constant_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, c): (AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul_by_constant(layouter, &x, c.0)
}

#[harness(range_lookup(8))]
pub fn mul_no_const_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, None)
}

#[harness(range_lookup(8))]
pub fn neg_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.neg(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn pow0_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 0)
}

#[harness(range_lookup(8))]
pub fn mul_with_const_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y, c): (AssignedNative<F>, AssignedNative<F>, L<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.mul(layouter, &x, &y, Some(c.0))
}

#[harness(range_lookup(8))]
pub fn pow1_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 1)
}

#[harness(range_lookup(8))]
pub fn pow2_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.pow(layouter, &x, 2)
}

#[harness(range_lookup(8))]
pub fn square_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.square(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn sub_native_gadget(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.sub(layouter, &x, &y)
}
