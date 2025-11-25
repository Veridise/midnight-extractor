use midnight_circuits::{
    biguint::BigUintGadget,
    ecc::{
        curves::CircuitCurve, foreign::ForeignEccChip, hash_to_curve::HashToCurveGadget,
        native::EccChip,
    },
    field::{
        decomposition::chip::P2RDecompositionChip,
        foreign::{params::MultiEmulationParams, FieldChip},
        NativeChip, NativeGadget,
    },
    hash::poseidon::PoseidonChip,
    map::map_gadget::MapGadget,
    parsing::ParserGadget,
    types::{AssignedField, AssignedForeignPoint, AssignedNative},
};
use midnight_proofs::halo2curves::group::Group;

use crate::chips::{native::NativeGadgetAdaptor, vector::VectorGadgetAdaptor};

pub mod adaptor;
pub mod ecc;
pub mod hash_to_curve;
pub mod native;
pub mod sha256;
pub mod vector;

/// Implementation of the required trait for [`AssignedField`].
pub type Mep = MultiEmulationParams;
/// Shorthand for [`NativeGadget`] with default configuration.
pub type NG<F> = NativeGadget<F, P2RDecompositionChip<F>, NativeChip<F>>;
/// Shorthad for [`BigUintGadget`] with default configuration.
pub type BG<F> = BigUintGadget<F, NG<F>>;
/// Adaptor interface to [`NativeGadget`].
pub type Nga<F> = NativeGadgetAdaptor<F, NG<F>>;
/// Adaptor interface to [`VectorGadget`].
pub type Vga<F> = VectorGadgetAdaptor<F, NG<F>>;
/// Shorthand for [`FieldChip`] with default configuration.
pub type FC<F, K> = FieldChip<F, K, Mep, Nga<F>>;
/// Shorthand for [`AssignedField`] with default configuration.
pub type AF<F, K> = AssignedField<F, K, Mep>;
/// Shorthand for [`AssignedForeignPoint`] with default configuration.
pub type Afp<F, K> = AssignedForeignPoint<F, K, Mep>;
/// Shorthand for [`ForeignEccChip`] with default configuration with [`NativeGadget`].
pub type Fecn<F, C> = ForeignEccChip<F, C, Mep, NG<F>, NG<F>>;
/// Shorthand for [`ForeignEccChip`] with default configuration with [`FieldChip`].
pub type Fecf<F, C> = ForeignEccChip<F, C, Mep, FC<F, <C as Group>::Scalar>, NG<F>>;
/// Shorthand for [`HashToCurveGadget`] with default configuration.
pub type Htc<C> = HashToCurveGadget<
    <C as CircuitCurve>::Base,
    C,
    AssignedNative<<C as CircuitCurve>::Base>,
    PoseidonChip<<C as CircuitCurve>::Base>,
    EccChip<C>,
>;
/// Shorthand for [`MapGadget`] with default configuration.
pub type MG<F> = MapGadget<F, NG<F>, PoseidonChip<F>>;
/// Shorthand for [`ParserGadget`] with default configuration.
pub type PG<F> = ParserGadget<F, NG<F>>;
