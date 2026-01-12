use std::collections::HashMap;

use alloy::{
    hex,
    primitives::{Address, Bytes, FixedBytes, Signature, U256, keccak256},
    providers::Provider,
    sol,
    sol_types::{Eip712Domain, SolStruct},
    transports::http::reqwest::Client,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::gateway_config::GatewayConfig::GatewayConfigInstance;
use serde::Deserialize;
use tracing::{debug, warn};

sol! {
    struct CiphertextResponseAttestation {
        bytes32 handle;
        uint256 keyId;
        bytes32 ciphertextDigest;
        uint256 epochId;
    }
}

#[derive(Clone, Debug)]
pub struct CoprocessorApi<P: Provider> {
    gateway_config_contract: GatewayConfigInstance<P>,
    http_client: Client,
    domain: Eip712Domain,
}

#[derive(Clone, Debug)]
pub struct CiphertextMaterial {
    pub handle: FixedBytes<32>,
    pub key_id: U256,
    pub epoch_id: U256,
    pub sns_ciphertext: Vec<u8>,
    pub sns_ciphertext_digest: FixedBytes<32>,
    pub ciphertext_format: Option<i16>,
}

#[derive(Debug, Clone)]
struct CoprocessorInfo {
    signer_address: Address,
    api_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CiphertextApiResponse {
    pub status: CiphertextStatus,
    pub handle: FixedBytes<32>,
    pub key_id: Option<U256>,
    pub sns_ciphertext: Option<Bytes>,
    pub sns_ciphertext_digest: Option<FixedBytes<32>>,
    pub ciphertext_format: Option<i16>,
    pub epoch_id: Option<U256>,
    pub signature: Option<Bytes>,
    pub signer_address: Option<Address>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum CiphertextStatus {
    Found,
    NotFound,
}

impl<P> CoprocessorApi<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    pub fn new(
        gateway_config_contract: GatewayConfigInstance<P>,
        http_client: Client,
        domain: Eip712Domain,
    ) -> Self {
        Self {
            gateway_config_contract,
            http_client,
            domain,
        }
    }

    pub async fn fetch_ciphertext(
        &self,
        handle: FixedBytes<32>,
    ) -> anyhow::Result<CiphertextMaterial> {
        let coprocessors = self.fetch_coprocessors().await?;
        if coprocessors.is_empty() {
            return Err(anyhow!("No coprocessors with APIs registered"));
        }

        let quorum = if coprocessors.len() >= 2 { 2 } else { 1 };
        let mut digest_counts: HashMap<FixedBytes<32>, usize> = HashMap::new();
        let mut digest_values: HashMap<FixedBytes<32>, CiphertextMaterial> = HashMap::new();
        let mut first_valid: Option<CiphertextMaterial> = None;

        for coprocessor in coprocessors {
            let response = match self
                .fetch_from_coprocessor(&coprocessor.api_url, handle)
                .await
            {
                Ok(response) => response,
                Err(err) => {
                    warn!(
                        error = %err,
                        api_url = %coprocessor.api_url,
                        "Failed to fetch ciphertext from coprocessor"
                    );
                    continue;
                }
            };

            let Some(material) =
                self.validate_response(handle, &coprocessor, response).await?
            else {
                continue;
            };

            let count = digest_counts
                .entry(material.sns_ciphertext_digest)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            if first_valid.is_none() {
                first_valid = Some(material.clone());
            }
            digest_values
                .entry(material.sns_ciphertext_digest)
                .or_insert_with(|| material.clone());

            if *count >= quorum {
                return Ok(material);
            }
        }

        if let Some(material) = first_valid {
            return Ok(material);
        }

        Err(anyhow!(
            "No coprocessor quorum reached for ciphertext handle: 0x{}",
            hex::encode(handle)
        ))
    }

    async fn fetch_coprocessors(&self) -> anyhow::Result<Vec<CoprocessorInfo>> {
        let response = self
            .gateway_config_contract
            .getCoprocessorsWithApis()
            .call()
            .await?;

        Ok(response
            .into_iter()
            .filter_map(|copro| {
                let api_url = copro.apiUrl.trim().to_string();
                if api_url.is_empty() {
                    return None;
                }

                Some(CoprocessorInfo {
                    signer_address: copro.signerAddress,
                    api_url,
                })
            })
            .collect())
    }

    async fn fetch_from_coprocessor(
        &self,
        api_url: &str,
        handle: FixedBytes<32>,
    ) -> anyhow::Result<CiphertextApiResponse> {
        let url = format!(
            "{}/v1/ciphertext/0x{}",
            api_url.trim_end_matches('/'),
            hex::encode(handle)
        );

        let response = self.http_client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!(
                "Coprocessor API request failed with status {}",
                response.status()
            ));
        }

        response.json::<CiphertextApiResponse>().await.map_err(Into::into)
    }

    async fn validate_response(
        &self,
        requested_handle: FixedBytes<32>,
        coprocessor: &CoprocessorInfo,
        response: CiphertextApiResponse,
    ) -> anyhow::Result<Option<CiphertextMaterial>> {
        match response.status {
            CiphertextStatus::Found => {}
            CiphertextStatus::NotFound => {
                debug!(
                    api_url = %coprocessor.api_url,
                    reason = response.reason.as_deref().unwrap_or("missing"),
                    "Coprocessor did not store ciphertext"
                );
                return Ok(None);
            }
        }

        if response.handle != requested_handle {
            warn!(
                api_url = %coprocessor.api_url,
                "Coprocessor returned mismatched handle"
            );
            return Ok(None);
        }

        let key_id = response
            .key_id
            .ok_or_else(|| anyhow!("Missing key_id in coprocessor response"))?;
        let sns_ciphertext = response
            .sns_ciphertext
            .ok_or_else(|| anyhow!("Missing sns_ciphertext in coprocessor response"))?;
        let sns_ciphertext_digest = response
            .sns_ciphertext_digest
            .ok_or_else(|| anyhow!("Missing sns_ciphertext_digest in coprocessor response"))?;
        let epoch_id = response
            .epoch_id
            .ok_or_else(|| anyhow!("Missing epoch_id in coprocessor response"))?;
        let signature = response
            .signature
            .ok_or_else(|| anyhow!("Missing signature in coprocessor response"))?;

        let computed_digest = keccak256(sns_ciphertext.as_ref());
        if computed_digest != sns_ciphertext_digest {
            warn!(
                api_url = %coprocessor.api_url,
                "Coprocessor ciphertext digest mismatch"
            );
            return Ok(None);
        }

        let attestation = CiphertextResponseAttestation {
            handle: requested_handle,
            keyId: key_id,
            ciphertextDigest: sns_ciphertext_digest,
            epochId: epoch_id,
        };
        let signing_hash = attestation.eip712_signing_hash(&self.domain);
        let signature = Signature::try_from(signature.as_ref())?;
        let recovered = signature.recover_address_from_prehash(&signing_hash)?;

        if recovered != coprocessor.signer_address {
            warn!(
                api_url = %coprocessor.api_url,
                expected = %coprocessor.signer_address,
                recovered = %recovered,
                "Coprocessor signature does not match configured signer"
            );
            return Ok(None);
        }

        if let Some(reported_signer) = response.signer_address {
            if reported_signer != recovered {
                warn!(
                    api_url = %coprocessor.api_url,
                    reported = %reported_signer,
                    recovered = %recovered,
                    "Coprocessor signer address mismatch"
                );
                return Ok(None);
            }
        }

        Ok(Some(CiphertextMaterial {
            handle: requested_handle,
            key_id,
            epoch_id,
            sns_ciphertext: sns_ciphertext.to_vec(),
            sns_ciphertext_digest,
            ciphertext_format: response.ciphertext_format,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::Address,
        providers::{ProviderBuilder, mock::Asserter},
        signers::{Signer, local::PrivateKeySigner},
    };
    use fhevm_gateway_bindings::gateway_config::GatewayConfig;

    fn build_domain() -> Eip712Domain {
        Eip712Domain {
            name: Some("FHEVM".into()),
            version: Some("1".into()),
            chain_id: Some(U256::from(1u64)),
            verifying_contract: Some(Address::ZERO),
            salt: None,
        }
    }

    fn build_api() -> CoprocessorApi<impl Provider + Clone + Send + Sync + 'static> {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        let gateway_config = GatewayConfig::new(Address::ZERO, provider);
        CoprocessorApi::new(gateway_config, Client::new(), build_domain())
    }

    #[tokio::test]
    async fn test_validate_response_accepts_valid_signature() {
        let api = build_api();
        let signer = PrivateKeySigner::random();
        let signer_address = signer.address();

        let handle = FixedBytes::from([0x11u8; 32]);
        let key_id = U256::from(7u64);
        let epoch_id = U256::from(42u64);
        let sns_ciphertext = vec![1u8, 2, 3, 4];
        let sns_ciphertext_digest = FixedBytes::from(keccak256(&sns_ciphertext));

        let attestation = CiphertextResponseAttestation {
            handle,
            keyId: key_id,
            ciphertextDigest: sns_ciphertext_digest,
            epochId: epoch_id,
        };
        let signing_hash = attestation.eip712_signing_hash(&build_domain());
        let signature = signer.sign_hash(&signing_hash).await.unwrap();

        let response = CiphertextApiResponse {
            status: CiphertextStatus::Found,
            handle,
            key_id: Some(key_id),
            sns_ciphertext: Some(Bytes::from(sns_ciphertext.clone())),
            sns_ciphertext_digest: Some(sns_ciphertext_digest),
            ciphertext_format: Some(11), // CompressedOnCpu
            epoch_id: Some(epoch_id),
            signature: Some(Bytes::from(Vec::from(signature))),
            signer_address: Some(signer_address),
            reason: None,
        };

        let coprocessor = CoprocessorInfo {
            signer_address,
            api_url: "http://localhost".to_string(),
        };

        let material = api
            .validate_response(handle, &coprocessor, response)
            .await
            .unwrap();
        let material = material.expect("expected valid response");
        assert_eq!(material.handle, handle);
        assert_eq!(material.key_id, key_id);
        assert_eq!(material.epoch_id, epoch_id);
        assert_eq!(material.sns_ciphertext, sns_ciphertext);
        assert_eq!(material.sns_ciphertext_digest, sns_ciphertext_digest);
        assert_eq!(material.ciphertext_format, Some(11));
    }

    #[tokio::test]
    async fn test_validate_response_rejects_invalid_signature() {
        let api = build_api();
        let signer = PrivateKeySigner::random();
        let wrong_signer = PrivateKeySigner::random();

        let handle = FixedBytes::from([0x22u8; 32]);
        let key_id = U256::from(1u64);
        let epoch_id = U256::from(2u64);
        let sns_ciphertext = vec![9u8, 8, 7];
        let sns_ciphertext_digest = FixedBytes::from(keccak256(&sns_ciphertext));

        let attestation = CiphertextResponseAttestation {
            handle,
            keyId: key_id,
            ciphertextDigest: sns_ciphertext_digest,
            epochId: epoch_id,
        };
        let signing_hash = attestation.eip712_signing_hash(&build_domain());
        let signature = wrong_signer.sign_hash(&signing_hash).await.unwrap();

        let response = CiphertextApiResponse {
            status: CiphertextStatus::Found,
            handle,
            key_id: Some(key_id),
            sns_ciphertext: Some(Bytes::from(sns_ciphertext)),
            sns_ciphertext_digest: Some(sns_ciphertext_digest),
            ciphertext_format: Some(10), // UncompressedOnCpu
            epoch_id: Some(epoch_id),
            signature: Some(Bytes::from(Vec::from(signature))),
            signer_address: Some(wrong_signer.address()),
            reason: None,
        };

        let coprocessor = CoprocessorInfo {
            signer_address: signer.address(),
            api_url: "http://localhost".to_string(),
        };

        let material = api
            .validate_response(handle, &coprocessor, response)
            .await
            .unwrap();
        assert!(material.is_none());
    }
}
