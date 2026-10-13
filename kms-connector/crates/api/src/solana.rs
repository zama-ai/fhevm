//! The Solana user-decryption body in its typed form, through the shared `zama-solana-request`
//! crate: the one mapping every consumer of the DTO (the connector endpoint, the relayer) calls.

use crate::types::SolanaUserDecryptionRequest;
use zama_solana_request::{
    HandleEntry, SolanaRequestError, SolanaUserDecryptFields, SolanaUserDecryptRequest,
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
            user_address: payload.userAddress.0,
            transport_key: payload.publicKey.to_vec(),
            allowed_scopes: payload
                .allowedScopes
                .iter()
                .map(|scope| {
                    scope[..]
                        .try_into()
                        .map_err(|_| SolanaFormError::ScopeLength(scope.len()))
                })
                .collect::<Result<_, _>>()?,
            start_timestamp: payload.requestValidity.startTimestamp,
            duration_seconds: payload.requestValidity.durationSeconds,
            verifying_program_id: payload.verifyingProgramId.0,
            extra_data: payload.extraData.to_vec(),
            signature: self.signature[..]
                .try_into()
                .map_err(|_| SolanaFormError::SignatureLength(self.signature.len()))?,
            entries: payload
                .handles
                .iter()
                .map(|e| HandleEntry {
                    handle: e.handle.0,
                    owner_address: e.ownerAddress.0,
                    encrypted_store: e.encryptedStore.0,
                })
                .collect(),
        };
        Ok(SolanaUserDecryptRequest::assemble(fields)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{RequestValidity, SolanaHandleEntry, SolanaUserDecryptionPayload};
    use alloy::primitives::{B256, Bytes};
    use zama_solana_acl::host_chain::solana_host_chain_id;
    use zama_solana_permit::{
        KMS_ROUTING_EXTRA_DATA_LEN, KMS_ROUTING_VERSION_BYTE, TRANSPORT_KEY_LEN,
    };

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
    fn assemble_puts_every_field_in_its_role() {
        let chain = solana_host_chain_id(7);
        let handle = |seed: u8| {
            let mut handle = [seed; 32];
            handle[22..30].copy_from_slice(&chain.to_be_bytes());
            handle
        };
        let entry = |seed: u8| HandleEntry {
            handle: handle(seed),
            owner_address: [seed + 1; 32],
            encrypted_store: [seed + 2; 32],
        };
        let mut extra_data = vec![7; KMS_ROUTING_EXTRA_DATA_LEN];
        extra_data[0] = KMS_ROUTING_VERSION_BYTE;
        let request = SolanaUserDecryptionRequest {
            attestationType: "solana-srfc38-user-decrypt-v1".into(),
            payload: SolanaUserDecryptionPayload {
                handles: [entry(0x10), entry(0x20)]
                    .iter()
                    .map(|e| SolanaHandleEntry {
                        handle: B256::from(e.handle),
                        ownerAddress: B256::from(e.owner_address),
                        encryptedStore: B256::from(e.encrypted_store),
                    })
                    .collect(),
                userAddress: B256::repeat_byte(1),
                publicKey: Bytes::from(vec![2; TRANSPORT_KEY_LEN]),
                allowedScopes: vec![Bytes::from(vec![3; 64])],
                requestValidity: RequestValidity {
                    startTimestamp: 4,
                    durationSeconds: 5,
                },
                verifyingProgramId: B256::repeat_byte(6),
                extraData: Bytes::from(extra_data.clone()),
            },
            signature: Bytes::from(vec![9; 64]),
        };

        let typed = request.assemble().unwrap();
        assert_eq!(typed.entries(), &[entry(0x10), entry(0x20)]);
        let permit = typed.permit();
        assert_eq!(permit.user_address().as_bytes(), &[1; 32]);
        assert_eq!(
            &permit.transport_key().as_bytes()[..],
            &[2; TRANSPORT_KEY_LEN]
        );
        assert_eq!(permit.allowed_scopes().as_slice()[0].as_bytes(), &[3; 64]);
        assert_eq!(permit.start_timestamp(), 4);
        assert_eq!(permit.duration_seconds(), 5);
        assert_eq!(permit.verifying_program_id().as_bytes(), &[6; 32]);
        assert_eq!(permit.chain_id(), chain);
        assert_eq!(permit.extra_data().to_extra_data(), extra_data);
        assert_eq!(typed.signature().as_bytes(), &[9; 64]);
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
