#![doc = include_str!("../README.md")]
//!
//! Includes some implementations of the [`DecomposeIn<Cell>`] trait for standard
//! types but is not exhaustive. The implemented types cover the needs of the
//! circuits crate.
//!
//! If other external types are required their implementation should be added to
//! this crate.

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/// Implementations of this trait represent complex types that aggregate a
/// collection of `AssignedCell` values.
pub trait DecomposeIn<Cell> {
    /// Returns an iterator of `Cell` instances.
    fn cells(&self) -> impl IntoIterator<Item = Cell>;
}

impl<Cell> DecomposeIn<Cell> for u32 {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        std::iter::empty()
    }
}

impl<Cell, T: DecomposeIn<Cell> + ?Sized> DecomposeIn<Cell> for &T {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        (*self).cells()
    }
}

impl<Cell, T: DecomposeIn<Cell> + ?Sized> DecomposeIn<Cell> for &mut T {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        (**self).cells()
    }
}

impl<Cell, T: DecomposeIn<Cell>, E> DecomposeIn<Cell> for Result<T, E> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<Cell, T: DecomposeIn<Cell>> DecomposeIn<Cell> for Option<T> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<Cell, T: DecomposeIn<Cell>> DecomposeIn<Cell> for [T] {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<Cell, T: DecomposeIn<Cell>, const N: usize> DecomposeIn<Cell> for [T; N] {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<Cell, T: DecomposeIn<Cell>> DecomposeIn<Cell> for Vec<T> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

macro_rules! chain {
    () => {
        std::iter::empty()
    };
    ($h:expr $(,$t:expr)* $(,)?) => {
        $h.into_iter().chain( chain!($( $t, )*))
    };
}

macro_rules! tuple_impl {
    () => {
        tuple_impl!(@impl [] [] [A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12] [0 1 2 3 4 5 6 7 8 9 10 11]);
    };

    (@impl [$($done:ident)*] [$($idxs:tt)*] [$head:ident $($rest:ident)*] [$i:tt $($is:tt)*]) => {
        //// Implement for tuple ($head, $done...)
        impl<Cell, $head: DecomposeIn<Cell>, $( $done: DecomposeIn<Cell>, )*> DecomposeIn<Cell> for (
                $head, $( $done, )*
            )
        {
            fn cells(&self) -> impl IntoIterator<Item = Cell> {
                chain!($(
                    self.$idxs.cells(),
                )*
                self.$i.cells())
            }
        }

        // Recurse
        tuple_impl!(
            @impl [$head $($done)*] [$($idxs)* $i] [$($rest)*] [$($is)*]
        );
    };

    // Stop when no identifiers remain
    (@impl [$($done:ident)*] [$($idxs:tt)*] [] $rem:tt) => {
        // Also emit the 0-tuple base case
        impl<Cell> DecomposeIn<Cell> for () {
            fn cells(&self) -> impl IntoIterator<Item = Cell> {
                std::iter::empty()
            }
        }
    };

}

tuple_impl!();
