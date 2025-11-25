use proc_macro2::TokenStream;
use quote::ToTokens as _;
use syn::{punctuated::Punctuated, Error, Ident, ImplItemFn, Pat, Token};

fn create_body(delegator: Ident, fname: &Ident, fargs: Punctuated<&Pat, Token![,]>) -> TokenStream {
    quote::quote! {
        { self.#delegator.#fname(#fargs) }
    }
}

pub fn delegated(delegator: Ident, mut f: ImplItemFn) -> Result<TokenStream, Error> {
    let fargs: Punctuated<&Pat, Token![,]> = f
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_type) => Some(pat_type.pat.as_ref()),
        })
        .collect();
    let body = create_body(delegator, &f.sig.ident, fargs);
    f.block = syn::parse2(body)?;
    Ok(f.into_token_stream())
}
