# Writing new harnesses

Adding more harnesses is done in the `crates/harnesses` crate. The crate is organized by instructions and, 
in some cases, by chips or gadgets that implement the instructions.

Conceptually, a harness is a function that receives a *chip*, a *layouter*, some inputs and returns a `Result`.
In reallity a harness is a function with a more abstract signature but that complexity is usually managed by a macro
that allows for a more declarative approach. However, if the macro is not an option see the section below 
on how to write harnesses from scratch.

## Writing declarative harnesses

A harnesses written declaratively has the following structure.

```rust
// The harnesses are NOT parametrized by a field.
use mdnt_extractor_core::fields::Blstrs as F;

// Sets the harness name of the function below.
#[entry("control-flow/select/native/native")]
// Marks the function as a harness
#[harness]
pub fn select_native(
    // The first argument is the chip. The only requirement is that the type is a &-reference.
    chip: &NativeChip<F>,
    // The second argument is the layouter. The actual layouter variable is not a 
    // `impl Layouter<F>`. This is syntactic sugar for a namesake variable that 
    // implements `midnight_proofs::circuit::Layouter`. So the result is the same.
    layouter: &mut impl Layouter<F>,
    // The third argument are the inputs. The type of the input must implement `LoadFromCells`.
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
    // Optional fourth argument that allows injecting additional IR for aiding verification.
    injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>
    // The output must be a Result<T, E> where T is a type that implements `StoreIntoCells` and 
    // E is `midnight_proofs::plonk::Error`.
) -> Result<AssignedNative<F>, Error> {
    // Runs the target method.
    chip.select(layouter, &cond, &a, &b)
}
```

And that's it. The list of harnesses is defined automatically at compile time and 
can be obtained by calling the `mdnt_harnesses::harnesses` function.
The extractor uses that function for selecting what harnesses need to be extracted.

The macros do a lot of heavy lifting in converting this form to how the harnesses look internally.
The `#[entry("...")]` macro registers the function in the list of harnesses. This registration is accomplised 
with `mdnt_extractor_core::entry!` and in some cases is actually better to use this macro instead of `#[entry]`.
For example, a harness that is parametrized by a constant parameter for selecting the size of some arrays.

```rust 
entry!("bar/example_10/foo/native", example::<10>);
entry!("bar/example_20/foo/native", example::<20>);
#[harness]
pub fn example<const N: usize>(
    chip: &FooChip<F>,
    layouter: &mut impl Layouter<F>,
    arr: [AssignedNative<F>; N]
) -> Result<AssignedNative<F>, Error> {
    chip.foo(layouter, &arr)
}
```

The harness macro family has 6 macros that can be used for declaring harnesses and offer some flexibility for covering most cases.

For most chips (the ones where `ChipArgs::Args == ()`) use the macros `harness`, `harness_mut`, and `unit_harness`. The
first macro is the one we saw above already. `harness_mut` is similar to `harness` but the first argument (the *chip* argument)
must be a `&mut`-reference instead. `unit_harness` is for methods that do have a return value (`Result<(),Error>`). For this 
macro split the inputs of the function into two sets and pass them as the 3rd and 4th argument. The 4th argument can be 
considered the *output* and additional constraints can be injected for aiding Picus with verification. That argument, however,
is not encoded as a Picus output. Is just a separation for readability. In these cases a vacuous output is generated 
that is constrained to be equal to 0.

If for the target chip `ChipArgs::Args != ()` then you need to use `harness_with_args`, `harness_with_args_mut`, and
`unit_harness_with_args`. These macros work identically to their counterparts but require declaring the type of
`ChipArgs::Args` (i.e. `#[harness_with_args(usize)]`). For providing the argument you need to define a function that has
the same name as the harness function followed by `_args` (i.e. `foo` would be `foo_args`). That function cannot take any
arguments and return a value of the declared type.

Since the most common argument type is `usize` we include a convenience macro `#[usize_args(<usize>)]` that automatically creates 
the function with the correct name and returns the value passed as argument to the macro.

All the macros accept an optional argument with an expression containing `LookupCallbacks`. For example, for extracting a 
harness like the first one but for `NativeGadget` instead we need to handle the range lookup that gadget uses. For that we can 
do as follows:

```rust
#[entry("control-flow/select/native-gadget/native")]
// The range_lookup function creates the appropiate handler for the lookup.
#[harness(range_lookup(8))]
pub fn select_native(
    chip: &NG<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}
```

## Writing harnesses from scratch (WIP)

If the macros shown above don't fit the needs of this new harness they can still be defined by hand. Below is an annotated 
version of the kind of code the macros generate that can serve as a starting point for creating a harness from scratch.
 
```rust
entry!("control-flow/select/native-gadget/native", select_native);
fn select_native(ctx: &Ctx) -> anyhow::Result<Output> {
    // The actual logic of the harness needs to be wrapped into a trait.
    // So we need to create a type for that.
    struct Circuit<'s, 'c>(PhantomData<(&'s (), &'c ())>);

    // This trait defines the types that the harness uses. 
    // The extractor will rely on this trait for finding the right types for 
    // creating the right final circuit.
    impl<'s, 'c> AbstractCircuitIO for Circuit<'s, 'c> {
        // The chip type. It must implement CircuitInitialization but is not enforced.
        type Chip = NativeChip<F>;

        // The type of the inputs. Must implement `LoadFromCells` and if the method takes several 
        // they can be grouped in tuples up to 12 elements.
        type Input = (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>);

        // The type of the outputs. Same as the inputs but must implement the 
        // `StoreIntoCells` trait instead.
        type Output = AssignedNative<F>;

        // These two types need to be the same as their namesakes in CircuitInitialization.
        type Config = <NativeChip<F> as CircuitInitialization<ExtractionLayouter<'s, 'c, F>>>::Config;

        type ConfigCols =
            <NativeChip<F> as CircuitInitialization<ExtractionLayouter<'s, 'c, F>>>::ConfigCols;
    }

    // The actual logic is defined with this trait.
    // `harness_mut` will use `AbstractCircuitMut`
    impl AbstractCircuit<F> for Circuit<'_, '_> {
        fn synthesize<L>(
            &self,
            chip: &Self::Chip,
            layouter: &mut L,
            (cond, a, b): Self::Input,
            _: &mut InjectedIR<
                RegionIndex,
                Expression<F>,
            >,
        ) -> anyhow::Result<Self::Output, Error>
        where
            L: Layouter<F>,
        {
            chip.select(layouter, &cond, &a, &b)
        }
    }

    // If the chip arguments is () then this trait can be used.
    impl NoChipArgs for Circuit<'_, '_> {}

    // The type we created and implemented the types for can be passed to this 
    // type. This type handles the linking between the inputs, outputs, and the harness logic.
    let ci: CircuitImpl<'_, F, Circuit, Function> =
        CircuitImpl::new(ctx, Circuit(Default::default()));
    // The first argument of this method is a type that implements the  
    // CircuitSynthesis trait, which is Haloumi's counterpart to the Circuit trait in Halo2.
    // As long as the value passed there meets the interface all the stuff above this line is not 
    // mandatory.
    // The second argument is an optional dyn reference to a LookupCallbacks implementation.
    // These callbacks are invoked when the circuit has lookups for getting the IR that needs to be 
    // generated for handling the lookup.
    ctx.lower_circuit(ci, None)
}

```
