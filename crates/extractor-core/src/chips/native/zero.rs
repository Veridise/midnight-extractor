use ff::PrimeField;
use midnight_circuits::{instructions::ZeroInstructions, midnight_proofs::circuit::AssignedCell};

use super::NativeGadgetAdaptor;

impl<F, N> ZeroInstructions<F, AssignedCell<F, F>> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: ZeroInstructions<F, AssignedCell<F, F>>,
{
}
