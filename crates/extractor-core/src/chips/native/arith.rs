use ff::PrimeField;
use midnight_circuits::{
    instructions::ArithInstructions,
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::Error,
    },
    types::InnerValue,
};

use super::NativeGadgetAdaptor;

impl<F, N> ArithInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ArithInstructions<F, AssignedCell<F, F>>,
{
    fn linear_combination(
        &self,
        layouter: &mut impl Layouter<F>,
        terms: &[(
            <AssignedCell<F, F> as InnerValue>::Element,
            AssignedCell<F, F>,
        )],
        constant: <AssignedCell<F, F> as InnerValue>::Element,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.linear_combination(layouter, terms, constant)
    }

    fn mul(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedCell<F, F>,
        y: &AssignedCell<F, F>,
        multiplying_constant: Option<<AssignedCell<F, F> as InnerValue>::Element>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.mul(layouter, x, y, multiplying_constant)
    }

    fn div(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedCell<F, F>,
        y: &AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.div(layouter, x, y)
    }

    fn inv(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.inv(layouter, x)
    }

    fn inv0(
        &self,
        layouter: &mut impl Layouter<F>,
        x: &AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        self.inner.inv0(layouter, x)
    }
}
