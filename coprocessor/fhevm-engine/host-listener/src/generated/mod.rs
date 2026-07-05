//! Generated Solana protocol event decoders.
//!
//! The Rust implementation is emitted by `host-listener/build.rs` into Cargo
//! `OUT_DIR`, so it is not reachable by rustfmt or Prettier. Consumers should
//! import this module instead of parsing Anchor event bytes directly.

pub mod zama_host_events {
    include!(concat!(env!("OUT_DIR"), "/zama_host_events.rs"));
}

// Codegen shares one Cursor helper set across files; not every file uses all of it.
#[allow(dead_code)]
pub mod confidential_token_events {
    include!(concat!(env!("OUT_DIR"), "/confidential_token_events.rs"));
}

pub mod solana_abi_schema_hashes {
    include!(concat!(env!("OUT_DIR"), "/solana_abi_schema_hashes.rs"));
}

#[allow(dead_code, unused_variables)]
pub mod zama_host_instructions {
    include!(concat!(env!("OUT_DIR"), "/zama_host_instructions.rs"));
}

pub use confidential_token_events::{
    decode_anchor_cpi_event as decode_confidential_token_anchor_cpi_event,
    decode_anchor_event as decode_confidential_token_anchor_event,
    ConfidentialTokenEvent, EVENT_VERSION as CONFIDENTIAL_TOKEN_EVENT_VERSION,
};
pub use solana_abi_schema_hashes::{
    SolanaAbiSchema, SOLANA_ABI_SCHEMAS, SOLANA_EVENT_VERSIONS,
};
pub use zama_host_events::{
    anchor_event_discriminator, decode_anchor_cpi_event, decode_anchor_event,
    FheBinaryOpCode, FheBinaryOpEvent, FheRandEvent, FheTernaryOpCode,
    FheTernaryOpEvent, TrivialEncryptEvent, ZamaHostEvent,
    ANCHOR_EVENT_IX_TAG_LE, EVENT_VERSION,
};
pub use zama_host_instructions::{
    decode_zama_host_instruction, ZamaHostInstruction,
};
