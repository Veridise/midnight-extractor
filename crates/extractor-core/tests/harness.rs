//! These tests ensure that the architecture enables creating circuits
//! following a similar structure to the one proc-macros use.

use std::marker::PhantomData;

use haloumi::CircuitSynthesis;
use mdnt_extractor_macros::{harness, harness_mut, unit_harness};
use mdnt_support::circuit::{CircuitInitialization, NoChipArgs};
use midnight_circuits::types::AssignedNative;
use midnight_proofs::plonk::{Advice, Column, ConstraintSystem, Error};

use mdnt_extractor_core::circuit::layouter::ExtractionLayouter;
use mdnt_extractor_core::circuit::{
    AbstractCircuit, AbstractCircuitIO, AbstractCircuitMut, AbstractUnitCircuit, CircuitImpl,
    Function, FunctionMut, Procedure,
};
use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_core::harness::Ctx;

struct FakeChip;

impl<L> CircuitInitialization<L> for FakeChip {
    type Config = [Column<Advice>; 5];

    type Args = ();

    type ConfigCols = Column<Advice>;

    type CS = ConstraintSystem<F>;

    type Error = Error;

    fn new_chip(_config: &Self::Config, _args: Self::Args) -> Self {
        todo!()
    }

    fn configure_circuit(_meta: &mut Self::CS, _columns: &Self::ConfigCols) -> Self::Config {
        todo!()
    }

    fn load_chip(&self, _layouter: &mut L, _config: &Self::Config) -> Result<(), Self::Error> {
        todo!()
    }
}

fn fake_synthesize(_c: impl CircuitSynthesis<F>) {}

#[test]
fn test_fakechip_function() {
    struct Circuit<Config, ConfigCols>(PhantomData<(Config, ConfigCols)>);

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug> AbstractCircuitIO
        for Circuit<Config, ConfigCols>
    {
        type Chip = FakeChip;

        type Input = AssignedNative<F>;

        type Output = AssignedNative<F>;

        type Config = Config;

        type ConfigCols = ConfigCols;
    }

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug> AbstractCircuit<F>
        for Circuit<Config, ConfigCols>
    {
        fn synthesize<L>(
            &self,
            _chip: &Self::Chip,
            _layouter: &mut L,
            input: Self::Input,
            _injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                midnight_proofs::circuit::RegionIndex,
                midnight_proofs::plonk::Expression<F>,
            >,
        ) -> anyhow::Result<Self::Output, Error>
        where
            L: midnight_proofs::circuit::Layouter<F>,
        {
            Ok(input)
        }
    }

    impl<A, B> NoChipArgs for Circuit<A, B> {}

    let ctx = Ctx::new(&[], false, false);
    let ci: CircuitImpl<
        '_,
        F,
        Circuit<
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::Config,
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::ConfigCols,
        >,
        Function,
    > = CircuitImpl::new(&ctx, Circuit(Default::default()));
    fake_synthesize(ci)
}

#[test]
fn test_fakechip_function_2() {
    struct Circuit<'s, 'c>(PhantomData<(&'s (), &'c ())>);

    impl<'s, 'c> AbstractCircuitIO for Circuit<'s, 'c> {
        type Chip = FakeChip;

        type Input = AssignedNative<F>;

        type Output = AssignedNative<F>;

        type Config = <FakeChip as CircuitInitialization<ExtractionLayouter<'s, 'c, F>>>::Config;

        type ConfigCols =
            <FakeChip as CircuitInitialization<ExtractionLayouter<'s, 'c, F>>>::ConfigCols;
    }

    impl AbstractCircuit<F> for Circuit<'_, '_> {
        fn synthesize<L>(
            &self,
            _chip: &Self::Chip,
            _layouter: &mut L,
            input: Self::Input,
            _injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                midnight_proofs::circuit::RegionIndex,
                midnight_proofs::plonk::Expression<F>,
            >,
        ) -> anyhow::Result<Self::Output, Error>
        where
            L: midnight_proofs::circuit::Layouter<F>,
        {
            Ok(input)
        }
    }

    impl NoChipArgs for Circuit<'_, '_> {}

    let ctx = Ctx::new(&[], false, false);
    let ci: CircuitImpl<'_, F, Circuit, Function> =
        CircuitImpl::new(&ctx, Circuit(Default::default()));
    fake_synthesize(ci)
}

#[harness]
fn test_fakechip_function_macro_impl(
    fake: &FakeChip,
    layouter: &mut impl Layouter<F>,
    i: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    Ok(i)
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn test_fakechip_function_macro() {
    let _ = test_fakechip_function_macro_impl(&Ctx::new(&[], false, false)).unwrap();
}

#[test]
fn test_fakechip_function_mut() {
    struct Circuit<Config, ConfigCols>(PhantomData<(Config, ConfigCols)>);

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug> AbstractCircuitIO
        for Circuit<Config, ConfigCols>
    {
        type Chip = FakeChip;

        type Input = AssignedNative<F>;

        type Output = AssignedNative<F>;

        type Config = Config;

        type ConfigCols = ConfigCols;
    }

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug> AbstractCircuitMut<F>
        for Circuit<Config, ConfigCols>
    {
        fn synthesize_mut<L>(
            &self,
            _chip: &mut Self::Chip,
            _layouter: &mut L,
            input: Self::Input,
            _injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                midnight_proofs::circuit::RegionIndex,
                midnight_proofs::plonk::Expression<F>,
            >,
        ) -> anyhow::Result<Self::Output, Error>
        where
            L: midnight_proofs::circuit::Layouter<F>,
        {
            Ok(input)
        }
    }

    impl<A, B> NoChipArgs for Circuit<A, B> {}

    let ctx = Ctx::new(&[], false, false);
    let ci: CircuitImpl<
        '_,
        F,
        Circuit<
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::Config,
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::ConfigCols,
        >,
        FunctionMut,
    > = CircuitImpl::new(&ctx, Circuit(Default::default()));
    fake_synthesize(ci)
}

#[harness_mut]
fn test_fakechip_function_mut_macro_impl(
    fake: &mut FakeChip,
    layouter: &mut impl Layouter<F>,
    i: AssignedNative<F>,
) -> Result<AssignedNative<F>, Error> {
    Ok(i)
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn test_fakechip_function_mut_macro() {
    let _ = test_fakechip_function_mut_macro_impl(&Ctx::new(&[], false, false)).unwrap();
}

#[test]
fn test_fakechip_procedure() {
    struct Circuit<Config, ConfigCols>(PhantomData<(Config, ConfigCols)>);

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug> AbstractCircuitIO
        for Circuit<Config, ConfigCols>
    {
        type Chip = FakeChip;

        type Input = AssignedNative<F>;

        type Output = AssignedNative<F>;

        type Config = Config;

        type ConfigCols = ConfigCols;
    }

    impl<Config: Clone + std::fmt::Debug, ConfigCols: Clone + std::fmt::Debug>
        AbstractUnitCircuit<F> for Circuit<Config, ConfigCols>
    {
        fn synthesize<L>(
            &self,
            _chip: &Self::Chip,
            _layouter: &mut L,
            _input: Self::Input,
            _output: Self::Output,
            _injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                midnight_proofs::circuit::RegionIndex,
                midnight_proofs::plonk::Expression<F>,
            >,
        ) -> anyhow::Result<(), Error>
        where
            L: midnight_proofs::circuit::Layouter<F>,
        {
            Ok(())
        }
    }

    impl<A, B> NoChipArgs for Circuit<A, B> {}

    let ctx = Ctx::new(&[], false, false);
    let ci: CircuitImpl<
        '_,
        F,
        Circuit<
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::Config,
            <FakeChip as CircuitInitialization<ExtractionLayouter<F>>>::ConfigCols,
        >,
        Procedure,
    > = CircuitImpl::new(&ctx, Circuit(Default::default()));
    fake_synthesize(ci)
}

#[unit_harness]
fn test_fakechip_procedure_macro_impl(
    fake: &mut FakeChip,
    layouter: &mut impl Layouter<F>,
    i: AssignedNative<F>,
    o: AssignedNative<F>,
) -> Result<(), Error> {
    Ok(())
}

#[test]
#[should_panic(expected = "not yet implemented")]
fn test_fakechip_procedure_macro() {
    let _ = test_fakechip_procedure_macro_impl(&Ctx::new(&[], false, false)).unwrap();
}
