# Supporting new assigned types

In order to use user defined types as inputs or outputs in a harness it is necessary to implement some traits.
These traits make the type compatible with the extraction system.

Let's do a small refresher on why this is necessary. A harness can be defined as follows.

```rust
#[entry("control-flow/select/native/native")]
#[harness]
pub fn select_native(
    chip: &NativeChip<F>,
    layouter: &mut impl Layouter<F>,
    (cond, a, b): (AssignedBit<F>, AssignedNative<F>, AssignedNative<F>),
) -> Result<AssignedNative<F>, Error> {
    chip.select(layouter, &cond, &a, &b)
}
```

In this example the type `(AssignedBit<F>, AssignedNative<F>, AssignedNative<F>)` is the type 
of the input and `AssignedNative<F>` is the type of the output. When lowering to Picus we cannot use 
this types directly because PCL does not support user defined types and everything is a felt. The extractor 
works around this issue by creating two auxiliary tables (one for inputs and another for outputs) that represent 
the inputs and outputs of the circuit as individual cells. In the example above we need 3 cells in the input auxiliary 
table and 1 cells in the output auxiliary table. We know this because `AssignedNative<F>` is an alias over Halo2's 
`AssignedCell<F,F>` and `AssignedBit<F>` wraps an `AssignedNative<F>`.

As a running example, we are going to add support to the following type as we explain the concepts.

```rust
struct AssignedFoo<F> {
    foo: [AssignedNative<F>; 10],
    bar: AssignedBit<F>
}
```

In order for the extractor to know how many cells a type occupies we have the [`CellReprSize`](https://docs.rs/mdnt-support/latest/mdnt_support/cells/trait.CellReprSize.html) trait from the `mdnt-support` crate. 
This trait has a constant `usize` that declares the amount of cells required. Types such as `AssignedCell<F,F>` or `[T;N]` already 
implement this trait, so we can leverage that for implementing the trait for `AssignedFoo`.

```rust
// The impl of this trait for AssignedCell shows that it, as expected, only takes up one cell.
impl<V, F> CellReprSize for AssignedCell<V, F> {
    const SIZE: usize = 1;
}

// For our type we can leverage the existing implementations.
impl<F> CellReprSize for AssignedFoo<F> {
    const SIZE: usize = <[AssignedNative<F>; 10]>::SIZE + <AssignedBit<F>>::SIZE; // Total 11 cells.
}
```

This trait requires that the type has a known size and won't work well with types that have dynamically sized types such as `Vec`.
The following pattern can be used to work around this limitation.

```rust
/// Dynamically sized version of AssignedFoo
struct AssignedFooDyn<F> {
    foo: Vec<AssignedNative<F>>,
    bar: AssignedBit<F>
}

/// We create a new type that has a known length.
struct LoadedFooDyn<F, const N: usize> {
    foo: AssignedFooDyn<F>,
    _marker: PhantomData<[(); N]>
}

impl<F, const N: usize> From<LoadedFooDyn<F, N>> for AssignedFooDyn<F> {
    fn from(value: LoadedFooDyn<F, N>) -> Self {
        value.foo
    }
}

// And implement the trait on this new type wrapper. The other traits that we will see 
// below need to be implemented on this trait as well.
impl<F, const N: usize> CellReprSize for LoadedFooDyn<F, N> {
    const SIZE: usize = <AssignedNative<F>>::SIZE * N + <AssignedBit<F>>::SIZE; // Total N+1 cells.
}
```

Now that we have the trait that declares the size of the type we can add support for reading, writing, or both.
For using the type as an input in a harness we need to implement [`LoadFromCells`](https://docs.rs/mdnt-support/latest/mdnt_support/cells/load/trait.LoadFromCells.html). And for using the type as an output 
we need to implement [`StoreIntoCells`](https://docs.rs/mdnt-support/latest/mdnt_support/cells/store/trait.StoreIntoCells.html). Both traits have 4 type parameters: `F`, `C`, `E`, and `L`. `F` is the type of the 
field and needs to implement the `ff::PrimeField` trait. `C` is the chip we are using for the harness. In the example at the 
beginning of this chapter it would be `NativeChip`. `E` is the extraction adaptor, which is a type that has a set of 
associated types representing different Halo2 concepts. `E` must implement the [`Halo2Types`](https://docs.rs/mdnt-support/latest/mdnt_support/trait.Halo2Types.html) trait in `mdnt-support`. Last, `L` does not necessarily need to implement any trait in order to satisfy either 
`LoadFromCells` or `StoreIntoCells`. However, some implementations may require an instance of a `Layouter` and `L` can be constrained 
to that trait (`L: Layouter<F>`).

```rust 
impl<F, L, C, E> LoadFromCells<F, C, E, L> for AssignedFoo<F>
where 
    F: PrimeField,
    E: Halo2Types<F>
{
    fn load(
        ctx: &mut ICtx<F, E>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, E, Adaptee = L>,
        injected_ir: &mut InjectedIR<E::RegionIndex, E::Expression>,
    ) -> Result<Self, E::Error> {
        // Inside this method we need to construct an instance of the type.
        Ok(Self {
            // We can leverage the existing implementations of the trait.
            foo: ctx.load(chip, layouter, injected_ir)?,
            bar: ctx.load(chip, layouter, injected_ir)?,
        })
    }
}

impl<F, L, C, E> StoreIntoCells<F, C, E, L> for AssignedFoo<F>
where 
    F: PrimeField,
    E: Halo2Types<F>
{
    fn store(
        self,
        ctx: &mut OCtx<F, E>,
        chip: &C,
        layouter: &mut impl LayoutAdaptor<F, E, Adaptee = L>,
        injected_ir: &mut InjectedIR<E::RegionIndex, E::Expression>,
    ) -> Result<(), E::Error> {
        // Inside this method we need to convert the type into a set of cells.
        // Again, we can leverage existing implementations.
        self.foo.store(ctx, chip, layouter, injected_ir)?;
        self.bar.store(ctx, chip, layouter, injected_ir)
    }
}
```

Inside those traits the programmer has the ability of performing the conversion in different ways.
The `ctx` gives access to the raw cells. The `chip` and `layouter` allow performing the conversion 
via some method the chip implements. For example, because the target chip implements a trait that 
already has the desired logic (i.e. adding `C: ThatTrait` to the where bounds). Lastly, `injected_ir` allows 
writing [Haloumi IR](https://docs.rs/haloumi-ir/latest/haloumi_ir/) directly for encoding semantic properties of the type. For example, the implementations for 
`AssignedBit` and `AssignedByte` in `midnight-circuits` emits IR that constraints the value to be within the 
range of the types; 0-1 and 0-255 respetively.
