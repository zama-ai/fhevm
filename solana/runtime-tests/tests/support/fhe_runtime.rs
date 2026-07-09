use std::collections::HashMap;

use anchor_lang::{AnchorDeserialize, Discriminator};
use zama_host::{
    FheBinaryOpCode, FheBinaryOpEvent, FheRandBoundedEvent, FheRandEvent, FheTernaryOpCode,
    FheTernaryOpEvent, TrivialEncryptEvent,
};

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
    TernaryOp(FheTernaryOpEvent),
    TrivialEncrypt(TrivialEncryptEvent),
    Rand(FheRandEvent),
    RandBounded(FheRandBoundedEvent),
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

impl FheBackend for CleartextBackend {
    fn seed_cleartext(&mut self, handle: Handle, value: TypedClearValue) {
        self.values.insert(handle, value);
    }

    fn ingest_event(&mut self, event: &SolanaFheEvent) -> Result<(), String> {
        match event {
            SolanaFheEvent::BinaryOp(event) => self.ingest_binary_op(event),
            SolanaFheEvent::TernaryOp(event) => self.ingest_ternary_op(event),
            SolanaFheEvent::TrivialEncrypt(event) => {
                let value = TypedClearValue {
                    fhe_type: event.fhe_type,
                    value: ClearValue::Uint(bytes_to_u128(event.plaintext)),
                };
                self.values.insert(event.result, value);
                Ok(())
            }
            SolanaFheEvent::Rand(event) => {
                let value = random_value(event.seed, event.fhe_type)?;
                self.values.insert(
                    event.result,
                    TypedClearValue {
                        fhe_type: event.fhe_type,
                        value: ClearValue::Uint(value),
                    },
                );
                Ok(())
            }
            SolanaFheEvent::RandBounded(event) => {
                let upper_bound = bounded_upper_bound_u128(event.upper_bound, event.fhe_type)?;
                let value = seed_to_u128(event.seed) % upper_bound;
                self.values.insert(
                    event.result,
                    TypedClearValue {
                        fhe_type: event.fhe_type,
                        value: ClearValue::Uint(value),
                    },
                );
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
        let (result_type, result) = match event.op {
            FheBinaryOpCode::Add => (
                lhs.fhe_type,
                wrapping_add(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Sub => (
                lhs.fhe_type,
                wrapping_sub(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Mul => (
                lhs.fhe_type,
                wrapping_mul(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Div => {
                if rhs_value == 0 {
                    return Err("division by zero in cleartext reference".to_string());
                }
                (lhs.fhe_type, mask(lhs_value / rhs_value, lhs.fhe_type)?)
            }
            FheBinaryOpCode::Rem => {
                if rhs_value == 0 {
                    return Err("remainder by zero in cleartext reference".to_string());
                }
                (lhs.fhe_type, mask(lhs_value % rhs_value, lhs.fhe_type)?)
            }
            FheBinaryOpCode::And => (lhs.fhe_type, mask(lhs_value & rhs_value, lhs.fhe_type)?),
            FheBinaryOpCode::Or => (lhs.fhe_type, mask(lhs_value | rhs_value, lhs.fhe_type)?),
            FheBinaryOpCode::Xor => (lhs.fhe_type, mask(lhs_value ^ rhs_value, lhs.fhe_type)?),
            FheBinaryOpCode::Shl => (
                lhs.fhe_type,
                shift_left(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Shr => (
                lhs.fhe_type,
                shift_right(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Rotl => (
                lhs.fhe_type,
                rotate_left(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Rotr => (
                lhs.fhe_type,
                rotate_right(lhs_value, rhs_value, lhs.fhe_type)?,
            ),
            FheBinaryOpCode::Eq => (0, u128::from(lhs_value == rhs_value)),
            FheBinaryOpCode::Ne => (0, u128::from(lhs_value != rhs_value)),
            FheBinaryOpCode::Ge => (0, u128::from(lhs_value >= rhs_value)),
            FheBinaryOpCode::Gt => (0, u128::from(lhs_value > rhs_value)),
            FheBinaryOpCode::Le => (0, u128::from(lhs_value <= rhs_value)),
            FheBinaryOpCode::Lt => (0, u128::from(lhs_value < rhs_value)),
            FheBinaryOpCode::Min => (lhs.fhe_type, lhs_value.min(rhs_value)),
            FheBinaryOpCode::Max => (lhs.fhe_type, lhs_value.max(rhs_value)),
        };

        self.values.insert(
            event.result,
            TypedClearValue {
                fhe_type: result_type,
                value: ClearValue::Uint(result),
            },
        );
        Ok(())
    }

    fn ingest_ternary_op(&mut self, event: &FheTernaryOpEvent) -> Result<(), String> {
        let control = self
            .values
            .get(&event.control)
            .copied()
            .ok_or_else(|| "missing control cleartext value".to_string())?;
        let if_true = self
            .values
            .get(&event.if_true)
            .copied()
            .ok_or_else(|| "missing true-branch cleartext value".to_string())?;
        let if_false = self
            .values
            .get(&event.if_false)
            .copied()
            .ok_or_else(|| "missing false-branch cleartext value".to_string())?;

        if control.fhe_type != 0 || if_true.fhe_type != if_false.fhe_type {
            return Err("cleartext ternary operand type mismatch".to_string());
        }

        let ClearValue::Uint(control_value) = control.value;
        let selected = match event.op {
            FheTernaryOpCode::IfThenElse => {
                if control_value != 0 {
                    if_true
                } else {
                    if_false
                }
            }
        };
        self.values.insert(event.result, selected);
        Ok(())
    }
}

pub fn decode_binary_op_event(data: &[u8]) -> Option<FheBinaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheBinaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheBinaryOpEvent::deserialize(&mut &*payload).ok()
}

pub fn decode_trivial_encrypt_event(data: &[u8]) -> Option<TrivialEncryptEvent> {
    let event_prefix = anchor_event_prefix(TrivialEncryptEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    TrivialEncryptEvent::deserialize(&mut &*payload).ok()
}

pub fn decode_ternary_op_event(data: &[u8]) -> Option<FheTernaryOpEvent> {
    let event_prefix = anchor_event_prefix(FheTernaryOpEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheTernaryOpEvent::deserialize(&mut &*payload).ok()
}

pub fn decode_fhe_rand_event(data: &[u8]) -> Option<FheRandEvent> {
    let event_prefix = anchor_event_prefix(FheRandEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheRandEvent::deserialize(&mut &*payload).ok()
}

pub fn decode_fhe_rand_bounded_event(data: &[u8]) -> Option<FheRandBoundedEvent> {
    let event_prefix = anchor_event_prefix(FheRandBoundedEvent::DISCRIMINATOR);
    let payload = data.strip_prefix(&event_prefix[..])?;
    FheRandBoundedEvent::deserialize(&mut &*payload).ok()
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

fn seed_to_u128(seed: [u8; 16]) -> u128 {
    u128::from_be_bytes(seed)
}

fn wrapping_add(lhs: u128, rhs: u128, fhe_type: u8) -> Result<u128, String> {
    if fhe_type == 6 {
        return Ok(lhs.wrapping_add(rhs));
    }
    Ok(lhs.wrapping_add(rhs) % modulus(fhe_type)?)
}

fn wrapping_sub(lhs: u128, rhs: u128, fhe_type: u8) -> Result<u128, String> {
    if fhe_type == 6 {
        return Ok(lhs.wrapping_sub(rhs));
    }
    let modulus = modulus(fhe_type)?;
    Ok(((lhs % modulus) + modulus - (rhs % modulus)) % modulus)
}

/// Truncates `value` to the width of `fhe_type` (identity for the full-width 128-bit type).
fn mask(value: u128, fhe_type: u8) -> Result<u128, String> {
    if fhe_type == 6 {
        return Ok(value);
    }
    Ok(value % modulus(fhe_type)?)
}

fn wrapping_mul(lhs: u128, rhs: u128, fhe_type: u8) -> Result<u128, String> {
    mask(lhs.wrapping_mul(rhs), fhe_type)
}

/// Left shift by `amount mod width`, truncated to the type width (matches FHE shift semantics).
fn shift_left(lhs: u128, amount: u128, fhe_type: u8) -> Result<u128, String> {
    let bits = fhe_type_width_bits(fhe_type)?;
    let shift = (amount % u128::from(bits)) as u32;
    mask(lhs.wrapping_shl(shift), fhe_type)
}

/// Right shift by `amount mod width` of the width-truncated operand.
fn shift_right(lhs: u128, amount: u128, fhe_type: u8) -> Result<u128, String> {
    let bits = fhe_type_width_bits(fhe_type)?;
    let shift = (amount % u128::from(bits)) as u32;
    Ok(mask(lhs, fhe_type)?.wrapping_shr(shift))
}

/// Rotate-left within the type width.
fn rotate_left(value: u128, amount: u128, fhe_type: u8) -> Result<u128, String> {
    let bits = fhe_type_width_bits(fhe_type)?;
    let value = mask(value, fhe_type)?;
    let shift = (amount % u128::from(bits)) as u32;
    if shift == 0 {
        return Ok(value);
    }
    mask((value << shift) | (value >> (bits - shift)), fhe_type)
}

/// Rotate-right within the type width.
fn rotate_right(value: u128, amount: u128, fhe_type: u8) -> Result<u128, String> {
    let bits = fhe_type_width_bits(fhe_type)?;
    let value = mask(value, fhe_type)?;
    let shift = (amount % u128::from(bits)) as u32;
    if shift == 0 {
        return Ok(value);
    }
    mask((value >> shift) | (value << (bits - shift)), fhe_type)
}

fn random_value(seed: [u8; 16], fhe_type: u8) -> Result<u128, String> {
    if fhe_type == 6 {
        return Ok(seed_to_u128(seed));
    }
    Ok(seed_to_u128(seed) % modulus(fhe_type)?)
}

fn bounded_upper_bound_u128(upper_bound: [u8; 32], fhe_type: u8) -> Result<u128, String> {
    let _bits = fhe_type_width_bits(fhe_type)?;
    if upper_bound[..16].iter().any(|byte| *byte != 0) {
        return Err(format!(
            "bounded random upper bound exceeds cleartext backend width for type {fhe_type}"
        ));
    }
    let upper_bound = bytes_to_u128(upper_bound);
    if upper_bound == 0 {
        return Err("zero random upper bound".to_string());
    }
    Ok(upper_bound)
}

fn modulus(fhe_type: u8) -> Result<u128, String> {
    let bits = fhe_type_width_bits(fhe_type)?;
    if bits == 128 {
        return Err("full-width u128 type has no u128 modulus".to_string());
    }
    Ok(1_u128 << bits)
}

fn fhe_type_width_bits(fhe_type: u8) -> Result<u32, String> {
    match fhe_type {
        0 => Ok(1),
        2 => Ok(8),
        3 => Ok(16),
        4 => Ok(32),
        5 => Ok(64),
        6 => Ok(128),
        other => Err(format!("unsupported cleartext uint type {other}")),
    }
}
