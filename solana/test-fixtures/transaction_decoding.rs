use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionDecodingFixture {
    pub name: String,
    pub static_account_tags: Vec<u8>,
    #[serde(default)]
    pub loaded_writable_account_tags: Vec<u8>,
    #[serde(default)]
    pub loaded_readonly_account_tags: Vec<u8>,
    pub top_level: Vec<CompiledInstructionFixture>,
    #[serde(default)]
    pub inner_groups: Vec<InnerInstructionGroupFixture>,
    pub expected: ExpectedOutcome,
}

impl TransactionDecodingFixture {
    pub fn account_tags(&self) -> impl Iterator<Item = u8> + '_ {
        self.static_account_tags
            .iter()
            .chain(&self.loaded_writable_account_tags)
            .chain(&self.loaded_readonly_account_tags)
            .copied()
    }
}

#[derive(Debug, Deserialize)]
pub struct CompiledInstructionFixture {
    pub program_id_index: u32,
    #[serde(default)]
    pub accounts: Vec<u8>,
    #[serde(default)]
    pub data: Vec<u8>,
    pub stack_height: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct InnerInstructionGroupFixture {
    pub index: u32,
    pub instructions: Vec<CompiledInstructionFixture>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ExpectedOutcome {
    Accept {
        instructions: Vec<ExpectedInstruction>,
    },
    Reject,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ExpectedInstruction {
    pub program_tag: u8,
    pub account_tags: Vec<u8>,
    pub data: Vec<u8>,
    pub top_level_index: u32,
    pub is_inner: bool,
}

pub fn transaction_decoding_fixtures() -> Vec<TransactionDecodingFixture> {
    serde_json::from_str(include_str!("transaction_decoding_v1.json"))
        .expect("shared transaction decoding fixtures must be valid JSON")
}

#[allow(dead_code)] // The Geyser adapter consumes raw bytes directly.
pub fn base58_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 58] =
        b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    if bytes.is_empty() {
        return String::new();
    }

    let leading_zeros = bytes.iter().take_while(|byte| **byte == 0).count();
    let mut digits = Vec::new();
    for &byte in bytes {
        let mut carry = byte as u32;
        for digit in &mut digits {
            carry += (*digit as u32) << 8;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    let mut encoded: Vec<u8> =
        std::iter::repeat_n(ALPHABET[0], leading_zeros).collect();
    encoded.extend(digits.iter().rev().map(|digit| ALPHABET[*digit as usize]));
    String::from_utf8(encoded).expect("base58 alphabet is valid UTF-8")
}
