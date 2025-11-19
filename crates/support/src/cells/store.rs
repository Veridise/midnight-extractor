//! Traits and types for storing arbitrary types into cells in a circuit.

use ff::{Field, PrimeField};

use crate::{
    cells::{
        ctx::{LayoutAdaptor, OCtx},
        CellReprSize,
    },
    circuit::injected::InjectedIR,
    error::Error,
    Halo2Types,
};

/// Trait for serializing arbitrary types to a set of circuit cells.
pub trait StoreIntoCells<F: Field, C, H: Halo2Types<F>>: CellReprSize {
    /// Stores an instance of Self into a set of cells.
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error>;
}

impl<const N: usize, F: PrimeField, C, H: Halo2Types<F>, T: StoreIntoCells<F, C, H>>
    StoreIntoCells<F, C, H> for [T; N]
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        self.into_iter().try_for_each(|t| t.store(ctx, chip, layouter, injected_ir))
    }
}

impl<F: Field, C, H: Halo2Types<F>> StoreIntoCells<F, C, H> for () {
    fn store(
        self,
        _ctx: &mut OCtx<F, H>,
        _chip: &C,
        _layouter: &mut impl LayoutAdaptor<F, H>,
        _injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        Ok(())
    }
}

impl<F: Field, C, H: Halo2Types<F>, A1: StoreIntoCells<F, C, H>, A2: StoreIntoCells<F, C, H>>
    StoreIntoCells<F, C, H> for (A1, A2)
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        self.0.store(ctx, chip, layouter, injected_ir)?;
        self.1.store(ctx, chip, layouter, injected_ir)
    }
}
