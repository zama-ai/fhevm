//! The Solana public-decrypt `extraData` container.
//!
//! A gateway public-decryption request carries one opaque `extraData` blob, and the gateway reads
//! only its version byte and the KMS context id. For a Solana handle the connector also needs to
//! know which encrypted value account the handle lives in — an account is not derivable from a
//! handle — so the version-`0x03` form carries that address after the context id. Nothing else
//! travels here: the `PublicDecryptLeaf` proof that establishes public-ness is fetched by the
//! connector from the coprocessor's leaf record and verified against the account's own peaks. A
//! proof a requester could hand in would be a proof the connector has to be talked into trusting.
//!
//! The client-side encoder is `buildSolanaPublicDecryptExtraData` in
//! `sdk/js-sdk/src/solana/actions/publicDecryptCertificate.ts` — a hand-mirrored codec across
//! languages; the two layouts change together, pinned by the shared byte vectors in
//! `solana/test-fixtures/user-decrypt/extra_data_v1.json`.

/// `extraData` version byte of the Solana public-decrypt form:
/// `0x03 ‖ context_id(32) ‖ encrypted_value_account(32)`.
pub const SOLANA_EXTRA_DATA_VERSION_PUBLIC_DECRYPT: u8 = 0x03;

/// The exact length of a public-decrypt blob: version, context id, account.
pub const SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN: usize = 1 + 32 + 32;

/// The parsed public-decrypt carrier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaPublicDecryptExtraData {
    /// The 32-byte KMS context id.
    pub context_id: [u8; 32],
    /// The encrypted value account the requested handle lives in.
    pub encrypted_value_account: [u8; 32],
}

/// Parses the public-decrypt form strictly: the version byte and the exact length, nothing
/// looser. Public decrypt fails closed on anything else — there is no proof-less path to fall
/// back to.
pub fn parse_solana_public_decrypt_extra_data(
    extra_data: &[u8],
) -> Option<SolanaPublicDecryptExtraData> {
    if extra_data.len() != SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN
        || extra_data[0] != SOLANA_EXTRA_DATA_VERSION_PUBLIC_DECRYPT
    {
        return None;
    }
    let mut context_id = [0; 32];
    context_id.copy_from_slice(&extra_data[1..33]);
    let mut encrypted_value_account = [0; 32];
    encrypted_value_account.copy_from_slice(&extra_data[33..65]);
    Some(SolanaPublicDecryptExtraData {
        context_id,
        encrypted_value_account,
    })
}

/// Encodes the public-decrypt form.
pub fn encode_solana_public_decrypt_extra_data(
    context_id: [u8; 32],
    encrypted_value_account: [u8; 32],
) -> Vec<u8> {
    let mut data = Vec::with_capacity(SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN);
    data.push(SOLANA_EXTRA_DATA_VERSION_PUBLIC_DECRYPT);
    data.extend_from_slice(&context_id);
    data.extend_from_slice(&encrypted_value_account);
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_carrier_round_trips() {
        let blob = encode_solana_public_decrypt_extra_data([7; 32], [9; 32]);
        assert_eq!(blob.len(), SOLANA_PUBLIC_DECRYPT_EXTRA_DATA_LEN);
        assert_eq!(
            parse_solana_public_decrypt_extra_data(&blob),
            Some(SolanaPublicDecryptExtraData {
                context_id: [7; 32],
                encrypted_value_account: [9; 32],
            })
        );
    }

    #[test]
    fn the_parser_requires_the_version_and_the_exact_length() {
        let blob = encode_solana_public_decrypt_extra_data([7; 32], [9; 32]);
        assert!(parse_solana_public_decrypt_extra_data(&[]).is_none());
        assert!(parse_solana_public_decrypt_extra_data(&blob[..blob.len() - 1]).is_none());
        let mut trailing = blob.clone();
        trailing.push(0);
        assert!(parse_solana_public_decrypt_extra_data(&trailing).is_none());
        let mut other_version = blob;
        other_version[0] = 0x01;
        assert!(parse_solana_public_decrypt_extra_data(&other_version).is_none());
    }
}
