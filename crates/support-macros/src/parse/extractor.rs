use proc_macro2::TokenStream;
use syn::{
    parse::{End, Parse},
    Attribute, Ident, Path, Token,
};

mod kw {
    syn::custom_keyword!(field);
    syn::custom_keyword!(from_scratch);
}

pub struct FieldPath {
    pub equal: Token![=],
    pub path: Path,
}

pub struct FieldCmd {
    pub field: kw::field,
    pub path: Option<FieldPath>,
}

impl Parse for FieldCmd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field = input.parse::<kw::field>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![=]) {
            Ok(Self {
                field,
                path: Some(FieldPath {
                    equal: input.parse()?,
                    path: input.parse()?,
                }),
            })
        } else if lookahead.peek(End) {
            Ok(Self { field, path: None })
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct FromScratchCmd {
    from_scratch: kw::from_scratch,
}

impl Parse for FromScratchCmd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            from_scratch: input.parse()?,
        })
    }
}

pub enum ExtractorCmd {
    Field(FieldCmd),
    FromScratch(FromScratchCmd),
}

impl ExtractorCmd {
    pub fn from_attr(a: &Attribute) -> Option<syn::Result<Self>> {
        a.path().is_ident("extractor").then(|| a.parse_args::<Self>())
    }
}

impl Parse for ExtractorCmd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::field) {
            input.parse().map(ExtractorCmd::Field)
        } else if lookahead.peek(kw::from_scratch) {
            input.parse().map(ExtractorCmd::FromScratch)
        } else {
            Err(lookahead.error())
        }
    }
}
