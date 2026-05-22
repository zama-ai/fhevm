//! Cleartext semantic backend: ingest `emit_cpi!` TFHE events and simulate add/sub/trivial locally.

use std::collections::HashMap;

use litesvm::types::TransactionMetadata;
use solana_sdk::pubkey::Pubkey;
use crate::events::collect_zama_host_events;
use zama_host_events::{FheBinaryOpCode, FheBinaryOpEvent, ZamaHostEvent};
use crate::semantic::{BackendError, SemanticBackend};

pub type Handle = [u8; 32];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClearValue {
    Uint(u128),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedClearValue {
    pub fhe_type: u8,
    pub value: ClearValue,
}

impl TypedClearValue {
    pub fn uint64(value: u64) -> Self {
        Self {
            fhe_type: 5,
            value: ClearValue::Uint(value as u128),
        }
    }
}

pub trait FheBackend {
    fn seed_cleartext(&mut self, handle: Handle, value: TypedClearValue);
    fn ingest_zama_host_event(&mut self, event: &ZamaHostEvent) -> Result<(), String>;
    fn decrypt_cleartext(&self, handle: Handle) -> Option<TypedClearValue>;
}

#[derive(Default)]
pub struct CleartextBackend {
    values: HashMap<Handle, TypedClearValue>,
}

impl CleartextBackend {
    /// Ingest all ZamaHost `emit_cpi!` events from one transaction (production-shaped path).
    pub fn ingest_transaction(
        &mut self,
        meta: &TransactionMetadata,
        account_keys: &[Pubkey],
        program_id: Pubkey,
    ) -> Result<(), String> {
        for event in collect_zama_host_events(meta, account_keys, program_id) {
            self.ingest_zama_host_event(&event)?;
        }
        Ok(())
    }
}

impl FheBackend for CleartextBackend {
    fn seed_cleartext(&mut self, handle: Handle, value: TypedClearValue) {
        self.values.insert(handle, value);
    }

    fn ingest_zama_host_event(&mut self, event: &ZamaHostEvent) -> Result<(), String> {
        match event {
            ZamaHostEvent::FheBinaryOp(event) => self.ingest_binary_op(event),
            ZamaHostEvent::TrivialEncrypt(event) => {
                let value = TypedClearValue {
                    fhe_type: event.fhe_type,
                    value: ClearValue::Uint(bytes_to_u128(event.plaintext)),
                };
                self.values.insert(event.result, value);
                Ok(())
            }
            ZamaHostEvent::FheRand(_) | ZamaHostEvent::AclAllowed(_) | ZamaHostEvent::InputVerified(_) => {
                Ok(())
            }
        }
    }

    fn decrypt_cleartext(&self, handle: Handle) -> Option<TypedClearValue> {
        self.values.get(&handle).copied()
    }
}

impl CleartextBackend {
    fn ingest_binary_op(&mut self, event: &FheBinaryOpEvent) -> Result<(), String> {
        let lhs = self
            .values
            .get(&event.lhs)
            .copied()
            .ok_or_else(|| "missing lhs cleartext value".to_string())?;
        let rhs = if event.scalar {
            TypedClearValue {
                fhe_type: lhs.fhe_type,
                value: ClearValue::Uint(bytes_to_u128(event.rhs)),
            }
        } else {
            self.values
                .get(&event.rhs)
                .copied()
                .ok_or_else(|| "missing rhs cleartext value".to_string())?
        };

        if lhs.fhe_type != rhs.fhe_type {
            return Err("cleartext operand type mismatch".to_string());
        }

        let ClearValue::Uint(lhs_value) = lhs.value;
        let ClearValue::Uint(rhs_value) = rhs.value;
        let result = match event.op {
            FheBinaryOpCode::Add => lhs_value
                .checked_add(rhs_value)
                .ok_or_else(|| "cleartext add overflow".to_string())?,
            FheBinaryOpCode::Sub => lhs_value
                .checked_sub(rhs_value)
                .ok_or_else(|| "cleartext sub underflow".to_string())?,
        };

        self.values.insert(
            event.result,
            TypedClearValue {
                fhe_type: lhs.fhe_type,
                value: ClearValue::Uint(result),
            },
        );
        Ok(())
    }
}

fn bytes_to_u128(bytes: [u8; 32]) -> u128 {
    u128::from_be_bytes(bytes[16..].try_into().expect("slice has length 16"))
}

impl SemanticBackend for CleartextBackend {
    fn seed_u64(&mut self, handle: Handle, value: u64) {
        self.seed_cleartext(handle, TypedClearValue::uint64(value));
    }

    fn ingest_host_transaction(
        &mut self,
        meta: &TransactionMetadata,
        account_keys: &[Pubkey],
        program_id: Pubkey,
    ) -> Result<(), BackendError> {
        self.ingest_transaction(meta, account_keys, program_id)
            .map_err(BackendError::Cleartext)
    }

    fn decrypt_u64(&self, handle: Handle) -> Result<u64, BackendError> {
        let Some(value) = self.decrypt_cleartext(handle) else {
            return Err(BackendError::MissingHandle { handle });
        };
        let ClearValue::Uint(raw) = value.value;
        u64::try_from(raw).map_err(|_| BackendError::UnexpectedType { handle })
    }
}
