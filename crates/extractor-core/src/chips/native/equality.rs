use ff::PrimeField;
use mdnt_extractor_macros::delegated;
use midnight_circuits::{
    instructions::EqualityInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::{AssignedBit, AssignedNative, InnerValue},
};

use super::NativeGadgetAdaptor;

macro_rules! equality_impl {
    ($assigned:ty) => {
        impl<F, N> EqualityInstructions<F, $assigned> for NativeGadgetAdaptor<F, N>
        where
            F: PrimeField,
            N: EqualityInstructions<F, $assigned>,
        {
            #[delegated(inner)]
            fn is_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(inner)]
            fn is_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(inner)]
            fn is_not_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(inner)]
            fn is_not_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: <$assigned as InnerValue>::Element,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}

equality_impl!(AssignedNative<F>);
equality_impl!(AssignedBit<F>);
