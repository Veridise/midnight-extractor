use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::{entry, harness_mut};
use midnight_circuits::{instructions::map::MapInstructions as _, types::AssignedNative};
use midnight_proofs::{circuit::Value, plonk::Error};

use crate::utils::range_lookup;
use mdnt_extractor_core::{cells::store::FreshVar, chips::MG};

#[entry("map/init/map/native")]
#[harness_mut(range_lookup(8))]
pub fn init(chip: &mut MG<F>, layouter: &mut impl Layouter<F>, _: ()) -> Result<FreshVar, Error> {
    chip.init(layouter, Value::unknown())?;
    Ok(FreshVar)
}

#[entry("map/insert/map/native")]
#[harness_mut(range_lookup(8))]
pub fn insert(
    chip: &mut MG<F>,
    layouter: &mut impl Layouter<F>,
    (key, value): (AssignedNative<F>, AssignedNative<F>),
) -> Result<FreshVar, Error> {
    chip.init(layouter, Value::unknown())?;
    chip.insert(layouter, &key, &value)?;
    Ok(FreshVar)
}

#[entry("map/get/map/native")]
#[harness_mut(range_lookup(8))]
pub fn get(
    chip: &mut MG<F>,
    layouter: &mut impl Layouter<F>,
    key: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    chip.init(layouter, Value::unknown())?;
    chip.get(layouter, &key)
}
