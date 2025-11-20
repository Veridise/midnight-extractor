use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{End, Parse},
    spanned::Spanned,
    Attribute, DeriveInput, Generics, Ident, Meta, Path, TypeParam, TypeParamBound,
};

use crate::parse::extractor::{ExtractorCmd, FieldCmd};

/// Internal implementation of [`super::derive_no_chip_args`].
pub fn derive_no_chip_args_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = input.generics;
    // Split generics into (impl generics) (ty generics) (where clause)
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics extractor_support::circuit::NoChipArgs for #name #ty_generics #where_clause {}
    }
}

/// Internal implementation of [`super::derive_circuit_initialization_from_scratch`].
pub fn derive_circuit_initialization_from_scratch_impl(
    input: DeriveInput,
) -> syn::Result<TokenStream> {
    let input_span = input.span();
    let name = input.ident;
    let mut generics = input.generics;

    let l = unique_layouter_ident(&generics);
    let f = find_field_param(&generics, input_span)?;
    let other_params = find_annotated_params(&generics);

    cleanup_helper_attrs(&mut generics);
    let where_clause = generics.make_where_clause();

    let predicates = &mut where_clause.predicates;

    predicates.push(syn::parse2(quote! {
        #l: midnight_proofs::circuit::Layouter<#f>
    })?);
    predicates.push(syn::parse2(quote! {
        #f: ff::PrimeField
    })?);
    for param in other_params {
        predicates.push(syn::parse2(quote! {
            #param: crate::testing_utils::FromScratch<#f>
        })?);
    }

    let type_params = generics.type_params();
    let lifetimes = generics.lifetimes();
    let const_params = generics.const_params();
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let as_from_scratch = quote! {
        <#name #ty_generics as crate::testing_utils::FromScratch<#f>>
    };
    let midnight_proofs = format_ident!("midnight_proofs");

    let code = quote! {
        impl<#(#lifetimes,)* #l, #(#type_params,)* #(#const_params,)*>
            extractor_support::circuit::CircuitInitialization<#l> for #name #ty_generics
        #where_clause
        {
            type Config = #as_from_scratch::Config;
            type Args = ();
            type ConfigCols =
                [#midnight_proofs::plonk::Column<#midnight_proofs::plonk::Instance>; 2];
            type CS = #midnight_proofs::plonk::ConstraintSystem<#f>;
            type Error = #midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                #as_from_scratch::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                #as_from_scratch::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut #l,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }

        }
    };
    log::debug!("generated:\n==============\n{code}\n==============");
    Ok(code)
}

fn cleanup_helper_attrs(generics: &mut Generics) {
    for ty in generics.type_params_mut() {
        ty.attrs
            .retain(|a| !(a.path().is_ident("field") || a.path().is_ident("from_scratch")))
    }
}

fn select_param(ty: &TypeParam) -> Option<syn::Result<Ident>> {
    let cmds = ty
        .attrs
        .iter()
        .filter_map(ExtractorCmd::from_attr)
        .filter(|cmd| matches!(cmd, Ok(ExtractorCmd::FromScratch(_)) | Err(_)))
        .collect::<syn::Result<Vec<_>>>();
    match cmds.as_deref() {
        Ok([]) => None,
        Ok(_) => Some(Ok(ty.ident.clone())),
        Err(err) => Some(Err(err.clone())),
    }
}

fn find_annotated_params(generics: &Generics) -> Vec<Ident> {
    generics
        .type_params()
        .filter_map(|ty| {
            ty.attrs
                .iter()
                .any(|a| a.path().is_ident("from_scratch"))
                .then(|| ty.ident.clone())
        })
        .collect()
}

fn unique_layouter_ident(generics: &Generics) -> syn::Ident {
    let idents = generics
        .type_params()
        .map(|t| &t.ident)
        .chain(generics.const_params().map(|c| &c.ident))
        .collect::<HashSet<_>>();
    let base = "__Layouter";
    std::iter::repeat(base)
        .enumerate()
        .map(|(n, base)| {
            if n == 0 {
                return format_ident!("{base}");
            }
            format_ident!("{base}{n}")
        })
        .find(|i| !idents.contains(&i))
        .unwrap()
}

fn try_field_attribute(
    cmd: syn::Result<ExtractorCmd>,
    type_ident: &Ident,
) -> Option<syn::Result<FieldParam>> {
    match cmd {
        Err(err) => Some(Err(err)),
        Ok(ExtractorCmd::FromScratch(_)) => None,
        Ok(ExtractorCmd::Field(FieldCmd {
            path: Some(path), ..
        })) => Some(Ok(FieldParam::Path(path.path))),
        Ok(ExtractorCmd::Field(FieldCmd { path: None, .. })) => {
            Some(Ok(FieldParam::Ident(type_ident.clone())))
        }
    }
}

enum FieldParam {
    Ident(Ident),
    Path(Path),
}

impl quote::ToTokens for FieldParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldParam::Ident(ident) => ident.to_tokens(tokens),
            FieldParam::Path(path) => path.to_tokens(tokens),
        }
    }
}

impl PartialEq<syn::Ident> for FieldParam {
    fn eq(&self, other: &syn::Ident) -> bool {
        match self {
            FieldParam::Ident(ident) => ident == other,
            FieldParam::Path(_) => false,
        }
    }
}

struct MaybePath(Option<Path>);

impl Parse for MaybePath {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let peek = input.lookahead1();
        if peek.peek(Ident) {
            Ok(Self(Some(input.parse()?)))
        } else if peek.peek(End) {
            Ok(Self(None))
        } else {
            Err(peek.error())
        }
    }
}

fn from_attr(a: &Attribute, ty: &TypeParam) -> syn::Result<FieldParam> {
    match &a.meta {
        Meta::Path(_) => return Ok(FieldParam::Ident(ty.ident.clone())),
        _ => {}
    }
    a.parse_args::<MaybePath>()?
        .0
        .ok_or_else(|| syn::Error::new_spanned(a, "was expecting a path to the field type"))
        .map(FieldParam::Path)
}

fn find_field_param(generics: &Generics, input_span: Span) -> syn::Result<FieldParam> {
    use TypeParamBound::Trait;
    // First look for types annotated with #[field].
    generics
        .type_params()
        .find_map(|ty| {
            ty.attrs
                .iter()
                .find_map(|a| a.path().is_ident("field").then(|| from_attr(a, ty)))
        })
        // If none was annotated that way then check for types that declare satisfying
        // the Field trait.
        .or_else(|| {
            generics.type_params().find_map(|ty| {
                ty.bounds
                    .iter()
                    .any(|b| {
                        let Trait(trait_bound) = b else {
                            return false;
                        };
                        let ident = trait_bound.path.segments.last().map(|s| s.ident.to_string());
                        matches!(ident.as_deref(), Some("Field" | "PrimeField"))
                    })
                    .then(|| Ok(FieldParam::Ident(ty.ident.clone())))
            })
        })
        // Lastly default to being named 'F'
        .or_else(|| {
            generics
                .type_params()
                .find_map(|ty| (ty.ident == "F").then(|| Ok(FieldParam::Ident(ty.ident.clone()))))
        })
        .ok_or_else(|| {
            syn::Error::new(
                input_span,
                "Derived struct requires at least one type parameter implementing ff::PrimeField",
            )
        })?
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use macro_expand::Context;
    use rstest::{fixture, rstest};
    use simplelog::{Config, TestLogger};

    fn derive_circuit_initialization_from_scratch_test(input: DeriveInput) -> TokenStream {
        derive_circuit_initialization_from_scratch_impl(input).unwrap()
    }

    #[fixture]
    fn ctx() -> Context<'static> {
        let _ = TestLogger::init(LevelFilter::Debug, Config::default());
        let mut ctx = Context::new();
        ctx.register_proc_macro_derive(
            "InitFromScratch".into(),
            derive_circuit_initialization_from_scratch_test,
            vec!["field".to_string(), "from_scratch".to_string()],
        );

        ctx
    }

    macro_rules! parse {
        ($s:expr) => {{
            let ts: proc_macro2::TokenStream = $s.parse().unwrap();
            ts
        }};
    }

    macro_rules! unparse {
        ($ts:expr) => {
            prettyplease::unparse(&syn::parse2($ts).unwrap())
        };
    }

    // Each #[should_panic] affects only the #[case] below it

    #[rstest]
    #[should_panic(
        expected = "Derived struct requires at least one type parameter implementing ff::PrimeField"
    )]
    #[case(
        "#[derive(InitFromScratch)] struct S {}",
        r"
        struct S {}
        impl<__Layouter> extractor_support::circuit::CircuitInitialization<__Layouter> for S 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<F> { f: F }
        ",
        r"
        struct S<F> { f: F }
        impl<__Layouter, F> extractor_support::circuit::CircuitInitialization<__Layouter> for S<F> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {
            type Config = <S<F> as crate::testing_utils::FromScratch<F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<F> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<F> as crate::testing_utils::FromScratch<F>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[should_panic(
        expected = "Derived struct requires at least one type parameter implementing ff::PrimeField"
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<__Layouter> { f: __Layouter }
        ",
        r"
        struct S<__Layouter> { f: __Layouter }
        impl<__Layouter1, __Layouter> extractor_support::circuit::CircuitInitialization<__Layouter1> for S<__Layouter> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {}
        "
    )]
    #[should_panic(
        expected = "Derived struct requires at least one type parameter implementing ff::PrimeField"
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a> { f: &'a str }
        ",
        r"
        struct S<'a> { f: &'a str }
        impl<'a, __Layouter> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, F> { f: &'a F }
        ",
        r"
        struct S<'a, F> { f: &'a F }
        impl<'a, __Layouter, F> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, F> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {
            type Config = <S<'a, F> as crate::testing_utils::FromScratch<F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, F: Copy> { f: &'a F }
        ",
        r"
        struct S<'a, F: Copy> { f: &'a F }
        impl<'a, __Layouter, F: Copy> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, F> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {
            type Config = <S<'a, F> as crate::testing_utils::FromScratch<F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, A: Field> { f: &'a A }
        ",
        r"
        struct S<'a, A: Field> { f: &'a A }
        impl<'a, __Layouter, A: Field> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, A> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<A>,
            A: ff::PrimeField,
        {
            type Config = <S<'a, A> as crate::testing_utils::FromScratch<A>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<A>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, A: PrimeField> { f: &'a A }
        ",
        r"
        struct S<'a, A: PrimeField> { f: &'a A }
        impl<'a, __Layouter, A: PrimeField> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, A> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<A>,
            A: ff::PrimeField,
        {
            type Config = <S<'a, A> as crate::testing_utils::FromScratch<A>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<A>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, #[field] A> { f: &'a A }
        ",
        r"
        struct S<'a, A> { f: &'a A }
        impl<'a, __Layouter,  A> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, A> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<A>,
            A: ff::PrimeField,
        {
            type Config = <S<'a, A> as crate::testing_utils::FromScratch<A>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<A>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, A> as crate::testing_utils::FromScratch<A>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, F> where F: Copy { f: &'a F }
        ",
        r"
        struct S<'a, F> where F: Copy { f: &'a F }
        impl<'a, __Layouter, F> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, F> 
        where 
            F: Copy,
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {
            type Config = <S<'a, F> as crate::testing_utils::FromScratch<F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, F> as crate::testing_utils::FromScratch<F>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        #[derive(InitFromScratch)]
        struct S<'a, F, const N: usize> where F: Copy { f: [&'a F; N] }
        ",
        r"
        struct S<'a, F, const N: usize> where F: Copy { f: [&'a F; N] }
        impl<'a, __Layouter, F, const N: usize> extractor_support::circuit::CircuitInitialization<__Layouter> for S<'a, F, N> 
        where 
            F: Copy,
            __Layouter: midnight_proofs::circuit::Layouter<F>,
            F: ff::PrimeField,
        {
            type Config = <S<'a, F, N> as crate::testing_utils::FromScratch<F>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<F>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<'a, F, N> as crate::testing_utils::FromScratch<F>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<'a, F, N> as crate::testing_utils::FromScratch<F>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        trait CT { type Base; }
        #[derive(InitFromScratch)]
        struct S<#[field(C::Base)] C: CT> { f: C::Base }
        ",
        r"
        trait CT { type Base; }
        struct S<C: CT> { f: C::Base }
        impl<__Layouter, C: CT> extractor_support::circuit::CircuitInitialization<__Layouter> for S<C> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<C::Base>,
            C::Base: ff::PrimeField,
        {
            type Config = <S<C> as crate::testing_utils::FromScratch<C::Base>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<C::Base>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<C> as crate::testing_utils::FromScratch<C::Base>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<C> as crate::testing_utils::FromScratch<C::Base>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]
    #[case(
        r"
        trait CT { type Base; }
        #[derive(InitFromScratch)]
        struct S<#[field(C::Base)] C: CT, #[from_scratch] N> { f: C::Base, n: N }
        ",
        r"
        trait CT { type Base; }
        struct S<C: CT, N> { f: C::Base, n: N }
        impl<__Layouter, C: CT, N> 
            extractor_support::circuit::CircuitInitialization<__Layouter> for S<C, N> 
        where 
            __Layouter: midnight_proofs::circuit::Layouter<C::Base>,
            C::Base: ff::PrimeField,
            N: crate::testing_utils::FromScratch<C::Base>
        {
            type Config = <S<C,N> as crate::testing_utils::FromScratch<C::Base>>::Config;
            type Args = ();
            type ConfigCols =
                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
            type CS = midnight_proofs::plonk::ConstraintSystem<C::Base>;
            type Error = midnight_proofs::plonk::Error;

            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
                <S<C,N> as crate::testing_utils::FromScratch<C::Base>>::new_from_scratch(config)
            }

            fn configure_circuit(
                meta: &mut Self::CS,
                instance_columns: &Self::ConfigCols,
            ) -> Self::Config {
                <S<C,N> as crate::testing_utils::FromScratch<C::Base>>::configure_from_scratch(meta, instance_columns)
            }

            fn load_chip(
                &self,
                layouter: &mut __Layouter,
                _config: &Self::Config,
            ) -> Result<(), Self::Error> {
                use crate::testing_utils::FromScratch;
                self.load_from_scratch(layouter)
            }}
        "
    )]

    fn init_from_scratch(ctx: Context, #[case] input: &str, #[case] expected: &str) {
        let input = parse!(input);
        let expected = unparse!(parse!(expected));
        let output = ctx.transform(input);
        let formatted = unparse!(output);
        similar_asserts::assert_eq!(expected, formatted);
    }
}
