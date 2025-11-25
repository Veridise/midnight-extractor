use ff::PrimeField;
use midnight_circuits::{
    midnight_proofs::{
        circuit::Layouter,
        plonk::{Column, ConstraintSystem, Instance},
    },
    testing_utils::FromScratch,
};
use midnight_proofs::plonk::Error;

use super::NativeGadgetAdaptor;

impl<F, N> FromScratch<F> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: FromScratch<F>,
{
    type Config = N::Config;

    fn new_from_scratch(config: &Self::Config) -> Self {
        Self {
            inner: N::new_from_scratch(config),
            injected_ir: Default::default(),
        }
    }

    fn configure_from_scratch(
        meta: &mut ConstraintSystem<F>,
        instance_columns: &[Column<Instance>; 2],
    ) -> Self::Config {
        N::configure_from_scratch(meta, instance_columns)
    }

    fn load_from_scratch(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        self.inner.load_from_scratch(layouter)
    }
}
