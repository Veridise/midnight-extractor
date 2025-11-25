use ff::Field;
use midnight_proofs::{
    circuit::RegionIndex,
    plonk::{Error, Expression},
    ExtractionSupport,
};

use mdnt_support::{
    cells::{
        ctx::{LayoutAdaptor, OCtx},
        store::StoreIntoCells,
        CellReprSize,
    },
    circuit::injected::InjectedIR,
};

/// A fresh variable that occupies one cell but doesn't write anything to it.
pub struct FreshVar;

impl CellReprSize for FreshVar {
    const SIZE: usize = 1;
}

impl<F: Field, C, L> StoreIntoCells<F, C, ExtractionSupport, L> for FreshVar {
    fn store(
        self,
        ctx: &mut OCtx<F, ExtractionSupport>,
        _chip: &C,
        layouter: &mut impl LayoutAdaptor<F, ExtractionSupport, Adaptee = L>,
        _injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<(), Error> {
        ctx.set_next_to_zero(layouter)
    }
}
