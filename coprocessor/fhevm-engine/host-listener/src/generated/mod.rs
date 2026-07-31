//! Generated Solana ABI golden-manifest constants.
//!
//! The Rust implementation is emitted by `host-listener/build.rs` into Cargo
//! `OUT_DIR`, so it is not reachable by rustfmt or Prettier.

pub mod solana_abi_schema_hashes {
    include!(concat!(env!("OUT_DIR"), "/solana_abi_schema_hashes.rs"));
}

pub use solana_abi_schema_hashes::{
    SolanaAbiSchema, SOLANA_ABI_SCHEMAS, SOLANA_EVENT_VERSIONS,
};
