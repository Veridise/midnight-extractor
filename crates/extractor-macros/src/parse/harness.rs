use std::ops::Deref;

use quote::ToTokens;
use syn::{
    parse::Parse, spanned::Spanned as _, AngleBracketedGenericArguments, Attribute, Block, FnArg,
    GenericArgument, GenericParam, Generics, Ident, ItemFn, Lifetime, LifetimeParam, Pat, PatType,
    Path, PathArguments, ReturnType, Type, TypeParamBound, TypePath, Visibility,
};

use crate::error::{Error, ErrorType};

pub struct HarnessFnCommon {
    chip: ArgParts,
    layouter: Pat,
    injected_ir: Option<Pat>,
    attrs: Vec<Attribute>,
    block: Block,
    vis: Visibility,
    ident: Ident,
    field_ty: Type,
    generics: Generics,
    extra_lifetimes: Option<(Lifetime, Lifetime)>,
}

impl HarnessFnCommon {
    #[allow(clippy::too_many_arguments)]
    fn new(
        chip: ArgParts,
        layouter: Pat,
        injected_ir: Option<Pat>,
        attrs: Vec<Attribute>,
        block: Block,
        vis: Visibility,
        ident: Ident,
        field_ty: Type,
        generics: Generics,
    ) -> Self {
        Self {
            chip,
            layouter,
            injected_ir,
            attrs,
            block,
            vis,
            ident,
            field_ty,
            generics,
            extra_lifetimes: None,
        }
    }

    pub fn vis(&self) -> &Visibility {
        &self.vis
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    pub fn field_ty(&self) -> &Type {
        &self.field_ty
    }

    pub fn chip_ty(&self) -> &Type {
        &self.chip.ty
    }

    pub fn chip_pat(&self) -> &Pat {
        &self.chip.pat
    }

    pub fn layouter_pat(&self) -> &Pat {
        &self.layouter
    }

    pub fn generics(&self) -> &Generics {
        &self.generics
    }

    pub fn injected_ir(&self) -> Option<&Pat> {
        self.injected_ir.as_ref()
    }

    pub fn extra_lifetimes(&self) -> (&Lifetime, &Lifetime) {
        self.extra_lifetimes.as_ref().map(|(a, b)| (a, b)).unwrap()
    }

    fn inject_extra_lifetimes(&mut self) {
        assert!(self.extra_lifetimes.is_none());
        let lifetime1 = Lifetime::new("'__1", self.generics.span());
        let lifetime2 = Lifetime::new("'__2", self.generics.span());
        self.extra_lifetimes = Some((lifetime1.clone(), lifetime2.clone()));
        self.generics.params.push(GenericParam::Lifetime(LifetimeParam::new(lifetime1)));
        self.generics.params.push(GenericParam::Lifetime(LifetimeParam::new(lifetime2)));
    }
}

struct ArgParts {
    pat: Pat,
    ty: Type,
}

pub enum Output {
    // -> ()
    Void,
    // -> Result<O, E>
    Result { ok: Box<Type>, err: Box<Type> },
}

pub struct HarnessFn {
    common: HarnessFnCommon,
    input: ArgParts,
    output: Output,
}

impl Deref for HarnessFn {
    type Target = HarnessFnCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl HarnessFn {
    pub fn input_ty(&self) -> &Type {
        &self.input.ty
    }

    pub fn input_pat(&self) -> &Pat {
        &self.input.pat
    }

    pub fn output_ty(&self) -> &Output {
        &self.output
    }

    fn with_extra_lifetimes(mut self) -> Self {
        self.common.inject_extra_lifetimes();
        self
    }
}

impl Parse for HarnessFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let f = ItemFn::parse(input)?;
        Ok(f.try_into()?)
    }
}

impl TryFrom<ItemFn> for HarnessFn {
    type Error = Error;

    fn try_from(f: ItemFn) -> Result<Self, Self::Error> {
        let inputs = f.sig.inputs;
        if !(3..=4).contains(&inputs.len()) {
            return Err(Error(
                ErrorType::UnexpectedParams {
                    from: 3,
                    to: 4,
                    lst: "chip, layouter, input[, injected_ir]",
                },
                inputs.into_token_stream(),
            ));
        }
        let mut inputs = inputs.into_iter();
        let chip = {
            let ArgParts {
                pat,
                ty: Type::Reference(syn::TypeReference { elem, .. }),
            } = get_arg(inputs.next().unwrap(), ensure_ref)?
            else {
                unreachable!()
            };
            ArgParts { pat, ty: *elem }
        };
        let layouter = get_arg(inputs.next().unwrap(), ensure_ref_mut)?;
        let field_ty = extract_field(&layouter.ty)?;
        let layouter = layouter.pat;
        let input = get_arg(inputs.next().unwrap(), |_| Ok(()))?;
        let injected_ir = inputs
            .next()
            .map(|arg| get_arg(arg, ensure_ref_mut))
            .transpose()?
            .map(|i| i.pat);
        let output = get_output(f.sig.output)?;
        Ok(Self {
            common: HarnessFnCommon::new(
                chip,
                layouter,
                injected_ir,
                f.attrs,
                *f.block,
                f.vis,
                f.sig.ident,
                field_ty,
                f.sig.generics,
            ),
            input,
            output,
        }
        .with_extra_lifetimes())
    }
}

pub struct UnitHarnessFn {
    common: HarnessFnCommon,
    input: ArgParts,
    output: ArgParts,
}

impl Deref for UnitHarnessFn {
    type Target = HarnessFnCommon;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl UnitHarnessFn {
    pub fn input_ty(&self) -> &Type {
        &self.input.ty
    }

    pub fn input_pat(&self) -> &Pat {
        &self.input.pat
    }

    pub fn output_ty(&self) -> &Type {
        &self.output.ty
    }

    pub fn output_pat(&self) -> &Pat {
        &self.output.pat
    }

    fn with_extra_lifetimes(mut self) -> Self {
        self.common.inject_extra_lifetimes();
        self
    }
}

impl Parse for UnitHarnessFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let f = ItemFn::parse(input)?;
        Ok(f.try_into()?)
    }
}

impl TryFrom<ItemFn> for UnitHarnessFn {
    type Error = Error;

    fn try_from(f: ItemFn) -> Result<Self, Self::Error> {
        let inputs = f.sig.inputs;
        if !(4..=5).contains(&inputs.len()) {
            return Err(Error(
                ErrorType::UnexpectedParams {
                    from: 4,
                    to: 5,
                    lst: "chip, layouter, input, output[, injected_ir]",
                },
                inputs.into_token_stream(),
            ));
        }
        let mut inputs = inputs.into_iter();
        let chip = {
            let ArgParts {
                pat,
                ty: Type::Reference(syn::TypeReference { elem, .. }),
            } = get_arg(inputs.next().unwrap(), ensure_ref)?
            else {
                unreachable!()
            };
            ArgParts { pat, ty: *elem }
        };
        let layouter = get_arg(inputs.next().unwrap(), ensure_ref_mut)?;
        let field_ty = extract_field(&layouter.ty)?;
        let layouter = layouter.pat;
        let input = get_arg(inputs.next().unwrap(), |_| Ok(()))?;
        let output = get_arg(inputs.next().unwrap(), |_| Ok(()))?;
        let injected_ir = inputs
            .next()
            .map(|arg| get_arg(arg, ensure_ref_mut))
            .transpose()?
            .map(|i| i.pat);
        Ok(Self {
            common: HarnessFnCommon::new(
                chip,
                layouter,
                injected_ir,
                f.attrs,
                *f.block,
                f.vis,
                f.sig.ident,
                field_ty,
                f.sig.generics,
            ),
            input,
            output,
        }
        .with_extra_lifetimes())
    }
}

fn ensure_ref(ty: &Type) -> Result<(), Error> {
    matches!(ty, Type::Reference(_))
        .then(|| ())
        .ok_or_else(|| Error(ErrorType::TypeNotReference, ty.to_token_stream()))
}

fn ensure_ref_mut(ty: &Type) -> Result<(), Error> {
    matches!(
        ty,
        Type::Reference(syn::TypeReference {
            mutability: Some(_),
            ..
        })
    )
    .then(|| ())
    .ok_or_else(|| Error(ErrorType::TypeNotMutReference, ty.to_token_stream()))
}

fn get_arg(
    arg: FnArg,
    validate_type: impl Fn(&Type) -> Result<(), Error>,
) -> Result<ArgParts, Error> {
    let FnArg::Typed(PatType { pat, ty, .. }) = arg else {
        return Err(Error(ErrorType::UnexpectedSelf, arg.to_token_stream()));
    };
    validate_type(&ty)?;
    Ok(ArgParts { pat: *pat, ty: *ty })
}

fn extract_field(ty: &Type) -> Result<Type, Error> {
    let Type::Reference(syn::TypeReference { elem, .. }) = ty else {
        extract_field_err(ty, "Was expecting a reference type")?
    };
    let Type::ImplTrait(syn::TypeImplTrait { bounds, .. }) = &**elem else {
        extract_field_err(ty, "Was expecting an implicit type parameter")?
    };
    let GenericArgument::Type(ty) = bounds
        .iter()
        .find_map(|bound| {
            let TypeParamBound::Trait(bound) = bound else {
                return None;
            };
            let args = type_params_from_path(&bound.path, "Layouter")?;

            (args.len() == 1).then_some(args)?.first().copied()
        })
        .ok_or_else(|| {
            Error(
                ErrorType::FailedExtractLayouterParam("Was expecting trait Layouter<F>"),
                bounds.to_token_stream(),
            )
        })?
    else {
        extract_field_err(bounds, "Was expecting F in Layouter<F> to be a type")?
    };

    Ok(ty.clone())
}

fn extract_field_err<T>(t: impl ToTokens, u: &'static str) -> Result<T, Error> {
    Err(Error(
        ErrorType::FailedExtractLayouterParam(u),
        t.to_token_stream(),
    ))
}

fn generic_arg(arg: &GenericArgument, idx: usize) -> Result<Type, Error> {
    let GenericArgument::Type(ty) = arg else {
        return Err(Error(
            ErrorType::ExpectedTypeInGeneric(idx),
            arg.to_token_stream(),
        ));
    };
    Ok(ty.clone())
}

/// Returns the angle bracketed arguments of the type if the end of the path matches the given
/// name.
fn type_params_from_path<'p>(path: &'p Path, name: &str) -> Option<Vec<&'p GenericArgument>> {
    let seg = path.segments.iter().next_back()?;
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
        (seg.ident == name).then_some(&seg.arguments)?
    else {
        return None;
    };
    Some(args.iter().collect())
}

/// Tries to get [`std::result::Result`].
fn try_parse_result_type(path: TypePath) -> Result<Output, Error> {
    let (ok, err) = type_params_from_path(&path.path, "Result")
        .and_then(|args| match &args[..] {
            [ok, err] => Some((*ok, *err)),
            _ => None,
        })
        .ok_or_else(|| Error(ErrorType::IncorrectReturnType, path.to_token_stream()))?;
    let ok = generic_arg(ok, 0)?;
    let err = generic_arg(err, 1)?;

    Ok(Output::Result {
        ok: Box::new(ok),
        err: Box::new(err),
    })
}

fn get_output(r: ReturnType) -> Result<Output, Error> {
    let ReturnType::Type(_, ty) = r else {
        return Ok(Output::Void);
    };
    match *ty {
        Type::Path(path) => try_parse_result_type(path),
        other => Err(Error(
            ErrorType::IncorrectReturnType,
            other.to_token_stream(),
        )),
    }
}
