use std::collections::HashMap;

use anchor_lang::{prelude::Pubkey, AnchorDeserialize, Discriminator};
use litesvm::types::TransactionMetadata;
use zama_host::{FheBinaryOpCode, FheBinaryOpEvent, TrivialEncryptEvent};

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

pub enum SolanaFheEvent {
    BinaryOp(FheBinaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
}

pub trait FheBackend {
    fn seed_cleartext(&mut self, handle: Handle, value: TypedClearValue);
    fn ingest_event(&mut self, event: &SolanaFheEvent) -> Result<(), String>;
    fn decrypt_cleartext(&self, handle: Handle) -> Option<TypedClearValue>;
}

#[derive(Default)]
pub struct CleartextBackend {
    values: HashMap<Handle, TypedClearValue>,
}

impl CleartextBackend {
    pub fn ingest_transaction(
        &mut self,
        meta: &TransactionMetadata,
        account_keys: &[Pubkey],
        program_id: Pubkey,
    ) -> Result<(), String> {
        for event in solana_fhe_events(meta, account_keys, program_id) {
            self.ingest_event(&event)?;
        }
        Ok(())
    }
}

impl FheBackend for CleartextBackend {
    fn seed_cleartext(&mut self, handle: Handle, value: TypedClearValue) {
        self.values.insert(handle, value);
    }

    fn ingest_event(&mut self, event: &SolanaFheEvent) -> Result<(), String> {
        match event {
            SolanaFheEvent::BinaryOp(event) => self.ingest_binary_op(event),
            SolanaFheEvent::TrivialEncrypt(event) => {
                let value = TypedClearValue {
                    fhe_type: event.fhe_type,
                    value: ClearValue::Uint(bytes_to_u128(event.plaintext)),
                };
                self.values.insert(event.result, value);
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

pub fn solana_fhe_events(
    meta: &TransactionMetadata,
    account_keys: &[Pubkey],
    program_id: Pubkey,
) -> Vec<SolanaFheEvent> {
    meta.inner_instructions
        .iter()
        .flatten()
        .filter(|ix| *ix.instruction.program_id(account_keys) == program_id)
        .filter_map(|ix| decode_fhe_event(&ix.instruction.data))
        .collect()
}

fn decode_fhe_event(data: &[u8]) -> Option<SolanaFheEvent> {
    decode_binary_op_event(data)
        .map(SolanaFheEvent::BinaryOp)
        .or_else(|| decode_trivial_encrypt_event(data).map(SolanaFheEvent::TrivialEncrypt))
}

fn decode_binary_op_event(data: &[u8]) -> Option<FheBinaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheBinaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheBinaryOpEvent::deserialize(&mut &*payload).ok()
}

fn decode_trivial_encrypt_event(data: &[u8]) -> Option<TrivialEncryptEvent> {
    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    TrivialEncryptEvent::deserialize(&mut &*payload).ok()
}

fn anchor_event_prefix(discriminator: &[u8]) -> Vec<u8> {
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(discriminator.iter().copied())
        .collect()
}

fn bytes_to_u128(bytes: [u8; 32]) -> u128 {
    u128::from_be_bytes(bytes[16..].try_into().expect("slice has length 16"))
}
