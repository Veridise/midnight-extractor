use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    spanned::Spanned, Attribute, Block, FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Visibility,
};

const INPUT_ATTR: &str = "input";
const OUTPUT_ATTR: &str = "output";
const LAYOUTER_ATTR: &str = "layouter";

/// Internal implementation of [`super::group`].
///
/// Refer to that macro for details about its user facing API.
pub fn group_impl(input_fn: ItemFn) -> syn::Result<TokenStream> {
    let fn_ident = &input_fn.sig.ident;
    let group_ident = format_ident!("__{}__group", fn_ident);

    let (layouter, io) = locate_attributes(&input_fn)?;
    let layouter = select_layouter(&layouter, input_fn.sig.span())?;
    let io_annotations = generate_io_annotations(io, &group_ident);
    let cleaned_inputs = clean_inputs(input_fn.sig.inputs.iter());

    Ok(emit_wrapped_fn(
        &input_fn.attrs,
        &input_fn.vis,
        &input_fn.sig.ident,
        cleaned_inputs,
        &input_fn.sig.output,
        layouter,
        &group_ident,
        io_annotations,
        &input_fn.block,
    ))
}

#[cfg(feature = "region-groups")]
fn emit_wrapped_fn(
    fn_attrs: &[Attribute],
    vis: &Visibility,
    fn_ident: &Ident,
    cleaned_inputs: impl Iterator<Item = FnArg>,
    output: &ReturnType,
    layouter: Ident,
    group_ident: &Ident,
    io_annotations: impl Iterator<Item = TokenStream>,
    user_block: &Block,
) -> TokenStream {
    quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident(#(#cleaned_inputs, )*) #output {

            #layouter.group(|| stringify!(#fn_ident), midnight_proofs::default_group_key!(), |#layouter,#[allow(non_snake_case)] #group_ident| {
                use picus_support::DecomposeIn as _;
                #(#io_annotations)*
                let inner_result = #user_block;
                #group_ident.annotate_outputs(self.cells())?;
                inner_result
            })
        }
    }
}

/// If region-groups is disabled we emit the function as is but with the inputs cleaned to avoid
/// errors with the attributes.
#[cfg(not(feature = "region-groups"))]
fn emit_wrapped_fn(
    fn_attrs: &[Attribute],
    vis: &Visibility,
    fn_ident: &Ident,
    cleaned_inputs: impl Iterator<Item = FnArg>,
    output: &ReturnType,
    _: Ident,
    _: &Ident,
    _: impl Iterator<Item = TokenStream>,
    user_block: &Block,
) -> TokenStream {
    quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident(#(#cleaned_inputs, )*) #output {
            #user_block
        }
    }
}

type AnnotatedPat<'a> = (ArgAttributes, &'a PatType);

/// Searches arguments that were annotated and splits them between `#[layouter]`
/// annotations and the others.
fn locate_attributes(
    input_fn: &ItemFn,
) -> syn::Result<(Vec<AnnotatedPat<'_>>, Vec<AnnotatedPat<'_>>)> {
    Ok(input_fn
        .sig
        .inputs
        .iter()
        .filter_map(ArgAttributes::try_from_arg)
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .partition(|(attr, _)| matches!(attr, ArgAttributes::Layouter)))
}

/// Searches the binding name in the list of arguments annotated with
/// `#[layouter]`.
///
/// If the list is empty the [`struct@Ident`] defaults to `layouter`.
/// Fails if the list has more than one element or the annotated argument is not
/// an identifier.
fn select_layouter(layouter: &[AnnotatedPat], span: Span) -> syn::Result<Ident> {
    Ok(match layouter {
        [] => format_ident!("layouter"),
        [(_, pat)] => match &*pat.pat {
            Pat::Ident(ident) => ident.ident.clone(),
            _ => {
                return Err(syn::Error::new(
                    span,
                    "Argument annotated with #[layouter] must be an identifier",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                span,
                "More than one #[layouter] annotation is not allowed",
            ));
        }
    })
}

fn generate_io_annotations(
    io: Vec<AnnotatedPat>,
    group_ident: &Ident,
) -> impl Iterator<Item = TokenStream> {
    io.into_iter()
        .map(|(attr, pat)| attr.emit_code(&pat.pat, group_ident).unwrap_or_default())
}

/// Removes the extra attributes in the arguments since those are fake.
fn clean_inputs<'a>(inputs: impl Iterator<Item = &'a FnArg>) -> impl Iterator<Item = FnArg> {
    inputs.cloned().map(|i| match i {
        FnArg::Typed(mut pat_type) => {
            pat_type.attrs.retain(|attr| {
                attr.path()
                    .get_ident()
                    .map(|ident| {
                        ident != INPUT_ATTR && ident != OUTPUT_ATTR && ident != LAYOUTER_ATTR
                    })
                    .unwrap_or(true)
            });
            FnArg::Typed(pat_type)
        }
        other => other,
    })
}

/// Possible attributes for the arguments of the annotated function.
#[derive(Copy, Clone, Eq, PartialEq)]
enum ArgAttributes {
    Input,
    Output,
    InputOutput,
    Layouter,
}

impl ArgAttributes {
    fn try_combine(self, other: Self) -> Result<Self, (Self, Self)> {
        use ArgAttributes::*;
        match (self, other) {
            (Input, Input) => Ok(Input),
            (Output, Output) => Ok(Output),
            (Input | Output | InputOutput, InputOutput)
            | (InputOutput, Input | Output)
            | (Input, Output)
            | (Output, Input) => Ok(InputOutput),
            (Layouter, Layouter) => Ok(Layouter),
            (Layouter, _) | (_, Layouter) => Err((self, other)),
        }
    }

    fn from_attr(attr: &Attribute) -> Option<Self> {
        // These attributes must be a path with a single segment (i.e. #[input]).
        let ident = attr.path().get_ident()?;
        if ident == INPUT_ATTR {
            return Some(ArgAttributes::Input);
        }
        if ident == OUTPUT_ATTR {
            return Some(ArgAttributes::Output);
        }
        if ident == LAYOUTER_ATTR {
            return Some(ArgAttributes::Layouter);
        }
        None
    }

    fn try_from_attrs(attrs: &[Attribute], span: proc_macro2::Span) -> Option<syn::Result<Self>> {
        attrs
            .iter()
            .filter_map(Self::from_attr)
            .try_fold(None::<Self>, |acc, attr| {
                acc.map(|acc| acc.try_combine(attr)).transpose().map_err(|(lhs, rhs)| {
                    syn::Error::new(span, format!("Incompatible attributes '{lhs}' and '{rhs}'"))
                })
            })
            .transpose()
    }

    fn try_from_arg(arg: &FnArg) -> Option<syn::Result<(Self, &syn::PatType)>> {
        let FnArg::Typed(pat) = arg else {
            return None;
        };
        Some(ArgAttributes::try_from_attrs(&pat.attrs, pat.span())?.map(|attr| (attr, pat)))
    }

    pub fn emit_code(self, pat: &syn::Pat, group_ident: &syn::Ident) -> Option<TokenStream> {
        match self {
            ArgAttributes::Input => Some(quote! { #pat.annotate_as_input(#group_ident)?; }),
            ArgAttributes::Output => Some(quote! { #pat.annotate_as_output(#group_ident)?; }),
            ArgAttributes::InputOutput => Some(
                quote! { #pat.annotate_as_input(#group_ident)?; #pat.annotate_as_output(#group_ident)?;},
            ),
            ArgAttributes::Layouter => None,
        }
    }
}

impl std::fmt::Display for ArgAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgAttributes::Input => "#[input]",
                ArgAttributes::Output => "#[output]",
                ArgAttributes::InputOutput => "#[input] #[output]",
                ArgAttributes::Layouter => "#[layouter]",
            }
        )
    }
}
