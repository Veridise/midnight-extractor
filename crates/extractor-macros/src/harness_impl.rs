use crate::parse::harness::{HarnessFn, Output, UnitHarnessFn};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens as _};
use syn::{Ident, ImplGenerics, Lifetime, Type, TypeGenerics, WhereClause};

pub mod cfg;

pub use cfg::*;

fn abstract_circuit_io<'g>(
    impl_generics: &ImplGenerics<'g>,
    ty_generics: &TypeGenerics<'g>,
    where_clause: Option<&'g WhereClause>,
    circuit_ty: &Ident,
    chip_ty: &Type,
    input_ty: &Type,
    out_ty: &TokenStream,
    field_ty: &Type,
    (s, c): (&Lifetime, &Lifetime),
) -> TokenStream {
    let ci = quote! {
        <#chip_ty as
            mdnt_support::circuit::CircuitInitialization<
                mdnt_extractor_core::circuit::layouter::ExtractionLayouter<#s, #c, #field_ty>>>
    };
    quote! {
            impl #impl_generics mdnt_extractor_core::circuit::traits::AbstractCircuitIO for #circuit_ty #ty_generics #where_clause {
                type Chip = #chip_ty;
                type Input = #input_ty;
                type Output = #out_ty;
                type Config = #ci::Config;
                type ConfigCols = #ci::ConfigCols;
            }
    }
}

pub fn harness_impl(
    f: HarnessFn,
    cfg: impl HarnessCfg,
    circuit_cfg: CircuitCfg,
) -> syn::Result<TokenStream> {
    let circuit_ty = format_ident!("Circuit");
    let vis = f.vis();
    let fn_ident = f.ident();
    let user_block = f.block();
    let fn_attrs = f.attrs();
    let field_ty = f.field_ty();
    let chip_ty = f.chip_ty();
    let chip_pat = f.chip_pat();
    let layouter_pat = f.layouter_pat();
    let input_ty = f.input_ty();
    let input_pat = f.input_pat();
    let (impl_generics, ty_generics, where_clause) = f.generics().split_for_impl();
    let (out_ty, err_ty) = match f.output_ty() {
        Output::Void => (
            quote! { () },
            quote! { midnight::midnight_proofs::plonk::Error },
        ),
        Output::Result { ok, err } => (quote! { #ok }, quote! { #err }),
    };

    let aux_tokens = cfg.aux_tokens();
    let chip_args = cfg.emit_chip_args_impl(fn_ident, field_ty, f.generics(), &circuit_ty);

    let injected_ir = match f.injected_ir() {
        Some(injected_ir) => quote! { #injected_ir },
        None => quote! { _ },
    };

    let abstract_circuit_trait;
    let synthesize_mthd;
    let chip_ref_ty;
    let tag_ty;
    if circuit_cfg.mut_chip() {
        abstract_circuit_trait = format_ident!("AbstractCircuitMut");
        synthesize_mthd = format_ident!("synthesize_mut");
        chip_ref_ty = quote::quote! { &mut Self::Chip };
        tag_ty = format_ident!("FunctionMut");
    } else {
        abstract_circuit_trait = format_ident!("AbstractCircuit");
        synthesize_mthd = format_ident!("synthesize");
        chip_ref_ty = quote::quote! { &Self::Chip };
        tag_ty = format_ident!("Function");
    }

    let circuit_io = abstract_circuit_io(
        &impl_generics,
        &ty_generics,
        where_clause,
        &circuit_ty,
        chip_ty,
        input_ty,
        &out_ty,
        field_ty,
        f.extra_lifetimes(),
    );
    let (s, c) = f.extra_lifetimes();
    Ok(quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident #impl_generics (ctx: &mdnt_extractor_core::harness::Ctx) -> anyhow::Result<mdnt_extractor_core::harness::Output> #where_clause {
            struct #circuit_ty #impl_generics(
                std::marker::PhantomData<(&#s (), &#c ())>
            );

            #circuit_io

            impl #impl_generics mdnt_extractor_core::circuit::#abstract_circuit_trait<#field_ty> for #circuit_ty #ty_generics #where_clause {
                fn #synthesize_mthd<__L>(&self,
                    #chip_pat : #chip_ref_ty,
                    #layouter_pat : &mut __L,
                    #input_pat : Self::Input,
                    #injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                                    midnight_proofs::circuit::RegionIndex,
                                    midnight_proofs::plonk::Expression< #field_ty>>
                ) -> std::result::Result<Self::Output, #err_ty>
                where __L: midnight_proofs::circuit::Layouter<#field_ty>
                {
                    #user_block
                }
            }
            #chip_args

            let circuit = mdnt_extractor_core::circuit::CircuitImpl::<#field_ty, #circuit_ty #ty_generics,
                mdnt_extractor_core::circuit::#tag_ty>::new(ctx, #circuit_ty(Default::default()));
            ctx.lower_circuit(circuit, #aux_tokens)
        }
    })
}

pub fn unit_harness_impl(f: UnitHarnessFn, cfg: impl HarnessCfg) -> syn::Result<TokenStream> {
    let circuit_ty = format_ident!("Circuit");
    let vis = f.vis();
    let fn_ident = f.ident();
    let user_block = f.block();
    let fn_attrs = f.attrs();
    let field_ty = f.field_ty();
    let chip_ty = f.chip_ty();
    let chip_pat = f.chip_pat();
    let layouter_pat = f.layouter_pat();
    let input_ty = f.input_ty();
    let input_pat = f.input_pat();
    let output_ty = f.output_ty();
    let output_pat = f.output_pat();
    let (impl_generics, ty_generics, where_clause) = f.generics().split_for_impl();

    let aux_tokens = cfg.aux_tokens();
    let chip_args = cfg.emit_chip_args_impl(fn_ident, field_ty, f.generics(), &circuit_ty);

    let injected_ir = match f.injected_ir() {
        Some(injected_ir) => quote! { #injected_ir },
        None => quote! { _ },
    };

    let circuit_io = abstract_circuit_io(
        &impl_generics,
        &ty_generics,
        where_clause,
        &circuit_ty,
        chip_ty,
        &syn::parse2(quote! { (#input_ty, #output_ty) })?,
        &quote! { mdnt_extractor_core::cells::store::FreshVar },
        field_ty,
        f.extra_lifetimes(),
    );

    let (s, c) = f.extra_lifetimes();
    Ok(quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident #impl_generics (ctx: &mdnt_extractor_core::harness::Ctx) -> anyhow::Result<mdnt_extractor_core::harness::Output> #where_clause {
            struct #circuit_ty #impl_generics(
                std::marker::PhantomData<(&#s (), &#c ())>
            );

            #circuit_io

            impl #impl_generics mdnt_extractor_core::circuit::AbstractCircuit<#field_ty> for #circuit_ty #ty_generics #where_clause {
                fn synthesize<__L>(&self,
                    #chip_pat : &Self::Chip,
                    #layouter_pat : &mut __L,
                    (#input_pat, #output_pat) : Self::Input,
                    #injected_ir: &mut mdnt_support::circuit::injected::InjectedIR<
                                    midnight_proofs::circuit::RegionIndex,
                                    midnight_proofs::plonk::Expression< #field_ty>>
                ) -> std::result::Result<Self::Output, midnight_proofs::plonk::Error>
                where __L: midnight_proofs::circuit::Layouter<#field_ty>{
                    {
                        #user_block
                    }?;
                    Ok(mdnt_extractor_core::cells::store::FreshVar)
                }
            }
            #chip_args

            let circuit = mdnt_extractor_core::circuit::CircuitImpl::<#field_ty, #circuit_ty #ty_generics, mdnt_extractor_core::circuit::Function>::new(ctx, #circuit_ty(Default::default()));
            ctx.lower_circuit(circuit, #aux_tokens)
        }
    })
}
