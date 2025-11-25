use ff::PrimeField;
use midnight_circuits::{
    field::AssignedBounded,
    instructions::ComparisonInstructions,
    midnight_proofs::{circuit::Layouter, plonk::Error},
    types::{AssignedBit, AssignedNative, InnerValue},
};

use super::NativeGadgetAdaptor;

impl<F, N> ComparisonInstructions<F, AssignedNative<F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ComparisonInstructions<F, AssignedNative<F>>,
{
    const MAX_BOUND_IN_BITS: u32 = N::MAX_BOUND_IN_BITS;

    fn bounded_of_element(
        &self,
        layouter: &mut impl Layouter<F>,
        n: usize,
        x: &AssignedNative<F>,
    ) -> Result<AssignedBounded<F>, Error> {
        self.inner.bounded_of_element(layouter, n, x)
    }

    fn element_of_bounded(
        &self,
        layouter: &mut impl Layouter<F>,
        bounded: &AssignedBounded<F>,
    ) -> Result<AssignedNative<F>, Error> {
        self.inner.element_of_bounded(layouter, bounded)
    }

    fn lower_than_fixed(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedBounded<F>,
        bound: <AssignedNative<F> as InnerValue>::Element,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[lower_than_fixed] Potential interesting site to hijack!");
        self.inner.lower_than_fixed(layouter, x, bound)
    }

    fn lower_than(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedBounded<F>,
        y: &AssignedBounded<F>,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[lower_than] Potential interesting site to hijack!");
        self.inner.lower_than(layouter, x, y)
    }

    fn leq(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedBounded<F>,
        y: &AssignedBounded<F>,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[leq] Potential interesting site to hijack!");
        self.inner.leq(layouter, x, y)
    }

    fn geq(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedBounded<F>,
        y: &AssignedBounded<F>,
    ) -> Result<AssignedBit<F>, Error> {
        log::warn!("[geq] Potential interesting site to hijack!");
        self.inner.geq(layouter, x, y)
    }
}
