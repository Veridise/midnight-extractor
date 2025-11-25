use ff::PrimeField;
use midnight_circuits::{
    instructions::ConversionInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::Error,
    },
    types::{AssignedBit, AssignedByte, InnerValue},
};

use super::NativeGadgetAdaptor;

impl<F, N> ConversionInstructions<F, AssignedBit<F>, AssignedCell<F, F>>
    for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ConversionInstructions<F, AssignedBit<F>, AssignedCell<F, F>>,
{
    fn convert_value(
        &self,
        x: &<AssignedBit<F> as InnerValue>::Element,
    ) -> Option<<AssignedCell<F, F> as InnerValue>::Element> {
        self.inner.convert_value(x)
    }

    fn convert(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedBit<F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.convert(layouter, x)
    }
}

impl<F, N> ConversionInstructions<F, AssignedByte<F>, AssignedCell<F, F>>
    for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ConversionInstructions<F, AssignedByte<F>, AssignedCell<F, F>>,
{
    fn convert_value(
        &self,
        x: &<AssignedByte<F> as InnerValue>::Element,
    ) -> Option<<AssignedCell<F, F> as InnerValue>::Element> {
        self.inner.convert_value(x)
    }

    fn convert(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedByte<F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.convert(layouter, x)
    }
}

impl<F, N> ConversionInstructions<F, AssignedCell<F, F>, AssignedByte<F>>
    for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ConversionInstructions<F, AssignedCell<F, F>, AssignedByte<F>>,
{
    fn convert_value(
        &self,
        x: &<AssignedCell<F, F> as InnerValue>::Element,
    ) -> Option<<AssignedByte<F> as InnerValue>::Element> {
        self.inner.convert_value(x)
    }

    fn convert(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedCell<F, F>,
    ) -> Result<AssignedByte<F>, Error> {
        self.inner.convert(layouter, x)
    }
}
