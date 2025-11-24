//! Re-exports of the field types used by the extractor.

use std::marker::PhantomData;

use ff::PrimeField as _;
pub use midnight_curves::{
    Fp as MidnightFp, Fq as Blstrs, Fr as JubjubFr, G1Projective as G1, JubjubExtended as Jubjub,
    JubjubSubgroup,
};
pub use midnight_proofs::halo2curves::secp256k1::{
    Fp as Secp256k1Fp, Fq as Secp256k1Fq, Secp256k1,
};
use midnight_proofs::{
    circuit::RegionIndex,
    plonk::{Error, Expression},
    ExtractionSupport,
};

use mdnt_support::{
    big_to_fe,
    cells::{
        ctx::{ICtx, LayoutAdaptor},
        load::LoadFromCells,
        CellReprSize,
    },
    circuit::injected::InjectedIR,
    fe_to_big,
};

macro_rules! load_impl {
    ($t:ident) => {
        impl<C> CellReprSize for $t<C> {
            const SIZE: usize = 0;
        }
        impl<C, L> LoadFromCells<Blstrs, C, ExtractionSupport, L> for $t<C> {
            fn load(
                ctx: &mut ICtx<Blstrs, ExtractionSupport>,
                _chip: &C,
                _layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
                _injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
            ) -> Result<Self, Error> {
                Ok($t(ctx.field_constant()?, Default::default()))
            }
        }
    };
}

pub struct LoadedBlstrs<C>(Blstrs, PhantomData<C>);
load_impl!(LoadedBlstrs);
pub struct LoadedMidnightFp<C>(MidnightFp, PhantomData<C>);
load_impl!(LoadedMidnightFp);
pub struct LoadedSecp256k1Fp<C>(Secp256k1Fp, PhantomData<C>);
load_impl!(LoadedSecp256k1Fp);
pub struct LoadedSecp256k1Fq<C>(Secp256k1Fq, PhantomData<C>);
load_impl!(LoadedSecp256k1Fq);

pub struct LoadedJubjubFr<C>(JubjubFr, PhantomData<C>);
impl<C> CellReprSize for LoadedJubjubFr<C> {
    const SIZE: usize = 0;
}
impl<C, L> LoadFromCells<Blstrs, C, ExtractionSupport, L> for LoadedJubjubFr<C> {
    fn load(
        ctx: &mut ICtx<Blstrs, ExtractionSupport>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<Blstrs, ExtractionSupport, Adaptee = L>,
        injected_ir: &mut InjectedIR<RegionIndex, Expression<Blstrs>>,
    ) -> Result<Self, Error> {
        assert!(
            Blstrs::NUM_BITS >= JubjubFr::NUM_BITS,
            "Loading foreign fields larger than native currently not supported"
        );
        let f = LoadedBlstrs::load(ctx, chip, layouter, injected_ir)?;
        Ok(Self(
            big_to_fe::<JubjubFr>(fe_to_big(f.0)),
            Default::default(),
        ))
    }
}
