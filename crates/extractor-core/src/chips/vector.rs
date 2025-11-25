use ff::PrimeField;
use midnight_circuits::{
    field::{decomposition::chip::P2RDecompositionChip, NativeChip, NativeGadget},
    instructions::{
        ArithInstructions, BinaryInstructions, ConversionInstructions, EqualityInstructions,
    },
    types::{AssignedBit, AssignedByte, AssignedNative, InnerValue},
    vec::{vector_gadget::VectorGadget, AssignedVector, Vectorizable},
};
use midnight_proofs::{
    circuit::Layouter,
    plonk::{ConstraintSystem, Error},
};

use mdnt_support::circuit::CircuitInitialization;

/// An adaptor wrapping [`VectorGadget`] that implements conversion between AssignedNative and AssignedByte.
#[derive(Clone, Debug)]
pub struct VectorGadgetAdaptor<F, C>(pub VectorGadget<F>, C)
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug;

impl<F, C, L> CircuitInitialization<L> for VectorGadgetAdaptor<F, C>
where
    F: PrimeField,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
        + Clone
        + std::fmt::Debug
        + CircuitInitialization<L, CS = ConstraintSystem<F>, Error = Error>,
    L: Layouter<F>,
{
    type Config = (
        <VectorGadget<F> as CircuitInitialization<L>>::Config,
        <C as CircuitInitialization<L>>::Config,
    );

    type Args = C::Args;

    type ConfigCols = (
        <VectorGadget<F> as CircuitInitialization<L>>::ConfigCols,
        <C as CircuitInitialization<L>>::ConfigCols,
    );
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn new_chip((vec_config, conv_config): &Self::Config, args: Self::Args) -> Self {
        Self(
            <VectorGadget<F> as CircuitInitialization<L>>::new_chip(vec_config, ()),
            C::new_chip(conv_config, args),
        )
    }

    fn configure_circuit(
        meta: &mut ConstraintSystem<F>,
        (vec_columns, conv_columns): &Self::ConfigCols,
    ) -> Self::Config {
        (
            <VectorGadget<F> as CircuitInitialization<L>>::configure_circuit(meta, vec_columns),
            C::configure_circuit(meta, conv_columns),
        )
    }

    fn load_chip(
        &self,
        layouter: &mut L,
        (vec_config, conv_config): &Self::Config,
    ) -> Result<(), Error> {
        self.0.load_chip(layouter, vec_config)?;
        self.1.load_chip(layouter, conv_config)
    }
}

impl<F, C, const M: usize, T, const A: usize> EqualityInstructions<F, AssignedVector<F, T, M, A>>
    for VectorGadgetAdaptor<F, C>
where
    F: PrimeField,
    T: Vectorizable,
    T::Element: Copy,
    C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> + Clone + std::fmt::Debug,
    VectorGadget<F>: EqualityInstructions<F, AssignedVector<F, T, M, A>>,
    NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>: ArithInstructions<F, AssignedNative<F>>
        + EqualityInstructions<F, T>
        + EqualityInstructions<F, AssignedNative<F>>
        + BinaryInstructions<F>,
{
    fn is_equal(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedVector<F, T, M, A>,
        y: &AssignedVector<F, T, M, A>,
    ) -> Result<AssignedBit<F>, Error> {
        self.0.is_equal(layouter, x, y)
    }

    fn is_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedVector<F, T, M, A>,
        constant: Vec<<T as InnerValue>::Element>,
    ) -> Result<AssignedBit<F>, Error> {
        self.0.is_equal_to_fixed(layouter, x, constant)
    }

    fn is_not_equal(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedVector<F, T, M, A>,
        y: &AssignedVector<F, T, M, A>,
    ) -> Result<AssignedBit<F>, Error> {
        self.0.is_not_equal(layouter, x, y)
    }

    fn is_not_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedVector<F, T, M, A>,
        constant: <AssignedVector<F, T, M, A> as InnerValue>::Element,
    ) -> Result<AssignedBit<F>, Error> {
        self.0.is_not_equal_to_fixed(layouter, x, constant)
    }
}

impl<
        F: PrimeField,
        C: ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>>
            + Clone
            + std::fmt::Debug
            + CircuitInitialization<F>,
    > ConversionInstructions<F, AssignedNative<F>, AssignedByte<F>> for VectorGadgetAdaptor<F, C>
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
