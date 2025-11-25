use std::{cell::RefCell, marker::PhantomData};

use anyhow::{Context, Result};
use configuration::Config;
use extractor_support::cells::ctx::{ICtx, InputDescr, OCtx};
use ff::PrimeField;
use haloumi::{to_plonk_error, CircuitIO, CircuitSynthesis, ExpressionInRow};
use haloumi_ir::stmt::IRStmt;
use midnight_circuit::midnight_proofs::{
    circuit::{Layouter, RegionIndex, SimpleFloorPlanner},
    plonk::{Advice, Circuit, ConstraintSystem, Error, Instance},
};

use crate::harness::Ctx;
use mdnt_support::{
    cells::{load::LoadFromCells, store::StoreIntoCells, CellReprSize},
    circuit::injected::InjectedIR,
};

pub mod assignment;
pub mod configuration;
pub mod traits;

pub use traits::*;

pub struct Function;
pub struct Procedure;
pub struct FunctionMut;

/// Scaffold for implementations of [`AbstractCircuit`].
///
/// The circuit has two modes; function or procedure. The mode is configured by passing either
/// [`Function`] or [`Procedure`] to the `M` type parameter. By default is set to [`Function`].
pub struct CircuitImpl<'a, F, C, M = Function> {
    abstract_circuit: C,
    constants: &'a [String],
    injected_ir: RefCell<InjectedIR<F>>,
    _mode: PhantomData<M>,
}

impl<'a, F, C, M> CircuitImpl<'a, F, C, M> {
    pub fn new<'c: 'a>(ctx: &'c Ctx<'a>, abstract_circuit: C) -> Self {
        Self {
            abstract_circuit,
            constants: ctx.constants(),
            injected_ir: Default::default(),
            _mode: Default::default(),
        }
    }

    /// Consumes the circuit wrapper and returns the extra IR added during synthesis.
    pub fn take_injected_ir<'ir>(self) -> Vec<(RegionIndex, IRStmt<ExpressionInRow<'ir, F>>)>
    where
        F: Clone,
    {
        self.injected_ir
            .into_inner()
            .into_iter()
            .map(|(idx, ir)| (idx, IRStmt::<ExpressionInRow<_>>::seq(ir).into()))
            .collect()
    }
}

macro_rules! contextualize {
    ($res:expr, $ctx:expr) => {
        $res.map_err(anyhow::Error::from).context($ctx)
    };
}

impl<F, C, M, Chip, Input, Output, Args> CircuitImpl<'_, F, C, M>
where
    F: PrimeField,
    C: AbstractCircuitIO<F, Chip = Chip, Input = Input, Output = Output> + ChipArgs<F, Args = Args>,
    Chip: CircuitInitialization<F, Args = Args>,
{
    fn create_chip(&self, config: &Config<F, C>) -> Chip {
        Chip::new_chip(&config.chip.inner, self.abstract_circuit.chip_args())
    }

    fn load_chip(
        &self,
        layouter: &mut impl Layouter<F>,
        chip: &Chip,
        config: &Config<F, C>,
    ) -> Result<(), Error> {
        contextualize!(
            chip.load_chip(layouter, &config.chip.inner),
            "Failed to load the chip"
        )
        .map_err(to_plonk_error)
    }

    fn load<L: LoadFromCells<F, Chip>>(
        &self,
        cells: impl IntoIterator<Item = InputDescr>,
        n_cells: usize,
        cell_type: &'static str,
        chip: &Chip,
        layouter: &mut impl Layouter<F>,
    ) -> Result<L, Error> {
        let mut injected_ir = self.injected_ir.borrow_mut();
        Ok(L::load(
            &mut ICtx::new(
                cells
                    .into_iter()
                    .enumerate()
                    .inspect(|(idx, i)| {
                        log::debug!("{cell_type} cell {}/{n_cells}: {i:?}", idx + 1)
                    })
                    .map(|(_, i)| i),
                self.constants,
            ),
            chip,
            layouter,
            &mut injected_ir,
        )?)
    }

    fn load_inputs(
        &self,
        config: &Config<F, C>,
        chip: &Chip,
        layouter: &mut impl Layouter<F>,
    ) -> Result<Input, Error>
    where
        Input: LoadFromCells<F, Chip>,
    {
        let inputs = config.inputs();
        let n_inputs = inputs.len();
        let input = self.load(inputs, n_inputs, "Input", chip, layouter);
        contextualize!(input, "Failed to load the inputs").map_err(to_plonk_error)
    }

    fn load_outputs(
        &self,
        config: &Config<F, C>,
        chip: &Chip,
        layouter: &mut impl Layouter<F>,
    ) -> Result<Output, Error>
    where
        Output: LoadFromCells<F, Chip>,
    {
        let outputs = config.outputs();
        let n_outputs = outputs.len();
        let output = self.load(
            outputs.into_iter().map(Into::into),
            n_outputs,
            "Output",
            chip,
            layouter,
        );
        contextualize!(output, "Failed to load the outputs").map_err(to_plonk_error)
    }

    fn store_outputs(
        &self,
        output: Output,
        config: &Config<F, C>,
        chip: &Chip,
        layouter: &mut impl Layouter<F>,
    ) -> Result<(), Error>
    where
        Output: StoreIntoCells<F, Chip>,
    {
        let outputs = config.outputs();
        // Store the results
        let n_outputs = outputs.len();
        let mut injected_ir = self.injected_ir.borrow_mut();
        contextualize!(
            output.store(
                &mut OCtx::new(
                    outputs
                        .into_iter()
                        .enumerate()
                        .inspect(|(idx, o)| log::debug!(
                            "Output cell {}/{}: {o:?}",
                            idx + 1,
                            n_outputs
                        ))
                        .map(|(_, o)| o),
                ),
                chip,
                layouter,
                &mut injected_ir,
            ),
            "Failed to write the outputs"
        )
        .map_err(to_plonk_error)?;
        Ok(())
    }
}

impl<F, C, Chip, Input, Output> Circuit<F> for CircuitImpl<'_, F, C, Function>
where
    F: PrimeField,
    C: AbstractCircuit<F>
        + ChipArgs<F>
        + AbstractCircuitIO<F, Chip = Chip, Input = Input, Output = Output>,
    Chip: CircuitInitialization<F, Args = C::Args>,
    Input: LoadFromCells<F, Chip>,
    Output: StoreIntoCells<F, Chip>,
{
    type Config = Config<F, C>;

    type FloorPlanner = SimpleFloorPlanner;

    type Params = ();

    fn without_witnesses(&self) -> Self {
        unimplemented!()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        Self::Config::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // Create and load chip.
        let chip = self.create_chip(&config);
        let input = self.load_inputs(&config, &chip, &mut layouter)?;
        let output = {
            let mut injected_ir = self.injected_ir.borrow_mut();
            // Call the inner circuit
            contextualize!(
                self.abstract_circuit.synthesize(&chip, &mut layouter, input, &mut injected_ir),
                "Failed to run the inner method"
            )
            .map_err(to_plonk_error)
        }?;

        self.store_outputs(output, &config, &chip, &mut layouter)?;

        self.load_chip(&mut layouter, &chip, &config)
    }
}

impl<F, C, Chip, Input, Output> Circuit<F> for CircuitImpl<'_, F, C, FunctionMut>
where
    F: PrimeField,
    C: AbstractCircuitMut<F>
        + ChipArgs<F>
        + AbstractCircuitIO<F, Chip = Chip, Input = Input, Output = Output>,
    Chip: CircuitInitialization<F, Args = C::Args>,
    Input: LoadFromCells<F, Chip>,
    Output: StoreIntoCells<F, Chip>,
{
    type Config = Config<F, C>;

    type FloorPlanner = SimpleFloorPlanner;

    type Params = ();

    fn without_witnesses(&self) -> Self {
        unimplemented!()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        Self::Config::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // Create and load chip.
        let mut chip = self.create_chip(&config);
        let input = self.load_inputs(&config, &chip, &mut layouter)?;
        let output = {
            let mut injected_ir = self.injected_ir.borrow_mut();
            // Call the inner circuit
            contextualize!(
                self.abstract_circuit.synthesize_mut(
                    &mut chip,
                    &mut layouter,
                    input,
                    &mut injected_ir
                ),
                "Failed to run the inner method"
            )
            .map_err(to_plonk_error)
        }?;

        self.store_outputs(output, &config, &chip, &mut layouter)?;

        self.load_chip(&mut layouter, &chip, &config)
    }
}

impl<F, C, Chip, Input, Output> Circuit<F> for CircuitImpl<'_, F, C, Procedure>
where
    F: PrimeField,
    C: AbstractUnitCircuit<F>
        + ChipArgs<F>
        + AbstractCircuitIO<F, Chip = Chip, Input = Input, Output = Output>,
    Chip: CircuitInitialization<F, Args = C::Args>,
    Input: LoadFromCells<F, Chip>,
    Output: LoadFromCells<F, Chip>,
{
    type Config = Config<F, C>;

    type FloorPlanner = SimpleFloorPlanner;

    type Params = ();

    fn without_witnesses(&self) -> Self {
        unimplemented!()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        Self::Config::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> std::result::Result<(), Error> {
        // Create and load chip.
        let chip = self.create_chip(&config);
        let input = self.load_inputs(&config, &chip, &mut layouter)?;
        let output = self.load_outputs(&config, &chip, &mut layouter)?;

        let mut injected_ir = self.injected_ir.borrow_mut();
        // Call the inner circuit
        contextualize!(
            self.abstract_circuit
                .synthesize(&chip, &mut layouter, input, output, &mut injected_ir),
            "Failed to run the inner method"
        )
        .map_err(to_plonk_error)?;

        self.load_chip(&mut layouter, &chip, &config)
    }
}

impl<F, C, M> CircuitSynthesis<F> for CircuitImpl<'_, F, C, M>
where
    F: PrimeField,
    C: AbstractCircuitIO<F> + ChipArgs<F>,
    Self: Circuit<F>,
    <Self as Circuit<F>>::Config: AbstractCircuitConfig,
{
    type Circuit = Self;
    type Config = <Self as Circuit<F>>::Config;
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn circuit(&self) -> &Self::Circuit {
        self
    }

    fn configure(cs: &mut Self::CS) -> Self::Config {
        <Self as Circuit<F>>::configure(cs)
    }

    fn advice_io(_: &Self::Config) -> anyhow::Result<CircuitIO<Advice>> {
        Ok(CircuitIO::empty())
    }

    fn instance_io(config: &Self::Config) -> anyhow::Result<CircuitIO<Instance>> {
        let inputs: Vec<_> = (0..C::Input::SIZE).collect();
        let outputs: Vec<_> = (0..C::Output::SIZE).collect();

        CircuitIO::new(
            &[(config.input_instance(), &inputs)],
            &[(config.output_instance(), &outputs)],
        )
    }

    fn synthesize(
        circuit: &Self::Circuit,
        config: Self::Config,
        synthesizer: &mut halo2_llzk_frontend::Synthesizer<F>,
        cs: &Self::CS,
    ) -> Result<(), Self::Error> {
        assignment::SynthesizerAssignment::synthesize(circuit, config, synthesizer, cs)
    }
}
