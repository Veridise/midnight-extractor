//! Error type for the support crate.

use std::{num::ParseIntError, str::ParseBoolError, sync::Arc};

use num_bigint::{BigInt, ParseBigIntError, TryFromBigIntError};
use thiserror::Error;

//use crate::fields::{Blstrs, MidnightFp, Secp256k1Fp};

/// Error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Parsing error while loading a field element from a string.
    #[error("Failure while parsing field element")]
    FieldParsingError,
    /// The circuit requested more constants than provided.
    #[error("Not enough constants")]
    NotEnoughConstants,
    /// The circuit did not declare enough cells for input or output.
    #[error("IO cell iterator was exhausted")]
    NotEnoughIOCells,
    /// Integer parse error.
    #[error("Parse failure")]
    IntParse(#[from] ParseIntError),
    /// Boolean parse error.
    #[error("Parse failure")]
    BoolParse(#[from] ParseBoolError),
    /// BigUint parse error.
    #[error("Parse failure")]
    BigUintParse(#[from] ParseBigIntError),
    /// Plonk synthesis error.
    #[error("Synthesis error")]
    Plonk(Arc<dyn std::error::Error>),
    /// An error represented with an static string.
    #[error("Error")]
    StrError(&'static str),
    ///// Error when a constant point is not in the elliptic curve
    //#[error("Point ({0}, {1}) is not in the curve")]
    //PointNotInCurve(Blstrs, Blstrs),
    ///// Error when a constant point is not in the elliptic curve (3D version)
    //#[error("Point ({0}, {1}, {2}) is not in the curve")]
    //Point3NotInCurve(MidnightFp, MidnightFp, MidnightFp),
    ///// Error when a constant point is not in the elliptic curve (3D version)
    //#[error("Point ({0:?}, {1:?}, {2:?}) is not in the curve")]
    //Point3NotInCurveSecp256k1(Secp256k1Fp, Secp256k1Fp, Secp256k1Fp),
    /// Int cast error.
    #[error(transparent)]
    IntCast(#[from] std::num::TryFromIntError),
    /// Big int cast error.
    #[error(transparent)]
    BigIntCast(#[from] TryFromBigIntError<BigInt>),
    /// Error when an encountered an unexpected number of elements.
    #[error("{header}Was expecting {expected} elements but got {actual}")]
    UnexpectedElements {
        /// Context header for the error
        header: &'static str,
        /// The expected number of elements.
        expected: usize,
        /// The number of elements.
        actual: usize,
    },
}

/// Ensures that the given predicate is true, returning error if not.
pub fn assert_expected_elements(
    header: &'static str,
    expected: usize,
    actual: usize,
    pred: impl FnOnce(usize, usize) -> bool,
) -> Result<(), Error> {
    if pred(expected, actual) {
        Ok(())
    } else {
        Err(Error::UnexpectedElements {
            header,
            expected,
            actual,
        })
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::StrError(value)
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[cfg(feature = "proofs")]
/// Alias for the error emitted by Halo2 during synthesis.
pub type PlonkError = midnight_proofs::plonk::Error;

#[cfg(feature = "proofs")]
impl From<Error> for PlonkError {
    fn from(value: Error) -> Self {
        match value {
            Error::Plonk(err) => err,
            err => Self::Transcript(std::io::Error::other(err)),
        }
    }
}
