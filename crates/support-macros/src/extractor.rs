use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

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
