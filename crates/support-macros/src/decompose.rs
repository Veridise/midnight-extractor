use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DataEnum, DeriveInput, Fields, Generics, Ident, Index, Type};

/// Internal implementation of [`super::derive_decompose_in_cells`].
pub fn derive_decompose_in_cells_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = input.generics;

    // Collect field types for where bounds
    let mut bounds = Vec::new();

    // Split generics into (impl generics) (ty generics) (where clause)
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) => {
            handle_fields(&data.fields, &mut bounds, Some(quote! { self. }), None)
        }
        Data::Enum(data) => handle_enum(data, &mut bounds),
        Data::Union(_) => {
            unimplemented!("Unions are not supported")
        }
    };

    quote! {
        impl #impl_generics picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for #name #ty_generics
        where
            #(#bounds,)*
            #where_clause
        {
            fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
                #body
            }
        }
    }
}

/// Creates the tokens for referencing a tuple field.
///
/// If `bind` is true then the tuple field is referenced by an identifier instead of the index.
/// This is done to be able to handle the two possible cases; a tuple struct and a tuple enum variant.
/// For former doesn't bind and is expected to do `self.{idx}` and the latter will bind the field
/// to an identifier with the format `f{idx}`.
fn format_tuple_field(idx: usize, bind: bool) -> TokenStream {
    if bind {
        format_ident!("__{idx}").into_token_stream()
    } else {
        Index::from(idx).into_token_stream()
    }
}

/// Gathers all the emitted code for handling the fields of a type.
///
/// Returns the body of the method required by the trait as a chained iterators calling the method
/// on each inner field. The where-clause bounds for each type are gathered in `bounds` and,
/// optionally, if it's necessary to create bindings they will get written to `var_names`. You
/// must pass a value for either `receiver` or `var_names`. Cannot pass both at the same time.
///
/// # Panics
///
/// If both `receiver` and `var_names` are [`Some`].
fn handle_fields(
    fields: &Fields,
    bounds: &mut Vec<TokenStream>,
    receiver: Option<TokenStream>,
    mut var_names: Option<&mut Vec<TokenStream>>,
) -> TokenStream {
    assert!(!(receiver.is_some() && var_names.is_some()));
    let field_calls = fields
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            let ty = &f.ty;
            let ident = f.ident.as_ref().map(ToTokens::to_token_stream);

            bounds.push(quote! { #ty: picus_support::DecomposeIn<midnight_proofs::circuit::Cell> });
            if let Some(var_names) = &mut var_names {
                var_names.push(ident.clone().unwrap_or_else(|| format_tuple_field(idx, true)));
            }

            ident.unwrap_or_else(|| format_tuple_field(idx, receiver.is_none()))
        })
        .map(|f| quote! { .chain(#receiver #f.cells()) });
    quote! {
            std::iter::empty() #(#field_calls)*
    }
}

fn handle_enum(data: &DataEnum, bounds: &mut Vec<TokenStream>) -> TokenStream {
    let variants = data.variants.iter().map(|v| {
        let name = &v.ident;
        let mut var_names = vec![];
        let body = handle_fields(&v.fields, bounds, None, Some(&mut var_names));
        let var_names = match v.fields {
            Fields::Named(_) => Some(quote! { { #( #var_names ),* } }),
            Fields::Unnamed(_) => Some(quote! { ( #( #var_names ),* ) }),
            Fields::Unit => None,
        };

        quote! {
            Self::#name #var_names => { #body }
        }
    });

    quote! {
        match self {
            #(#variants),*
        }
    }
}
