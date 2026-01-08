# Extraction

After [building the tool](./installation.md) you can use the `midnight-extractor` CLI to extract the circuits. 

## Quickstart

To extract all the circuits you can use the `extract-all.sh` script located in the `scripts` directory.
The Picus representation of the extracted circuits can be found in the `picus_files` directory.

## CLI usage

The `midnight-extractor` CLI builds a list of circuits at runtime and extracts them into PCL. 
The CLI has a series of flags and arguments that allows controlling the selection of circuits to extract. The CLI also has
flags for controlling the output produced by the tool.

> Use the `--help` flag to see the full list of options.

### Circuit selection

Each circuit has 4 properties for filtering and selection; **instruction**, **chip**, **type**, and **name**.

The **instruction** groups together circuits that are related and, in general, corresponds to an instruction-like trait 
(i.e. `arithmetic` corresponds to `ArithInstructions` or `ecc` to `EccInstructions`). The instructions are selected as 
positional arguments. Adding instructions to the CLI invocation creates a set of selected instructions. If no 
instructions are passed to the CLI then all instructions are considered.

The **chip** declares what concrete implementation of the **instruction** is used by the circuit. Different chips 
implement the same instruction traits and this parameter allows configuring what chip is going to be targeted.
To select a chip use the `--chip <name>` flag. This will make the tool filter out any circuit that is not implemented 
by the chip. If the flag is not passed then the tool will consider all chips. For ignoring chips instead use the 
`--ignore-chips <chip[,chip,...]>` flag with a comma separated list of chips to ignore.

The **type** declares what high-level type the circuit is using and can be selected with the `--type <name>` flag. 
Similarly to `--chip`, not passing this flag makes the tool consider all types. The circuits distinguish by type 
because some circuits implement the same instructions for multiple types. For example, the `native` chip implements 
the `equality` instructions for the `native` and `bit` types.

The **name** describes the functionality the circuit is trying to represent. In general corresponds to methods in 
one of the instruction-like traits. Some methods have a variable number of arguments. In cases like that 
multiple circuits may be created with different combinations of arguments and the **name** will contain information
about the combination (i.e. the circuits `hash/hash_1/sha256/byte` and `hash/hash_10/sha256/byte` operate on 1 and 10 
input values respectively). 

You can filter by **name** with both a whitelist and a blacklist. If the whitelist is passed,
any circuit outside of the list is discarded. And, if the blacklist is passed, then any circuit inside it is discarded.
If neither the whitelist nor the blacklist are set then they have no effect and all circuit names will be considered for the 
extraction. Both lists can be configured as comma separated lists passed as arguments to the `--method-whitelist` and 
`--method-blacklist` flags.

You can combine these flags in any way you want. You can also pass the `--list` flag to the tool to make it print the 
selected circuits instead of extracting them. This is useful for debugging a circuit selection that is not producing the 
desired results.

### Constants 

Some harnesses require a list of literal values that will be used as compile-time constants, representing 
off-circuit values in the harness' input. To pass constants use either the `--constants` or the `--constants-file` flags, 
but they cannot be used at the same time. 

`--constants` expects a comma separated list of literal values. All types expect their decimal representation with the 
exception of `bit` with expectes either `true` or `false`. 

`--constants-file` expects a path to a file containing lines 
of comma separated values. These values have the same requirements as the `--constants` flag.

> If a harness requires more constants than supplied extraction will fail.

## Output directory structure

The output directory can be selected with the `-o` flag with `picus_files` used as a default if the flag is omitted.
The tool writes one Picus file per extracted harness. These files are organized hierarchically 
by **instruction**, **name**, **chip**, and **type**. 

For example, the harnesses for `equality` in the `native` chip will produce 
the following structure.

```
└── picus_files
    └── equality
        ├── is_equal_to
        │   └── native
        │       ├── bit
        │       │   └── output.picus
        │       └── native
        │           └── output.picus
        └── is_equal_to_fixed
            └── native
                ├── bit
                │   └── output.picus
                └── native
                    └── output.picus
```
