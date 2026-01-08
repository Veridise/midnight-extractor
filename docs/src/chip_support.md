# Supporting new chips

In order to use user defined chips or gadgets in a harness it is necessary to implement a trait
that makes it compatible with the extraction system.

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

This harness targets the `NativeChip`. When the extractor attempts to extract the harness it needs to know 
some things about this chip, but it doesn't know what the `NativeChip` is. This information is obtained via the 
[`CircuitInitialization`](https://docs.rs/mdnt-support/latest/mdnt_support/circuit/trait.CircuitInitialization.html) trait.
This trait, similar in nature to Halo2's `Circuit` trait, enables the extractor to configure and create chips that implement it.

Let's consider the following chip as an example.

```rust 
struct FooConfig {
    advice: [Column<Advice>; ADVICE_WIDTH],
    fixed: [Column<Fixed>; FIXED_WIDTH],
    table: [TableColumn; TABLE_WIDTH]
}

struct FooChip<F> {
    config: FooConfig,
    native: NativeChip<F>
}

impl<F: PrimeField> FooChip<F> {
    fn configure(meta: &mut ConstraintSystem<F>) -> FooConfig {
        // Create an instance of the config... 
    }

    fn new(config: FooConfig, native: NativeChip<F>) -> Self {
        Self { config, native }
    }

    fn load_table(&self, layouter: impl Layouter<F>) -> Result<(), Error> {
        // Loads the lookup table...
    }
}
```

The example chip has a lookup table and depends on `NativeChip`, which already implements the trait we are going to implement.
The trait is parametrized in `L`, that we can use as an implementation of a `Layouter`.

```rust
impl<F, L> CircuitInitialization<L> for FooChip<F>
where
    F: PrimeField,
    L: Layouter<F>,
{
    // Type that is generated from the ConstraintSystem, usually the Chip's config.
    type Config = (FooConfig, <NativeChip<F> as CircuitInitialization<F>>::Config);

    // Any external arguments the chip may need. If the chip does not have any then 
    // an empty tuple is sufficient.
    type Args = ();

    // Any type that is part of the configuration that could be potentially shared with other chips.
    // FooChip does not have any of that in this case so we just put NativeChip's.
    type ConfigCols = <NativeChip<F> as CircuitInitialization<F>>::ConfigCols;

    // These two types are required because mdnt-support does not have a dependency on any 
    // halo2 library. We need to declare what types are used as ConstraintSystem and as Error.
    // Usually these will be the types `<halo2-crate>::plonk::{ConstraintSystem, Error}`.
    type CS = ConstraintSystem<F>;
    type Error = Error;

    /// Initializes a new chip with the given config and arguments, if any.
    fn new_chip((config, native_config): &Self::Config, _: Self::Args) -> Self {
        Self::new(config, NativeChip::new_chip(native_config, ()))
    }

    /// Creates a new configuration.
    fn configure_circuit(
        meta: &mut Self::CS,
        config_cols: &Self::ConfigCols,
    ) -> Self::Config {
        (Self::configure(meta), NativeChip::configure_circuit(meta, config_cols))
    }

    /// Performs any required loading, such as lookup tables.
    /// This method is called by the extractor after the harness' method has been executed.
    fn load_chip(&self, layouter: &mut L, (_, native_config): &Self::Config) -> Result<(), Self::Error> {
        self.load_table(layouter)?;
        self.native.load_chip(layouter, native_config)
    }
}

```

After implementing this trait the type can be used as the _chip_ argument in any harness.
