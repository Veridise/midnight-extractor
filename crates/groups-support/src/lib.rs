//! Support traits for the `picus::group` macro.
//!
//! Includes some implementations of the [`DecomposeInCells`] trait for standard
//! types but is not exhaustive. The implemented types cover the needs of the
//! circuits crate.
//!
//! If other external types are required their implementation should be added to
//! this crate.

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

#[cfg(feature = "region-groups")]
use midnight_proofs::{circuit::groups::RegionsGroup, plonk::Error};
use midnight_proofs::{
    circuit::{AssignedCell, Cell},
    halo2curves::ff::Field,
};

/// Implementations of this trait represent complex types that aggregate a
/// collection of [`AssignedCell`] values.
pub trait DecomposeInCells {
    /// Returns an iterator of [`Cell`] instances.
    fn cells(&self) -> impl IntoIterator<Item = Cell>;

    /// Annotates the type as an input.
    ///
    /// By default annotates all the cells of the type as inputs.
    #[cfg(feature = "region-groups")]
    fn annotate_as_input(&self, group: &mut RegionsGroup) -> Result<(), Error> {
        group.annotate_inputs(self.cells())
    }

    /// Annotates the type as an output.
    ///
    /// By default annotates all the cells of the type as outputs.
    #[cfg(feature = "region-groups")]
    fn annotate_as_output(&self, group: &mut RegionsGroup) -> Result<(), Error> {
        group.annotate_outputs(self.cells())
    }
}

impl DecomposeInCells for Cell {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        [*self]
    }
}

impl<V, F: Field> DecomposeInCells for AssignedCell<V, F> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        [self.cell()]
    }
}

impl DecomposeInCells for u32 {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        std::iter::empty()
    }
}

impl<T: DecomposeInCells, E> DecomposeInCells for Result<T, E> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<T: DecomposeInCells> DecomposeInCells for Option<T> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<T: DecomposeInCells> DecomposeInCells for &[T] {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<T: DecomposeInCells, const N: usize> DecomposeInCells for [T; N] {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}

impl<T: DecomposeInCells> DecomposeInCells for Vec<T> {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.iter().flat_map(|t| t.cells())
    }
}
