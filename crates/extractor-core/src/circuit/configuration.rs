use crate::circuit::AbstractCircuitIO;
use ff::{Field, PrimeField};
use mdnt_support::cells::ctx::{Cell, InputDescr, OutputDescr};
use mdnt_support::cells::CellReprSize;
use midnight_circuits::midnight_proofs::plonk::{
    Advice, Column, ColumnType, ConstraintSystem, Fixed, Instance,
};

use super::{AbstractCircuitConfig, CircuitInitialization};

use mdnt_support::circuit::configuration::AutoConfigure;

/// Configuration for a chip type that implements [`CircuitInitialization`].
#[derive(Debug)]
pub struct ChipConfig<F, C>
where
    F: PrimeField,
    C: CircuitInitialization<F>,
{
    pub cfg: C::ConfigCols,
    pub inner: C::Config,
}

impl<F, C> Clone for ChipConfig<F, C>
where
    F: PrimeField,
    C: CircuitInitialization<F>,
{
    fn clone(&self) -> Self {
        Self {
            cfg: self.cfg.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<F, C> ChipConfig<F, C>
where
    F: PrimeField,
    C: CircuitInitialization<F>,
{
    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let cfg = C::ConfigCols::configure(meta);
        let inner = C::configure_circuit(meta, &cfg);
        Self { cfg, inner }
    }
}

/// Configuration for a IO instance column.
///
/// Each IO instance column has an associated advice column that can be used for writting
/// intermediate values if required by the circuit.
#[derive(Clone, Copy, Debug)]
pub struct IOColumn {
    pub instance: Column<Instance>,
    pub helper: Column<Advice>,
}

impl IOColumn {
    fn configure<F: Field>(meta: &mut ConstraintSystem<F>) -> Self {
        let instance = meta.instance_column();
        let helper = meta.advice_column();

        meta.enable_equality(helper);
        meta.enable_equality(instance);
        Self { instance, helper }
    }

    fn to_cell<C: ColumnType>(self, row: usize) -> Cell<C>
    where
        Column<C>: From<Column<Instance>>,
    {
        (self.instance.into(), row).into()
    }

    fn descrs<C: ColumnType, D>(
        &self,
        mut ctor: impl FnMut(Cell<C>, Column<Advice>) -> D,
    ) -> impl Iterator<Item = D>
    where
        Column<C>: From<Column<Instance>>,
    {
        (0..).map(move |row| ctor(self.to_cell(row), self.helper))
    }
}

/// Configuration for the circuit IO used by the circuit scaffold.
#[derive(Clone, Copy, Debug)]
pub struct IOConfig {
    pub input: IOColumn,
    pub output: IOColumn,
}

impl IOConfig {
    fn configure<F: Field>(meta: &mut ConstraintSystem<F>) -> Self {
        Self {
            input: IOColumn::configure(meta),
            output: IOColumn::configure(meta),
        }
    }

    fn inputs(&self) -> impl Iterator<Item = InputDescr> {
        self.input.descrs(InputDescr::new)
    }

    fn outputs(&self) -> impl Iterator<Item = OutputDescr> {
        self.output.descrs(OutputDescr::new)
    }
}

/// Helper struct that defines a fixed column designed for constants if the constraint system has
/// not defined one already.
#[derive(Clone, Copy, Debug)]
pub struct Constants {
    _helper: Option<Column<Fixed>>,
}

impl Constants {
    fn configure<F: Field>(meta: &mut ConstraintSystem<F>) -> Self {
        let helper = if meta.constants().is_empty() {
            let fixed_helper = meta.fixed_column();
            meta.enable_constant(fixed_helper);
            Some(fixed_helper)
        } else {
            None
        };
        Self { _helper: helper }
    }
}

/// Configuration for a circuit.
pub struct Config<F, C>
where
    F: PrimeField,
    C: AbstractCircuitIO<F>,
{
    pub io: IOConfig,
    pub chip: ChipConfig<F, C::Chip>,
    pub constants: Constants,
}

impl<F, C> Config<F, C>
where
    F: PrimeField,
    C: AbstractCircuitIO<F>,
{
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        log::info!(
            "Circuit has {} inputs and {} outputs",
            C::Input::SIZE,
            C::Output::SIZE
        );
        Self {
            io: IOConfig::configure(meta),
            chip: ChipConfig::configure(meta),
            constants: Constants::configure(meta),
        }
    }
}

impl<F, C> Clone for Config<F, C>
where
    C: AbstractCircuitIO<F>,
    F: PrimeField,
{
    fn clone(&self) -> Self {
        Self {
            chip: self.chip.clone(),
            io: self.io,
            constants: self.constants,
        }
    }
}

impl<F, C> AbstractCircuitConfig for Config<F, C>
where
    F: PrimeField,
    C: AbstractCircuitIO<F>,
{
    fn inputs(&self) -> Vec<InputDescr> {
        self.io.inputs().take(C::Input::SIZE).collect()
    }

    fn outputs(&self) -> Vec<OutputDescr> {
        self.io.outputs().take(C::Output::SIZE).collect()
    }

    fn input_instance(&self) -> Column<Instance> {
        self.io.input.instance
    }

    fn output_instance(&self) -> Column<Instance> {
        self.io.output.instance
    }
}
