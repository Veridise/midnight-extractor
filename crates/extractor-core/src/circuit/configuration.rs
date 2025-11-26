use crate::circuit::AbstractCircuitIO;
use ff::{Field, PrimeField};
use mdnt_support::cells::ctx::{Cell, InputDescr, OutputDescr};
use mdnt_support::cells::CellReprSize;
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::midnight_proofs::plonk::{
    Advice, Column, ColumnType, ConstraintSystem, Fixed, Instance,
};
use midnight_proofs::ExtractionSupport;

use super::AbstractCircuitConfig;

use mdnt_support::circuit::configuration::AutoConfigure;

/// Configuration for a chip type that implements [`CircuitInitialization`].
#[derive(Debug, Clone)]
pub struct ChipConfig<C, I> {
    pub cfg: C,
    pub inner: I,
}

//impl<L, C> Clone for ChipConfig<L, C>
//where
//    C: CircuitInitialization<L>,
//{
//    fn clone(&self) -> Self {
//        Self {
//            cfg: self.cfg.clone(),
//            inner: self.inner.clone(),
//        }
//    }
//}

impl<C, I> ChipConfig<C, I> {
    fn configure<CI, L, F>(meta: &mut ConstraintSystem<F>) -> Self
    where
        CI: CircuitInitialization<L, Config = I, ConfigCols = C>,
        F: PrimeField,
    {
        let cfg = C::Cols::configure(meta);
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
    fn configure<L: Field>(meta: &mut ConstraintSystem<L>) -> Self {
        let instance = meta.instance_column();
        let helper = meta.advice_column();

        meta.enable_equality(helper);
        meta.enable_equality(instance);
        Self { instance, helper }
    }

    fn to_cell<C: ColumnType>(self, row: usize) -> Cell<Column<C>>
    where
        Column<C>: From<Column<Instance>>,
    {
        (self.instance.into(), row).into()
    }

    fn descrs<C: ColumnType, D>(
        &self,
        ctor: impl Fn(Cell<Column<C>>, Column<Advice>) -> D,
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
    fn configure<L: Field>(meta: &mut ConstraintSystem<L>) -> Self {
        Self {
            input: IOColumn::configure(meta),
            output: IOColumn::configure(meta),
        }
    }

    fn inputs<F: PrimeField>(&self) -> impl Iterator<Item = InputDescr<F, ExtractionSupport>> {
        self.input.descrs(InputDescr::new)
    }

    fn outputs<F: PrimeField>(&self) -> impl Iterator<Item = OutputDescr<F, ExtractionSupport>> {
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
    fn configure<L: Field>(meta: &mut ConstraintSystem<L>) -> Self {
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
#[derive(Clone)]
pub struct Config<C, I> {
    pub io: IOConfig,
    pub chip: ChipConfig<C, I>,
    pub constants: Constants,
}

impl<L, C> Config<L, C> {
    pub fn configure<CI, L, F>(meta: &mut ConstraintSystem<F>) -> Self
    where
        F: PrimeField,
        CI: CircuitInitialization<L, Config = I, ConfigCols = C>,
    {
        log::info!(
            "Circuit has {} inputs and {} outputs",
            C::Input::SIZE,
            C::Output::SIZE
        );
        Self {
            io: IOConfig::configure(meta),
            chip: ChipConfig::configure::<CI, L, F>(meta),
            constants: Constants::configure(meta),
        }
    }
}

//impl< C,I> Clone for Config< C,I>
//{
//    fn clone(&self) -> Self {
//        Self {
//            chip: self.chip.clone(),
//            io: self.io,
//            constants: self.constants,
//        }
//    }
//}

impl<L, C> AbstractCircuitConfig for Config<L, C>
where
    C: AbstractCircuitIO<L>,
{
    fn inputs<F: PrimeField>(&self) -> Vec<InputDescr<F, ExtractionSupport>> {
        self.io.inputs().take(C::Input::SIZE).collect()
    }

    fn outputs<F: PrimeField>(&self) -> Vec<OutputDescr<F, ExtractionSupport>> {
        self.io.outputs().take(C::Output::SIZE).collect()
    }

    fn input_instance(&self) -> Column<Instance> {
        self.io.input.instance
    }

    fn output_instance(&self) -> Column<Instance> {
        self.io.output.instance
    }
}
