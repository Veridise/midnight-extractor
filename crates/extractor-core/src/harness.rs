use anyhow::Context as _;
use ff::PrimeField;
use haloumi::{
    driver::Driver,
    ir::{generate::IRGenParamsBuilder, ResolvedIRCircuit},
    CircuitSynthesis, LookupCallbacks,
};
use haloumi_core::info_traits::ConstraintSystemInfo;
use mdnt_support::circuit::ChipArgs;
use midnight_proofs::plonk::Expression;

use crate::{
    circuit::{AbstractCircuitIO, CircuitImpl},
    patterns::Patterns,
};

/// Information required for executing a harness.
pub struct Ctx<'s> {
    constants: &'s [String],
    debug_comments: bool,
    disable_decomposition_pattern: bool,
}

impl<'s> Ctx<'s> {
    pub fn new(
        constants: &'s [String],
        debug_comments: bool,
        disable_decomposition_pattern: bool,
    ) -> Self {
        Self {
            constants,
            debug_comments,
            disable_decomposition_pattern,
        }
    }

    /// Lowers the circuit to Picus using the driver.
    pub fn lower_circuit<'c, F, C, M, CS>(
        &self,
        circuit: CircuitImpl<'c, F, C, M>,
        lookups: Option<&dyn LookupCallbacks<F, Expression<F>>>,
    ) -> anyhow::Result<ResolvedIRCircuit>
    where
        F: PrimeField + Ord,
        C: AbstractCircuitIO + ChipArgs,
        CircuitImpl<'c, F, C, M>: CircuitSynthesis<F, CS = CS>, //<CircuitImpl<'c, F, C, M> as Circuit<F>>::Config: AbstractCircuitConfig,
        CS: ConstraintSystemInfo<F, Polynomial = Expression<F>>,
    {
        let mut driver = Driver::default();
        let syn = driver.synthesize(&circuit).context("Synthesis failed")?;
        log::info!("Synthesis completed");
        log::info!("Synthesized circuit = {syn:#?}");

        let mut ir_params = IRGenParamsBuilder::new();

        let patterns = Patterns {
            decompose_core: !self.disable_decomposition_pattern,
        };
        ir_params.gate_callbacks(&patterns);
        if self.debug_comments {
            ir_params.with_debug_comments();
        }
        if let Some(lookups) = lookups {
            ir_params.lookup_callbacks(lookups);
        }

        let mut unresolved =
            driver.generate_ir(&syn, ir_params.build()).context("IR generation failed")?;
        let (status, errs) = unresolved.validate();
        if status.is_err() {
            for err in errs {
                log::error!("{err}");
            }
            anyhow::bail!("Failed due to validation errors on unresolved IR");
        }

        log::info!("Generated unresolved IR");
        let injected = circuit.take_injected_ir();
        unresolved.inject_ir(injected, &syn).context("IR injection failed")?;
        log::info!("Injected additional IR");
        let resolved = unresolved.resolve().context("IR resolution failed")?;

        {
            log::info!("Driver = {driver:#?}");
        }
        let (status, errs) = resolved.validate();
        if status.is_err() {
            for err in errs {
                log::error!("{err}");
            }
            anyhow::bail!("Failed due to validation errors on resolved IR");
        }
        Ok(resolved)
    }

    pub fn constants(&self) -> &[String] {
        self.constants
    }
}

/// Output produced by a harness function.
pub type Output = ResolvedIRCircuit;

/// Type representing the harness logic.
pub type Harness = fn(&Ctx) -> anyhow::Result<Output>;

/// Entry in the harness table.
#[derive(Copy, Clone, Debug)]
pub struct Entry(&'static str, Harness);

impl Entry {
    /// Creates a new entry
    pub const fn new(name: &'static str, harness: Harness) -> Self {
        Self(name, harness)
    }

    /// Returns the name of the entry.
    pub fn name(&self) -> &'static str {
        self.0
    }

    /// Returns the harness function.
    pub fn harness(&self) -> Harness {
        self.1
    }
}

inventory::collect!(Entry);

/// Registers a harness in the registry.
#[macro_export]
macro_rules! entry {
    ($name:literal, $harness:path) => {
        inventory::submit!($crate::harness::Entry::new($name, $harness));
    };
}
