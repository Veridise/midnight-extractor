use ff::{Field, PrimeField};
use haloumi_ir_gen::{gates::callbacks::GateCallbacks, gates::rewrite::GateRewritePattern};

pub mod decompose_core;

use decompose_core::DecomposeCorePattern;
use midnight_proofs::plonk::Expression;

pub struct Patterns {
    pub decompose_core: bool,
}

impl<F: PrimeField> GateCallbacks<F, Expression<F>> for Patterns {
    fn patterns(&self) -> Vec<Box<dyn GateRewritePattern<F, Expression<F>>>>
    where
        F: Field,
    {
        let mut patterns: Vec<Box<dyn GateRewritePattern<F, Expression<F>>>> = vec![];
        if self.decompose_core {
            patterns.push(Box::new(DecomposeCorePattern {}))
        }
        patterns
    }
}
