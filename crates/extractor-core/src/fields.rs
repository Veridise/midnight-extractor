//! Re-exports of the field types used by the extractor.

use std::marker::PhantomData;

use ff::PrimeField;
pub use midnight_curves::{
    secp256k1::{Fp as Secp256k1Fp, Fq as Secp256k1Fq, Secp256k1},
    Fp as MidnightFp, Fq as Blstrs, Fr as JubjubFr, G1Projective as G1, JubjubExtended as Jubjub,
    JubjubSubgroup,
};
use midnight_proofs::{
    circuit::RegionIndex,
    plonk::{Error, Expression},
    ExtractionSupport,
};

use mdnt_support::{
    cells::{
        ctx::{ICtx, LayoutAdaptor},
        load::LoadFromCells,
        CellReprSize,
    },
    circuit::injected::InjectedIR,
};

pub struct Zero<T>(PhantomData<T>);

impl<T> CellReprSize for Zero<T> {
    const SIZE: usize = 0;
}

#[repr(transparent)]
pub struct Loaded<F>(pub F);

impl<F: PrimeField> CellReprSize for Loaded<F> {
    const SIZE: usize = 0;
}

impl<C, L, F: PrimeField> LoadFromCells<Blstrs, C, ExtractionSupport, L> for Loaded<F> {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        _chip: &C,
        _layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
        _injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        Ok(Self(ctx.field_constant()?))
    }
}
