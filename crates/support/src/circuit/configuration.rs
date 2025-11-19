//! Traits related to circuit configuration.

//use midnight_proofs::plonk::{Advice, Column, ConstraintSystem, Fixed, Instance, TableColumn};

/// Helper trait that enables composing types that can handle circuit
/// configuration.
pub trait AutoConfigure<CS, Output = Self> {
    /// Creates an instance of self using the constraint system.
    fn configure(meta: &mut CS) -> Output;
}

/// Creates an implementation of [`AutoConfigure`].
#[macro_export]
macro_rules! auto_conf_impl {
    ($T:ty, $method:ident) => {
        $crate::auto_conf_impl!($T, $method, midnight_proofs);
    };
    ($T:ty, $method:ident, $proofs:ident) => {
        impl<F: ff::Field> AutoConfigure<$proofs::plonk::ConstraintSystem<F>, $T> for $T {
            fn configure(meta: &mut $proofs::plonk::ConstraintSystem<F>) -> $T {
                meta.$method()
            }
        }
    };
}

impl<CS, T, const N: usize> AutoConfigure<CS> for [T; N]
where
    T: AutoConfigure<CS>,
{
    fn configure(meta: &mut CS) -> [T; N] {
        std::array::from_fn(|_| T::configure(meta))
    }
}

impl<CS> AutoConfigure<CS> for () {
    fn configure(_: &mut CS) -> Self {
        ()
    }
}

macro_rules! tuple_auto_conf_impl {
    ($($t:ident),+) => {
        impl<CS, $( $t: AutoConfigure<CS>, )+> AutoConfigure<CS> for ( $( $t, )+ ) {
            fn configure(meta: &mut CS) -> ( $( $t, )+ ) {
                (
                    $( $t::configure(meta), )+
                )
            }
        }
    };
}

tuple_auto_conf_impl!(A1, A2);
