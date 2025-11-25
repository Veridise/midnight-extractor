use ff::PrimeField;
use midnight_circuits::{
    instructions::AssertionInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::Error,
    },
    types::{AssignedBit, AssignedByte, InnerValue},
};

use super::NativeGadgetAdaptor;

macro_rules! assertion_impl {
    ($A:ty) => {
        impl<F, N> AssertionInstructions<F, $A> for NativeGadgetAdaptor<F, N>
        where
            F: PrimeField,
            N: AssertionInstructions<F, $A>,
        {
            fn assert_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$A,
                y: &$A,
            ) -> Result<(), Error> {
                self.inner.assert_equal(layouter, x, y)
            }

            fn assert_not_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$A,
                y: &$A,
            ) -> Result<(), Error> {
                self.inner.assert_not_equal(layouter, x, y)
            }

            fn assert_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$A,
                constant: <$A as InnerValue>::Element,
            ) -> Result<(), Error> {
                self.inner.assert_equal_to_fixed(layouter, x, constant)
            }

            fn assert_not_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$A,
                constant: <$A as InnerValue>::Element,
            ) -> Result<(), Error> {
                self.inner.assert_not_equal_to_fixed(layouter, x, constant)
            }
        }
    };
}

assertion_impl!(AssignedBit<F>);
assertion_impl!(AssignedByte<F>);
assertion_impl!(AssignedCell<F, F>);
