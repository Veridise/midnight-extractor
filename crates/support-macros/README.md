# mdnt-support-macros

Convenience macros for declaring a group out of a function declaration.

## Installation 

The generated code requires that the `mdnt-groups-support` crate is renamed to `picus-support`.
And, if the `extractor-derive` feature is enabled then the `mdnt-support` crate needs to be renamed to 
`extractor-support`.

You can add these dependencies as follows:

```text
[dependencies.picus]
version = "..."
package = "mdnt-support-macros"
# Optional features
features = ["extractor-derive"]

[dependencies.picus-support]
version = "..."
package = "mdnt-groups-support"

# If `extractor-derive` is enabled
[dependencies.extractor-support]
version = "..."
package = "mdnt-support"
```

## Usage 

### `#[picus::group]`

The macro `#[picus::group]` creates a group annotation around the body of a function.
The macro doesn't actually do anything unless the `region-groups` feature is enabled.
This way it is compatible with halo2 implementations that do not support the region groups API.
If the feature is disabled (is on by default) then the macro simply cleans up its helper attributes 
and leaves the original function untouched.

Functions annotated this way must have an argument that implements the
layouter trait. By default an argument named `layouter` is considered to be
that argument since that's the convention. If the argument has a different
name it must be annotated with `#[layouter]` such that the macro can locate
it.

The inputs and outputs of the gruop are derived from the arguments of the
function and its return value. The return value of the function is always
annotated as an output and arguments can be annotated with `#[input]` and/or
`#[output]` to signify the kind of IO they represent.

Any type that is treated as IO of the group must implement the
`DecomposeIn<Cell>` trait since the macro will rely on that trait for making
the annotations. Where `Cell` is the type `midnight_proofs::circuit::Cell`. 

### Derive macros

The crate also contains some derive macros: `DecomposeInCells`, `NoChipArgs`, and `InitFromScratch`. 
The last two are guarded by the `extractor-derive` feature flag.

`DecomposeInCells` implements the `picus_support::DecomposeIn` trait using `midnight_proofs::circuit::Cell` as the cell.
Only structs and enums are supported.

`NoChipArgs` implements the `extractor_support::circuit::NoChipArgs` marker trait. For more details about this trait check
its documentation.

`InitFromScratch` implements the `extractor_support::circuit::CircuitInitialization` leveraging an implementation of
`midnight_circuits::testing_utils::FromScratch`. That trait is referred as `crate::testing_utils::FromScratch` by the 
macro and therefore this derive macro can only be used in the `midnight-circuits` crate. 

The macro requires that the type has at least one type parameter that implements the `ff::PrimeField` trait and tries to search for it.
For circumstances where the macro can't figure it out, includes a helper attribute for setting the type that should be used as the field.
In addition, to support when the type has parameters that require implementing `FromScratch` the macro has another helper for annotating 
this requirements. To see how these helpers work check the examples below.

### Examples

Example of `#[picus::group]`.

```ignore
#[picus::group]
fn foo(&self, layouter: &mut impl Layouter<F>, inputs: #[input] &[AssignedNative<F>]) ->
Result<AssignedNative<F>, Error> {
    // The body of this function is now wrapped in a call to `layouter.group()`.
    inputs.iter().try_fold(F::ZERO, |acc, i| self.bar(layouter, i, acc))
    // The return value is annotated as an output and gets forwarded untouched.
}
```

Examples of `derive(InitFromScratch)`

```ignore
// Basic case without any necessary configuration
#[derive(InitFromScratch)]
struct ChipA<F: Field> { ... }

// Configuring the field 
trait Foo { type Bar: ff::PrimeField; }
#[derive(InitFromScratch)]
#[field(C::Bar)]
struct ChipB<C: Foo> { ... }

// Annotating required implementations of FromScratch
// ChipC<F, D> only implements FromScratch if D implements it.
#[derive(InitFromScratch)]
#[from_scratch(D)]
struct ChipC<F: Field, D> { ... }
```


