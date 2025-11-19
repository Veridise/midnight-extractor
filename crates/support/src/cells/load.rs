//! Traits and types for loading arbitrary types from cells in a circuit.

use std::mem::MaybeUninit;

use ff::{Field, PrimeField};
use num_bigint::BigUint;

use crate::{
    cells::{
        ctx::{ICtx, LayoutAdaptor},
        CellReprSize,
    },
    circuit::injected::InjectedIR,
    Halo2Types,
};

/// Trait for deserializing arbitrary types from a set of circuit cells.
pub trait LoadFromCells<F: Field, C, H: Halo2Types<F>, L>: Sized + CellReprSize {
    /// Loads an instance of Self from a set of cells.
    fn load(
        ctx: &mut ICtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<Self, H::Error>;
}

impl<const N: usize, F: PrimeField, C, H: Halo2Types<F>, L, T: LoadFromCells<F, C, H, L>>
    LoadFromCells<F, C, H, L> for [T; N]
{
    fn load(
        ctx: &mut ICtx<F, H>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<Self, H::Error> {
        let mut out: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        for e in &mut out[..] {
            e.write(T::load(ctx, chip, layouter, injected_ir)?);
        }
        Ok(out.map(|e| unsafe { e.assume_init() }))
    }
}

macro_rules! load_const {
    ($t:ty) => {
        impl<C, F: PrimeField, H: Halo2Types<F>, L> LoadFromCells<F, C, H, L> for $t {
            fn load(
                ctx: &mut ICtx<F, H>,
                _chip: &C,
                _layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
                _injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
            ) -> Result<Self, H::Error> {
                Ok(ctx.primitive_constant()?)
            }
        }
    };
}

load_const!(bool);
load_const!(u8);
load_const!(usize);
load_const!(BigUint);

impl<F: Field, C, H: Halo2Types<F>, L> LoadFromCells<F, C, H, L> for () {
    fn load(
        _: &mut ICtx<F, H>,
        _: &C,
        _: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        _: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<Self, H::Error> {
        Ok(())
    }
}

macro_rules! load_tuple {
    ($($t:ident),+) => {
        impl<F:Field, C, H:Halo2Types<F>,L, $( $t: LoadFromCells<F,C,H,L>, )+> LoadFromCells<F,C,H,L> for (
                $( $t, )+
            )
        {
            fn load(
                ctx: &mut ICtx<F,H>,
                chip: &C,
                layouter: &mut impl LayoutAdaptor<F,H,Adaptee=L>,
                injected_ir: &mut InjectedIR<H::RegionIndex,H::Expression>,
            ) -> Result<Self, H::Error>
            {
                Ok(($( $t::load(ctx, chip, layouter, injected_ir)?, )+))
            }
        }
    };
}

load_tuple!(A1, A2);
load_tuple!(A1, A2, A3);
load_tuple!(A1, A2, A3, A4);
load_tuple!(A1, A2, A3, A4, A5);
