use quote::ToTokens;
use syn::{
    parse::Parse, AngleBracketedGenericArguments, Attribute, Block, FnArg, GenericArgument,
    Generics, Ident, ItemFn, Pat, PatType, Path, PathArguments, ReturnType, Type, TypeParamBound,
    TypePath, Visibility,
};

use crate::error::{Error, ErrorType};

struct HarnessFnCommon {
    chip: ArgParts,
    layouter: Pat,
    injected_ir: Option<Pat>,
    attrs: Vec<Attribute>,
    block: Block,
    vis: Visibility,
    ident: Ident,
    field_ty: Type,
    generics: Generics,
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

impl HarnessFn {
    pub fn vis(&self) -> &Visibility {
        &self.common.vis
    }

    pub fn ident(&self) -> &Ident {
        &self.common.ident
    }

    pub fn block(&self) -> &Block {
        &self.common.block
    }

    pub fn attrs(&self) -> &[Attribute] {
        &self.common.attrs
    }

    pub fn field_ty(&self) -> &Type {
        &self.common.field_ty
    }

    pub fn chip_ty(&self) -> &Type {
        &self.common.chip.ty
    }

    pub fn chip_pat(&self) -> &Pat {
        &self.common.chip.pat
    }

    pub fn layouter_pat(&self) -> &Pat {
        &self.common.layouter
    }

    pub fn input_ty(&self) -> &Type {
        &self.input.ty
    }

    pub fn input_pat(&self) -> &Pat {
        &self.input.pat
    }

    pub fn output_ty(&self) -> &Output {
        &self.output
    }

    pub fn generics(&self) -> &Generics {
        &self.common.generics
    }

    pub fn injected_ir(&self) -> Option<&Pat> {
        self.common.injected_ir.as_ref()
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
            common: HarnessFnCommon {
                chip,
                layouter,
                injected_ir,
                attrs: f.attrs,
                block: *f.block,
                vis: f.vis,
                ident: f.sig.ident,
                generics: f.sig.generics,
                field_ty,
            },
            input,
            output,
        })
    }
}

pub struct UnitHarnessFn {
    common: HarnessFnCommon,
    input: ArgParts,
    output: ArgParts,
}

impl UnitHarnessFn {
    pub fn vis(&self) -> &Visibility {
        &self.common.vis
    }

    pub fn ident(&self) -> &Ident {
        &self.common.ident
    }

    pub fn block(&self) -> &Block {
        &self.common.block
    }

    pub fn attrs(&self) -> &[Attribute] {
        &self.common.attrs
    }

    pub fn field_ty(&self) -> &Type {
        &self.common.field_ty
    }

    pub fn chip_ty(&self) -> &Type {
        &self.common.chip.ty
    }

    pub fn chip_pat(&self) -> &Pat {
        &self.common.chip.pat
    }

    pub fn layouter_pat(&self) -> &Pat {
        &self.common.layouter
    }

    pub fn generics(&self) -> &Generics {
        &self.common.generics
    }

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

    pub fn injected_ir(&self) -> Option<&Pat> {
        self.common.injected_ir.as_ref()
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
            common: HarnessFnCommon {
                chip,
                layouter,
                injected_ir,
                attrs: f.attrs,
                block: *f.block,
                vis: f.vis,
                ident: f.sig.ident,
                generics: f.sig.generics,
                field_ty,
            },
            input,
            output,
        })
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
