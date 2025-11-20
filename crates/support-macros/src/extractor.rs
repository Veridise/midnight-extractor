use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, DeriveInput, Generics, Ident, TypeParamBound};

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

//($C:ty, $F:ident $(, $other:ident)* $(where $($conds:tt)+ )?) => {
//        impl<L, $F, $( $other,)*> $crate::circuit::CircuitInitialization<L> for $C
//        where
//            L: midnight_proofs::circuit::Layouter<$F>,
//            $F: ff::PrimeField,
//            $C: crate::testing_utils::FromScratch<$F>
//            $(, $($conds)+)?
//        {
//            type Config = <$C as crate::testing_utils::FromScratch<$F>>::Config;
//            type Args = ();
//            type ConfigCols =
//                [midnight_proofs::plonk::Column<midnight_proofs::plonk::Instance>; 2];
//            type CS = midnight_proofs::plonk::ConstraintSystem<$F>;
//            type Error = midnight_proofs::plonk::Error;
//
//            fn new_chip(config: &Self::Config, _: Self::Args) -> Self {
//                <$C as crate::testing_utils::FromScratch<$F>>::new_from_scratch(config)
//            }
//
//            fn configure_circuit(
//                meta: &mut Self::CS,
//                instance_columns: &Self::ConfigCols,
//            ) -> Self::Config {
//                <$C as crate::testing_utils::FromScratch<$F>>::configure_from_scratch(meta, instance_columns)
//            }
//
//            fn load_chip(
//                &self,
//                layouter: &mut L,
//                _config: &Self::Config,
//            ) -> Result<(), Self::Error> {
//                use crate::testing_utils::FromScratch;
//                self.load_from_scratch(layouter)
//            }
//        }
//    };
//

/// Internal implementation of [`super::derive_circuit_initialization_from_scratch`].
pub fn derive_circuit_initialization_from_scratch_impl(
    input: DeriveInput,
) -> syn::Result<TokenStream> {
    let input_span = input.span();
    let name = input.ident;
    let mut generics = input.generics;

    let l = unique_layouter_ident(&generics);
    let f = find_field_param(&generics)
        .ok_or_else(|| {
            syn::Error::new(
                input_span,
                "Derived struct requires at least one type parameter implementing ff::PrimeField",
            )
        })?
        .clone();
    let where_clause = generics.make_where_clause();

    let predicates = &mut where_clause.predicates;

    predicates.push(syn::parse2(quote! {
        #l: midnight_proofs::circuit::Layouter<#f>
    })?);
    predicates.push(syn::parse2(quote! {
        #f: ff::PrimeField
    })?);

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
                layouter: &mut L,
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

fn find_field_param(generics: &Generics) -> Option<&Ident> {
    use TypeParamBound::Trait;
    generics.type_params().find_map(|ty| {
        let has_bound = ty.bounds.iter().any(|b| {
            let Trait(trait_bound) = b else {
                return false;
            };
            let ident = trait_bound.path.segments.last().map(|s| s.ident.to_string());
            matches!(ident.as_deref(), Some("Field" | "PrimeField"))
        });
        // Fallback to the name 'F' since that's the custom.
        (has_bound || ty.ident == "F").then_some(&ty.ident)
    })
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
        // Register the inner implementation using testable implementation
        ctx.register_proc_macro_derive(
            "InitFromScratch".into(),
            derive_circuit_initialization_from_scratch_test,
            vec![],
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
                layouter: &mut L,
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
                layouter: &mut L,
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
                layouter: &mut L,
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
                layouter: &mut L,
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
                layouter: &mut L,
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
                layouter: &mut L,
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
                layouter: &mut L,
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
        similar_asserts::assert_eq!(formatted, expected);
    }
}
