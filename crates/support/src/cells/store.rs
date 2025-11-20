//! Traits and types for storing arbitrary types into cells in a circuit.

use ff::{Field, PrimeField};

use crate::{
    cells::{
        ctx::{LayoutAdaptor, OCtx},
        CellReprSize,
    },
    circuit::injected::InjectedIR,
    Halo2Types,
};

/// Trait for serializing arbitrary types to a set of circuit cells.
pub trait StoreIntoCells<F: Field, C, H: Halo2Types<F>, L>: CellReprSize {
    /// Stores an instance of Self into a set of cells.
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error>;
}

impl<const N: usize, F: PrimeField, C, H: Halo2Types<F>, L, T: StoreIntoCells<F, C, H, L>>
    StoreIntoCells<F, C, H, L> for [T; N]
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        self.into_iter().try_for_each(|t| t.store(ctx, chip, layouter, injected_ir))
    }
}

impl<F: Field, C, H: Halo2Types<F>, L> StoreIntoCells<F, C, H, L> for () {
    fn store(
        self,
        _ctx: &mut OCtx<F, H>,
        _chip: &C,
        _layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        _injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        Ok(())
    }
}

impl<
        F: Field,
        C,
        H: Halo2Types<F>,
        L,
        A1: StoreIntoCells<F, C, H, L>,
        A2: StoreIntoCells<F, C, H, L>,
    > StoreIntoCells<F, C, H, L> for (A1, A2)
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error> {
        self.0.store(ctx, chip, layouter, injected_ir)?;
        self.1.store(ctx, chip, layouter, injected_ir)
    }
}

/// Helper trait for containers of [`StoreIntoCells`] implementations.
///
/// This trait does not require implementing [`CellReprSize`] so it's
/// convenient for types such as Vec or Option.
pub trait StoreIntoCellsDyn<F: Field, C, H: Halo2Types<F>, L> {
    /// Stores an instance of Self into a set of cells.
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<(), H::Error>;
}

impl<T, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for T
where
    T: StoreIntoCells<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self.store(ctx, chip, layouter, injected_ir)
    }
}

impl<T, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for Vec<T>
where
    T: StoreIntoCellsDyn<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self.into_iter().try_for_each(|e| e.store(ctx, chip, layouter, injected_ir))
    }
}

impl<T, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for Option<T>
where
    T: StoreIntoCellsDyn<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self.into_iter().try_for_each(|e| e.store(ctx, chip, layouter, injected_ir))
    }
}

impl<T, E, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for Result<T, E>
where
    T: StoreIntoCellsDyn<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
    H::Error: From<E>,
{
    fn store(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self?.store(ctx, chip, layouter, injected_ir)
    }
}
