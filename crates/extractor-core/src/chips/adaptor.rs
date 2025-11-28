use ff::{Field, PrimeField};
use mdnt_extractor_macros::delegated;
use mdnt_support::circuit::CircuitInitialization;
use midnight_circuits::{
    instructions::{
        ArithInstructions, AssertionInstructions, AssignmentInstructions, CanonicityInstructions,
        ControlFlowInstructions, ConversionInstructions, DecompositionInstructions,
        EqualityInstructions, FieldInstructions, PublicInputInstructions, ZeroInstructions,
    },
    types::{AssignedBit, AssignedByte, AssignedNative, InnerConstants, InnerValue, Instantiable},
};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use num_bigint::BigUint;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Neg;

/// Adaptor wrapper for using chips in harnesses that require certain instruction implementations
/// that are not implemented by the adapted chip.
///
/// The adaptor's instruction implementations only use the support type. The support type is the
/// one meant to handle the _movie magic_ behind the harnesses and the inner logic of the harness
/// needs to manually access the adaptee.
///
/// # Restrictions
///
/// - The support type cannot have circuit arguments (aka `<S as CircuitInitialization<F>>::Args == ()`).
/// - Both the adaptee and the support type need to have the same `ConfigCols` type.
pub struct HarnessAdaptor<A, S> {
    pub(super) adaptee: A,
    support: S,
}

impl<A, S> HarnessAdaptor<A, S> {
    pub fn new(adaptee: A, support: S) -> Self {
        Self { adaptee, support }
    }

    pub fn load_support_chip<F, L>(
        &self,
        layouter: &mut L,
        config: &S::Config,
    ) -> Result<(), S::Error>
    where
        S: CircuitInitialization<L>,
        L: Layouter<F>,
        F: Field,
    {
        self.support.load_chip(layouter, config)
    }
}

impl<A, S: Default> From<A> for HarnessAdaptor<A, S> {
    fn from(adaptee: A) -> Self {
        Self {
            adaptee,
            support: Default::default(),
        }
    }
}

impl<A: Debug, S: Debug> Debug for HarnessAdaptor<A, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HarnessAdaptor")
            .field("adaptee", &self.adaptee)
            .field("support", &self.support)
            .finish()
    }
}

impl<A: Clone, S: Clone> Clone for HarnessAdaptor<A, S> {
    fn clone(&self) -> Self {
        Self {
            adaptee: self.adaptee.clone(),
            support: self.support.clone(),
        }
    }
}

macro_rules! assertion {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> AssertionInstructions<F, $assigned>
            for HarnessAdaptor<A, S>
        where
            S: AssertionInstructions<F, $assigned>,
            $assigned: InnerValue<Element = Element>,
        {
            #[delegated(support)]
            fn assert_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn assert_not_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn assert_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn assert_not_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<(), Error> {
            }
        }
    };
}
macro_rules! assignment {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> AssignmentInstructions<F, $assigned>
            for HarnessAdaptor<A, S>
        where
            S: AssignmentInstructions<F, $assigned>,
            $assigned: InnerValue<Element = Element>,
        {
            #[delegated(support)]
            fn assign(
                &self,
                layouter: &mut impl Layouter<F>,
                value: Value<<$assigned as InnerValue>::Element>,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn assign_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn assign_many(
                &self,
                layouter: &mut impl Layouter<F>,
                values: &[Value<<$assigned as InnerValue>::Element>],
            ) -> Result<Vec<$assigned>, Error> {
            }

            #[delegated(support)]
            fn assign_many_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                values: &[<$assigned as InnerValue>::Element],
            ) -> Result<Vec<$assigned>, Error> {
            }
        }
    };
}
macro_rules! arith {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> ArithInstructions<F, $assigned> for HarnessAdaptor<A, S>
        where
            S: ArithInstructions<F, $assigned>,
            $assigned: InnerValue<Element = Element>,
            Element: Neg<Output = Element> + PartialEq + Add<Output = Element> + From<u64>,
            A: Debug + Clone,
        {
            #[delegated(support)]
            fn linear_combination(
                &self,
                layouter: &mut impl Layouter<F>,
                terms: &[(<$assigned as InnerValue>::Element, $assigned)],
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn mul(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
                multiplying_constant: Option<<$assigned as InnerValue>::Element>,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn div(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn inv(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn inv0(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn add(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn sub(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn neg(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn add_constant(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn add_constants(
                &self,
                layouter: &mut impl Layouter<F>,
                xs: &[$assigned],
                constants: &[<$assigned as InnerValue>::Element],
            ) -> Result<Vec<$assigned>, Error> {
            }

            #[delegated(support)]
            fn mul_by_constant(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn square(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn pow(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                n: u64,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn add_and_mul(
                &self,
                layouter: &mut impl Layouter<F>,
                (a, x): (<$assigned as InnerValue>::Element, &$assigned),
                (b, y): (<$assigned as InnerValue>::Element, &$assigned),
                (c, z): (<$assigned as InnerValue>::Element, &$assigned),
                k: <$assigned as InnerValue>::Element,
                m: <$assigned as InnerValue>::Element,
            ) -> Result<$assigned, Error> {
            }
        }
    };
}
macro_rules! canonicity {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> CanonicityInstructions<F, $assigned>
            for HarnessAdaptor<A, S>
        where
            S: CanonicityInstructions<F, $assigned>,
            $assigned: Instantiable<F, Element = Element> + InnerConstants,
            Element: PrimeField,
            A: Clone + Debug,
        {
            #[delegated(support)]
            fn is_canonical(
                &self,
                layouter: &mut impl Layouter<F>,
                bits: &[AssignedBit<F>],
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(support)]
            fn le_bits_lower_than(
                &self,
                layouter: &mut impl Layouter<F>,
                bits: &[AssignedBit<F>],
                bound: BigUint,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(support)]
            fn le_bits_geq_than(
                &self,
                layouter: &mut impl Layouter<F>,
                bits: &[AssignedBit<F>],
                bound: BigUint,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}
macro_rules! control_flow {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> ControlFlowInstructions<F, $assigned>
            for HarnessAdaptor<A, S>
        where
            S: ControlFlowInstructions<F, $assigned>,
            $assigned: InnerValue<Element = Element>,
        {
            #[delegated(support)]
            fn select(
                &self,
                layouter: &mut impl Layouter<F>,
                cond: &AssignedBit<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn cond_assert_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                cond: &AssignedBit<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn cond_swap(
                &self,
                layouter: &mut impl Layouter<F>,
                cond: &AssignedBit<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<($assigned, $assigned), Error> {
            }
        }
    };
}
macro_rules! conversion {
    ($assigned:ty,$target:ty) => {
        impl<F: PrimeField, A, S> ConversionInstructions<F, $assigned, $target>
            for HarnessAdaptor<A, S>
        where
            S: ConversionInstructions<F, $assigned, $target>,
            $target: InnerValue,
            $assigned: InnerValue,
        {
            #[delegated(support)]
            fn convert_value(
                &self,
                x: &<$assigned as InnerValue>::Element,
            ) -> Option<<$target as InnerValue>::Element> {
            }

            #[delegated(support)]
            fn convert(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<$target, Error> {
            }
        }
    };
}
macro_rules! decomposition {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> DecompositionInstructions<F, $assigned>
            for HarnessAdaptor<A, S>
        where
            S: DecompositionInstructions<F, $assigned>,
            $assigned: Instantiable<F> + InnerValue<Element = Element> + InnerConstants,
            Element: PrimeField,
            A: Debug + Clone,
        {
            #[delegated(support)]
            fn assigned_to_le_bits(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                nb_bits: Option<usize>,
                enforce_canonical: bool,
            ) -> Result<Vec<AssignedBit<F>>, Error> {
            }

            #[delegated(support)]
            fn assigned_to_le_bytes(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                nb_bytes: Option<usize>,
            ) -> Result<Vec<AssignedByte<F>>, Error> {
            }

            #[delegated(support)]
            fn assigned_to_le_chunks(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                nb_bits_per_chunk: usize,
                nb_chunks: Option<usize>,
            ) -> Result<Vec<AssignedNative<F>>, Error> {
            }

            #[delegated(support)]
            fn assigned_to_be_bits(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                nb_bits: Option<usize>,
                enforce_canonical: bool,
            ) -> Result<Vec<AssignedBit<F>>, Error> {
            }

            #[delegated(support)]
            fn assigned_to_be_bytes(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                nb_bytes: Option<usize>,
            ) -> Result<Vec<AssignedByte<F>>, Error> {
            }

            #[delegated(support)]
            fn assigned_from_le_bits(
                &self,
                layouter: &mut impl Layouter<F>,
                bits: &[AssignedBit<F>],
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn assigned_from_be_bits(
                &self,
                layouter: &mut impl Layouter<F>,
                bits: &[AssignedBit<F>],
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn assigned_from_le_bytes(
                &self,
                layouter: &mut impl Layouter<F>,
                bytes: &[AssignedByte<F>],
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn assigned_from_be_bytes(
                &self,
                layouter: &mut impl Layouter<F>,
                bytes: &[AssignedByte<F>],
            ) -> Result<$assigned, Error> {
            }

            #[delegated(support)]
            fn sgn0(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}
macro_rules! equality {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S> EqualityInstructions<F, $assigned> for HarnessAdaptor<A, S>
        where
            S: EqualityInstructions<F, $assigned>,
            $assigned: InnerValue,
        {
            #[delegated(support)]
            fn is_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(support)]
            fn is_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                constant: <$assigned as InnerValue>::Element,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(support)]
            fn is_not_equal(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }

            #[delegated(support)]
            fn is_not_equal_to_fixed(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
                y: <$assigned as InnerValue>::Element,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}
macro_rules! field {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S, Element> FieldInstructions<F, $assigned> for HarnessAdaptor<A, S>
        where
            S: FieldInstructions<F, $assigned>,
            $assigned: InnerValue<Element = Element> + InnerConstants + Instantiable<F>,
            Element: PrimeField,
            A: Debug + Clone,
        {
            #[delegated(support)]
            fn order(&self) -> BigUint {}

            #[delegated(support)]
            fn assert_qr(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn is_square(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}
macro_rules! public_input {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S> PublicInputInstructions<F, $assigned> for HarnessAdaptor<A, S>
        where
            S: PublicInputInstructions<F, $assigned>,
            $assigned: Instantiable<F>,
        {
            #[delegated(support)]
            fn as_public_input(
                &self,
                layouter: &mut impl Layouter<F>,
                assigned: &$assigned,
            ) -> Result<Vec<AssignedNative<F>>, Error> {
            }

            #[delegated(support)]
            fn constrain_as_public_input(
                &self,
                layouter: &mut impl Layouter<F>,
                assigned: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn assign_as_public_input(
                &self,
                layouter: &mut impl Layouter<F>,
                value: Value<<$assigned as InnerValue>::Element>,
            ) -> Result<$assigned, Error> {
            }
        }
    };
}
macro_rules! zero {
    ($assigned:ty) => {
        impl<F: PrimeField, A, S> ZeroInstructions<F, $assigned> for HarnessAdaptor<A, S>
        where
            S: ZeroInstructions<F, $assigned>,
            $assigned: InnerConstants,
        {
            #[delegated(support)]
            fn assert_zero(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn assert_non_zero(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<(), Error> {
            }

            #[delegated(support)]
            fn is_zero(
                &self,
                layouter: &mut impl Layouter<F>,
                x: &$assigned,
            ) -> Result<AssignedBit<F>, Error> {
            }
        }
    };
}

assertion!(AssignedNative<F>);
assignment!(AssignedNative<F>);
assignment!(AssignedBit<F>);
arith!(AssignedNative<F>);
canonicity!(AssignedNative<F>);
control_flow!(AssignedNative<F>);
conversion!(AssignedNative<F>, AssignedBit<F>);
conversion!(AssignedBit<F>, AssignedNative<F>);
conversion!(AssignedByte<F>, AssignedNative<F>);
decomposition!(AssignedNative<F>);
equality!(AssignedNative<F>);
field!(AssignedNative<F>);
public_input!(AssignedNative<F>);
zero!(AssignedNative<F>);
