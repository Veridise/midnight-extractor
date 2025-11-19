//! Macros for supporting extraction.

/// Implements [`crate::circuit::CircuitInitialization`] for the given type
/// based on its FromScratch implementation.
#[macro_export]
macro_rules! circuit_initialization_from_scratch {
    ($C:ty, $F:ident $(, $other:ident)* $(where $($conds:tt)+ )?) => {
        impl<L, $F, $( $other,)*> $crate::circuit::CircuitInitialization<$F> for $C
        where
            L: midnight_proofs::circuit::Layouter<$F>,
            $F: ff::PrimeField,
            $C: crate::testing_utils::FromScratch<$F>
            $(, $($conds)+)?
        {
            type Config = <$C as crate::testing_utils::FromScratch<$F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<$F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <$C as crate::testing_utils::FromScratch<$F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <$C as crate::testing_utils::FromScratch<$F>>::configure_from_scratch(meta, instance_columns)
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
    };
}
