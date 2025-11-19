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
    error::Error,
};

/// Trait for deserializing arbitrary types from a set of circuit cells.
pub trait LoadFromCells<F: Field, C>: Sized + CellReprSize {
    /// Loads an instance of Self from a set of cells.
    fn load<L, R, E>(
        ctx: &mut ICtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<Self, Error>
    where
        L: LayoutAdaptor<F>;
}

//impl<F: PrimeField, C> LoadFromCells<F, C> for AssignedCell<F, F> {
//    fn load<L, R, E>(
//        ctx: &mut ICtx<L::InstanceCol, L::AdviceCol>,
//        _: &C,
//        layouter: &mut L,
//        _: &mut InjectedIR<R, E>,
//    ) -> Result<Self, Error>
//    where
//        L: LayoutAdaptor<F>,
//    {
//        ctx.assign_next(layouter)
//    }
//}

impl<const N: usize, F: PrimeField, C, T: LoadFromCells<F, C>> LoadFromCells<F, C> for [T; N] {
    fn load<L, R, E>(
        ctx: &mut ICtx<L::InstanceCol, L::AdviceCol>,
        chip: &C,
        layouter: &mut L,
        injected_ir: &mut InjectedIR<R, E>,
    ) -> Result<Self, Error>
    where
        L: LayoutAdaptor<F>,
    {
        let mut out: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        for e in &mut out[..] {
            e.write(T::load(ctx, chip, layouter, injected_ir)?);
        }
        Ok(out.map(|e| unsafe { e.assume_init() }))
    }
}

macro_rules! load_const {
    ($t:ty) => {
        impl<C, F: PrimeField> LoadFromCells<F, C> for $t {
            fn load<L, R, E>(
                ctx: &mut ICtx<L::InstanceCol, L::AdviceCol>,
                _chip: &C,
                _layouter: &mut L,
                _injected_ir: &mut InjectedIR<R, E>,
            ) -> Result<Self, Error>
            where
                L: LayoutAdaptor<F>,
            {
                ctx.primitive_constant()
            }
        }
    };
}

load_const!(bool);
load_const!(u8);
load_const!(usize);
load_const!(BigUint);

impl<F: Field, C> LoadFromCells<F, C> for () {
    fn load<L, R, E>(
        _: &mut ICtx<L::InstanceCol, L::AdviceCol>,
        _: &C,
        _: &mut L,
        _: &mut InjectedIR<R, E>,
    ) -> Result<Self, Error>
    where
        L: LayoutAdaptor<F>,
    {
        Ok(())
    }
}

macro_rules! load_tuple {
    ($($t:ident),+) => {
        impl<F:Field,C,$( $t: LoadFromCells<F,C>, )+> LoadFromCells<F,C> for (
                $( $t, )+
            )
        {
            fn load<L,R,E>(
                ctx: &mut ICtx<L::InstanceCol, L::AdviceCol>,
                chip: &C,
                layouter: &mut L,
                injected_ir: &mut InjectedIR<R,E>,
            ) -> Result<Self, Error> where
        L: LayoutAdaptor<F>{
                Ok(($( $t::load(ctx, chip, layouter, injected_ir)?, )+))
            }
        }
    };
}

load_tuple!(A1, A2);
load_tuple!(A1, A2, A3);
load_tuple!(A1, A2, A3, A4);
load_tuple!(A1, A2, A3, A4, A5);
