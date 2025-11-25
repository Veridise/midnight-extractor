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

use crate::{
    chips::adaptor::{EmptyAdaptor, HarnessAdaptor},
    chips::Htc,
};

pub type HtcAdaptor<C> = HarnessAdaptor<
    Htc<C>,
    //EmptyAdaptor<<Htc<C> as CircuitInitialization<<C as CircuitCurve>::Base>>::ConfigCols>,
    (),
>;

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
