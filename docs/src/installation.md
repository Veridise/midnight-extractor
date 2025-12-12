# Installation

The extraction tool needs to be built from source. This is because the harness definitions depend on 
the `circuits` crate in [midnight-zk](https://github.com/midnightntwrk/midnight-zk). 


Once inside of the directory where the source is located you can build and run the tool.

First, verify that the version of `midnight-circuits` that the tool is linked against 
is the version you are interesting in extracting from.

Check that the dependency is pointing to the right repository and the right revision.
Also make sure that it has the `extraction` feature enabled. Without this feature the 
circuits do not include the necessary integrations.

For example:

```
$ cargo info midnight-circuits
midnight-circuits
version: 5.0.1 (from https://github.com/Veridise/midnight-zk#0d63a4d4)
license: unknown
rust-version: unknown
features:
 +default              = []
  bench-internal       = [midnight-proofs/bench-internal]
  extraction           = [extractor-support, testing, midnight-proofs/extraction, picus/extractor-derive]
  extractor-support    = [dep:extractor-support]
  heap_profiling       = []
  serde                = [dep:serde]
  serde_derive         = [dep:serde_derive]
  serde_json           = [dep:serde_json]
  testing              = [num-bigint/rand]
  truncated-challenges = [midnight-proofs/truncated-challenges]
```

Once everything looks good you can build the tool as usual with `cargo build --release`.
