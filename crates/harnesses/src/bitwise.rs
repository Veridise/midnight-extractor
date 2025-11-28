use crate::utils::range_lookup;
use ff::PrimeField;
use mdnt_extractor_core::{chips::NG, entry};
use mdnt_extractor_macros::{harness, harness_with_args};
use midnight_circuits::{
    compact_std_lib::ZkStdLib, instructions::BitwiseInstructions as _,
    midnight_proofs::plonk::Error, types::AssignedNative,
};

entry!("bitwise/bnot/native-gadget/native", bnot_native);
entry!("bitwise/band/native-gadget/native", band_native);
entry!("bitwise/bor/native-gadget/native", bor_native);
entry!("bitwise/bxor/native-gadget/native", bxor_native);

entry!("bitwise/bnot/stdlib/native", bnot_stdlib);
entry!("bitwise/band/stdlib/native", band_stdlib);
entry!("bitwise/bor/stdlib/native", bor_stdlib);
entry!("bitwise/bxor/stdlib/native", bxor_stdlib);

type F = mdnt_extractor_core::fields::Blstrs;

#[harness(range_lookup(8))]
pub fn bnot_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.bnot(layouter, &x, (F::NUM_BITS - 1).try_into().unwrap())
}
#[harness(range_lookup(8))]
pub fn band_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.band(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}
#[harness(range_lookup(8))]
pub fn bor_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.bor(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}
#[harness(range_lookup(8))]
pub fn bxor_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.bxor(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}

fn bnot_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn bnot_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    x: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.bnot(layouter, &x, (F::NUM_BITS - 1).try_into().unwrap())
}
fn band_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn band_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.band(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}
fn bor_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn bor_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.bor(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}
fn bxor_stdlib_args() -> usize {
    8
}
#[harness_with_args(usize, range_lookup(8))]
pub fn bxor_stdlib(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y): (AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.bxor(layouter, &x, &y, F::NUM_BITS.try_into().unwrap())
}
