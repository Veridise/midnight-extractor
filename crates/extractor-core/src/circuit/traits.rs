//! Traits for defining harness circuits.

use anyhow::Result;
use ff::PrimeField;
use mdnt_support::cells::ctx::{InputDescr, OutputDescr};
use mdnt_support::cells::CellReprSize;
use midnight_proofs::circuit::RegionIndex;
use midnight_proofs::plonk::Expression;
use midnight_proofs::ExtractionSupport;
use midnight_proofs::{
    circuit::Layouter,
    plonk::{Column, Error, Instance},
};

use mdnt_support::{
    cells::{load::LoadFromCells, store::StoreIntoCells},
    circuit::injected::InjectedIR,
};

use mdnt_support::circuit::{ChipArgs, CircuitInitialization, NoChipArgs};

/// Super trait for extracting IO from an abstract circuit.
pub trait AbstractCircuitIO {
    /// Type that implements the main logic.
    type Chip;
    /// Input type of the chip.
    type Input: CellReprSize;
    /// Output type of the chip.
    type Output: CellReprSize;
    /// Configuration of the circuit.
    type Config: Clone + std::fmt::Debug;
    /// Configuration columns of the circuit.
    type ConfigCols: Clone + std::fmt::Debug;
}

/// Main trait for defining harness that return a value.
///
/// The actual logic of the circuit is defined in an implementation of this trait with the circuit
/// implementation struct acting as scaffolding and glue.
///
/// For harnesses that return `()` see [`AbstractUnitCircuit`].
pub trait AbstractCircuit<F: PrimeField>: AbstractCircuitIO {
    fn synthesize<L>(
        &self,
        chip: &Self::Chip,
        layouter: &mut L,
        input: Self::Input,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self::Output, Error>
    where
        L: Layouter<F> //    <Self as AbstractCircuitIO>::Input: LoadFromCells<F, Self::Chip, ExtractionSupport, L>,
                       //    <Self as AbstractCircuitIO>::Output: StoreIntoCells<F, Self::Chip, ExtractionSupport, L>
    ;
}

/// Main trait for defining harness that return a value.
///
/// This trait is analogous to [`AbstractCircuit`] but accepts a mutable reference to the chip
/// instead.
///
/// The actual logic of the circuit is defined in an implementation of this trait with the circuit
/// implementation struct acting as scaffolding and glue.
///
/// For harnesses that return `()` see [`AbstractUnitCircuit`].
pub trait AbstractCircuitMut<F: PrimeField>: AbstractCircuitIO {
    fn synthesize_mut<L>(
        &self,
        chip: &mut Self::Chip,
        layouter: &mut L,
        input: Self::Input,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self::Output, Error>
    where
        L: Layouter<F> //    <Self as AbstractCircuitIO>::Input: LoadFromCells<F, Self::Chip, ExtractionSupport, L>,
                       //    <Self as AbstractCircuitIO>::Output: StoreIntoCells<F, Self::Chip, ExtractionSupport, L>
    ;
}

/// Main trait for defining harness that do not return a value.
///
/// When defining the harness both inputs and outputs are passed as arguments. The distinction
/// between inputs and outputs is meant for the lowering backend. When lowering to Picus the
/// outputs must be the arguments of the function under test that are, conceptually, a function of
/// the inputs.
///
/// The actual logic of the circuit is defined in an implementation of this trait with the circuit
/// implementation struct acting as scaffolding and glue.
///
/// For harnesses that return a value see [`AbstractCircuit`].
pub trait AbstractUnitCircuit<F: PrimeField>: AbstractCircuitIO {
    fn synthesize<L>(
        &self,
        chip: &Self::Chip,
        layouter: &mut L,
        input: Self::Input,
        output: Self::Output,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<(), Error>
    where
        L: Layouter<F> //    <Self as AbstractCircuitIO>::Input: LoadFromCells<F, Self::Chip, ExtractionSupport, L>,
                       //    <Self as AbstractCircuitIO>::Output: LoadFromCells<F, Self::Chip, ExtractionSupport, L>
    ;
}

/// Trait for obtaining information about the configuration of a circuit.
pub trait AbstractCircuitConfig {
    /// Returns the list of [`InputDescr`] that make up the inputs.
    fn inputs<F: PrimeField>(&self) -> Vec<InputDescr<F, ExtractionSupport>>;

    /// Returns the column that represents the inputs.
    fn input_instance(&self) -> Column<Instance>;

    /// Returns the list of [`OutputDescr`] that make up the outputs.
    fn outputs<F: PrimeField>(&self) -> Vec<OutputDescr<F, ExtractionSupport>>;

    /// Returns the column that represents the outputs.
    fn output_instance(&self) -> Column<Instance>;
}
