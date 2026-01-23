use crate::{
    circuit::layouter::{AdaptsLayouter, ExtractionLayouter},
    harness::Ctx,
};
use anyhow::{Context, Result};
use configuration::Config;
use ff::{Field, PrimeField};
use haloumi::{
    expressions::ExpressionInRow, AdviceIO, CircuitIO, CircuitSynthesis, InstanceIO, Synthesizer,
};
use haloumi_ir::stmt::IRStmt;
use mdnt_support::{
    cells::{
        ctx::{ICtx, InputDescr, OCtx},
        load::LoadFromCells,
        store::StoreIntoCells,
        CellReprSize,
    },
    circuit::{
        configuration::AutoConfigure, injected::InjectedIR, ChipArgs, CircuitInitialization,
    },
};
use midnight_proofs::{
    circuit::{Layouter, RegionIndex},
    plonk::{ConstraintSystem, Error, Expression},
    ExtractionSupport,
};
use std::{cell::RefCell, marker::PhantomData};

//pub mod assignment;
pub mod configuration;
pub mod layouter;
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
    injected_ir: RefCell<InjectedIR<RegionIndex, Expression<F>>>,
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
    pub fn take_injected_ir<'ir>(
        self,
    ) -> Vec<(RegionIndex, IRStmt<ExpressionInRow<'ir, Expression<F>, F>>)>
    where
        F: Field,
    {
        self.injected_ir
            .into_inner()
            .into_iter()
            .map(|(idx, ir)| {
                (
                    idx,
                    IRStmt::<ExpressionInRow<_, F>>::seq(
                        ir.into_iter().map(|s| s.map(&mut |(row, e)| ExpressionInRow::new(row, e))),
                    )
                    .into(),
                )
            })
            .collect()
    }
}

macro_rules! contextualize {
    ($res:expr, $ctx:expr) => {
        $res.map_err(anyhow::Error::from).context($ctx)
    };
}

impl<F, C, M> CircuitImpl<'_, F, C, M>
where
    F: PrimeField,
{
    fn create_chip<L>(&self, config: &Config<C>) -> C::Chip
    where
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<
            L,
            Args = C::Args,
            Config = C::Config,
            ConfigCols = C::ConfigCols,
        >,
    {
        C::Chip::new_chip(&config.chip.inner, self.abstract_circuit.chip_args())
    }

    fn load_chip<L>(
        &self,
        layouter: &mut L,
        chip: &C::Chip,
        config: &Config<C>,
    ) -> Result<(), Error>
    where
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<
            L,
            Args = C::Args,
            Config = C::Config,
            ConfigCols = C::ConfigCols,
            Error = Error,
        >,
    {
        contextualize!(
            chip.load_chip(layouter, &config.chip.inner),
            "Failed to load the chip"
        )
        .map_err(to_plonk_error)
    }

    fn load<Load, L>(
        &self,
        cells: impl IntoIterator<Item = InputDescr<F, ExtractionSupport>>,
        n_cells: usize,
        cell_type: &'static str,
        chip: &C::Chip,
        layouter: &mut L,
    ) -> Result<Load, Error>
    where
        Load: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<L, Args = C::Args, Error = Error>,
        L: Layouter<F>,
    {
        let mut layouter = AdaptsLayouter::new(layouter);
        let mut injected_ir = self.injected_ir.borrow_mut();
        Load::load(
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
            &mut layouter,
            &mut injected_ir,
        )
    }

    fn load_inputs<L>(
        &self,
        config: &Config<C>,
        chip: &C::Chip,
        layouter: &mut L,
    ) -> Result<C::Input, Error>
    where
        C::Input: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<L, Args = C::Args, Error = Error>,
        L: Layouter<F>,
    {
        let inputs = config.inputs();
        let n_inputs = inputs.len();
        let input = self.load(inputs, n_inputs, "Input", chip, layouter);
        contextualize!(input, "Failed to load the inputs").map_err(to_plonk_error)
    }

    fn load_outputs<L>(
        &self,
        config: &Config<C>,
        chip: &C::Chip,
        layouter: &mut L,
    ) -> Result<C::Output, Error>
    where
        C::Output: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<L, Args = C::Args, Error = Error>,
        L: Layouter<F>,
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

    fn store_outputs<L>(
        &self,
        output: C::Output,
        config: &Config<C>,
        chip: &C::Chip,
        layouter: &mut L,
    ) -> Result<(), Error>
    where
        C::Output: StoreIntoCells<F, C::Chip, ExtractionSupport, L>,
        C: AbstractCircuitIO + ChipArgs,
        C::Chip: CircuitInitialization<L, Args = C::Args, Error = Error>,
        L: Layouter<F>,
    {
        let mut layouter = AdaptsLayouter::new(layouter);
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
                &mut layouter,
                &mut injected_ir,
            ),
            "Failed to write the outputs"
        )
        .map_err(to_plonk_error)?;
        Ok(())
    }
}

impl<F, C> CircuitImpl<'_, F, C, Function> {
    fn synthesize_inner<L>(
        &self,
        config: Config<C>,
        mut layouter: L,
    ) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractCircuit<F> + ChipArgs + AbstractCircuitIO,
        C::Chip: CircuitInitialization<
            L,
            Args = C::Args,
            Config = C::Config,
            ConfigCols = C::ConfigCols,
            CS = ConstraintSystem<F>,
            Error = Error,
        >,
        C::Input: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C::Output: StoreIntoCells<F, C::Chip, ExtractionSupport, L>,
    {
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

impl<F, C> CircuitImpl<'_, F, C, FunctionMut> {
    fn synthesize_inner<L>(
        &self,
        config: Config<C>,
        mut layouter: L,
    ) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractCircuitMut<F> + ChipArgs + AbstractCircuitIO,
        C::Chip: CircuitInitialization<
            L,
            Args = C::Args,
            Config = C::Config,
            ConfigCols = C::ConfigCols,
            CS = ConstraintSystem<F>,
            Error = Error,
        >,
        C::Input: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C::Output: StoreIntoCells<F, C::Chip, ExtractionSupport, L>,
    {
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

impl<F, C> CircuitImpl<'_, F, C, Procedure> {
    fn synthesize_inner<L>(
        &self,
        config: Config<C>,
        mut layouter: L,
    ) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractUnitCircuit<F> + ChipArgs + AbstractCircuitIO,
        C::Chip: CircuitInitialization<
            L,
            Args = C::Args,
            Config = C::Config,
            ConfigCols = C::ConfigCols,
            CS = ConstraintSystem<F>,
            Error = Error,
        >,
        C::Input: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
        C::Output: LoadFromCells<F, C::Chip, ExtractionSupport, L>,
    {
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

impl<F, C> CircuitSynthesis<F> for CircuitImpl<'_, F, C, Function>
where
    F: PrimeField,
    C: AbstractCircuit<F> + ChipArgs,
    C::Chip: for<'a, 'b> CircuitInitialization<
        ExtractionLayouter<'a, 'b, F>,
        Args = C::Args,
        Config = C::Config,
        ConfigCols = C::ConfigCols,
        CS = ConstraintSystem<F>,
        Error = Error,
    >,
    C::ConfigCols: AutoConfigure<ConstraintSystem<F>>,
    C::Input:
        for<'a, 'b> LoadFromCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
    C::Output:
        for<'a, 'b> StoreIntoCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
{
    type Circuit = Self;
    type Config = Config<C>;
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn circuit(&self) -> &Self::Circuit {
        self
    }

    fn configure(cs: &mut Self::CS) -> Self::Config {
        Self::Config::configure(cs)
    }

    fn advice_io(_: &Self::Config) -> anyhow::Result<AdviceIO> {
        Ok(CircuitIO::empty())
    }

    fn instance_io(config: &Self::Config) -> anyhow::Result<InstanceIO> {
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
        synthesizer: &mut Synthesizer<F>,
        cs: &Self::CS,
    ) -> Result<(), Self::Error> {
        let layouter = ExtractionLayouter::new(synthesizer, cs.constants());
        circuit.synthesize_inner(config, layouter)
    }
}

impl<F, C> CircuitSynthesis<F> for CircuitImpl<'_, F, C, FunctionMut>
where
    F: PrimeField,
    C: AbstractCircuitMut<F> + ChipArgs,
    C::Chip: for<'a, 'b> CircuitInitialization<
        ExtractionLayouter<'a, 'b, F>,
        Args = C::Args,
        Config = C::Config,
        ConfigCols = C::ConfigCols,
        CS = ConstraintSystem<F>,
        Error = Error,
    >,
    C::ConfigCols: AutoConfigure<ConstraintSystem<F>>,
    C::Input:
        for<'a, 'b> LoadFromCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
    C::Output:
        for<'a, 'b> StoreIntoCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
{
    type Circuit = Self;
    type Config = Config<C>;
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn circuit(&self) -> &Self::Circuit {
        self
    }

    fn configure(cs: &mut Self::CS) -> Self::Config {
        Self::Config::configure(cs)
    }

    fn advice_io(_: &Self::Config) -> anyhow::Result<AdviceIO> {
        Ok(CircuitIO::empty())
    }

    fn instance_io(config: &Self::Config) -> anyhow::Result<InstanceIO> {
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
        synthesizer: &mut Synthesizer<F>,
        cs: &Self::CS,
    ) -> Result<(), Self::Error> {
        let layouter = ExtractionLayouter::new(synthesizer, cs.constants());
        circuit.synthesize_inner(config, layouter)
    }
}
impl<F, C> CircuitSynthesis<F> for CircuitImpl<'_, F, C, Procedure>
where
    F: PrimeField,
    C: AbstractUnitCircuit<F> + ChipArgs,
    C::Chip: for<'a, 'b> CircuitInitialization<
        ExtractionLayouter<'a, 'b, F>,
        Args = C::Args,
        Config = C::Config,
        ConfigCols = C::ConfigCols,
        CS = ConstraintSystem<F>,
        Error = Error,
    >,
    C::ConfigCols: AutoConfigure<ConstraintSystem<F>>,
    C::Input:
        for<'a, 'b> LoadFromCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
    C::Output:
        for<'a, 'b> LoadFromCells<F, C::Chip, ExtractionSupport, ExtractionLayouter<'a, 'b, F>>,
{
    type Circuit = Self;
    type Config = Config<C>;
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn circuit(&self) -> &Self::Circuit {
        self
    }

    fn configure(cs: &mut Self::CS) -> Self::Config {
        Self::Config::configure(cs)
    }

    fn advice_io(_: &Self::Config) -> anyhow::Result<AdviceIO> {
        Ok(CircuitIO::empty())
    }

    fn instance_io(config: &Self::Config) -> anyhow::Result<InstanceIO> {
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
        synthesizer: &mut Synthesizer<F>,
        cs: &Self::CS,
    ) -> Result<(), Self::Error> {
        let layouter = ExtractionLayouter::new(synthesizer, cs.constants());
        circuit.synthesize_inner(config, layouter)
    }
}

pub fn to_plonk_error<E>(error: E) -> Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Error::Transcript(std::io::Error::other(error))
}
