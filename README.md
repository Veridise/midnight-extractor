# Midnight's Halo2 circuits extraction tool

This repository contains the release of the extraction tool. Parts of the tool will be added to this repository as they are finalized.

## Repository organization

The `crates` directory contains the different crates that make up the tool:

- `crates/support`: Integration interface for enabling the extraction of chips and types.
- `crates/support-macros`: Client macros for helping with integrating with the extractor.
- `crates/groups-support`: Support for working with the group macros.
- `crates/extractor`: Main extraction logic, exposed by the `midnight-extractor` CLI.
- `crates/extractor-macros`: Proc-macros used aiding development of the extractor.
- `crates/extractor-core`: Core types shared across modules in the extractor.
- `crates/harnesses`: All the harnesses created as entry points for extraction.

The `docs` directory contains the user manual of the tool, describing how to run the tool, and how to add new harnesses and maintain existing ones.
