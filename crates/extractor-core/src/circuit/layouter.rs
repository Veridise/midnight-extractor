use std::{cell::RefCell, cmp, collections::HashMap};

use ff::Field;
use haloumi::synthesis::synthesizer::Synthesizer;
use haloumi_core::synthesis::SynthesizerLike;
use mdnt_support::cells::ctx::LayoutAdaptor;
use midnight_proofs::{
    circuit::{
        groups::{self, GroupKeyInstance},
        layouter::{RegionColumn, RegionLayouter, RegionShape},
        AssignedCell, Cell, Layouter, Region, RegionIndex, RegionStart, Table, TableLayouter,
        Value,
    },
    plonk::{
        Advice, Any, Challenge, Column, Error, Fixed, Instance, Selector, TableColumn, TableError,
    },
    utils::rational::Rational,
    ExtractionSupport,
};

#[derive(Debug)]
pub struct ExtractionLayouter<'s, 'c, F: Field> {
    synthesizer: &'s mut Synthesizer<F>,
    constants: &'c [Column<Fixed>],
    /// Stores the starting row for each region.
    regions: Vec<RegionStart>,

    /// Stores the first empty row for each column.
    columns: HashMap<RegionColumn, usize>,
    /// Stores the table fixed columns.
    table_columns: Vec<TableColumn>,
    /// Group depth
    group_depth: usize,
}

impl<'s, 'c, F: Field> ExtractionLayouter<'s, 'c, F> {
    pub fn new(synthesizer: &'s mut Synthesizer<F>, constants: &'c [Column<Fixed>]) -> Self {
        Self {
            synthesizer,
            constants,
            regions: Default::default(),
            columns: Default::default(),
            table_columns: Default::default(),
            group_depth: Default::default(),
        }
    }
}

impl<F: Field> Layouter<F> for ExtractionLayouter<'_, '_, F> {
    type Root = Self;

    fn assign_region<A, AR, N, NR>(&mut self, name: N, mut assignment: A) -> Result<AR, Error>
    where
        A: FnMut(Region<'_, F>) -> Result<AR, Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        let region_index = self.regions.len();

        let name: String = name().into();
        log::info!(
            "{}> Entering region '{name}' ({region_index})",
            "-".repeat(self.group_depth)
        );

        // Get shape of the region.
        let mut shape = RegionShape::new(region_index.into());
        {
            let region: &mut dyn RegionLayouter<F> = &mut shape;
            assignment(region.into())?;
        }

        // Lay out this region. We implement the simplest approach here: position the
        // region starting at the earliest row for which none of the columns are in use.
        let mut region_start = 0;
        for column in shape.columns() {
            region_start = cmp::max(region_start, self.columns.get(column).cloned().unwrap_or(0));
        }
        self.regions.push(region_start.into());

        // Update column usage information.
        for column in shape.columns() {
            self.columns.insert(*column, region_start + shape.row_count());
        }

        // Assign region cells.
        self.synthesizer
            //.enter_region(name, Some(region_index.into()), Some(region_start.into()));
            .enter_region(name, None, Some(region_start.into()));
        let mut region = ExtractionLayouterRegion::new(self, region_index.into());
        let result = {
            let region: &mut dyn RegionLayouter<F> = &mut region;
            assignment(region.into())
        }?;
        let constants_to_assign = region.constants;
        self.synthesizer.exit_region();

        // Assign constants. For the simple floor planner, we assign constants in order
        // in the first `constants` column.
        if self.constants.is_empty() {
            if !constants_to_assign.is_empty() {
                return Err(Error::NotEnoughColumnsForConstants);
            }
        } else {
            let constants_column = self.constants[0];
            let next_constant_row =
                self.columns.entry(Column::<Any>::from(constants_column).into()).or_default();
            for (constant, advice) in constants_to_assign {
                self.synthesizer.on_fixed_assigned(
                    constants_column,
                    *next_constant_row,
                    constant.evaluate(),
                );
                self.synthesizer.copy(
                    constants_column,
                    *next_constant_row,
                    advice.column,
                    *self.regions[*advice.region_index] + advice.row_offset,
                );
                *next_constant_row += 1;
            }
        }

        Ok(result)
    }

    fn assign_table<A, N, NR>(&mut self, name: N, mut assignment: A) -> Result<(), Error>
    where
        A: FnMut(Table<'_, F>) -> Result<(), Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        self.synthesizer.enter_region(name().into(), None, None);
        let mut table = ExtractionTableLayouter::new(self.synthesizer, &self.table_columns);
        {
            let table: &mut dyn TableLayouter<F> = &mut table;
            assignment(table.into())
        }?;
        let default_and_assigned = table.default_and_assigned;
        self.synthesizer.exit_region();

        // Check that all table columns have the same length `first_unused`,
        // and all cells up to that length are assigned.
        let first_unused = compute_table_lengths(&default_and_assigned)?;

        // Record these columns so that we can prevent them from being used again.
        for column in default_and_assigned.keys() {
            self.table_columns.push(*column);
        }

        for (col, (default_val, _)) in default_and_assigned {
            // default_val must be Some because we must have assigned
            // at least one cell in each column, and in that case we checked
            // that all cells up to first_unused were assigned.
            self.synthesizer.fill_from_row(
                col.inner(),
                first_unused,
                steal(default_val.unwrap())
                    .ok_or_else(|| Error::Synthesis("Unknown default value".to_string()))?
                    .evaluate(),
            );
        }

        self.synthesizer.mark_region_as_table();
        Ok(())
    }

    fn constrain_instance(
        &mut self,
        cell: Cell,
        instance: Column<Instance>,
        row: usize,
    ) -> Result<(), Error> {
        self.synthesizer.copy(
            cell.column,
            *self.regions[*cell.region_index] + cell.row_offset,
            instance,
            row,
        );
        Ok(())
    }

    fn get_challenge(&self, _challenge: Challenge) -> Value<F> {
        Value::unknown()
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn push_namespace<NR, N>(&mut self, name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
        self.synthesizer.push_namespace(name_fn().into());
    }

    fn pop_namespace(&mut self, gadget_name: Option<String>) {
        self.synthesizer.pop_namespace(gadget_name);
    }

    fn push_group<N, NR, K>(&mut self, name: N, key: K)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
        K: groups::GroupKey,
    {
        self.group_depth += 1;
        let name: String = name().into();
        log::info!("{}> Pushing group '{name}'", "-".repeat(self.group_depth));

        self.synthesizer.enter_group(name, *GroupKeyInstance::from(key));
    }

    fn pop_group(&mut self, meta: groups::RegionsGroup) {
        log::info!("{}> Popping group", "-".repeat(self.group_depth));
        log::info!(
            "{}>   Inputs:  {:?}",
            "-".repeat(self.group_depth),
            Vec::from_iter(meta.inputs().map(CellDbg))
        );
        log::info!(
            "{}>   Outputs: {:?}",
            "-".repeat(self.group_depth),
            Vec::from_iter(meta.outputs().map(CellDbg))
        );
        self.group_depth -= 1;
        self.synthesizer.exit_group(meta)
    }
}

#[derive(Debug)]
struct ExtractionLayouterRegion<'r, 'a, 'b, F: Field> {
    layouter: &'r mut ExtractionLayouter<'a, 'b, F>,
    region_index: RegionIndex,
    /// Stores the constants to be assigned, and the cells to which they are
    /// copied.
    constants: Vec<(Rational<F>, Cell)>,
}

impl<'r, 'a, 'b, F: Field> ExtractionLayouterRegion<'r, 'a, 'b, F> {
    fn new(layouter: &'r mut ExtractionLayouter<'a, 'b, F>, region_index: RegionIndex) -> Self {
        Self {
            layouter,
            region_index,
            constants: vec![],
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a, 'b, F: Field> RegionLayouter<F> for ExtractionLayouterRegion<'_, 'a, 'b, F> {
    fn enable_selector<'v>(
        &'v mut self,
        annotation: &'v (dyn Fn() -> String + 'v),
        selector: &Selector,
        offset: usize,
    ) -> Result<(), Error> {
        log::info!(
            "{}> Enabled selector {} @ R{}+{offset}(={}) (note: {:?})",
            "-".repeat(self.layouter.group_depth),
            selector.index(),
            *self.region_index,
            *self.layouter.regions[*self.region_index] + offset,
            annotation()
        );
        self.layouter.synthesizer.enable_selector(
            selector,
            *self.layouter.regions[*self.region_index] + offset,
        );
        Ok(())
    }

    fn name_column<'v>(
        &'v mut self,
        _annotation: &'v (dyn Fn() -> String + 'v),
        _column: Column<Any>,
    ) {
    }

    fn assign_advice<'v>(
        &'v mut self,
        annotation: &'v (dyn Fn() -> String + 'v),
        column: Column<Advice>,
        offset: usize,
        _to: &'v mut (dyn FnMut() -> Value<Rational<F>> + 'v),
    ) -> Result<Cell, Error> {
        log::info!(
            "{}> Assigned advice to Adv:{} @ R{}+{offset}(={}) (note: {:?})",
            "-".repeat(self.layouter.group_depth),
            column.index(),
            *self.region_index,
            *self.layouter.regions[*self.region_index] + offset,
            annotation()
        );
        self.layouter
            .synthesizer
            .on_advice_assigned(column, *self.layouter.regions[*self.region_index] + offset);

        Ok(Cell {
            region_index: self.region_index,
            row_offset: offset,
            column: column.into(),
        })
    }

    fn assign_advice_from_constant<'v>(
        &'v mut self,
        annotation: &'v (dyn Fn() -> String + 'v),
        column: Column<Advice>,
        offset: usize,
        constant: Rational<F>,
    ) -> Result<Cell, Error> {
        log::info!(
            "{}> Assigned advice to Adv:{} @ R{}+{offset}(={}) with constant (note: {:?})",
            "-".repeat(self.layouter.group_depth),
            column.index(),
            *self.region_index,
            *self.layouter.regions[*self.region_index] + offset,
            annotation()
        );
        let advice =
            self.assign_advice(annotation, column, offset, &mut || Value::known(constant))?;
        self.constrain_constant(advice, constant)?;

        Ok(advice)
    }

    fn assign_advice_from_instance<'v>(
        &mut self,
        annotation: &'v (dyn Fn() -> String + 'v),
        instance: Column<Instance>,
        row: usize,
        advice: Column<Advice>,
        offset: usize,
    ) -> Result<(Cell, Value<F>), Error> {
        log::info!(
            "{}> Assigned advice to Adv:{} @ R{}+{offset}(={}) with instance (Ins:{}, {row}) (note: {:?})",
            "-".repeat(self.layouter.group_depth),
            advice.index(),
            *self.region_index,
            *self.layouter.regions[*self.region_index] + offset,
            instance.index(),
            annotation()
        );
        let cell = self.assign_advice(annotation, advice, offset, &mut || Value::unknown())?;

        self.layouter.synthesizer.copy(
            cell.column,
            *self.layouter.regions[*cell.region_index] + cell.row_offset,
            instance,
            row,
        );

        Ok((cell, Value::unknown()))
    }

    fn instance_value(
        &mut self,
        _instance: Column<Instance>,
        _row: usize,
    ) -> Result<Value<F>, Error> {
        Ok(Value::unknown())
    }

    fn assign_fixed<'v>(
        &'v mut self,
        annotation: &'v (dyn Fn() -> String + 'v),
        column: Column<Fixed>,
        offset: usize,
        to: &'v mut (dyn FnMut() -> Value<Rational<F>> + 'v),
    ) -> Result<Cell, Error> {
        log::info!(
            "{}> Assigned fixed to Fix:{} @ R{}+{offset}(={}) (note: {:?})",
            "-".repeat(self.layouter.group_depth),
            column.index(),
            *self.region_index,
            *self.layouter.regions[*self.region_index] + offset,
            annotation()
        );

        self.layouter.synthesizer.on_fixed_assigned(
            column,
            *self.layouter.regions[*self.region_index] + offset,
            steal(to())
                .ok_or_else(|| {
                    Error::Synthesis(format!(
                        "Unknown fixed value assigned to cell ({}, {offset})",
                        column.index()
                    ))
                })?
                .evaluate(),
        );

        Ok(Cell {
            region_index: self.region_index,
            row_offset: offset,
            column: column.into(),
        })
    }

    fn constrain_constant(&mut self, cell: Cell, constant: Rational<F>) -> Result<(), Error> {
        self.constants.push((constant, cell));
        Ok(())
    }

    fn constrain_equal(&mut self, left: Cell, right: Cell) -> Result<(), Error> {
        log::info!(
            "{}> {:?}(={}) === {:?}(={})",
            "-".repeat(self.layouter.group_depth),
            CellDbg(left),
            *self.layouter.regions[*left.region_index] + left.row_offset,
            CellDbg(right),
            *self.layouter.regions[*right.region_index] + right.row_offset,
        );
        self.layouter.synthesizer.copy(
            left.column,
            *self.layouter.regions[*left.region_index] + left.row_offset,
            right.column,
            *self.layouter.regions[*right.region_index] + right.row_offset,
        );

        Ok(())
    }
}

#[derive(Debug)]
struct ExtractionTableLayouter<'r, 'a, F: Field> {
    synthesizer: &'a mut Synthesizer<F>,
    used_columns: &'r [TableColumn],
    /// maps from a fixed column to a pair (default value, vector saying which
    /// rows are assigned)
    #[allow(clippy::type_complexity)]
    pub default_and_assigned: HashMap<TableColumn, (Option<Value<Rational<F>>>, Vec<bool>)>,
}

impl<'r, 'a, F: Field> ExtractionTableLayouter<'r, 'a, F> {
    /// Returns a new SimpleTableLayouter
    pub fn new(synthesizer: &'a mut Synthesizer<F>, used_columns: &'r [TableColumn]) -> Self {
        ExtractionTableLayouter {
            synthesizer,
            used_columns,
            default_and_assigned: HashMap::default(),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'r, 'a, F: Field> TableLayouter<F> for ExtractionTableLayouter<'r, 'a, F> {
    fn assign_cell<'v>(
        &'v mut self,
        _annotation: &'v (dyn Fn() -> String + 'v),
        column: TableColumn,
        offset: usize,
        to: &'v mut (dyn FnMut() -> Value<Rational<F>> + 'v),
    ) -> Result<(), Error> {
        if self.used_columns.contains(&column) {
            return Err(Error::TableError(TableError::UsedColumn(column)));
        }

        let entry = self.default_and_assigned.entry(column).or_default();

        let value = to();
        self.synthesizer.on_fixed_assigned(
            column.inner(),
            offset, // tables are always assigned starting at row 0
            steal(value.evaluate())
                .ok_or_else(|| Error::Synthesis("Unknown table value".to_string()))?,
        );

        match (entry.0.is_none(), offset) {
            // Use the value at offset 0 as the default value for this table column.
            (true, 0) => entry.0 = Some(value),
            // Since there is already an existing default value for this table column,
            // the caller should not be attempting to assign another value at offset 0.
            (false, 0) => {
                return Err(Error::TableError(TableError::OverwriteDefault(
                    column,
                    format!("{:?}", entry.0.unwrap()),
                    format!("{value:?}"),
                )));
            }
            _ => (),
        }
        if entry.1.len() <= offset {
            entry.1.resize(offset + 1, false);
        }
        entry.1[offset] = true;

        Ok(())
    }
}

/// Wrapper over [`Layouter`] that implements
/// [`LayoutAdaptor`](extractor_support::cells::ctx::LayoutAdaptor).
#[derive(Debug)]
pub struct AdaptsLayouter<'l, L> {
    layouter: &'l mut L,
}

impl<'l, L> AdaptsLayouter<'l, L> {
    /// Constructs a new wrapper.
    pub fn new(layouter: &'l mut L) -> Self {
        Self { layouter }
    }
}

impl<F: Field, L: Layouter<F>> LayoutAdaptor<F, ExtractionSupport> for AdaptsLayouter<'_, L> {
    type Adaptee = L;

    fn adaptee_ref(&self) -> &L {
        self.layouter
    }

    fn adaptee_ref_mut(&mut self) -> &mut L {
        self.layouter
    }

    fn constrain_instance(
        &mut self,
        cell: Cell,
        instance_col: Column<Instance>,
        instance_row: usize,
    ) -> Result<(), Error> {
        self.layouter.constrain_instance(cell, instance_col, instance_row)
    }

    fn constrain_advice_constant(
        &mut self,
        advice_col: Column<Advice>,
        advice_row: usize,
        constant: F,
    ) -> Result<Cell, Error> {
        Ok(self
            .layouter
            .assign_region(
                || format!("Adv[{}, {advice_row}] == 0", advice_col.index()),
                |mut region| {
                    region.assign_advice_from_constant(
                        || format!("Adv[{}, {advice_row}]", advice_col.index()),
                        advice_col,
                        advice_row,
                        constant,
                    )
                },
            )?
            .cell())
    }

    fn assign_advice_from_instance<V>(
        &mut self,
        advice_col: Column<Advice>,
        advice_row: usize,
        instance_col: Column<Instance>,
        instance_row: usize,
    ) -> Result<AssignedCell<V, F>, Error>
    where
        V: Clone,
        Rational<F>: for<'v> From<&'v V>,
    {
        let c = self.layouter.assign_region(
            || "ins",
            |mut region| {
                region.assign_advice(
                    || {
                        format!(
                            "Adv[{}, +{advice_row}] == Ins[{}, {instance_row}]",
                            advice_col.index(),
                            instance_col.index()
                        )
                    },
                    advice_col,
                    advice_row,
                    || Value::unknown(),
                )
            },
        )?;

        self.layouter.constrain_instance(c.cell(), instance_col, instance_row)?;
        Ok(c)
    }

    fn copy_advice<V>(
        &mut self,
        ac: &AssignedCell<V, F>,
        region: &mut Region<'_, F>,
        advice_col: Column<Advice>,
        advice_row: usize,
    ) -> Result<AssignedCell<V, F>, Error>
    where
        V: Clone,
        Rational<F>: for<'v> From<&'v V>,
    {
        ac.copy_advice(|| "", region, advice_col, advice_row)
    }

    fn region<A, AR, N, NR>(&mut self, name: N, assignment: A) -> Result<AR, Error>
    where
        A: FnMut(Region<'_, F>) -> Result<AR, Error>,
        N: Fn() -> NR,
        NR: Into<String>,
    {
        self.layouter.assign_region(name, assignment)
    }
}

#[allow(clippy::type_complexity)]
fn compute_table_lengths<F: std::fmt::Debug>(
    default_and_assigned: &HashMap<TableColumn, (Option<Value<Rational<F>>>, Vec<bool>)>,
) -> Result<usize, Error> {
    let column_lengths: Result<Vec<_>, Error> = default_and_assigned
        .iter()
        .map(|(col, (default_value, assigned))| {
            if default_value.is_none() || assigned.is_empty() {
                return Err(Error::TableError(TableError::ColumnNotAssigned(*col)));
            }
            if assigned.iter().all(|b| *b) {
                // All values in the column have been assigned
                Ok((col, assigned.len()))
            } else {
                Err(Error::TableError(TableError::ColumnNotAssigned(*col)))
            }
        })
        .collect();
    let column_lengths = column_lengths?;
    column_lengths
        .into_iter()
        .try_fold((None, 0), |acc, (col, col_len)| {
            if acc.1 == 0 || acc.1 == col_len {
                Ok((Some(*col), col_len))
            } else {
                let mut cols = [(*col, col_len), (acc.0.unwrap(), acc.1)];
                cols.sort();
                Err(Error::TableError(TableError::UnevenColumnLengths(
                    cols[0], cols[1],
                )))
            }
        })
        .map(|col_len| col_len.1)
}

fn steal<T>(value: Value<T>) -> Option<T> {
    let data = RefCell::new(None);
    value.map(|t| data.replace(Some(t)));
    data.replace(None)
}

struct CellDbg(Cell);

impl std::fmt::Debug for CellDbg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cell = &self.0;
        write!(
            f,
            "({}:{}, R{}+{})",
            match cell.column.column_type() {
                Any::Advice(_) => "Adv",
                Any::Fixed => "Fix",
                Any::Instance => "Ins",
            },
            cell.column.index(),
            *cell.region_index,
            cell.row_offset
        )
    }
}
