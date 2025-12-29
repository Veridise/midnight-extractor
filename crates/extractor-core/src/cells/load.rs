use std::{borrow::Borrow, ops::Deref};

use ff::{Field, PrimeField};
use haloumi_ir::stmt::IRStmt;
use mdnt_support::{
    cell_to_expr,
    cells::{
        ctx::{ICtx, LayoutAdaptor},
        CellReprSize,
    },
    circuit::injected::InjectedIR,
};
use midnight_circuits::{
    ecc::{curves::CircuitCurve, native::AssignedScalarOfNativeCurve as ScalarVar},
    field::AssignedBounded,
    instructions::{ComparisonInstructions, ConversionInstructions, EccInstructions},
    midnight_proofs::{
        circuit::{AssignedCell, Layouter},
        plonk::{Error, Expression},
    },
    types::{AssignedNative, AssignedNativePoint},
};
use midnight_curves::CurveExt as _;
use midnight_proofs::circuit::RegionIndex;
use midnight_proofs::ExtractionSupport;
use num_bigint::BigUint;

pub use crate::fields::{
    Blstrs, Jubjub, JubjubFr, JubjubSubgroup, MidnightFp, Secp256k1, Secp256k1Fp, Secp256k1Fq, G1,
};
use crate::fields::{Loaded, Zero};
pub use mdnt_support::cells::load::LoadFromCells;

pub struct LoadedJubjub(Jubjub);

impl CellReprSize for LoadedJubjub {
    const SIZE: usize = <Zero<Jubjub> as CellReprSize>::SIZE;
}

#[derive(Debug)]
pub struct PointNotInCurve(Blstrs, Blstrs);

impl std::fmt::Display for PointNotInCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point ({}, {}) is not in the curve", self.0, self.1)
    }
}

impl std::error::Error for PointNotInCurve {}

impl From<PointNotInCurve> for Error {
    fn from(value: PointNotInCurve) -> Self {
        Self::Transcript(std::io::Error::other(value))
    }
}

impl<Chip, L> LoadFromCells<Blstrs, Chip, ExtractionSupport, L> for LoadedJubjub {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        chip: &Chip,
        layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        let x = Loaded::<Blstrs>::load(ctx, chip, layouter, injected_ir)?.0;
        let y = Loaded::<Blstrs>::load(ctx, chip, layouter, injected_ir)?.0;

        Ok(Jubjub::from_xy(x, y).ok_or(PointNotInCurve(x, y)).map(LoadedJubjub)?)
    }
}

pub struct LoadedJubjubSubgroup(JubjubSubgroup);

impl From<LoadedJubjubSubgroup> for JubjubSubgroup {
    fn from(value: LoadedJubjubSubgroup) -> Self {
        value.0
    }
}

impl CellReprSize for LoadedJubjubSubgroup {
    const SIZE: usize = <Zero<JubjubSubgroup> as CellReprSize>::SIZE;
}

impl<Chip, L> LoadFromCells<Blstrs, Chip, ExtractionSupport, L> for LoadedJubjubSubgroup {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        chip: &Chip,
        layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,

        injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        LoadedJubjub::load(ctx, chip, layouter, injected_ir)
            .map(|c| c.0.into_subgroup())
            .map(LoadedJubjubSubgroup)
    }
}

pub struct LoadedG1(G1);

impl From<LoadedG1> for G1 {
    fn from(value: LoadedG1) -> Self {
        value.0
    }
}

impl CellReprSize for LoadedG1 {
    const SIZE: usize = <Zero<G1> as CellReprSize>::SIZE;
}

#[derive(Debug)]
pub struct Point3NotInCurve(MidnightFp, MidnightFp, MidnightFp);

impl std::fmt::Display for Point3NotInCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point ({}, {}, {}) is not in the curve",
            self.0, self.1, self.2
        )
    }
}

impl std::error::Error for Point3NotInCurve {}

impl From<Point3NotInCurve> for Error {
    fn from(value: Point3NotInCurve) -> Self {
        Self::Transcript(std::io::Error::other(value))
    }
}

impl<Chip, L> LoadFromCells<Blstrs, Chip, ExtractionSupport, L> for LoadedG1 {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        chip: &Chip,
        layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        let x = Loaded::<MidnightFp>::load(ctx, chip, layouter, injected_ir)?.0;
        let y = Loaded::<MidnightFp>::load(ctx, chip, layouter, injected_ir)?.0;
        let z = Loaded::<MidnightFp>::load(ctx, chip, layouter, injected_ir)?.0;

        Ok(G1::new_jacobian(x, y, z)
            .into_option()
            .ok_or(Point3NotInCurve(x, y, z))
            .map(LoadedG1)?)
    }
}

pub struct LoadedSecp256k1(Secp256k1);

impl From<LoadedSecp256k1> for Secp256k1 {
    fn from(value: LoadedSecp256k1) -> Self {
        value.0
    }
}

impl CellReprSize for LoadedSecp256k1 {
    const SIZE: usize = <Zero<Secp256k1> as CellReprSize>::SIZE;
}

#[derive(Debug)]
pub struct Point3NotInCurveSecp256k1(Secp256k1Fp, Secp256k1Fp, Secp256k1Fp);

impl std::fmt::Display for Point3NotInCurveSecp256k1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point ({:?}, {:?}, {:?}) is not in the curve",
            self.0, self.1, self.2
        )
    }
}

impl std::error::Error for Point3NotInCurveSecp256k1 {}

impl From<Point3NotInCurveSecp256k1> for Error {
    fn from(value: Point3NotInCurveSecp256k1) -> Self {
        Self::Transcript(std::io::Error::other(value))
    }
}

impl<Chip, L> LoadFromCells<Blstrs, Chip, ExtractionSupport, L> for LoadedSecp256k1 {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        chip: &Chip,
        layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        let x = Loaded::<Secp256k1Fp>::load(ctx, chip, layouter, injected_ir)?.0;
        let y = Loaded::<Secp256k1Fp>::load(ctx, chip, layouter, injected_ir)?.0;
        let z = Loaded::<Secp256k1Fp>::load(ctx, chip, layouter, injected_ir)?.0;

        Ok(Secp256k1::new_jacobian(x, y, z)
            .into_option()
            .ok_or(Point3NotInCurveSecp256k1(x, y, z))
            .map(LoadedSecp256k1)?)
    }
}

mod sealed {
    pub trait ZeroTraitSealed {}
}

pub trait ZeroTrait: Eq + sealed::ZeroTraitSealed {
    const ZERO: Self;
}

impl<T: ZeroTrait> CellReprSize for NonZero<T> {
    const SIZE: usize = 0;
}

pub struct NonZero<T: ZeroTrait>(pub T);

impl<F, C, T, L> LoadFromCells<F, C, ExtractionSupport, L> for NonZero<T>
where
    T: LoadFromCells<F, C, ExtractionSupport, L> + ZeroTrait + CellReprSize,
    F: Field,
{
    fn load(
        ctx: &mut ICtx<F, ExtractionSupport>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self, Error> {
        loop {
            let t = T::load(ctx, chip, layouter, injected_ir)?;
            if t != T::ZERO {
                return Ok(NonZero(t));
            }
        }
    }
}

/// Requires that the given constant is >1.
pub struct Gt1<S>(pub S);

impl<S: Eq> PartialEq<S> for Gt1<S> {
    fn eq(&self, other: &S) -> bool {
        self.0 == *other
    }
}

impl<S: Eq> PartialEq<S> for Gt1<Loaded<S>> {
    fn eq(&self, other: &S) -> bool {
        self.0 .0 == *other
    }
}

impl<S: CellReprSize> CellReprSize for Gt1<S> {
    const SIZE: usize = S::SIZE;
}

macro_rules! gt1_impl {
    ($f:ty) => {
        impl<C, L> LoadFromCells<Blstrs, C, ExtractionSupport, L> for Gt1<Loaded<$f>> {
            fn load(
                ctx: &mut ICtx<Blstrs, ExtractionSupport>,
                chip: &C,
                layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
                injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
            ) -> Result<Self, Error> {
                loop {
                    let s = Loaded::<$f>::load(ctx, chip, layouter, injected_ir)?;
                    if s.0 != <$f>::ZERO && s.0 != <$f>::ONE {
                        return Ok(Gt1(s));
                    }
                }
            }
        }
    };
}

gt1_impl!(JubjubFr);
gt1_impl!(Blstrs);
gt1_impl!(Secp256k1Fq);

impl sealed::ZeroTraitSealed for u8 {}
impl ZeroTrait for u8 {
    const ZERO: Self = 0;
}

impl sealed::ZeroTraitSealed for usize {}
impl ZeroTrait for usize {
    const ZERO: Self = 0;
}

impl sealed::ZeroTraitSealed for BigUint {}
impl ZeroTrait for BigUint {
    const ZERO: Self = BigUint::ZERO;
}

/// Helper for declaring [`AssignedBounded`] inputs in a harness that are bounded to the given
/// value.
pub struct AssignedBoundedLoad<F, const BOUND: usize>(AssignedBounded<F>)
where
    F: PrimeField;

impl<F: PrimeField, const BOUND: usize> CellReprSize for AssignedBoundedLoad<F, BOUND> {
    const SIZE: usize = <AssignedNative<F> as CellReprSize>::SIZE;
}

impl<F, C, const BOUND: usize, L> LoadFromCells<F, C, ExtractionSupport, L>
    for AssignedBoundedLoad<F, BOUND>
where
    F: PrimeField,
    C: ComparisonInstructions<F, AssignedNative<F>>,
    L: Layouter<F>,
{
    fn load(
        ctx: &mut ICtx<F, ExtractionSupport>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self, Error> {
        let native = AssignedNative::load(ctx, chip, layouter, injected_ir)?;
        Ok(chip
            .bounded_of_element(layouter.adaptee_ref_mut(), BOUND, &native)
            .map(AssignedBoundedLoad)?)
    }
}

impl<F: PrimeField, const B: usize> Borrow<AssignedBounded<F>> for AssignedBoundedLoad<F, B> {
    fn borrow(&self) -> &AssignedBounded<F> {
        &self.0
    }
}

impl<F: PrimeField, const B: usize> Deref for AssignedBoundedLoad<F, B> {
    type Target = AssignedBounded<F>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Helper for loading bounded scalars. The constant value represents the number of bits
pub struct BoundedScalarVar<C: CircuitCurve, const BITS: usize>(ScalarVar<C>);

impl<C: CircuitCurve, const BITS: usize> From<BoundedScalarVar<C, BITS>> for ScalarVar<C> {
    fn from(value: BoundedScalarVar<C, BITS>) -> Self {
        value.0
    }
}

impl<CV: CircuitCurve, const BITS: usize> CellReprSize for BoundedScalarVar<CV, BITS> {
    const SIZE: usize = <AssignedCell<CV::Base, CV::Base> as CellReprSize>::SIZE;
}

impl<F, S, CV, C, const BITS: usize, L> LoadFromCells<F, C, ExtractionSupport, L>
    for BoundedScalarVar<CV, BITS>
where
    F: PrimeField,
    S: PrimeField,
    CV: CircuitCurve<Base = F, Scalar = S>,
    C: EccInstructions<F, CV, Point = AssignedNativePoint<CV>, Coordinate = AssignedCell<F, F>>
        + ConversionInstructions<F, AssignedCell<F, F>, ScalarVar<CV>>,
    L: Layouter<F>,
{
    fn load(
        ctx: &mut ICtx<F, ExtractionSupport>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self, Error> {
        let cell = AssignedCell::load(ctx, chip, layouter, injected_ir)?;

        let lhs = cell_to_expr!(&cell, F)?;
        let rhs = Expression::Constant(F::from(1 << BITS));
        injected_ir.entry(cell.cell().region_index).or_default().push(IRStmt::lt(
            (cell.cell().row_offset, lhs),
            (cell.cell().row_offset, rhs),
        ));
        Ok(chip.convert(layouter.adaptee_ref_mut(), &cell).map(BoundedScalarVar)?)
    }
}

/// Helper for loading bounded native values. The constant value represents the number of bits
pub struct BoundedNative<F: PrimeField, const BITS: usize>(AssignedNative<F>);

impl<F: PrimeField, const BITS: usize> From<BoundedNative<F, BITS>> for AssignedNative<F> {
    fn from(value: BoundedNative<F, BITS>) -> Self {
        value.0
    }
}

impl<F: PrimeField, const BITS: usize> CellReprSize for BoundedNative<F, BITS> {
    const SIZE: usize = <AssignedNative<F> as CellReprSize>::SIZE;
}

impl<F, C, const BITS: usize, L> LoadFromCells<F, C, ExtractionSupport, L>
    for BoundedNative<F, BITS>
where
    F: PrimeField,
{
    fn load(
        ctx: &mut ICtx<F, ExtractionSupport>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
    ) -> Result<Self, Error> {
        let cell = AssignedNative::load(ctx, chip, layouter, injected_ir)?;

        let lhs = cell_to_expr!(&cell, F)?;
        let rhs = Expression::Constant(F::from(1 << BITS));
        injected_ir.entry(cell.cell().region_index).or_default().push(IRStmt::lt(
            (cell.cell().row_offset, lhs),
            (cell.cell().row_offset, rhs),
        ));
        Ok(BoundedNative(cell))
    }
}
