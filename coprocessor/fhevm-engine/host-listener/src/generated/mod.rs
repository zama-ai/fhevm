//! Generated Solana protocol event decoders.
//!
//! The Rust implementation is emitted by `host-listener/build.rs` into Cargo
//! `OUT_DIR`, so it is not reachable by rustfmt or Prettier. Consumers should
//! import this module instead of parsing Anchor CPI event bytes directly.

pub mod zama_host_events {
    include!(concat!(env!("OUT_DIR"), "/zama_host_events.rs"));
}

pub use zama_host_events::{
    anchor_event_discriminator, decode_anchor_cpi_event, AclAllowedEvent,
    FheBinaryOpCode, FheBinaryOpEvent, FheRandBoundedEvent, FheRandEvent,
    FheTernaryOpCode, FheTernaryOpEvent, InputVerifiedEvent,
    TrivialEncryptEvent, ZamaHostEvent, ANCHOR_EVENT_IX_TAG_LE, EVENT_VERSION,
};
