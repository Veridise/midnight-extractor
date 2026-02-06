use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Type {
    Native,
    Bit,
    Field,
    Byte,
    Biguint,
    Scalar,
    Point,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Native => "native",
                Type::Bit => "bit",
                Type::Field => "field",
                Type::Byte => "byte",
                Type::Biguint => "biguint",
                Type::Scalar => "scalar",
                Type::Point => "point",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Chip {
    Native,
    NativeGadget,
    Field,
    Poseidon,
    Pow2Range,
    P2RDecomposition,
    Sha256,
    Ecc,
    ForeignEccNative,
    ForeignEccField,
    Vector,
    Biguint,
    Stdlib,
    Automaton,
    Base64,
    HashToCurve,
    Map,
    Parser,
    VarlenPoseidon,
    VarlenSha256,
    RipeMd160,
    #[cfg(feature = "sha3")]
    Sha3,
    #[cfg(feature = "sha3")]
    Packed,
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip::Native => write!(f, "native"),
            Chip::Field => write!(f, "field"),
            Chip::Poseidon => write!(f, "poseidon"),
            Chip::Sha256 => {
                write!(f, "sha256")
            }
            Chip::Pow2Range => write!(f, "pow2range"),
            Chip::P2RDecomposition => {
                if f.alternate() {
                    write!(f, "p2r_decomposition")
                } else {
                    write!(f, "p2r-decomposition")
                }
            }
            Chip::NativeGadget => {
                if f.alternate() {
                    write!(f, "native_gadget")
                } else {
                    write!(f, "native-gadget")
                }
            }
            Chip::Ecc => write!(f, "ecc"),
            Chip::Vector => write!(f, "vector"),
            Chip::Biguint => write!(f, "biguint"),
            Chip::Stdlib => write!(f, "stdlib"),
            Chip::ForeignEccNative => {
                if f.alternate() {
                    write!(f, "foreign_ecc_native")
                } else {
                    write!(f, "foreign-ecc-native")
                }
            }
            Chip::ForeignEccField => {
                if f.alternate() {
                    write!(f, "foreign_ecc_field")
                } else {
                    write!(f, "foreign-ecc-field")
                }
            }
            Chip::Automaton => write!(f, "automaton"),
            Chip::Base64 => write!(f, "base64"),
            Chip::HashToCurve => {
                if f.alternate() {
                    write!(f, "hash_to_curve")
                } else {
                    write!(f, "hash-to-curve")
                }
            }
            Chip::Map => write!(f, "map"),
            Chip::Parser => write!(f, "parser"),
            Chip::VarlenPoseidon => {
                if f.alternate() {
                    write!(f, "varlen_poseidon")
                } else {
                    write!(f, "varlen-poseidon")
                }
            }
            Chip::VarlenSha256 => {
                if f.alternate() {
                    write!(f, "varlen_sha256")
                } else {
                    write!(f, "varlen-sha256")
                }
            }
            Chip::RipeMd160 => {
                if f.alternate() {
                    write!(f, "ripemd160")
                } else {
                    write!(f, "ripemd160")
                }
            }
            #[cfg(feature = "sha3")]
            Chip::Sha3 => write!(f, "sha3"),
            #[cfg(feature = "sha3")]
            Chip::Packed => write!(f, "packed"),
        }
    }
}
