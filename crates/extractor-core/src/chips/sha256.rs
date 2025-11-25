use ff::PrimeField;
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::{
    field::{decomposition::chip::P2RDecompositionChip, NativeChip, NativeGadget},
    hash::sha256::Sha256Chip,
    instructions::{hash::HashCPU, ConversionInstructions, HashInstructions},
    midnight_proofs::{
        circuit::Layouter,
        plonk::{Column, ConstraintSystem, Error, Instance},
    },
    testing_utils::FromScratch,
    types::{AssignedByte, AssignedNative},
};

/// An adaptor wrapping Sha256 that implements conversion between AssignedNative and AssignedByte.
#[derive(Clone, Debug)]
pub struct Sha256Adaptor<F, C>(Sha256Chip<F>, C)
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug;

impl<F, C, L> CircuitInitialization<L> for Sha256Adaptor<F, C>
where
    F: PrimeField,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
        + Clone
        + std::fmt::Debug
        + CircuitInitialization<L, Args = (), CS = ConstraintSystem<F>, Error = Error>,
    L: Layouter<F>,
{
    type Config = (
        <Sha256Chip<F> as CircuitInitialization<L>>::Config,
        <C as CircuitInitialization<L>>::Config,
    );

    type Args = ();

    type ConfigCols = (
        <Sha256Chip<F> as CircuitInitialization<L>>::ConfigCols,
        <C as CircuitInitialization<L>>::ConfigCols,
    );
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn new_chip((sha256_config, conv_config): &Self::Config, (): Self::Args) -> Self {
        Self(
            <Sha256Chip<F> as CircuitInitialization<L>>::new_chip(sha256_config, ()),
            C::new_chip(conv_config, ()),
        )
    }

    fn configure_circuit(
        meta: &mut ConstraintSystem<F>,
        (sha256_columns, conv_columns): &Self::ConfigCols,
    ) -> Self::Config {
        (
            <Sha256Chip<F> as CircuitInitialization<L>>::configure_circuit(meta, sha256_columns),
            C::configure_circuit(meta, conv_columns),
        )
    }

    fn load_chip(
        &self,
        layouter: &mut L,
        (sha256_config, conv_config): &Self::Config,
    ) -> Result<(), Error> {
        self.0.load_chip(layouter, sha256_config)?;
        self.1.load_chip(layouter, conv_config)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > HashCPU<u8, [u8; 32]> for Sha256Adaptor<F, C>
{
    fn hash(inputs: &[u8]) -> [u8; 32] {
        <Sha256Chip<F> as HashCPU<u8, [u8; 32]>>::hash(inputs)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > HashInstructions<F, AssignedByte<F>, [AssignedByte<F>; 32]> for Sha256Adaptor<F, C>
{
    fn hash(
        &self,
        layouter: &mut impl Layouter<F>,
        inputs: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<F>; 32], Error> {
        self.0.hash(layouter, inputs)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> for Sha256Adaptor<F, C>
{
    fn convert_value(&self, x: &F) -> Option<u8> {
        self.1.convert_value(x)
    }

    fn convert(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedNative<F>,
    ) -> Result<AssignedByte<F>, Error> {
        self.1.convert(layouter, x)
    }
}

#[derive(Clone)]
pub struct Sha256AdaptorConfig<F, C>
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + FromScratch<F>,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
{
    sha256: <Sha256Chip<F> as FromScratch<F>>::Config,
    converter: C::Config,
}

impl<F, C> std::fmt::Debug for Sha256AdaptorConfig<F, C>
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + FromScratch<F>,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sha256AdaptorConfig")
            .field("sha256", &self.sha256)
            .field("converter", &self.converter)
            .finish()
    }
}

impl<F, C> FromScratch<F> for Sha256Adaptor<F, C>
where
    F: PrimeField,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
        + FromScratch<F>
        + Clone
        + std::fmt::Debug,
{
    type Config = Sha256AdaptorConfig<F, C>;

    fn new_from_scratch(config: &Self::Config) -> Self {
        Self(
            Sha256Chip::new_from_scratch(&config.sha256),
            C::new_from_scratch(&config.converter),
        )
    }

    fn configure_from_scratch(
        meta: &mut ConstraintSystem<F>,
        instance_columns: &[Column<Instance>; 2],
    ) -> Self::Config {
        Self::Config {
            sha256: Sha256Chip::<F>::configure_from_scratch(meta, instance_columns),
            converter: C::configure_from_scratch(meta, instance_columns),
        }
    }

    fn load_from_scratch(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        self.0.load_from_scratch(layouter)?;
        self.1.load_from_scratch(layouter)
    }
}
