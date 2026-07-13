//! Generated Solana protocol event value types.
//!
//! The Rust implementation is emitted by `host-listener/build.rs` into Cargo
//! `OUT_DIR`, so it is not reachable by rustfmt or Prettier. Ingestion
//! reconstructs these semantic values from instruction data; no emitted-event
//! decoder is generated.

pub mod zama_host_events {
    include!(concat!(env!("OUT_DIR"), "/zama_host_events.rs"));
}

pub mod solana_abi_schema_hashes {
    include!(concat!(env!("OUT_DIR"), "/solana_abi_schema_hashes.rs"));
}

#[allow(dead_code, unused_variables)]
pub mod zama_host_instructions {
    include!(concat!(env!("OUT_DIR"), "/zama_host_instructions.rs"));
}

pub use solana_abi_schema_hashes::{
    SolanaAbiSchema, SOLANA_ABI_SCHEMAS, SOLANA_EVENT_VERSIONS,
};
pub use zama_host_events::{
    FheBinaryOpCode, FheBinaryOpEvent, FheIsInEvent, FheMulDivEvent,
    FheRandBoundedEvent, FheRandEvent, FheSumEvent, FheTernaryOpCode,
    FheTernaryOpEvent, FheUnaryOpCode, FheUnaryOpEvent, ProducedPublicOutput,
    PublicOutputsProducedEvent, TrivialEncryptEvent, EVENT_VERSION,
    PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION,
};
pub use zama_host_instructions::{
    decode_zama_host_instruction, ZamaHostInstruction,
};

#[cfg(test)]
mod tests {
    use super::{
        ProducedPublicOutput, PublicOutputsProducedEvent,
        PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION,
    };

    #[test]
    fn generated_public_outputs_produced_event_includes_nested_records() {
        let event = PublicOutputsProducedEvent {
            version: PUBLIC_OUTPUTS_PRODUCED_EVENT_VERSION,
            outputs: vec![ProducedPublicOutput {
                step_index: 2,
                encrypted_value: [3; 32],
                output_handle: [4; 32],
            }],
        };
        assert_eq!(event.outputs[0].step_index, 2);
    }
}
