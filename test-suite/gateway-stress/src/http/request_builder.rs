use crate::{
    config::{BlockchainConfig, CiphertextConfig},
    db::request_builder::COMMON_PUBLIC_KEY,
    decryption::{
        types::DecryptionType,
        user::{DURATION_SECONDS, EXTRA_DATA},
    },
    eip712::user_decrypt_v2_eip712_signature,
    http::types::HttpDecryptionRequest,
};
use alloy::{
    primitives::{Address, Bytes, U256},
    signers::local::PrivateKeySigner,
};
use anyhow::anyhow;
use kms_connector_api::{
    AttestationType, HandleEntry, PublicDecryptionRequest, RequestValidity, UserDecryptionPayload,
    UserDecryptionRequest,
};
use std::{str::FromStr, time::SystemTime};

/// Builds RFC 033 request bodies whose content-derived decryption ids are all distinct.
///
/// See [README.md](./README.md#http_path_and_decryption_id_uniqueness) section.
pub struct HttpRequestBuilder {
    nonce: U256,
    user_ct: Vec<CiphertextConfig>,
    public_ct: Vec<CiphertextConfig>,
    allowed_contract: Address,
    blockchain: Option<BlockchainConfig>,
}

impl HttpRequestBuilder {
    pub fn new(
        nonce_start: U256,
        user_ct: Vec<CiphertextConfig>,
        public_ct: Vec<CiphertextConfig>,
        allowed_contract: Address,
        blockchain: Option<BlockchainConfig>,
    ) -> Self {
        Self {
            nonce: nonce_start,
            user_ct,
            public_ct,
            allowed_contract,
            blockchain,
        }
    }

    pub async fn build_requests(
        &mut self,
        decryption_type: DecryptionType,
        count: u32,
    ) -> anyhow::Result<Vec<HttpDecryptionRequest>> {
        let mut requests = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let request = match decryption_type {
                DecryptionType::Public => {
                    HttpDecryptionRequest::Public(self.build_public_request()?)
                }
                DecryptionType::UserV2 => {
                    HttpDecryptionRequest::UserV2(self.build_user_v2_request().await?)
                }
                DecryptionType::User => {
                    return Err(anyhow!(
                        "Legacy `user` decryption has no HTTP route, use `user-v2`"
                    ));
                }
            };
            requests.push(request);
        }
        Ok(requests)
    }

    fn build_public_request(&mut self) -> anyhow::Result<PublicDecryptionRequest> {
        if self.public_ct.is_empty() {
            return Err(anyhow!("No `[[public_ct]]` handle configured"));
        }
        Ok(PublicDecryptionRequest {
            ctHandles: self.public_ct.iter().map(|ct| ct.handle).collect(),
            extraData: self.unique_extra_data(),
        })
    }

    async fn build_user_v2_request(&mut self) -> anyhow::Result<UserDecryptionRequest> {
        if self.user_ct.is_empty() {
            return Err(anyhow!("No `[[user_ct]]` handle configured"));
        }
        let blockchain = self.blockchain.clone().ok_or_else(|| {
            anyhow!(
                "`user-v2` decryption requires the [blockchain] section (private key, decryption \
                address, host chain id) in the config file to sign the RFC-016 payload"
            )
        })?;

        let signer = PrivateKeySigner::from_str(&blockchain.private_key)
            .map_err(|e| anyhow!("Invalid private key: {e}"))?;
        let user_address = signer.address();

        let start_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let allowed_contracts = vec![self.allowed_contract];
        let extra_data = self.unique_extra_data();

        let signature = user_decrypt_v2_eip712_signature(
            blockchain.decryption_address,
            blockchain.host_chain_id,
            user_address,
            COMMON_PUBLIC_KEY,
            allowed_contracts.clone(),
            start_timestamp,
            DURATION_SECONDS,
            extra_data.to_vec(),
            &blockchain.private_key,
        )
        .await?;

        Ok(UserDecryptionRequest {
            attestationType: AttestationType::Eip712UnifiedUserDecryptV1.to_string(),
            payload: UserDecryptionPayload {
                handles: self
                    .user_ct
                    .iter()
                    .map(|ct| HandleEntry {
                        handle: ct.handle,
                        contractAddress: self.allowed_contract,
                        ownerAddress: user_address,
                    })
                    .collect(),
                userAddress: user_address,
                publicKey: alloy::hex::decode(COMMON_PUBLIC_KEY)?.into(),
                allowedContracts: allowed_contracts,
                requestValidity: RequestValidity {
                    startTimestamp: start_timestamp,
                    durationSeconds: DURATION_SECONDS,
                },
                extraData: extra_data,
            },
            signature,
        })
    }

    /// The v2 `extraData` suffixed with the next nonce.
    fn unique_extra_data(&mut self) -> Bytes {
        let nonce = self.nonce;
        self.nonce += U256::ONE;
        let mut data = EXTRA_DATA.to_vec();
        data.extend_from_slice(&nonce.to_be_bytes::<32>());
        data.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{B256, FixedBytes};
    use std::collections::HashSet;

    fn builder(blockchain: Option<BlockchainConfig>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(
            U256::from(69),
            vec![CiphertextConfig {
                handle: FixedBytes::repeat_byte(0xaa),
            }],
            vec![
                CiphertextConfig {
                    handle: FixedBytes::repeat_byte(0x11),
                },
                CiphertextConfig {
                    handle: FixedBytes::repeat_byte(0x22),
                },
            ],
            Address::repeat_byte(0x33),
            blockchain,
        )
    }

    fn blockchain() -> BlockchainConfig {
        BlockchainConfig {
            gateway_url: String::new(),
            host_chain_id: 12345,
            gateway_chain_id: 54321,
            decryption_address: Address::repeat_byte(0x44),
            private_key: "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .to_string(),
        }
    }

    #[tokio::test]
    async fn public_requests_have_distinct_ids_and_v2_extra_data() {
        let requests = builder(None)
            .build_requests(DecryptionType::Public, 50)
            .await
            .unwrap();
        let ids: HashSet<B256> = requests.iter().map(|r| r.id()).collect();
        assert_eq!(ids.len(), 50);

        for r in &requests {
            let HttpDecryptionRequest::Public(body) = r else {
                panic!("expected a public request")
            };
            assert_eq!(body.ctHandles.len(), 2);
            assert_eq!(r.handle_count(), 2);
            // v2 header preserved, 32-byte nonce appended.
            assert_eq!(body.extraData.len(), 65 + 32);
            assert_eq!(&body.extraData[..65], &EXTRA_DATA[..]);
        }
    }

    #[tokio::test]
    async fn user_v2_requests_have_distinct_ids_and_sign_the_nonce() {
        let requests = builder(Some(blockchain()))
            .build_requests(DecryptionType::UserV2, 5)
            .await
            .unwrap();
        let ids: HashSet<B256> = requests.iter().map(|r| r.id()).collect();
        assert_eq!(ids.len(), 5);

        let HttpDecryptionRequest::UserV2(body) = &requests[0] else {
            panic!("expected a user request")
        };
        assert_eq!(body.payload.extraData.len(), 65 + 32);
        assert_eq!(
            body.payload.handles[0].ownerAddress,
            body.payload.userAddress
        );
        assert_eq!(body.signature.len(), 65);
        assert_eq!(
            body.payload.requestValidity.durationSeconds,
            DURATION_SECONDS
        );
    }

    #[tokio::test]
    async fn legacy_user_is_rejected() {
        assert!(
            builder(None)
                .build_requests(DecryptionType::User, 1)
                .await
                .is_err()
        );
    }
}
