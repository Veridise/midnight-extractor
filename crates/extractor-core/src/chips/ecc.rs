use midnight_circuits::{
    ecc::{
        curves::{CircuitCurve, EdwardsCurve},
        native::{AssignedScalarOfNativeCurve as ScalarVar, EccChip},
    },
    instructions::{
        AssertionInstructions, AssignmentInstructions, ConversionInstructions,
        DecompositionInstructions, EccInstructions, EqualityInstructions, PublicInputInstructions,
        ZeroInstructions,
    },
    midnight_proofs::{
        circuit::{Layouter, Value},
        plonk::Error,
    },
    types::{AssignedBit, AssignedNative, AssignedNativePoint, InnerValue},
};

use crate::chips::{adaptor::HarnessAdaptor, NG};

pub type EccChipAdaptor<C> = HarnessAdaptor<EccChip<C>, NG<<C as CircuitCurve>::Base>>;

impl<C: EdwardsCurve> EccChipAdaptor<C> {
    pub fn ecc(&self) -> &EccChip<C> {
        &self.adaptee
    }
}

impl<C: EdwardsCurve> EccInstructions<C::Base, C> for EccChipAdaptor<C> {
    fn msm_by_bounded_scalars(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        scalars: &[(Self::Scalar, usize)],
        bases: &[Self::Point],
    ) -> Result<Self::Point, Error> {
        self.ecc().msm_by_bounded_scalars(layouter, scalars, bases)
    }

    type Point = <EccChip<C> as EccInstructions<C::Base, C>>::Point;

    type Coordinate = <EccChip<C> as EccInstructions<C::Base, C>>::Coordinate;

    type Scalar = <EccChip<C> as EccInstructions<C::Base, C>>::Scalar;

    fn add(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        p: &Self::Point,
        q: &Self::Point,
    ) -> Result<Self::Point, Error> {
        self.ecc().add(layouter, p, q)
    }

    fn double(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        p: &Self::Point,
    ) -> Result<Self::Point, Error> {
        self.ecc().double(layouter, p)
    }

    fn negate(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        p: &Self::Point,
    ) -> Result<Self::Point, Error> {
        self.ecc().negate(layouter, p)
    }

    fn msm(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        scalars: &[Self::Scalar],
        bases: &[Self::Point],
    ) -> Result<Self::Point, Error> {
        self.ecc().msm(layouter, scalars, bases)
    }

    fn mul_by_constant(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        scalar: <C>::Scalar,
        base: &Self::Point,
    ) -> Result<Self::Point, Error> {
        self.ecc().mul_by_constant(layouter, scalar, base)
    }

    fn point_from_coordinates(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &Self::Coordinate,
        y: &Self::Coordinate,
    ) -> Result<Self::Point, Error> {
        self.ecc().point_from_coordinates(layouter, x, y)
    }

    fn x_coordinate(&self, point: &Self::Point) -> Self::Coordinate {
        self.ecc().x_coordinate(point)
    }

    fn y_coordinate(&self, point: &Self::Point) -> Self::Coordinate {
        self.ecc().y_coordinate(point)
    }

    fn base_field(&self) -> &impl DecompositionInstructions<C::Base, Self::Coordinate> {
        self.ecc().base_field()
    }
}

impl<C: EdwardsCurve> AssertionInstructions<C::Base, AssignedNativePoint<C>> for EccChipAdaptor<C> {
    fn assert_equal(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        y: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.ecc().assert_equal(layouter, x, y)
    }

    fn assert_not_equal(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        y: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.ecc().assert_not_equal(layouter, x, y)
    }

    fn assert_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        constant: <AssignedNativePoint<C> as InnerValue>::Element,
    ) -> Result<(), Error> {
        self.ecc().assert_equal_to_fixed(layouter, x, constant)
    }

    fn assert_not_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        constant: <AssignedNativePoint<C> as InnerValue>::Element,
    ) -> Result<(), Error> {
        self.ecc().assert_not_equal_to_fixed(layouter, x, constant)
    }
}

impl<C: EdwardsCurve> AssignmentInstructions<C::Base, AssignedNativePoint<C>>
    for EccChipAdaptor<C>
{
    fn assign_many(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        values: &[Value<<AssignedNativePoint<C> as InnerValue>::Element>],
    ) -> Result<Vec<AssignedNativePoint<C>>, Error> {
        self.ecc().assign_many(layouter, values)
    }

    fn assign_many_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        values: &[<AssignedNativePoint<C> as InnerValue>::Element],
    ) -> Result<Vec<AssignedNativePoint<C>>, Error> {
        self.ecc().assign_many_fixed(layouter, values)
    }

    fn assign(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        value: Value<<AssignedNativePoint<C> as InnerValue>::Element>,
    ) -> Result<AssignedNativePoint<C>, Error> {
        self.ecc().assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        constant: <AssignedNativePoint<C> as InnerValue>::Element,
    ) -> Result<AssignedNativePoint<C>, Error> {
        self.ecc().assign_fixed(layouter, constant)
    }
}

impl<C: EdwardsCurve> PublicInputInstructions<C::Base, AssignedNativePoint<C>>
    for EccChipAdaptor<C>
{
    fn as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        assigned: &AssignedNativePoint<C>,
    ) -> Result<Vec<AssignedNative<C::Base>>, Error> {
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

impl<C: EdwardsCurve> EqualityInstructions<C::Base, AssignedNativePoint<C>> for EccChipAdaptor<C> {
    fn is_equal(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        y: &AssignedNativePoint<C>,
    ) -> Result<AssignedBit<C::Base>, Error> {
        self.ecc().is_equal(layouter, x, y)
    }

    fn is_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        constant: <AssignedNativePoint<C> as InnerValue>::Element,
    ) -> Result<AssignedBit<C::Base>, Error> {
        self.ecc().is_equal_to_fixed(layouter, x, constant)
    }

    fn is_not_equal(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        y: &AssignedNativePoint<C>,
    ) -> Result<AssignedBit<C::Base>, Error> {
        self.ecc().is_not_equal(layouter, x, y)
    }

    fn is_not_equal_to_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
        constant: <AssignedNativePoint<C> as InnerValue>::Element,
    ) -> Result<AssignedBit<C::Base>, Error> {
        self.ecc().is_not_equal_to_fixed(layouter, x, constant)
    }
}

impl<C: EdwardsCurve> AssignmentInstructions<C::Base, ScalarVar<C>> for EccChipAdaptor<C> {
    fn assign(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        value: Value<<ScalarVar<C> as InnerValue>::Element>,
    ) -> Result<ScalarVar<C>, Error> {
        self.ecc().assign(layouter, value)
    }

    fn assign_fixed(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        constant: <ScalarVar<C> as InnerValue>::Element,
    ) -> Result<ScalarVar<C>, Error> {
        self.ecc().assign_fixed(layouter, constant)
    }
}

impl<C: EdwardsCurve> ConversionInstructions<C::Base, AssignedNative<C::Base>, ScalarVar<C>>
    for EccChipAdaptor<C>
{
    fn convert_value(
        &self,
        x: &<AssignedNative<C::Base> as InnerValue>::Element,
    ) -> Option<<ScalarVar<C> as InnerValue>::Element> {
        self.ecc().convert_value(x)
    }

    fn convert(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNative<C::Base>,
    ) -> Result<ScalarVar<C>, Error> {
        self.ecc().convert(layouter, x)
    }
}

impl<C: EdwardsCurve> PublicInputInstructions<C::Base, ScalarVar<C>> for EccChipAdaptor<C> {
    fn as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        assigned: &ScalarVar<C>,
    ) -> Result<Vec<AssignedNative<C::Base>>, Error> {
        self.ecc().as_public_input(layouter, assigned)
    }

    fn constrain_as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        assigned: &ScalarVar<C>,
    ) -> Result<(), Error> {
        self.ecc().constrain_as_public_input(layouter, assigned)
    }

    fn assign_as_public_input(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        value: Value<<ScalarVar<C> as InnerValue>::Element>,
    ) -> Result<ScalarVar<C>, Error> {
        self.ecc().assign_as_public_input(layouter, value)
    }
}

impl<C: EdwardsCurve> ZeroInstructions<C::Base, AssignedNativePoint<C>> for EccChipAdaptor<C> {
    fn assert_zero(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.ecc().assert_zero(layouter, x)
    }

    fn assert_non_zero(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
    ) -> Result<(), Error> {
        self.ecc().assert_non_zero(layouter, x)
    }

    fn is_zero(
        &self,
        layouter: &mut impl Layouter<C::Base>,
        x: &AssignedNativePoint<C>,
    ) -> Result<AssignedBit<C::Base>, Error> {
        self.ecc().is_zero(layouter, x)
    }
}
