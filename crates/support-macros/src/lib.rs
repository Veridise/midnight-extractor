#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, parse_macro_input};

mod decompose;
#[cfg(feature = "extractor-derive")]
mod extractor;
mod group_impl;

/// Creates a group annotation around the body of a function.
///
/// Functions annotated this way must have an argument that implements the
/// layouter trait. By default an argument named `layouter` is considered to be
/// that argument since that's the convention. If the argument has a different
/// name it must be annotated with `#[layouter]` such that the macro can locate
/// it.
///
/// The inputs and outputs of the gruop are derived from the arguments of the
/// function and its return value. The return value of the function is always
/// annotated as an output and arguments can be annotated with `#[input]` and/or
/// `#[output]` to signify the kind of IO they represent.
///
/// Any type that is treated as IO of the group must implement the
/// `DecomposeInCells` trait since the macro will rely on that trait for making
/// the annotations.
///
/// # Example
///
/// ```ignore
/// #[picus::group]
/// fn foo(&self, layouter: &mut impl Layouter<F>, inputs: #[input] &[AssignedNative<F>]) ->
/// Result<AssignedNative<F>, Error> {
///     // The body of this function is now wrapped in a call to `layouter.group()`.
///     inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
///     // The return value is annotated as an output and gets forwarded untouched.
/// }
/// ```
#[proc_macro_attribute]
pub fn group(_: TokenStream, item: TokenStream) -> TokenStream {
    match group_impl::group_impl(parse_macro_input!(item as ItemFn)) {
        Ok(tok) => tok,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

/// Derive macro for the `DecomposeInCells` trait.
///
/// Requires that every inner element implements the trait and unions are
/// currently not supported.
#[proc_macro_derive(DecomposeInCells)]
pub fn derive_decompose_in_cells(input: TokenStream) -> TokenStream {
    decompose::derive_decompose_in_cells_impl(parse_macro_input!(input as DeriveInput)).into()
}

/// Derive macro for the `NoChipArgs` trait.
#[cfg(feature = "extractor-derive")]
#[proc_macro_derive(NoChipArgs, attributes(support_module))]
pub fn derive_no_chip_args(input: TokenStream) -> TokenStream {
    match extractor::derive_no_chip_args_impl(parse_macro_input!(input as DeriveInput)) {
        Ok(tok) => tok,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derive macro for the `CircuitInitialization` trait that leverages an implementation of
/// `FromScratch`
///
/// The trait is referred to as `crate::testing_utils::FromScratch` by the
/// macro and therefore this derive macro can only be used in the `midnight-circuits` crate. The macro requires that the
/// type has at least one type parameter that implements the `ff::PrimeField` trait and tries to search for it. For circumstances
/// where the macro can't figure it out the macro includes a helper attribute for setting what type should be used as the field.
/// In addition, to support when the type has parameters that require implementing `FromScratch` the macro has another helper for annotating
/// this requirements. To see how these helpers work check the examples below.
///
/// # Examples
///
/// ```ignore
/// // Basic case without any necessary configuration
/// #[derive(InitFromScratch)]
/// struct ChipA<F: Field> { ... }
///
/// // Configuring the field
/// trait Foo { type Bar: ff::PrimeField; }
/// #[derive(InitFromScratch)]
/// #[field(C::Bar)]
/// struct ChipB<C: Foo> { ... }
///
/// // Annotating required implementations of FromScratch
/// // ChipC<F, D> only implements FromScratch if D implements it.
/// #[derive(InitFromScratch)]
/// #[from_scratch(D)]
/// struct ChipC<F: Field, D> { ... }
/// ```
#[cfg(feature = "extractor-derive")]
#[proc_macro_derive(InitFromScratch, attributes(field, from_scratch))]
pub fn derive_circuit_initialization_from_scratch(input: TokenStream) -> TokenStream {
    match extractor::derive_circuit_initialization_from_scratch_impl(parse_macro_input!(
        input as DeriveInput
    )) {
        Ok(tok) => tok,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
