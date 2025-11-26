use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Expr, Generics, Ident, Token, Type};

pub trait HarnessCfg: Parse {
    fn aux_tokens(&self) -> TokenStream;

    fn emit_chip_args_impl(
        &self,
        fn_ident: &Ident,
        field_param: &Type,
        generics: &Generics,
        circuit_ty: &Ident,
    ) -> TokenStream;
}

pub struct WithArgsCfg {
    aux: Option<Expr>,
    args_type: syn::Type,
}

impl Parse for WithArgsCfg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args_type = input.parse()?;
        let aux = if input.is_empty() {
            None
        } else {
            <Token![,]>::parse(input)?;
            Some(Expr::parse(input)?)
        };
        Ok(Self { args_type, aux })
    }
}

impl HarnessCfg for WithArgsCfg {
    fn aux_tokens(&self) -> TokenStream {
        match &self.aux {
            Some(expr) => quote! { Some(& #expr) },
            None => quote! { None },
        }
    }

    fn emit_chip_args_impl(
        &self,
        fn_ident: &Ident,
        field_param: &Type,
        generics: &Generics,
        circuit_ty: &Ident,
    ) -> TokenStream {
        let args_type = &self.args_type;
        let args_fn = Ident::new(&format!("{}_args", fn_ident), fn_ident.span());
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! {
            impl #impl_generics mdnt_support::circuit::ChipArgs<#field_param> for #circuit_ty #ty_generics #where_clause {
                type Args = #args_type;

                fn chip_args(&self) -> Self::Args {
                    #args_fn ()
                }

            }
        }
    }
}

pub struct NoArgsCfg {
    aux: Option<Expr>,
}

impl Parse for NoArgsCfg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { aux: None });
        }

        input.parse::<Expr>().map(|e| Self { aux: Some(e) })
    }
}

impl HarnessCfg for NoArgsCfg {
    fn aux_tokens(&self) -> TokenStream {
        match &self.aux {
            Some(expr) => quote! { Some(& #expr) },
            None => quote! { None },
        }
    }

    fn emit_chip_args_impl(
        &self,
        _: &Ident,
        _: &Type,
        generics: &Generics,
        circuit_ty: &Ident,
    ) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        quote! { impl #impl_generics mdnt_support::circuit::NoChipArgs for #circuit_ty #ty_generics #where_clause {} }
    }
}

pub struct CircuitCfg {
    mut_chip: bool,
}

impl CircuitCfg {
    pub fn new() -> Self {
        Self { mut_chip: false }
    }

    pub fn with_mut_chip(mut self) -> Self {
        self.mut_chip = true;
        self
    }

    pub fn mut_chip(&self) -> bool {
        self.mut_chip
    }
}
