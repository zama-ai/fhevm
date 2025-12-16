# Testing Quickstart

This document is a short companion to the main documentation for running tests
in the FHEVM repository.

## Rust tests

Most low-level components (such as cryptographic primitives and core logic)
are tested via Rust:

    cargo test

You can also run a subset of tests in a specific crate:

    cd crates/fhevm-core
    cargo test

## JavaScript / TypeScript tests

Front-end and integration parts that use Node.js or PNPM can typically be
tested via:

    pnpm install
    pnpm test

If the repository provides multiple packages, refer to the root `package.json`
or workspace configuration to see which scripts are available.

## Linting and formatting

Depending on your local setup, the following commands may be available:

- `cargo fmt` / `cargo clippy` for Rust
- `pnpm lint` for JavaScript/TypeScript
- custom scripts under `scripts/` (for example, `format_all.sh` if present)

Running these before opening a pull request usually avoids most CI failures.
