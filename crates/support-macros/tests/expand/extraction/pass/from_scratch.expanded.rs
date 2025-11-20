use mdnt_support_macros::InitFromScratch;
struct S2<F> {
    f: F,
}
impl<__Layouter, F> extractor_support::circuit::CircuitInitialization<__Layouter>
for S2<F>
where
    __Layouter: midnight_proofs::circuit::Layouter<F>,
    F: ff::PrimeField,
{
    type Config = <S2<F> as crate::testing_utils::FromScratch<F>>::Config;
    type Args = ();
    type ConfigCols = [midnight_proofs::plonk::Column<
        midnight_proofs::plonk::Instance,
    >; 2];
    type CS = midnight_proofs::plonk::ConstraintSystem<F>;
    type Error = midnight_proofs::plonk::Error;
    fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
        <S2<F> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
    }
    fn configure_circuit(
        meta: &mut Self::CS,
        instance_columns: &Self::ConfigCols,
    ) -> Self::Config {
        <S2<
            F,
        > as crate::testing_utils::FromScratch<
            F,
        >>::configure_from_scratch(meta, instance_columns)
    }
    fn load_chip(
        &self,
        layouter: &mut L,
        _config: &Self::Config,
    ) -> Result<(), Self::Error> {
        use crate::testing_utils::FromScratch;
        self.load_from_scratch(layouter)
    }
}
