use crate::utils::range_lookup;
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_core::{chips::BG, entry};
use mdnt_extractor_macros::unit_harness;
use midnight_circuits::biguint::extraction::LoadedBigUint;
use midnight_circuits::instructions::AssertionInstructions as _;
use num_bigint::BigUint;

entry!(
    "assertion/assert_equal_64bits/biguint/biguint",
    assert_equal_biguint_gadget_64
);
entry!(
    "assertion/assert_equal_to_fixed_64bits/biguint/biguint",
    assert_equal_to_fixed_biguint_gadget_64
);
entry!(
    "assertion/assert_not_equal_64bits/biguint/biguint",
    assert_not_equal_biguint_gadget_64
);
entry!(
    "assertion/assert_not_equal_to_fixed_64bits/biguint/biguint",
    assert_not_equal_to_fixed_biguint_gadget_64
);

entry!(
    "assertion/assert_equal_300bits/biguint/biguint",
    assert_equal_biguint_gadget_300
);
entry!(
    "assertion/assert_equal_to_fixed_300bits/biguint/biguint",
    assert_equal_to_fixed_biguint_gadget_300
);
entry!(
    "assertion/assert_not_equal_300bits/biguint/biguint",
    assert_not_equal_biguint_gadget_300
);
entry!(
    "assertion/assert_not_equal_to_fixed_300bits/biguint/biguint",
    assert_not_equal_to_fixed_biguint_gadget_300
);

#[unit_harness(range_lookup(8))]
pub fn assert_equal_biguint_gadget_64(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, 64>,
    y: LoadedBigUint<F, 64>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x.into(), &y.into())
}

#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_biguint_gadget_64(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, 64>,
    y: LoadedBigUint<F, 64>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x.into(), &y.into())
}

#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_biguint_gadget_64(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    f: BigUint,
    y: LoadedBigUint<F, 64>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y.into(), f)
}

#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_biguint_gadget_64(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    f: BigUint,
    y: LoadedBigUint<F, 64>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y.into(), f)
}

#[unit_harness(range_lookup(8))]
pub fn assert_equal_biguint_gadget_300(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, 300>,
    y: LoadedBigUint<F, 300>,
) -> Result<(), Error> {
    chip.assert_equal(layouter, &x.into(), &y.into())
}

#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_biguint_gadget_300(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    x: LoadedBigUint<F, 300>,
    y: LoadedBigUint<F, 300>,
) -> Result<(), Error> {
    chip.assert_not_equal(layouter, &x.into(), &y.into())
}

#[unit_harness(range_lookup(8))]
pub fn assert_equal_to_fixed_biguint_gadget_300(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    f: BigUint,
    y: LoadedBigUint<F, 300>,
) -> Result<(), Error> {
    chip.assert_equal_to_fixed(layouter, &y.into(), f)
}

#[unit_harness(range_lookup(8))]
pub fn assert_not_equal_to_fixed_biguint_gadget_300(
    chip: &BG<F>,
    layouter: &mut impl Layouter<F>,
    f: BigUint,
    y: LoadedBigUint<F, 300>,
) -> Result<(), Error> {
    chip.assert_not_equal_to_fixed(layouter, &y.into(), f)
}
