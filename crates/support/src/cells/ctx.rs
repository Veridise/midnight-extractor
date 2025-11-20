//! Supporting types for loading and storing from cells.

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use ff::{Field, PrimeField};
use mdnt_groups_support::DecomposeIn;
//use midnight_proofs::{
//    circuit::{AssignedCell, Layouter},
//    plonk::{Advice, Any, Column, ColumnType, Instance},
//};

use crate::{
    cells::load::LoadFromCells, circuit::injected::InjectedIR, error::Error, parse_field,
    Halo2Types,
};

//#[cfg(feature = "proofs")]

/// Adaptor trait that defines the required behavior from a Layouter.
pub trait LayoutAdaptor<F: Field, Halo2: Halo2Types<F>> {
    /// Adapted type
    type Adaptee;

    /// Returns a reference to the adaptee.
    fn adaptee_ref(&self) -> &Self::Adaptee;

    /// Returns a mutable reference to the adaptee.
    fn adaptee_ref_mut(&mut self) -> &mut Self::Adaptee;

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
pub struct InputDescr<F: Field, H: Halo2Types<F>> {
    cell: Cell<H::InstanceCol>,
    temp: Cell<H::AdviceCol>,
    _marker: PhantomData<F>,
}

impl<F: Field, H: Halo2Types<F>> InputDescr<F, H> {
    /// Creates a new input description.
    pub fn new(cell: Cell<H::InstanceCol>, temp: H::AdviceCol) -> Self {
        Self {
            cell,
            temp: Cell::first_row(temp),
            _marker: Default::default(),
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

impl<F: Field, H: Halo2Types<F>> From<OutputDescr<F, H>> for InputDescr<F, H> {
    fn from(descr: OutputDescr<F, H>) -> Self {
        InputDescr {
            cell: (descr.cell.col(), descr.cell.row).into(),
            temp: descr.helper,
            _marker: Default::default(),
        }
    }
}

/// A description for an output. Comprises an instance cell acting as the output
/// and a support advice cell.
#[derive(Debug)]
pub struct OutputDescr<F: Field, H: Halo2Types<F>> {
    cell: Cell<H::InstanceCol>,
    helper: Cell<H::AdviceCol>,
    _marker: PhantomData<F>,
}

impl<F: Field, H: Halo2Types<F>> OutputDescr<F, H> {
    /// Creates a new output description.
    pub fn new(cell: Cell<H::InstanceCol>, helper: H::AdviceCol) -> Self {
        Self {
            cell,
            helper: Cell {
                col: helper,
                row: 0,
            },
            _marker: Default::default(),
        }
    }

    fn set_to_zero(&self, layouter: &mut impl LayoutAdaptor<F, H>) -> Result<(), H::Error> {
        let helper_cell =
            layouter.constrain_advice_constant(self.helper.col, self.helper.row, F::ZERO)?;
        layouter.constrain_instance(helper_cell, self.cell.col, self.cell.row)?;
        Ok(())
    }

    fn assign(
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
pub struct ICtx<'i, 's, F: Field, H: Halo2Types<F>> {
    inner: IOCtx<'i, InputDescr<F, H>>,
    constants: Box<dyn Iterator<Item = &'s str> + 's>,
}

impl<'i, 's, F: Field, H: Halo2Types<F>> ICtx<'i, 's, F, H> {
    /// Creates a new input context.
    pub fn new(i: impl Iterator<Item = InputDescr<F, H>> + 'i, constants: &'s [String]) -> Self {
        Self {
            inner: IOCtx::new(i),
            constants: Box::new(constants.iter().map(|s| s.as_str())),
        }
    }

    /// Tries to parse a constant as a field element.
    pub fn field_constant(&mut self) -> Result<F, Error>
    where
        F: PrimeField,
    {
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
    pub fn assign_next(
        &mut self,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<H::AssignedCell, H::Error> {
        let i = self.next()?;
        layouter.assign_advice_from_instance(i.temp(), i.temp_offset(), i.col(), i.row())
    }

    /// Loads an instance from a set of cells.
    pub fn load<T, C, L>(
        &mut self,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, H, Adaptee = L>,
        injected_ir: &mut InjectedIR<H::RegionIndex, H::Expression>,
    ) -> Result<T, H::Error>
    where
        T: LoadFromCells<F, C, H, L>,
    {
        T::load(self, chip, layouter, injected_ir)
    }
}

impl<'i, F: Field, H: Halo2Types<F>> Deref for ICtx<'i, '_, F, H> {
    type Target = IOCtx<'i, InputDescr<F, H>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<F: Field, H: Halo2Types<F>> DerefMut for ICtx<'_, '_, F, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<F: Field, H: Halo2Types<F>> std::fmt::Debug for ICtx<'_, '_, F, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ICtx")
            .field("inner", &self.inner)
            .field("constants", &"<iterator>")
            .finish()
    }
}

/// Context type for the [`StoreIntoCells`](super::store::StoreIntoCells) trait.
#[derive(Debug)]
pub struct OCtx<'o, F: Field, H: Halo2Types<F>> {
    inner: IOCtx<'o, OutputDescr<F, H>>,
}

impl<'o, F: Field, H: Halo2Types<F>> OCtx<'o, F, H> {
    /// Creates a new output context.
    pub fn new(input: impl Iterator<Item = OutputDescr<F, H>> + 'o) -> Self {
        Self {
            inner: IOCtx::new(input),
        }
    }

    /// Sets the next output to zero.
    pub fn set_next_to_zero(
        &mut self,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<(), H::Error> {
        self.next()?.set_to_zero(layouter)
    }

    /// Sets the next output to the given value.
    pub fn assign_next(
        &mut self,
        value: impl DecomposeIn<H::Cell>,
        layouter: &mut impl LayoutAdaptor<F, H>,
    ) -> Result<(), H::Error> {
        for cell in value.cells() {
            self.next()?.assign(cell, layouter)?;
        }
        Ok(())
    }
}

impl<'o, F: Field, H: Halo2Types<F>> Deref for OCtx<'o, F, H> {
    type Target = IOCtx<'o, OutputDescr<F, H>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<F: Field, H: Halo2Types<F>> DerefMut for OCtx<'_, F, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
