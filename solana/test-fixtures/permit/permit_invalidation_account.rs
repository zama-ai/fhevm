//! Schema of the byte-exact `PermitInvalidation` account fixture.
//!
//! The account is written by the host program's `revoke_permits` instruction, which takes
//! it as an unchecked account — so its layout reaches no generated interface description,
//! and the KMS Connector that consumes it reads the raw bytes out of a Solana account
//! snapshot and decodes fixed offsets by hand. This file and
//! `permit_invalidation_account_v1.json` are that contract: the JSON carries the bytes of
//! one account the program really wrote, together with a table saying where each value
//! sits inside them, and this file is the Rust half a consumer includes with `#[path]` to
//! deserialize it.
//!
//! Two conventions carry weight and are not stylistic:
//!
//! * **Every 64-bit number is a decimal string.** A JSON number arrives in a TypeScript
//!   consumer as an IEEE-754 double and silently loses precision above 2^53. The
//!   watermark is unsigned seconds and the clock is signed seconds; neither may be
//!   rounded. Offsets, lengths and the one-byte bump stay JSON numbers — they cannot
//!   reach that range.
//! * **The field table is the normative layout, not a comment on it.** Offsets and
//!   lengths are stated per field rather than implied by a struct definition, because the
//!   consumer that reads this account has no struct definition to derive them from.
//!
//! Only serde is required, so this file compiles in any consumer's test target.

use serde::{Deserialize, Serialize};

/// Schema identifier written into every file this shape can parse.
pub const PERMIT_INVALIDATION_ACCOUNT_SCHEMA: &str = "zama-solana-permit-invalidation-account/v1";

/// A fixture file: one account, fully described.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermitInvalidationAccountFixture {
    /// Schema identifier; a consumer that does not recognize it must refuse the file
    /// rather than guess.
    pub schema: String,
    /// What this file pins, in prose.
    pub description: String,
    /// How to regenerate it.
    pub regenerate_with: String,
    /// The program that owns the account.
    pub program: Program,
    /// How the account's address is derived, so a reader can find it.
    pub address: AddressDerivation,
    /// What produced the committed bytes.
    pub produced_by: Production,
    /// The account as it appears in a snapshot.
    pub account: AccountData,
    /// Where each value sits inside `account.data_hex`, in layout order.
    pub fields: Vec<Field>,
}

/// The owning program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    /// Program id, base58 — the form an RPC account filter takes.
    pub id_base58: String,
    /// Program id, hex — the raw 32 bytes.
    pub id_hex: String,
}

/// The program-derived address of the account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressDerivation {
    /// Seeds in derivation order, for `find_program_address` under `program`.
    pub seeds: Vec<Seed>,
    /// Address, base58.
    pub address_base58: String,
    /// Address, hex.
    pub address_hex: String,
    /// The bump the canonical derivation lands on for this user.
    pub bump: u8,
}

/// One derivation seed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed {
    /// What the seed is, for a reader assembling the list.
    pub name: String,
    /// Whether the seed is fixed or supplied per account.
    pub kind: SeedKind,
    /// The seed bytes, hex.
    pub hex: String,
    /// The seed as text, for the fixed literal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utf8: Option<String>,
}

/// Where a seed's bytes come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeedKind {
    /// A fixed byte string compiled into the program; identical for every account.
    Utf8Literal,
    /// A 32-byte public key, supplied by the reader looking the account up.
    Pubkey,
}

/// What wrote the committed bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Production {
    /// Instruction that wrote them.
    pub instruction: String,
    /// State of the account before that instruction ran.
    pub prior_account_state: String,
    /// The runtime clock it saw, unix seconds, as a decimal string.
    pub clock_unix_timestamp: String,
}

/// The account, as a snapshot hands it over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountData {
    /// Owner after the write, base58. A reader that trusts the contents must check it.
    pub owner_base58: String,
    /// Length of the data in bytes. The account is created at exactly this size and never
    /// resized, so a reader may treat any other length as a different account.
    pub data_len: usize,
    /// The whole account data, hex. This is the normative field.
    pub data_hex: String,
    /// The leading eight bytes, hex, repeated for readers that only discriminate.
    pub discriminator_hex: String,
    /// The string those eight bytes are the sha256 prefix of.
    pub discriminator_preimage_utf8: String,
    /// The layout in one line, for a human reading the diff.
    pub layout: String,
}

/// One field of the account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    /// Field name, matching the on-chain record.
    pub name: String,
    /// Byte offset from the start of the account data.
    pub offset: usize,
    /// Field width in bytes.
    pub length: usize,
    /// How to turn those bytes into a value.
    pub encoding: FieldEncoding,
    /// The value: hex for byte fields, a decimal string for numbers.
    pub value: String,
    /// The same value in base58, for the public-key field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_base58: Option<String>,
    /// What the field means.
    pub comment: String,
}

/// How a field's bytes decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldEncoding {
    /// Opaque bytes; `value` is hex.
    Bytes,
    /// A 32-byte public key; `value` is hex and `value_base58` its base58 form.
    Pubkey,
    /// Little-endian unsigned 64-bit; `value` is a decimal string.
    U64Le,
    /// A single unsigned byte; `value` is a decimal string.
    U8,
}

impl PermitInvalidationAccountFixture {
    /// The account data as bytes.
    pub fn data_bytes(&self) -> Option<Vec<u8>> {
        from_hex(&self.account.data_hex)
    }

    /// The declared field of this name.
    pub fn field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name == name)
    }
}

// ---------------------------------------------------------------------------
// Hex, without a dependency
// ---------------------------------------------------------------------------

/// Encodes bytes as lowercase hex.
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
        out.push(char::from_digit(u32::from(byte & 0x0f), 16).expect("nibble"));
    }
    out
}

/// Decodes lowercase or uppercase hex. Returns `None` on odd length or a bad digit.
pub fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let bytes = hex.as_bytes();
    let mut out = Vec::with_capacity(hex.len() / 2);
    for pair in bytes.chunks(2) {
        let high = (pair[0] as char).to_digit(16)?;
        let low = (pair[1] as char).to_digit(16)?;
        out.push((high * 16 + low) as u8);
    }
    Some(out)
}
