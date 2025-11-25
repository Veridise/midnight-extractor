use proc_macro2::TokenStream;
use quote::quote;

pub mod cfg;

pub use cfg::*;

use crate::parse::harness::{HarnessFn, Output, UnitHarnessFn};

pub fn harness_impl(
    f: HarnessFn,
    cfg: impl HarnessCfg,
    circuit_cfg: CircuitCfg,
) -> syn::Result<TokenStream> {
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
    let chip_args = cfg.emit_chip_args_impl(fn_ident, field_ty, f.generics());

    let injected_ir = match f.injected_ir() {
        Some(injected_ir) => quote! { #injected_ir },
        None => quote! { _ },
    };

    let abstract_circuit_trait;
    let synthesize_mthd;
    let chip_ref_ty;
    let tag_ty;
    if circuit_cfg.mut_chip() {
        abstract_circuit_trait = quote::format_ident!("AbstractCircuitMut");
        synthesize_mthd = quote::format_ident!("synthesize_mut");
        chip_ref_ty = quote::quote! { &mut Self::Chip };
        tag_ty = quote::format_ident!("FunctionMut");
    } else {
        abstract_circuit_trait = quote::format_ident!("AbstractCircuit");
        synthesize_mthd = quote::format_ident!("synthesize");
        chip_ref_ty = quote::quote! { &Self::Chip };
        tag_ty = quote::format_ident!("Function");
    }

    Ok(quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident #impl_generics (ctx: &crate::harness::Ctx) -> anyhow::Result<crate::harness::Output> #where_clause {
            struct Circuit #impl_generics;

            impl #impl_generics crate::circuit::traits::AbstractCircuitIO<#field_ty> for Circuit #ty_generics #where_clause {
                type Chip = #chip_ty;
                type Input = #input_ty;
                type Output = #out_ty;
            }

            impl #impl_generics crate::circuit::#abstract_circuit_trait<#field_ty> for Circuit #ty_generics #where_clause {
                fn #synthesize_mthd(&self,
                    #chip_pat : #chip_ref_ty,
                    #layouter_pat : &mut impl midnight::midnight_proofs::circuit::Layouter<#field_ty>,
                    #input_pat : Self::Input,
                    #injected_ir: &mut crate::circuit::injected::InjectedIR<#field_ty>
                ) -> std::result::Result<Self::Output, #err_ty> {
                    #user_block
                }
            }
            #chip_args

            let circuit = crate::circuit::CircuitImpl::<#field_ty, Circuit #ty_generics, crate::circuit::#tag_ty>::new(ctx, Circuit{});
            ctx.lower_circuit(circuit, #aux_tokens)
        }
    })
}

pub fn unit_harness_impl(f: UnitHarnessFn, cfg: impl HarnessCfg) -> syn::Result<TokenStream> {
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
    let chip_args = cfg.emit_chip_args_impl(fn_ident, field_ty, f.generics());

    let injected_ir = match f.injected_ir() {
        Some(injected_ir) => quote! { #injected_ir },
        None => quote! { _ },
    };

    Ok(quote! {
        #(#fn_attrs)*
        #vis fn #fn_ident #impl_generics (ctx: &crate::harness::Ctx) -> anyhow::Result<crate::harness::Output> #where_clause {
            struct Circuit #impl_generics;

            impl #impl_generics crate::circuit::traits::AbstractCircuitIO<#field_ty> for Circuit #ty_generics #where_clause {
                type Chip = #chip_ty;
                type Input = #input_ty;
                type Output = #output_ty;
            }

            impl #impl_generics crate::circuit::AbstractUnitCircuit<#field_ty> for Circuit #ty_generics #where_clause {
                fn synthesize(&self,
                    #chip_pat : &Self::Chip,
                    #layouter_pat : &mut impl midnight::midnight_proofs::circuit::Layouter<#field_ty>,
                    #input_pat : Self::Input,
                    #output_pat : Self::Output,
                    #injected_ir: &mut crate::circuit::injected::InjectedIR<#field_ty>
                ) -> std::result::Result<(), midnight::midnight_proofs::plonk::Error> {
                    #user_block
                }
            }
            #chip_args

            let circuit = crate::circuit::CircuitImpl::<#field_ty, Circuit #ty_generics, crate::circuit::Procedure>::new(ctx, Circuit{});
            ctx.lower_circuit(circuit, #aux_tokens)
        }
    })
}
