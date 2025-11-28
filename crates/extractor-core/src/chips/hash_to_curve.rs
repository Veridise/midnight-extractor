use ff::PrimeField;
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::{
    ecc::{
        curves::{CircuitCurve, EdwardsCurve},
        hash_to_curve::{MapToCurveCPU, MapToEdwardsParams},
        native::EccChip,
    },
    hash::poseidon::constants::PoseidonField,
    instructions::{HashToCurveInstructions as _, PublicInputInstructions},
    midnight_proofs::{
        circuit::{Layouter, Value},
        plonk::Error,
    },
    types::{AssignedNativePoint, InnerValue},
};
use midnight_proofs::plonk::ConstraintSystem;

use crate::{chips::adaptor::HarnessAdaptor, chips::Htc};

pub type HtcAdaptor<C> = HarnessAdaptor<
    Htc<C>,
    //EmptyAdaptor<<Htc<C> as CircuitInitialization<<C as CircuitCurve>::Base>>::ConfigCols>,
    (),
>;

impl<C, L> CircuitInitialization<L> for HtcAdaptor<C>
where
    L: Layouter<C::Base>,
    C: EdwardsCurve + MapToEdwardsParams<C::Base> + MapToCurveCPU<C>,
    C::Base: PoseidonField,
{
    type Config = <Htc<C> as CircuitInitialization<L>>::Config;

    type Args = <Htc<C> as CircuitInitialization<L>>::Args;

    type ConfigCols = <Htc<C> as CircuitInitialization<L>>::ConfigCols;
    type CS = ConstraintSystem<C::Base>;
    type Error = Error;

    fn new_chip(config: &Self::Config, args: Self::Args) -> Self {
        <Htc<C> as CircuitInitialization<L>>::new_chip(config, args).into()
    }

    fn configure_circuit(
        meta: &mut ConstraintSystem<C::Base>,
        columns: &Self::ConfigCols,
    ) -> Self::Config {
        <Htc<C> as CircuitInitialization<L>>::configure_circuit(meta, columns)
    }

    fn load_chip(&self, layouter: &mut L, config: &Self::Config) -> Result<(), Self::Error> {
        self.adaptee.load_chip(layouter, config)
    }
}

impl<F, C> HtcAdaptor<C>
where
    C: EdwardsCurve + MapToEdwardsParams<F> + CircuitCurve<Base = F> + MapToCurveCPU<C>,
    F: PoseidonField,
{
    pub fn htc(&self) -> &Htc<C> {
        &self.adaptee
    }

    pub fn ecc(&self) -> &EccChip<C> {
        self.adaptee.ecc_chip()
    }
}

impl<F, C> PublicInputInstructions<C::Base, AssignedNativePoint<C>> for HtcAdaptor<C>
where
    C: EdwardsCurve + MapToEdwardsParams<F> + CircuitCurve<Base = F> + MapToCurveCPU<C>,
    F: PrimeField + PoseidonField,
{
    fn as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        assigned: &AssignedNativePoint<C>,
    ) -> Result<Vec<midnight_circuits::types::AssignedNative<C::Base>>, Error> {
        self.ecc().as_public_input(layouter, assigned)
    }

    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        assigned: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.ecc().constrain_as_public_input(layouter, assigned)
    }

    fn assign_as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        value: Value<<AssignedNativePoint<C> as InnerValue>::Element>,
    ) -> Result<AssignedNativePoint<C>, Error> {
        self.ecc().assign_as_public_input(layouter, value)
    }
}
