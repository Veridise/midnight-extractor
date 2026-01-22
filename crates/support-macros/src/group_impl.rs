use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    spanned::Spanned, Attribute, Block, FnArg, Ident, ItemFn, Pat, PatType, ReturnType, Visibility,
};
#[cfg(feature = "region-groups")]
use syn::{ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::parse::group::GroupArgs;

const INPUT_ATTR: &str = "input";
const OUTPUT_ATTR: &str = "output";
const LAYOUTER_ATTR: &str = "layouter";

/// Internal implementation of [`super::group`].
///
/// Refer to that macro's documentation for details about its user facing API.
pub fn group_impl(input_fn: ItemFn, args: GroupArgs) -> syn::Result<TokenStream> {
    let fn_ident = &input_fn.sig.ident;
    let (impl_generics, ty_generics, where_clause) = input_fn.sig.generics.split_for_impl();
    let group_ident = format_ident!("__{}__group", fn_ident);

    let (layouter, io) = locate_attributes(&input_fn)?;
    log::debug!("layouter = {layouter:?}");
    log::debug!("io = {io:?}");
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
        args.crate_name(),
        &impl_generics,
        &ty_generics,
        where_clause,
    ))
}

#[allow(clippy::too_many_arguments)]
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
    support_crate: Path,
    impl_generics: &ImplGenerics,
    _ty_generics: &TypeGenerics,
    where_clause: Option<&WhereClause>,
) -> TokenStream {
    quote! {
        #(#fn_attrs)*
        #vis fn  #fn_ident #impl_generics (#(#cleaned_inputs, )*) #output #where_clause {

            #layouter.group(|| stringify!(#fn_ident), midnight_proofs::default_group_key!(), |#layouter,#[allow(non_snake_case)] #group_ident| {
                use #support_crate::DecomposeIn as _;
                #(#io_annotations)*
                let inner_result = #user_block;
                #group_ident.annotate_as_output(&inner_result)?;
                inner_result
            })
        }
    }
}

/// If region-groups is disabled we emit the function as is but with the inputs cleaned to avoid
/// errors with the attributes.
#[allow(clippy::too_many_arguments)]
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
    impl_generics: &ImplGenerics,
    _ty_generics: &TypeGenerics,
    where_clause: Option<&WhereClause>,
) -> TokenStream {
    quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident #impl_generics (#(#cleaned_inputs, )*) #output #where_clause {
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
        log::debug!("   Creating ArgAttributes from attr = {attr:?}");
        // These attributes must be a path with a single segment (i.e. #[input]).
        let ident = attr.path().get_ident()?;
        log::debug!("   Attribute is an identifier: {ident}");
        if ident == INPUT_ATTR {
            log::debug!("     Creating Input");
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
                Ok(match acc {
                    Some(acc) => Some(acc.try_combine(attr).map_err(|(lhs, rhs)| {
                        syn::Error::new(
                            span,
                            format!("Incompatible attributes '{lhs}' and '{rhs}'"),
                        )
                    })?),
                    None => Some(attr),
                })
            })
            .transpose()
    }

    fn try_from_arg(arg: &FnArg) -> Option<syn::Result<(Self, &syn::PatType)>> {
        log::debug!("Creating ArgAttributes from arg = {arg:?}");
        let FnArg::Typed(pat) = arg else {
            return None;
        };
        Some(ArgAttributes::try_from_attrs(&pat.attrs, pat.span())?.map(|attr| (attr, pat)))
    }

    pub fn emit_code(self, pat: &syn::Pat, group_ident: &syn::Ident) -> Option<TokenStream> {
        match self {
            ArgAttributes::Input => Some(quote! { #group_ident.annotate_as_input(&#pat)?; }),
            ArgAttributes::Output => Some(quote! { #group_ident.annotate_as_output(&#pat)?; }),
            ArgAttributes::InputOutput => Some(
                quote! { #group_ident.annotate_as_input(&#pat)?; #group_ident.annotate_as_output(&#pat)?;},
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

#[cfg(feature = "region-groups")]
#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use macro_expand::Context;
    use rstest::{fixture, rstest};
    use simplelog::{Config, TestLogger};

    macro_rules! _try {
        ($p:expr, $s:expr) => {
            $p.map_err(|err| ParseError::new(err, stringify!($s))).unwrap()
        };
    }

    #[fixture]
    fn ctx() -> Context<'static> {
        let _ = TestLogger::init(LevelFilter::Debug, Config::default());
        let mut ctx = Context::new();
        ctx.register_proc_macro_attribute("group".into(), |input, attr| {
            group_impl(input, _try!(syn::parse2(attr), attr)).unwrap()
        });

        ctx
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct ParseError {
        err: syn::Error,
        ctx: &'static str,
        span: Span,
    }

    impl ParseError {
        fn new(err: impl Into<syn::Error>, ctx: &'static str) -> Self {
            let err = err.into();
            Self {
                ctx,
                span: err.span(),
                err,
            }
        }
    }

    macro_rules! parse {
        ($s:expr) => {{
            let ts: proc_macro2::TokenStream = _try!($s.parse(), $s);
            ts
        }};
    }

    macro_rules! unparse {
        ($ts:expr) => {
            prettyplease::unparse(&_try!(syn::parse2($ts), $ts))
        };
    }

    #[rstest]
    #[case(
        r"#[group]
        fn foo( layouter: &mut impl Layouter<F>, #[input] inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
        }
        ",
        r"
        fn foo( layouter: &mut impl Layouter<F>, inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            layouter.group(
                || stringify!(foo),
                midnight_proofs::default_group_key!(),
                |layouter, #[allow(non_snake_case)] __foo__group| {
                    use picus_support::DecomposeIn as _;
                    __foo__group.annotate_as_input(&inputs)?;
                    let inner_result = {
                        inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
                    };
                    __foo__group.annotate_as_output(&inner_result)?;
                    inner_result
                }
            )
        }
        "
    )]
    #[case(
        r"#[group]
        fn foo<const M: usize>( layouter: &mut impl Layouter<F>, #[input] inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
        }
        ",
        r"
        fn foo<const M: usize>( layouter: &mut impl Layouter<F>, inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            layouter.group(
                || stringify!(foo),
                midnight_proofs::default_group_key!(),
                |layouter, #[allow(non_snake_case)] __foo__group| {
                    use picus_support::DecomposeIn as _;
                    __foo__group.annotate_as_input(&inputs)?;
                    let inner_result = {
                        inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
                    };
                    __foo__group.annotate_as_output(&inner_result)?;
                    inner_result
                }
            )
        }
        "
    )]
    #[case::different_crate(
        r"#[group(crate = ::other)]
        fn foo( layouter: &mut impl Layouter<F>, #[input] inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
        }
        ",
        r"
        fn foo( layouter: &mut impl Layouter<F>, inputs:  &[AssignedNative<F>]) -> Result<AssignedNative<F>, Error> {
            layouter.group(
                || stringify!(foo),
                midnight_proofs::default_group_key!(),
                |layouter, #[allow(non_snake_case)] __foo__group| {
                    use ::other::DecomposeIn as _;
                    __foo__group.annotate_as_input(&inputs)?;
                    let inner_result = {
                        inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
                    };
                    __foo__group.annotate_as_output(&inner_result)?;
                    inner_result
                }
            )
        }
        "
    )]
    fn derive_macro_test(ctx: Context, #[case] input: &str, #[case] expected: &str) {
        log::debug!("Parsing input:\n{input}");
        let input = parse!(input);
        log::debug!("Parsed input:\n{input}");
        log::debug!("Parsing expected:\n{expected}");
        let expected = unparse!(parse!(expected));
        log::debug!("Transforming input");
        let output = ctx.transform(input);
        let formatted = unparse!(output);
        log::debug!("Produced output:\n{formatted}");
        similar_asserts::assert_eq!(expected, formatted);
    }
}
