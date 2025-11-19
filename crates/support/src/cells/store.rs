//! Traits and types for storing arbitrary types into cells in a circuit.

use ff::{Field, PrimeField};

use crate::{
    cells::{
        ctx::{LayoutAdaptor, OCtx},
        CellReprSize,
    },
    circuit::injected::InjectedIR,
    error::Error,
};

/// Trait for serializing arbitrary types to a set of circuit cells.
pub trait StoreIntoCells<F: Field, C>: CellReprSize {
    /// Stores an instance of Self into a set of cells.
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F>;
}

//impl<F: PrimeField, C> StoreIntoCells<F, C> for AssignedCell<F, F> {
//    fn store<L, R, E>(
//        self,
//        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
//        _: &C,
//        layouter: &mut L,
//        _: &mut InjectedIR<R, E>,
//    ) -> Result<(), Error>
//    where
//        L: LayoutAdaptor<F>,
//    {
//        ctx.assign_next(self, layouter)
//    }
//}

impl<const N: usize, F: PrimeField, C, T: StoreIntoCells<F, C>> StoreIntoCells<F, C> for [T; N] {
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F>,
    {
        self.into_iter().try_for_each(|t| t.store(ctx, chip, layouter, injected_ir))
    }
}

impl<F: Field, C> StoreIntoCells<F, C> for () {
    fn store<L, R, E>(
        self,
        _ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        _chip: &C,
        _layouter: &mut L,
        _injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F>,
    {
        Ok(())
    }
}

impl<F: Field, C, A1: StoreIntoCells<F, C>, A2: StoreIntoCells<F, C>> StoreIntoCells<F, C>
    for (A1, A2)
{
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F>,
    {
        self.0.store(ctx, chip, layouter, injected_ir)?;
        self.1.store(ctx, chip, layouter, injected_ir)
    }
}
