use std::{cell::RefCell, marker::PhantomData};

use anyhow::{Context, Result};
use configuration::Config;
use ff::PrimeField;
use haloumi::{expressions::ExpressionInRow, CircuitIO, CircuitSynthesis};
use haloumi_ir::stmt::IRStmt;
use mdnt_support::cells::ctx::{ICtx, InputDescr, OCtx};
use midnight_circuits::midnight_proofs::{
    circuit::{Layouter, RegionIndex},
    plonk::{Advice, Circuit, ConstraintSystem, Error, Instance},
};
use midnight_proofs::{plonk::Expression, ExtractionSupport};

use crate::circuit::layouter::{AdaptsLayouter, ExtractionLayouter};
use crate::harness::Ctx;
use mdnt_support::{
    cells::{load::LoadFromCells, store::StoreIntoCells, CellReprSize},
    circuit::{injected::InjectedIR, AbstractCircuitIO, ChipArgs, CircuitInitialization},
};

pub mod assignment;
pub mod configuration;
mod layouter;
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
    ) -> Vec<(RegionIndex, IRStmt<ExpressionInRow<'ir, Expression<F>>>)>
    where
        F: Clone,
    {
        self.injected_ir
            .into_inner()
            .into_iter()
            .map(|(idx, ir)| {
                (
                    idx,
                    IRStmt::<ExpressionInRow<_>>::seq(
                        ir.into_iter().map(|s| s.map(&|(row, e)| ExpressionInRow::new(row, e))),
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

impl<F, C, M, Chip, Input, Output, Args, E> CircuitImpl<'_, F, C, M>
where
    F: PrimeField,
    C: for<'a> AbstractCircuitIO<
            ExtractionLayouter<'a, F>,
            Chip = Chip,
            Input = Input,
            Output = Output,
        > + ChipArgs<Args = Args>,
    Chip: for<'a> CircuitInitialization<ExtractionLayouter<'a, F>, Args = Args, Error = E>,
    E: std::error::Error,
{
    fn create_chip<L>(&self, config: &Config<L, C>) -> Chip
    where
        C: AbstractCircuitIO<L, Chip = Chip, Input = Input, Output = Output>
            + ChipArgs<Args = Args>,
        Chip: CircuitInitialization<L, Args = Args>,
    {
        Chip::new_chip(&config.chip.inner, self.abstract_circuit.chip_args())
    }

    fn load_chip<L>(
        &self,
        layouter: &mut L,
        chip: &Chip,
        config: &Config<ExtractionLayouter<'_, F>, C>,
    ) -> Result<(), Error>
    where
        F: PrimeField,
        C: AbstractCircuitIO<L, Chip = Chip, Input = Input, Output = Output>
            + ChipArgs<Args = Args>,
        Chip: CircuitInitialization<L, Args = Args, Error = E>,
        E: std::error::Error,
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
        chip: &Chip,
        layouter: &mut L,
    ) -> Result<Load, Error>
    where
        Load: LoadFromCells<F, Chip, ExtractionSupport, L>,
        L: Layouter<F>,
    {
        let mut layouter = AdaptsLayouter::new(layouter);
        let mut injected_ir = self.injected_ir.borrow_mut();
        Ok(Load::load(
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
        )?)
    }

    fn load_inputs<L>(
        &self,
        config: &Config<L, C>,
        chip: &Chip,
        layouter: &mut L,
    ) -> Result<Input, Error>
    where
        Input: LoadFromCells<F, Chip, ExtractionSupport, L>,
        L: Layouter<F>,
    {
        let inputs = config.inputs();
        let n_inputs = inputs.len();
        let input = self.load(inputs, n_inputs, "Input", chip, layouter);
        contextualize!(input, "Failed to load the inputs").map_err(to_plonk_error)
    }

    fn load_outputs<L>(
        &self,
        config: &Config<L, C>,
        chip: &Chip,
        layouter: &mut L,
    ) -> Result<Output, Error>
    where
        Output: LoadFromCells<F, Chip, ExtractionSupport, L>,
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
        output: Output,
        config: &Config<L, C>,
        chip: &Chip,
        layouter: &mut L,
    ) -> Result<(), Error>
    where
        Output: StoreIntoCells<F, Chip, ExtractionSupport, L>,
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

impl<F, C, Chip, Input, Output, E> CircuitImpl<'_, F, C, Function> {
    fn synthesize<L>(&self, config: Config<L, C>, mut layouter: L) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractCircuit<F, L>
            + ChipArgs
            + AbstractCircuitIO<L, Chip = Chip, Input = Input, Output = Output>,
        Chip: CircuitInitialization<L, Args = C::Args, CS = ConstraintSystem<F>, Error = Error>,
        Input: LoadFromCells<F, Chip, ExtractionSupport, L>,
        Output: LoadFromCells<F, Chip, ExtractionSupport, L>,
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

impl<F, C, Chip, Input, Output> CircuitImpl<'_, F, C, FunctionMut> {
    fn synthesize<L>(&self, config: Config<L, C>, mut layouter: L) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractCircuitMut<F, L>
            + ChipArgs
            + AbstractCircuitIO<L, Chip = Chip, Input = Input, Output = Output>,
        Chip: CircuitInitialization<L, Args = C::Args, CS = ConstraintSystem<F>, Error = Error>,
        Input: LoadFromCells<F, Chip, ExtractionSupport, L>,
        Output: LoadFromCells<F, Chip, ExtractionSupport, L>,
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

impl<F, C, Chip, Input, Output> CircuitImpl<'_, F, C, Procedure> {
    fn synthesize<L>(&self, config: Config<L, C>, mut layouter: L) -> std::result::Result<(), Error>
    where
        L: Layouter<F>,
        F: PrimeField,
        C: AbstractUnitCircuit<F, L>
            + ChipArgs
            + AbstractCircuitIO<L, Chip = Chip, Input = Input, Output = Output>,
        Chip: CircuitInitialization<L, Args = C::Args, CS = ConstraintSystem<F>, Error = Error>,
        Input: LoadFromCells<F, Chip, ExtractionSupport, L>,
        Output: LoadFromCells<F, Chip, ExtractionSupport, L>,
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

impl<F, C, M> CircuitSynthesis<F> for CircuitImpl<'_, F, C, M>
where
    F: PrimeField,
    C: AbstractCircuitIO + ChipArgs,
    Self: Circuit<F>,
    //<Self as Circuit<F>>::Config: AbstractCircuitConfig,
{
    type Circuit = Self;
    type Config<'a> = Config<ExtractionLayouter<'a, F>, C>;
    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn circuit(&self) -> &Self::Circuit {
        self
    }

    fn configure(cs: &mut Self::CS) -> Self::Config {
        Self::Config::configure(cs)
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

    fn synthesize<'a>(
        circuit: &Self::Circuit,
        config: Self::Config<'a>,
        synthesizer: &'a mut haloumi::Synthesizer<F>,
        _cs: &Self::CS,
    ) -> Result<(), Self::Error> {
        let layouter = ExtractionLayouter::new(synthesizer);
        circuit.synthesize(config, layouter)
        //assignment::SynthesizerAssignment::synthesize(circuit, config, synthesizer, cs)
    }
}
