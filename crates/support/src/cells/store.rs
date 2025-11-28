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

macro_rules! store_tuple {
    () => {
        store_tuple!(@impl [] [] [A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12] [0 1 2 3 4 5 6 7 8 9 10 11]);
    };

    (@impl [$($done:ident)*] [$($idxs:tt)*] [$head:ident $($rest:ident)*] [$i:tt $($is:tt)*]) => {
        // Implement for tuple ($head, $done...)
        impl<
            F: Field, C, H: Halo2Types<F>, L,
            $head: StoreIntoCells<F, C, H, L>,
            $( $done: StoreIntoCells<F, C, H, L>, )*
        > StoreIntoCells<F, C, H, L> for ($head, $( $done, )*)
        {
            fn store(
                self,
                ctx: &mut OCtx<F, H>,
                chip: &C,
                layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
                injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
            ) -> Result<(), H::Error> {
                // Call fields by index
                $(
                    self.$idxs.store(ctx, chip, layouter, injected_ir)?;
                )*
                self.$i.store(ctx, chip, layouter, injected_ir)?;
                Ok(())
            }
        }

        // Recurse
        store_tuple!(
            @impl [$head $($done)*] [$($idxs)* $i] [$($rest)*] [$($is)*]
        );
    };

    // Stop when no identifiers remain
    (@impl [$($done:ident)*] [$($idxs:tt)*] [] $rem:tt) => {
        // Also emit the 0-tuple base case
        impl<F: Field, C, H: Halo2Types<F>, L> StoreIntoCells<F, C, H, L> for () {
            fn store(
                self,
                _: &mut OCtx<F, H>,
                _: &C,
                _: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
                _: &mut InjectedIR<H::RegionIndex, H::Expression>,
            ) -> Result<(), H::Error> {
                Ok(())
            }
        }
    };

}

store_tuple!();

/// Helper trait for containers of [`StoreIntoCells`] implementations.
///
/// This trait does not require implementing [`CellReprSize`] so it's
/// convenient for types such as Vec or Option.
pub trait StoreIntoCellsDyn<F: Field, C, H: Halo2Types<F>, L> {
    /// Stores an instance of Self into a set of cells.
    fn store_dyn(
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
    fn store_dyn(
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
    fn store_dyn(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self.into_iter().try_for_each(|e| e.store_dyn(ctx, chip, layouter, injected_ir))
    }
}

impl<T, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for Option<T>
where
    T: StoreIntoCellsDyn<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
{
    fn store_dyn(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self.into_iter().try_for_each(|e| e.store_dyn(ctx, chip, layouter, injected_ir))
    }
}

impl<T, E, F, C, H, L> StoreIntoCellsDyn<F, C, H, L> for Result<T, E>
where
    T: StoreIntoCellsDyn<F, C, H, L>,
    F: Field,
    H: Halo2Types<F>,
    H::Error: From<E>,
{
    fn store_dyn(
        self,
        ctx: &mut OCtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<
            <H as Halo2Types<F>>::RegionIndex,
            <H as Halo2Types<F>>::Expression,
        >,
    ) -> Result<(), <H as Halo2Types<F>>::Error> {
        self?.store_dyn(ctx, chip, layouter, injected_ir)
    }
}
