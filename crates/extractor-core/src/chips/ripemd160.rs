use ff::PrimeField;
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::{
    field::{decomposition::chip::P2RDecompositionChip, NativeChip, NativeGadget},
    hash::ripemd160::RipeMD160Chip,
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
pub struct RipeMD160Adaptor<F, C>(RipeMD160Chip<F>, C)
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug;

impl<F, C, L> CircuitInitialization<L> for RipeMD160Adaptor<F, C>
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
        <RipeMD160Chip<F> as CircuitInitialization<L>>::Config,
        <C as CircuitInitialization<L>>::Config,
    );

    type Args = ();

    type ConfigCols = (
        <RipeMD160Chip<F> as CircuitInitialization<L>>::ConfigCols,
        <C as CircuitInitialization<L>>::ConfigCols,
    );
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn new_chip((ripemd160_config, conv_config): &Self::Config, (): Self::Args) -> Self {
        Self(
            <RipeMD160Chip<F> as CircuitInitialization<L>>::new_chip(ripemd160_config, ()),
            C::new_chip(conv_config, ()),
        )
    }

    fn configure_circuit(
        meta: &mut ConstraintSystem<F>,
        (ripemd160_columns, conv_columns): &Self::ConfigCols,
    ) -> Self::Config {
        (
            <RipeMD160Chip<F> as CircuitInitialization<L>>::configure_circuit(meta, ripemd160_columns),
            C::configure_circuit(meta, conv_columns),
        )
    }

    fn load_chip(
        &self,
        layouter: &mut L,
        (ripemd160_config, conv_config): &Self::Config,
    ) -> Result<(), Error> {
        self.0.load_chip(layouter, ripemd160_config)?;
        self.1.load_chip(layouter, conv_config)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > HashCPU<u8, [u8; 20]> for RipeMD160Adaptor<F, C>
{
    fn hash(inputs: &[u8]) -> [u8; 20] {
        <RipeMD160Chip<F> as HashCPU<u8, [u8; 20]>>::hash(inputs)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > HashInstructions<F, AssignedByte<F>, [AssignedByte<F>; 20]> for RipeMD160Adaptor<F, C>
{
    fn hash(
        &self,
        layouter: &mut impl Layouter<F>,
        inputs: &[AssignedByte<F>],
    ) -> Result<[AssignedByte<F>; 20], Error> {
        self.0.hash(layouter, inputs)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    > ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> for RipeMD160Adaptor<F, C>
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
pub struct RipeMD160AdaptorConfig<F, C>
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + FromScratch<F>,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
{
    ripemd160: <RipeMD160Chip<F> as FromScratch<F>>::Config,
    converter: C::Config,
}

impl<F, C> std::fmt::Debug for RipeMD160AdaptorConfig<F, C>
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + FromScratch<F>,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RipeMD160AdaptorConfig")
            .field("ripemd160", &self.ripemd160)
            .field("converter", &self.converter)
            .finish()
    }
}

impl<F, C> FromScratch<F> for RipeMD160Adaptor<F, C>
where
    F: PrimeField,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: FromScratch<F>,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
        + FromScratch<F>
        + Clone
        + std::fmt::Debug,
{
    type Config = RipeMD160AdaptorConfig<F, C>;

    fn new_from_scratch(config: &Self::Config) -> Self {
        Self(
            RipeMD160Chip::new_from_scratch(&config.ripemd160),
            C::new_from_scratch(&config.converter),
        )
    }

    fn configure_from_scratch(
        meta: &mut ConstraintSystem<F>,
        instance_columns: &[Column<Instance>; 2],
    ) -> Self::Config {
        Self::Config {
            ripemd160: RipeMD160Chip::<F>::configure_from_scratch(meta, instance_columns),
            converter: C::configure_from_scratch(meta, instance_columns),
        }
    }

    fn load_from_scratch(&self, layouter: &mut impl Layouter<F>) -> Result<(), Error> {
        self.0.load_from_scratch(layouter)?;
        self.1.load_from_scratch(layouter)
    }
}
