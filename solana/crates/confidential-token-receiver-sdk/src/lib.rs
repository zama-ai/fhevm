//! Receiver hook ABI helpers for the Solana confidential token PoC.
//!
//! Receiver programs use this crate to write the canonical return-data payload
//! expected by `confidential-token` after a transfer-and-call hook.

use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::program::set_return_data;

/// Magic prefix for confidential transfer receiver return data.
pub const TRANSFER_RECEIVER_RETURN_MAGIC: &[u8; 23] = b"ZAMA_CT_RECEIVER_RET_V0";
/// Number of 32-byte fields following the magic prefix.
pub const TRANSFER_RECEIVER_RETURN_FIELD_COUNT: usize = 7;
/// Serialized payload length, including the magic prefix.
pub const TRANSFER_RECEIVER_RETURN_LEN: usize =
    TRANSFER_RECEIVER_RETURN_MAGIC.len() + (32 * TRANSFER_RECEIVER_RETURN_FIELD_COUNT);

/// Decode error for receiver hook return data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferReceiverReturnDecodeError {
    /// The payload length does not match the fixed ABI length.
    InvalidLength { actual: usize },
    /// The payload does not start with the receiver-return magic prefix.
    InvalidMagic,
}

impl core::fmt::Display for TransferReceiverReturnDecodeError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength { actual } => write!(
                formatter,
                "invalid receiver return-data length {actual}, expected {TRANSFER_RECEIVER_RETURN_LEN}"
            ),
            Self::InvalidMagic => formatter.write_str("invalid receiver return-data magic"),
        }
    }
}

impl std::error::Error for TransferReceiverReturnDecodeError {}

/// Canonical return-data payload for a confidential transfer receiver hook.
///
/// A receiver program must write this payload with Solana `set_return_data`
/// before returning from its hook instruction. The token program rejects the
/// hook result unless every field matches the transfer and callback-success ACL
/// records supplied to the caller instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferReceiverReturn {
    /// Confidential mint for the transfer.
    pub mint: Pubkey,
    /// Original sender confidential token account.
    pub from_token_account: Pubkey,
    /// Original recipient confidential token account.
    pub to_token_account: Pubkey,
    /// Prior transfer's encrypted all-or-zero sent handle.
    pub sent_handle: [u8; 32],
    /// ZamaHost ACL record for `sent_handle`.
    pub sent_acl_record: Pubkey,
    /// Receiver-produced encrypted callback-success bit.
    pub callback_success_handle: [u8; 32],
    /// ZamaHost ACL record for `callback_success_handle`.
    pub callback_success_acl_record: Pubkey,
}

impl TransferReceiverReturn {
    /// Serialized payload length, including the magic prefix.
    pub const LEN: usize = TRANSFER_RECEIVER_RETURN_LEN;

    /// Decodes and validates the fixed receiver return-data payload.
    pub fn decode(data: &[u8]) -> Result<Self, TransferReceiverReturnDecodeError> {
        if data.len() != Self::LEN {
            return Err(TransferReceiverReturnDecodeError::InvalidLength { actual: data.len() });
        }
        if !data.starts_with(&TRANSFER_RECEIVER_RETURN_MAGIC[..]) {
            return Err(TransferReceiverReturnDecodeError::InvalidMagic);
        }
        let mut offset = TRANSFER_RECEIVER_RETURN_MAGIC.len();
        Ok(Self {
            mint: read_return_pubkey(data, &mut offset),
            from_token_account: read_return_pubkey(data, &mut offset),
            to_token_account: read_return_pubkey(data, &mut offset),
            sent_handle: read_return_bytes32(data, &mut offset),
            sent_acl_record: read_return_pubkey(data, &mut offset),
            callback_success_handle: read_return_bytes32(data, &mut offset),
            callback_success_acl_record: read_return_pubkey(data, &mut offset),
        })
    }

    /// Encodes this payload in the canonical receiver return-data format.
    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::LEN);
        data.extend_from_slice(TRANSFER_RECEIVER_RETURN_MAGIC);
        data.extend_from_slice(self.mint.as_ref());
        data.extend_from_slice(self.from_token_account.as_ref());
        data.extend_from_slice(self.to_token_account.as_ref());
        data.extend_from_slice(&self.sent_handle);
        data.extend_from_slice(self.sent_acl_record.as_ref());
        data.extend_from_slice(&self.callback_success_handle);
        data.extend_from_slice(self.callback_success_acl_record.as_ref());
        data
    }

    /// Writes this payload to Solana return data for the current instruction.
    pub fn set_return_data(&self) {
        set_return_data(&self.encode());
    }
}

/// Encodes the expected return data for a confidential transfer receiver hook.
pub fn transfer_receiver_return_data(
    mint: Pubkey,
    from_token_account: Pubkey,
    to_token_account: Pubkey,
    sent_handle: [u8; 32],
    sent_acl_record: Pubkey,
    callback_success_handle: [u8; 32],
    callback_success_acl_record: Pubkey,
) -> Vec<u8> {
    TransferReceiverReturn {
        mint,
        from_token_account,
        to_token_account,
        sent_handle,
        sent_acl_record,
        callback_success_handle,
        callback_success_acl_record,
    }
    .encode()
}

/// Writes the canonical receiver hook payload to Solana return data.
pub fn set_transfer_receiver_return_data(payload: &TransferReceiverReturn) {
    payload.set_return_data();
}

fn read_return_pubkey(data: &[u8], offset: &mut usize) -> Pubkey {
    Pubkey::new_from_array(read_return_bytes32(data, offset))
}

fn read_return_bytes32(data: &[u8], offset: &mut usize) -> [u8; 32] {
    let end = *offset + 32;
    let mut output = [0; 32];
    output.copy_from_slice(&data[*offset..end]);
    *offset = end;
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_receiver_return() -> TransferReceiverReturn {
        TransferReceiverReturn {
            mint: Pubkey::new_unique(),
            from_token_account: Pubkey::new_unique(),
            to_token_account: Pubkey::new_unique(),
            sent_handle: [1; 32],
            sent_acl_record: Pubkey::new_unique(),
            callback_success_handle: [2; 32],
            callback_success_acl_record: Pubkey::new_unique(),
        }
    }

    #[test]
    fn transfer_receiver_return_round_trips() {
        let payload = sample_receiver_return();
        let encoded = payload.encode();

        assert_eq!(encoded.len(), TRANSFER_RECEIVER_RETURN_LEN);
        assert_eq!(encoded.len(), TransferReceiverReturn::LEN);
        assert_eq!(TransferReceiverReturn::decode(&encoded).unwrap(), payload);
    }

    #[test]
    fn transfer_receiver_return_compatibility_encoder_matches_struct_encoder() {
        let payload = sample_receiver_return();

        assert_eq!(
            transfer_receiver_return_data(
                payload.mint,
                payload.from_token_account,
                payload.to_token_account,
                payload.sent_handle,
                payload.sent_acl_record,
                payload.callback_success_handle,
                payload.callback_success_acl_record,
            ),
            payload.encode()
        );
    }

    #[test]
    fn transfer_receiver_return_rejects_wrong_magic_or_length() {
        let mut encoded = sample_receiver_return().encode();
        encoded[0] ^= 0xff;
        assert_eq!(
            TransferReceiverReturn::decode(&encoded),
            Err(TransferReceiverReturnDecodeError::InvalidMagic)
        );

        let mut truncated = sample_receiver_return().encode();
        truncated.pop();
        assert_eq!(
            TransferReceiverReturn::decode(&truncated),
            Err(TransferReceiverReturnDecodeError::InvalidLength {
                actual: TRANSFER_RECEIVER_RETURN_LEN - 1
            })
        );
    }
}
