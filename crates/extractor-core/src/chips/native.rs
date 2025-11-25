mod arith;
mod assertion;
mod assignment;
mod binary;
mod canonicity;
mod comparison;
mod control_flow;
mod conversion;
mod decomposition;
mod division;
mod equality;
mod field;
mod from_scratch;
mod public_input;
mod range_check;
mod unsafe_conversion;
mod zero;

use std::{cell::RefCell, fmt, rc::Rc};

use ff::PrimeField;
use haloumi_ir::stmt::IRStmt;
use midnight_circuits::instructions::NativeInstructions;
use midnight_proofs::{circuit::RegionIndex, plonk::Expression};

use mdnt_support::circuit::injected::InjectedIR;

pub struct NativeGadgetAdaptor<F, N> {
    inner: N,
    injected_ir: Rc<RefCell<InjectedIR<RegionIndex, Expression<F>>>>,
}

impl<F, N> NativeGadgetAdaptor<F, N> {
    pub fn inject(&self, region: RegionIndex, stmt: IRStmt<(usize, Expression<F>)>) {
        self.injected_ir.borrow_mut().entry(region).or_default().push(stmt);
    }

    pub fn take_injected_ir(&self) -> InjectedIR<RegionIndex, Expression<F>> {
        self.injected_ir.take()
    }
}

impl<F: fmt::Debug, N: fmt::Debug> fmt::Debug for NativeGadgetAdaptor<F, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeGadgetAdaptor")
            .field("inner", &self.inner)
            .field("injected_ir", &self.injected_ir)
            .finish()
    }
}

impl<F, N: Clone> Clone for NativeGadgetAdaptor<F, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            injected_ir: self.injected_ir.clone(),
        }
    }
}

impl<F, N> NativeInstructions<F> for NativeGadgetAdaptor<F, N>
where
    F: PrimeField,
    N: NativeInstructions<F>,
{
}
