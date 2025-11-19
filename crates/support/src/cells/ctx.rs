//! Supporting types for loading and storing from cells.

use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use ff::{Field, PrimeField};
use mdnt_groups_support::DecomposeIn;
//use midnight_proofs::{
//    circuit::{AssignedCell, Layouter},
//    plonk::{Advice, Any, Column, ColumnType, Instance},
//};

use crate::{error::Error, parse_field, Halo2Types};

//#[cfg(feature = "proofs")]

/// Adaptor trait that defines the required behavior from a Layouter.
pub trait LayoutAdaptor<F: Field, Halo2: Halo2Types> {
    /// Type for instance columns.
    type InstanceCol: std::fmt::Debug + Copy + Clone;
    /// Type for advice columns.
    type AdviceCol: std::fmt::Debug + Copy + Clone;
    /// Type for a cell.
    type Cell: std::fmt::Debug + Copy + Clone;
    /// Region type.
    type Region<'a>;

    /// Constraints two cells to be equal.
    ///
    /// The left hand side cell could be any cell and the right hand side is an instance cell.
    fn constrain_instance(
        &mut self,
        cell: Halo2::Cell,
        instance_col: Halo2::InstanceCol,
        instance_row: usize,
    ) -> Result<(), Halo2::Error>;

    /// Constraints an advice cell to a constant value.
    fn constrain_advice_constant(
        &mut self,
        advice_col: Halo2::AdviceCol,
        advice_row: usize,
        constant: F,
    ) -> Result<Halo2::Cell, Halo2::Error>;

    /// Assigns an advice cell from an instance cell.
    fn assign_advice_from_instance(
        &mut self,
        advice_col: Halo2::AdviceCol,
        advice_row: usize,
        instance_col: Halo2::InstanceCol,
        instance_row: usize,
    ) -> Result<Halo2::AssignedCell, Halo2::Error>;

    /// Copies the cell's contents into the given advice cell.
    fn copy_advice(
        &mut self,
        ac: &Halo2::AssignedCell,
        region: &mut Halo2::Region<'_>,
        advice_col: Halo2::AdviceCol,
        advice_row: usize,
    ) -> Result<Halo2::AssignedCell, Halo2::Error>;

    /// Enters the scope of a region.
    fn region<A, AR, N, NR>(&mut self, name: N, assignment: A) -> Result<AR, Halo2::Error>
    where
        A: FnMut(Halo2::Region<'_>) -> Result<AR, Halo2::Error>,
        N: Fn() -> NR,
        NR: Into<String>;
}

/// A cell in the table.
#[derive(Debug)]
pub struct Cell<C> {
    col: C,
    row: usize,
}

impl<C> Cell<C> {
    /// Creates a new cell.
    pub fn new(col: C, row: usize) -> Self {
        Self { col, row }
    }

    /// Creates a new cell in row 0.
    pub fn first_row(col: C) -> Self {
        Self::new(col, 0)
    }

    /// Returns the column of the cell.
    pub fn col(&self) -> C
    where
        C: Copy,
    {
        self.col
    }

    /// Returns the row of the cell.
    pub fn row(&self) -> usize {
        self.row
    }
}

impl<C> From<(C, usize)> for Cell<C> {
    fn from((col, row): (C, usize)) -> Self {
        Self::new(col, row)
    }
}

/// A description for an input. Comprises an instance cell that represents the
/// actual input and an advice cell that is used for integrating better with
/// regions.
#[derive(Debug)]
pub struct InputDescr<H: Halo2Types> {
    cell: Cell<H::InstanceCol>,
    temp: Cell<H::AdviceCol>,
}

impl<H: Halo2Types> InputDescr<H> {
    /// Creates a new input description.
    pub fn new(cell: Cell<H::InstanceCol>, temp: H::AdviceCol) -> Self {
        Self {
            cell,
            temp: Cell::first_row(temp),
        }
    }

    /// Returns the column of the instance cell.
    pub fn col(&self) -> H::InstanceCol {
        self.cell.col()
    }

    /// Returns the row of the instance cell.
    pub fn row(&self) -> usize {
        self.cell.row()
    }

    /// Returns the column of the helper advice cell.
    pub fn temp(&self) -> H::AdviceCol {
        self.temp.col()
    }

    /// Returns the row of the helper advice cell.
    pub fn temp_offset(&self) -> usize {
        self.temp.row()
    }
}

impl<H: Halo2Types> From<OutputDescr<H>> for InputDescr<H> {
    fn from(descr: OutputDescr<H>) -> Self {
        InputDescr {
            cell: (descr.cell.col(), descr.cell.row).into(),
            temp: descr.helper,
        }
    }
}

/// A description for an output. Comprises an instance cell acting as the output
/// and a support advice cell.
#[derive(Debug)]
pub struct OutputDescr<H: Halo2Types> {
    cell: Cell<H::InstanceCol>,
    helper: Cell<H::AdviceCol>,
}

impl<H: Halo2Types> OutputDescr<H> {
    /// Creates a new output description.
    pub fn new(cell: Cell<H::InstanceCol>, helper: H::AdviceCol) -> Self {
        Self {
            cell,
            helper: Cell {
                col: helper,
                row: 0,
            },
        }
    }

    fn set_to_zero<F>(&self, layouter: &mut impl LayoutAdaptor<F, H>) -> Result<(), H::Error>
    where
        F: Field,
    {
        let helper_cell =
            layouter.constrain_advice_constant(self.helper.col, self.helper.row, F::ZERO)?;
        layouter.constrain_instance(helper_cell, self.cell.col, self.cell.row)?;
        Ok(())
    }

    fn assign<F: Field>(
        &self,
        cell: H::Cell,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<(), H::Error> {
        layouter.constrain_instance(cell, self.cell.col(), self.cell.row())?;
        Ok(())
    }
}

/// Context type for the [`LoadFromCells`](super::load::LoadFromCells) and
/// [`StoreIntoCells`](super::store::StoreIntoCells) traits.
pub struct IOCtx<'io, IO> {
    io: Box<dyn Iterator<Item = IO> + 'io>,
}

impl<'io, IO> IOCtx<'io, IO> {
    /// Creates a new IO context.
    pub fn new(io: impl Iterator<Item = IO> + 'io) -> Self {
        Self { io: Box::new(io) }
    }

    /// Returns the next IO object or fails if there aren't any more objects.
    pub fn next(&mut self) -> Result<IO, Error> {
        self.io.next().ok_or_else(|| Error::NotEnoughIOCells)
    }
}

impl<IO> std::fmt::Debug for IOCtx<'_, IO> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOCtx").field("io", &"<iterator>").finish()
    }
}

/// Context type for the [`LoadFromCells`](super::load::LoadFromCells) trait.
pub struct ICtx<'i, 's, H: Halo2Types> {
    inner: IOCtx<'i, InputDescr<H>>,
    constants: Box<dyn Iterator<Item = &'s str> + 's>,
}

impl<'i, 's, H: Halo2Types> ICtx<'i, 's, H> {
    /// Creates a new input context.
    pub fn new(i: impl Iterator<Item = InputDescr<H>> + 'i, constants: &'s [String]) -> Self {
        Self {
            inner: IOCtx::new(i),
            constants: Box::new(constants.iter().map(|s| s.as_str())),
        }
    }

    /// Tries to parse a constant as a field element.
    pub fn field_constant<F: PrimeField>(&mut self) -> Result<F, Error> {
        self.constants
            .next()
            .ok_or_else(|| Error::NotEnoughConstants)
            .and_then(parse_field::<F>)
    }

    /// Tries to parse a primitive constant.
    pub fn primitive_constant<T, E>(&mut self) -> Result<T, Error>
    where
        T: FromStr<Err = E>,
        Error: From<E>,
    {
        Ok(T::from_str(
            self.constants.next().ok_or_else(|| Error::NotEnoughConstants)?,
        )?)
    }

    /// Assigns the next input to a cell.
    pub fn assign_next<F: PrimeField>(
        &mut self,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<H::AssignedCell, H::Error> {
        let i = self.next()?;
        layouter.assign_advice_from_instance(i.temp(), i.temp_offset(), i.col(), i.row())
    }
}

impl<'i, H: Halo2Types> Deref for ICtx<'i, '_, H> {
    type Target = IOCtx<'i, InputDescr<H>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<H: Halo2Types> DerefMut for ICtx<'_, '_, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<H: Halo2Types> std::fmt::Debug for ICtx<'_, '_, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ICtx")
            .field("inner", &self.inner)
            .field("constants", &"<iterator>")
            .finish()
    }
}

/// Context type for the [`StoreIntoCells`](super::store::StoreIntoCells) trait.
#[derive(Debug)]
pub struct OCtx<'o, H: Halo2Types> {
    inner: IOCtx<'o, OutputDescr<H>>,
}

impl<'o, H: Halo2Types> OCtx<'o, H> {
    /// Creates a new output context.
    pub fn new(input: impl Iterator<Item = OutputDescr<H>> + 'o) -> Self {
        Self {
            inner: IOCtx::new(input),
        }
    }

    /// Sets the next output to zero.
    pub fn set_next_to_zero<F: Field>(
        &mut self,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<(), H::Error> {
        self.next()?.set_to_zero(layouter)
    }

    /// Sets the next output to the given value.
    pub fn assign_next<F>(
        &mut self,
        value: impl DecomposeIn<H::Cell>,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<(), H::Error>
    where
        F: PrimeField,
    {
        for cell in value.cells() {
            self.next()?.assign(cell, layouter)?;
        }
        Ok(())
    }
}

impl<'o, H: Halo2Types> Deref for OCtx<'o, H> {
    type Target = IOCtx<'o, OutputDescr<H>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<H: Halo2Types> DerefMut for OCtx<'_, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
