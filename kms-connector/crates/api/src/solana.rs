//! The Solana user-decryption body in its typed form, through the shared `zama-solana-request`
//! crate: the one mapping every consumer of the DTO (the connector endpoint, the relayer) calls.

use crate::types::SolanaUserDecryptionRequest;
use zama_solana_request::{
    SolanaEntryClaims, SolanaRequestBlob, SolanaRequestError, SolanaUserDecryptFields,
    SolanaUserDecryptRequest,
};

/// Why a Solana user body is not a well-formed request. Nothing here reads a chain or a clock.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SolanaFormError {
    #[error("an allowed scope is {0} bytes, expected 64")]
    ScopeLength(usize),
    #[error("signature is {0} bytes, expected 64")]
    SignatureLength(usize),
    #[error(transparent)]
    Request(#[from] SolanaRequestError),
}

impl SolanaUserDecryptionRequest {
    /// The typed request: the permit decoded strictly, one entry per handle, every handle on one
    /// Solana chain. The signature is not verified.
    pub fn assemble(&self) -> Result<SolanaUserDecryptRequest, SolanaFormError> {
        let payload = &self.payload;
        let fields = SolanaUserDecryptFields {
            handles: payload.handles.iter().map(|e| e.handle.0).collect(),
            transport_key: payload.publicKey.to_vec(),
            start_timestamp: payload.requestValidity.startTimestamp,
            duration_seconds: payload.requestValidity.durationSeconds,
            extra_data: payload.extraData.to_vec(),
        };
        let blob = SolanaRequestBlob {
            user_address: payload.userAddress.0,
            allowed_scopes: payload
                .allowedScopes
                .iter()
                .map(|scope| {
                    scope[..]
                        .try_into()
                        .map_err(|_| SolanaFormError::ScopeLength(scope.len()))
                })
                .collect::<Result<_, _>>()?,
            verifying_program_id: payload.verifyingProgramId.0,
            signature: self.signature[..]
                .try_into()
                .map_err(|_| SolanaFormError::SignatureLength(self.signature.len()))?,
            entries: payload
                .handles
                .iter()
                .map(|e| SolanaEntryClaims {
                    owner_address: e.ownerAddress.0,
                    encrypted_store: e.encryptedStore.0,
                })
                .collect(),
        };
        Ok(SolanaUserDecryptRequest::assemble(fields, blob)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RequestValidity, SolanaHandleEntry, SolanaUserDecryptionPayload};
    use alloy::primitives::{B256, Bytes};

    fn user(scope_len: usize, signature_len: usize) -> SolanaUserDecryptionRequest {
        SolanaUserDecryptionRequest {
            attestationType: "solana-srfc38-user-decrypt-v1".into(),
            payload: SolanaUserDecryptionPayload {
                handles: vec![SolanaHandleEntry {
                    handle: B256::repeat_byte(2),
                    ownerAddress: B256::repeat_byte(2),
                    encryptedStore: B256::repeat_byte(3),
                }],
                userAddress: B256::ZERO,
                publicKey: Bytes::new(),
                allowedScopes: vec![Bytes::from(vec![5; scope_len])],
                requestValidity: RequestValidity {
                    startTimestamp: 1,
                    durationSeconds: 60,
                },
                verifyingProgramId: B256::ZERO,
                extraData: Bytes::new(),
            },
            signature: Bytes::from(vec![4; signature_len]),
        }
    }

    #[test]
    fn assemble_names_the_byte_length_it_refuses() {
        assert_eq!(
            user(63, 64).assemble(),
            Err(SolanaFormError::ScopeLength(63))
        );
        assert_eq!(
            user(64, 65).assemble(),
            Err(SolanaFormError::SignatureLength(65))
        );
    }

    #[test]
    fn assemble_applies_the_shared_request_rules() {
        // Chain 0x0202…02 has chain-type byte 0x02, not Solana's 0x01.
        assert!(matches!(
            user(64, 64).assemble(),
            Err(SolanaFormError::Request(
                SolanaRequestError::NotSolanaChain(_)
            ))
        ));
        let mut no_entries = user(64, 64);
        no_entries.payload.handles.clear();
        assert_eq!(
            no_entries.assemble(),
            Err(SolanaFormError::Request(SolanaRequestError::NoHandles))
        );
    }
}
