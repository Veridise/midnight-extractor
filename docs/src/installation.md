# Installation

The extraction tool needs to build from source because the harness definitions depend on 
the `circuits` crate in [midnight-zk](https://github.com/midnightntwrk/midnight-zk). At 
the time of writing the tool actually depends on the [forked version of the crate](https://github.com/Veridise/midnight-zk).

After cloning [the repository](https://github.com/Veridise/midnight-extractor), 
verify that the version of `midnight-circuits` that the tool is linked against 
is the version you are interesting in extracting from.

Check that the dependency is pointing to the right repository and the right revision.
Also make sure that it has the `extraction` feature enabled. Without this feature the 
circuits will not include the necessary integrations.

For example:

```
$ cargo info midnight-circuits
midnight-circuits
version: 5.0.1 (from https://github.com/Veridise/midnight-zk#7042b270)
license: unknown
rust-version: unknown
features:
 +default              = []
  dev-curves           = [midnight-curves/dev-curves, midnight-proofs/dev-curves]
  extraction           = [extractor-support, testing, midnight-proofs/extraction, picus/extractor-derive]
  extractor-support    = [dep:extractor-support]
  heap_profiling       = []
  serde                = [dep:serde]
  serde_derive         = [dep:serde_derive]
  serde_json           = [dep:serde_json]
  testing              = [num-bigint/rand]
  truncated-challenges = [midnight-proofs/truncated-challenges]
```

After verifying everything looks good you can build the tool with the usual commands.
The tool can be found in `target/release/midnight-extractor`.

```bash
cargo build --release
target/release/midnight-extractor --help
```

