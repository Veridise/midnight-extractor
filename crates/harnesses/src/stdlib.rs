use mdnt_extractor_macros::{entry, harness_with_args, unit_harness_with_args, usize_args};
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::{
    compact_std_lib::ZkStdLib,
    ecc::native::EccChip,
    instructions::PublicInputInstructions,
    types::{AssignedBit, AssignedByte, AssignedNative, AssignedNativePoint, InnerValue},
};
use midnight_proofs::{
    circuit::Layouter,
    plonk::{ConstraintSystem, Error},
};

use crate::utils::{lookup_mux, plain_spread_lookup, range_lookup};
use mdnt_extractor_core::entry as add_entry;

type C = mdnt_extractor_core::fields::Jubjub;
type F = mdnt_extractor_core::fields::Blstrs;

#[usize_args(8)]
#[entry("stdlib/assert_true/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_true(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    _: (),
    input: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_true(layouter, &input)
}

#[usize_args(8)]
#[entry("stdlib/assert_false/stdlib/bit")]
#[unit_harness_with_args(usize, range_lookup(8))]
pub fn assert_false(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    _: (),
    input: AssignedBit<F>,
) -> Result<(), Error> {
    chip.assert_false(layouter, &input)
}

#[usize_args(8)]
#[entry("stdlib/lower_than/stdlib/native")]
#[harness_with_args(usize, range_lookup(8))]
pub fn lower_than(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    (x, y, n): (AssignedNative<F>, AssignedNative<F>, usize),
) -> Result<AssignedBit<F>, Error> {
    chip.lower_than(layouter, &x, &y, n as u32)
}

add_entry!("stdlib/poseidon_1/stdlib/native", poseidon::<1>);
add_entry!("stdlib/poseidon_10/stdlib/native", poseidon::<10>);
add_entry!("stdlib/poseidon_100/stdlib/native", poseidon::<100>);
#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn poseidon<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    input: [AssignedNative<F>; N],
) -> Result<AssignedNative<F>, Error> {
    chip.poseidon(layouter, &input)
}

add_entry!("stdlib/sha256_1/stdlib/byte", sha256::<1>);
add_entry!("stdlib/sha256_10/stdlib/byte", sha256::<10>);
add_entry!("stdlib/sha256_100/stdlib/byte", sha256::<100>);
#[usize_args(8)]
#[harness_with_args(usize,lookup_mux::<F>()
            .with("pow2range column check", range_lookup(8))
            .with("plain-spreaded lookup",plain_spread_lookup("Spread", "Unspread"))
)]
pub fn sha256<const N: usize>(
    chip: &ZkStdLib,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
) -> Result<[AssignedByte<F>; 32], Error> {
    chip.sha256(layouter, &input)
}

add_entry!("stdlib/hash_to_curve_1/stdlib/byte", hash_to_curve::<1>);
add_entry!("stdlib/hash_to_curve_10/stdlib/byte", hash_to_curve::<10>);
add_entry!("stdlib/hash_to_curve_100/stdlib/byte", hash_to_curve::<100>);
#[usize_args(8)]
#[harness_with_args(usize, range_lookup(8))]
pub fn hash_to_curve<const N: usize>(
    chip: &ZkStdLibAdaptor,
    layouter: &mut impl Layouter<F>,
    input: [AssignedNative<F>; N],
) -> Result<AssignedNativePoint<C>, Error> {
    chip.0.hash_to_curve(layouter, &input)
}

/// Adaptor for the [`hash_to_curve`] to harness.
///
/// Required for storing the output since [`ZkStdLib`] doesn't implement the required interfaces.
struct ZkStdLibAdaptor(ZkStdLib, EccChip<C>);

impl<L: Layouter<F>> CircuitInitialization<L> for ZkStdLibAdaptor {
    type Config = (
        <ZkStdLib as CircuitInitialization<L>>::Config,
        <EccChip<C> as CircuitInitialization<L>>::Config,
    );

    type Args = <ZkStdLib as CircuitInitialization<L>>::Args;

    type ConfigCols = (
        <ZkStdLib as CircuitInitialization<L>>::ConfigCols,
        <EccChip<C> as CircuitInitialization<L>>::ConfigCols,
    );

    type CS = ConstraintSystem<F>;
    type Error = Error;

    fn new_chip((zkstdlib, ecc): &Self::Config, args: Self::Args) -> Self {
        Self(
            <ZkStdLib as CircuitInitialization<L>>::new_chip(zkstdlib, args),
            <EccChip<C> as CircuitInitialization<L>>::new_chip(ecc, ()),
        )
    }

    fn configure_circuit(
        meta: &mut ConstraintSystem<F>,
        (zkstdlib, ecc): &Self::ConfigCols,
    ) -> Self::Config {
        (
            <ZkStdLib as CircuitInitialization<L>>::configure_circuit(meta, zkstdlib),
            <EccChip<C> as CircuitInitialization<L>>::configure_circuit(meta, ecc),
        )
    }

    fn load_chip(&self, layouter: &mut L, (zkstdlib, ecc): &Self::Config) -> Result<(), Error> {
        self.0.load_chip(layouter, zkstdlib)?;
        self.1.load_chip(layouter, ecc)
    }
}

impl PublicInputInstructions<F, AssignedNativePoint<C>> for ZkStdLibAdaptor {
    fn as_public_input(
        &self,
        layouter: &mut impl midnight_proofs::circuit::Layouter<F>,
        assigned: &AssignedNativePoint<C>,
    ) -> Result<Vec<AssignedNative<F>>, Error> {
        self.1.as_public_input(layouter, assigned)
    }

    fn constrain_as_public_input(
        &self,
        layouter: &mut impl midnight_proofs::circuit::Layouter<F>,
        assigned: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.1.constrain_as_public_input(layouter, assigned)
    }

    fn assign_as_public_input(
        &self,
        layouter: &mut impl midnight_proofs::circuit::Layouter<F>,
        value: midnight_proofs::circuit::Value<<AssignedNativePoint<C> as InnerValue>::Element>,
    ) -> Result<AssignedNativePoint<C>, Error> {
        self.1.assign_as_public_input(layouter, value)
    }
}
