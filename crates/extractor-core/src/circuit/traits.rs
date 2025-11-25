//! Traits for defining harness circuits.

use anyhow::Result;
use extractor_support::cells::ctx::{InputDescr, OutputDescr};
use group::ff::PrimeField;
use midnight::midnight_proofs::{
    circuit::Layouter,
    plonk::{Column, Error, Instance},
};

use crate::{
    cells::{load::LoadFromCells, store::StoreIntoCells},
    circuit::injected::InjectedIR,
};

pub use extractor_support::circuit::{
    AbstractCircuitIO, ChipArgs, CircuitInitialization, NoChipArgs,
};

/// Main trait for defining harness that return a value.
///
/// The actual logic of the circuit is defined in an implementation of this trait with the circuit
/// implementation struct acting as scaffolding and glue.
///
/// For harnesses that return `()` see [`AbstractUnitCircuit`].
pub trait AbstractCircuit<F: PrimeField>: AbstractCircuitIO<F>
where
    <Self as AbstractCircuitIO<F>>::Input: LoadFromCells<F, Self::Chip>,
    <Self as AbstractCircuitIO<F>>::Output: StoreIntoCells<F, Self::Chip>,
{
    fn synthesize(
        &self,
        chip: &Self::Chip,
        layouter: &mut impl Layouter<F>,
        input: Self::Input,
        injected_ir: &mut InjectedIR<F>,
    ) -> Result<Self::Output, Error>;
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
pub trait AbstractCircuitMut<F: PrimeField>: AbstractCircuitIO<F>
where
    <Self as AbstractCircuitIO<F>>::Input: LoadFromCells<F, Self::Chip>,
    <Self as AbstractCircuitIO<F>>::Output: StoreIntoCells<F, Self::Chip>,
{
    fn synthesize_mut(
        &self,
        chip: &mut Self::Chip,
        layouter: &mut impl Layouter<F>,
        input: Self::Input,
        injected_ir: &mut InjectedIR<F>,
    ) -> Result<Self::Output, Error>;
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
pub trait AbstractUnitCircuit<F: PrimeField>: AbstractCircuitIO<F>
where
    <Self as AbstractCircuitIO<F>>::Input: LoadFromCells<F, Self::Chip>,
    <Self as AbstractCircuitIO<F>>::Output: LoadFromCells<F, Self::Chip>,
{
    fn synthesize(
        &self,
        chip: &Self::Chip,
        layouter: &mut impl Layouter<F>,
        input: Self::Input,
        output: Self::Output,
        injected_ir: &mut InjectedIR<F>,
    ) -> Result<(), Error>;
}

/// Trait for obtaining information about the configuration of a circuit.
///
/// Used by [`halo2_llzk_frontend::CircuitCallbacks`].
pub trait AbstractCircuitConfig {
    /// Returns the list of [`InputDescr`] that make up the inputs.
    fn inputs(&self) -> Vec<InputDescr>;

    /// Returns the column that represents the inputs.
    fn input_instance(&self) -> Column<Instance>;

    /// Returns the list of [`OutputDescr`] that make up the outputs.
    fn outputs(&self) -> Vec<OutputDescr>;

    /// Returns the column that represents the outputs.
    fn output_instance(&self) -> Column<Instance>;
}
