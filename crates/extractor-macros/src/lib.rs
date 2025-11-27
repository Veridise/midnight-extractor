use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::{
    error::tokenize,
    harness_impl::{harness_impl, unit_harness_impl, CircuitCfg, NoArgsCfg, WithArgsCfg},
    parse::harness::{HarnessFn, UnitHarnessFn},
};

mod delegated_impl;
mod error;
mod harness_impl;
mod parse;

#[proc_macro_attribute]
pub fn harness(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(harness_impl(
        parse_macro_input!(item as HarnessFn),
        parse_macro_input!(attr as NoArgsCfg),
        CircuitCfg::new()
    ))
}

#[proc_macro_attribute]
pub fn harness_with_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(harness_impl(
        parse_macro_input!(item as HarnessFn),
        parse_macro_input!(attr as WithArgsCfg),
        CircuitCfg::new()
    ))
}

#[proc_macro_attribute]
pub fn harness_mut(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(harness_impl(
        parse_macro_input!(item as HarnessFn),
        parse_macro_input!(attr as NoArgsCfg),
        CircuitCfg::new().with_mut_chip()
    ))
}

#[proc_macro_attribute]
pub fn harness_with_args_mut(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(harness_impl(
        parse_macro_input!(item as HarnessFn),
        parse_macro_input!(attr as WithArgsCfg),
        CircuitCfg::new().with_mut_chip()
    ))
}

#[proc_macro_attribute]
pub fn unit_harness(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(unit_harness_impl(
        parse_macro_input!(item as UnitHarnessFn),
        parse_macro_input!(attr as NoArgsCfg),
    ))
}

#[proc_macro_attribute]
pub fn unit_harness_with_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(unit_harness_impl(
        parse_macro_input!(item as UnitHarnessFn),
        parse_macro_input!(attr as WithArgsCfg),
    ))
}

#[proc_macro_attribute]
pub fn entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    let name = parse_macro_input!(attr as syn::LitStr);
    let fname = &f.sig.ident;
    quote::quote! {
        mdnt_extractor_core::entry!(#name, #fname);
        #f
    }
    .into()
}

#[proc_macro_attribute]
pub fn usize_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);
    let fname = quote::format_ident!("{}_args", f.sig.ident);
    let n = parse_macro_input!(attr as syn::LitInt);
    quote::quote! {
        fn #fname() -> usize { #n }
        #f
    }
    .into()
}

#[proc_macro_attribute]
pub fn delegated(attr: TokenStream, item: TokenStream) -> TokenStream {
    tokenize!(delegated_impl::delegated(
        parse_macro_input!(attr as syn::Ident),
        parse_macro_input!(item as syn::ImplItemFn),
    ))
}
