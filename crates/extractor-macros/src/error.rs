use proc_macro2::TokenStream;

pub enum ErrorType {
    TypeNotReference,
    TypeNotMutReference,
    UnexpectedSelf,
    UnexpectedParams {
        from: usize,
        to: usize,
        lst: &'static str,
    },
    FailedExtractLayouterParam(&'static str),
    IncorrectReturnType,
    ExpectedTypeInGeneric(usize),
}

pub struct Error(pub ErrorType, pub TokenStream);

impl From<Error> for syn::Error {
    fn from(value: Error) -> Self {
        Self::new_spanned(
            value.1,
            match value.0 {
                ErrorType::TypeNotReference => "parameter must be a &-reference".to_string(),
                ErrorType::TypeNotMutReference => "parameter must be a &mut-reference".to_string(),
                ErrorType::UnexpectedSelf => {
                    "unexpected self parameter; expected typed parameter".to_string()
                }
                ErrorType::UnexpectedParams { from, to, lst } => {
                    format!("expected between {from} and {to} parameters: {lst}")
                }
                ErrorType::FailedExtractLayouterParam(msg) => {
                    format!("failed to extract F from &mut impl Layouter<F>: {msg}")
                }
                ErrorType::IncorrectReturnType => {
                    "return type must be () or Result<Output, Error>".to_string()
                }
                ErrorType::ExpectedTypeInGeneric(idx) => {
                    format!("expected a type for generic argument #{idx}")
                }
            },
        )
    }
}

macro_rules! tokenize {
    ($result:expr) => {
        match $result {
            Ok(tok) => tok,
            Err(err) => err.to_compile_error(),
        }
        .into()
    };
}

pub(super) use tokenize;
