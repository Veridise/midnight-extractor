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
pub trait StoreIntoCells<F: Field, C, AC, Cell>: CellReprSize {
    /// Stores an instance of Self into a set of cells.
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F, AC, Cell = Cell>;
}

impl<const N: usize, F: PrimeField, C, AC, Cell, T: StoreIntoCells<F, C, AC, Cell>>
    StoreIntoCells<F, C, AC, Cell> for [T; N]
{
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F, AC, Cell = Cell>,
    {
        self.into_iter().try_for_each(|t| t.store(ctx, chip, layouter, injected_ir))
    }
}

impl<F: Field, C, AC, Cell> StoreIntoCells<F, C, AC, Cell> for () {
    fn store<L, R, E>(
        self,
        _ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        _chip: &C,
        _layouter: &mut L,
        _injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F, AC, Cell = Cell>,
    {
        Ok(())
    }
}

impl<
        F: Field,
        C,
        AC,
        Cell,
        A1: StoreIntoCells<F, C, AC, Cell>,
        A2: StoreIntoCells<F, C, AC, Cell>,
    > StoreIntoCells<F, C, AC, Cell> for (A1, A2)
{
    fn store<L, R, E>(
        self,
        ctx: &mut OCtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<(), Error>
    where
        L: LayoutAdaptor<F, AC, Cell = Cell>,
    {
        self.0.store(ctx, chip, layouter, injected_ir)?;
        self.1.store(ctx, chip, layouter, injected_ir)
    }
}
