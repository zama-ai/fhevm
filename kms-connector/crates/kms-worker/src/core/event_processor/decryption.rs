use crate::core::{
    config::Config,
    event_processor::{CiphertextManager, ProcessingError, ciphertext::VerifiedCiphertexts},
};
use alloy::{
    consensus::Transaction,
    primitives::{Address, B256, Bytes, FixedBytes, U256},
    providers::Provider,
};
use anyhow::anyhow;
use connector_utils::types::{KmsGrpcRequest, extra_data::parse_extra_data, u256_to_request_id};
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    Eip712DomainMsg, PublicDecryptionRequest, SigningSchemeType, UserDecryptionRequest,
};
use zama_solana_permit::PermitFields;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The entity used to verify and collect the ciphertexts of decryption requests.
    ciphertext_manager: CiphertextManager<GP>,
}

impl<GP> DecryptionProcessor<GP>
where
    GP: Provider + Clone + 'static,
{
    pub fn new(
        config: &Config,
        gateway_provider: GP,
        ciphertext_manager: CiphertextManager<GP>,
    ) -> Self {
        let domain = Eip712DomainMsg {
            name: config.decryption_contract.domain_name.clone(),
            version: config.decryption_contract.domain_version.clone(),
            chain_id: U256::from(config.gateway_chain_id).to_be_bytes_vec(),
            verifying_contract: config.decryption_contract.address.to_string(),
            salt: None,
        };
        let decryption_contract =
            Decryption::new(config.decryption_contract.address, gateway_provider);

        Self {
            domain,
            decryption_contract,
            ciphertext_manager,
        }
    }

    pub async fn prepare_decryption_request(
        &self,
        decryption_id: U256,
        handles: &[B256],
        extra_data: &Bytes,
        user_decrypt_data: Option<UserDecryptionExtraData>,
    ) -> Result<KmsGrpcRequest, ProcessingError> {
        if handles.is_empty() {
            return Err(ProcessingError::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!("No handles found in the request, cannot proceed"),
            ));
        }

        let parsed_extra_data = parse_extra_data(extra_data)
            .map_err(|e| ProcessingError::irrecoverable(ErrorCode::Unprocessable, e))?;

        let VerifiedCiphertexts {
            ciphertexts,
            key_id,
        } = self.ciphertext_manager.verify_and_retrieve(handles).await?;

        let request_id = Some(u256_to_request_id(decryption_id));
        let kms_extra_data = kms_decryption_extra_data(extra_data);

        if let Some(user_decrypt_data) = user_decrypt_data {
            let client_address = user_decrypt_data.client_address;
            let enc_key = user_decrypt_data.public_key.to_vec();
            let user_decryption_request = UserDecryptionRequest {
                request_id,
                client_address,
                key_id: Some(u256_to_request_id(key_id)),
                domain: Some(self.domain.clone()),
                enc_key,
                typed_ciphertexts: ciphertexts,
                extra_data: kms_extra_data,
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
                signing_metadata: user_decrypt_data.signing_metadata,
                // Currently hardcoded to Ecdsa256k1 as it is the only scheme used for Ethereum.
                // Solana responses are EIP-712 signed under the Gateway domain too.
                signing_schemes: vec![SigningSchemeType::Ecdsa256k1 as i32],
            };

            Ok(user_decryption_request.into())
        } else {
            let public_decryption_request = PublicDecryptionRequest {
                request_id,
                ciphertexts,
                key_id: Some(u256_to_request_id(key_id)),
                domain: Some(self.domain.clone()),
                extra_data: kms_extra_data,
                epoch_id: parsed_extra_data.epoch_id.map(u256_to_request_id),
                context_id: parsed_extra_data.context_id.map(u256_to_request_id),
                // Currently hardcoded to Ecdsa256k1 as it is the only scheme used for Ethereum.
                signing_schemes: vec![SigningSchemeType::Ecdsa256k1 as i32],
            };
            Ok(public_decryption_request.into())
        }
    }

    /// Fetches the calldata of a given transaction.
    ///
    /// Only allows transactions sent directly to the `Decryption` contract.
    pub async fn fetch_calldata(
        &self,
        tx_hash: FixedBytes<32>,
    ) -> Result<Vec<u8>, ProcessingError> {
        let decryption_address = *self.decryption_contract.address();

        let tx = self
            .decryption_contract
            .provider()
            .get_transaction_by_hash(tx_hash)
            .await
            .map_err(ProcessingError::transient)?
            .ok_or_else(|| {
                ProcessingError::transient(anyhow!("No transaction found with hash {tx_hash}!"))
            })?;

        if tx.to() != Some(decryption_address) {
            return Err(ProcessingError::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!(
                    "Transaction {tx_hash} was sent to {:?} rather than directly to the Decryption \
                    contract {decryption_address}: its calldata cannot be associated with the user \
                    decryption event.",
                    tx.to(),
                ),
            ));
        }

        Ok(tx.input().to_vec())
    }
}

fn kms_decryption_extra_data(extra_data: &Bytes) -> Vec<u8> {
    // relayer-sdk <=0.4.2 sends 0x00 but verifies the KMS signature against empty extraData.
    if extra_data.as_ref() == [0x00] {
        Vec::new()
    } else {
        extra_data.to_vec()
    }
}

pub struct UserDecryptionExtraData {
    /// The checksummed EVM user address. Empty for Solana requests.
    pub client_address: String,
    pub public_key: Bytes,
    pub signing_metadata: Vec<kms_grpc::kms::v1::SigningMetadata>,
}

impl UserDecryptionExtraData {
    pub fn new(user_address: Address, public_key: Bytes) -> Self {
        Self {
            client_address: user_address.to_checksum(None),
            public_key,
            signing_metadata: vec![],
        }
    }

    /// RFC-021: the KMS identifies a Solana user by its ed25519 pubkey, so `client_address`
    /// stays empty and the identity travels in the signing metadata.
    pub fn new_solana(permit: &PermitFields) -> Self {
        Self {
            client_address: String::new(),
            public_key: Bytes::copy_from_slice(permit.transport_key().as_bytes()),
            signing_metadata: vec![kms_grpc::kms::v1::SigningMetadata::solana(
                permit.user_address().as_bytes().to_vec(),
                permit.verifying_program_id().as_bytes().to_vec(),
            )],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use alloy::{
        hex,
        providers::{ProviderBuilder, RootProvider, mock::Asserter},
        rpc::types::Transaction as RpcTransaction,
        sol_types::SolCall,
    };
    use connector_utils::tests::rand::{rand_address, rand_digest, rand_handle};
    use fhevm_gateway_bindings::decryption::Decryption::userDecryptionRequest_2Call as userDecryptionRequestCall;
    use rstest::rstest;

    fn setup_test_processor(
        asserter: Asserter,
        config: Config,
    ) -> DecryptionProcessor<RootProvider> {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let ciphertext_manager = CiphertextManager::for_test(mock_provider.clone());
        DecryptionProcessor::new(&config, mock_provider, ciphertext_manager)
    }

    #[test]
    fn kms_decryption_extra_data_normalizes_legacy_zero_marker() {
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x00])),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn kms_decryption_extra_data_keeps_empty_and_versioned_values() {
        assert_eq!(kms_decryption_extra_data(&Bytes::new()), Vec::<u8>::new());
        assert_eq!(
            kms_decryption_extra_data(&Bytes::from_static(&[0x01, 0x02])),
            vec![0x01, 0x02]
        );
    }

    /// Where the legacy request transaction was sent, relative to the Decryption contract.
    enum TxTarget {
        /// Sent directly to the Decryption contract (the only accepted case).
        Decryption,
        /// Sent to some intermediary contract.
        Intermediary,
        /// Contract-creation transaction (`to` is `None`). Should be unreachable in theory.
        Creation,
    }

    /// Only calldata coming from a transaction sent directly to the Decryption contract can be
    /// associated with the user decryption event, so any other target is rejected.
    #[rstest]
    #[case::accepts_direct_decryption_tx(TxTarget::Decryption, true)]
    #[case::rejects_tx_not_sent_to_decryption_contract(TxTarget::Intermediary, false)]
    #[case::rejects_contract_creation_tx(TxTarget::Creation, false)]
    #[tokio::test]
    async fn fetch_calldata_only_accepts_direct_decryption_tx(
        #[case] target: TxTarget,
        #[case] should_succeed: bool,
    ) {
        let asserter = Asserter::new();
        let decryption_address = rand_address();
        let mut config = Config::default();
        config.decryption_contract.address = decryption_address;
        let processor = setup_test_processor(asserter.clone(), config);

        let tx_hash = rand_digest();
        let calldata = userDecryptionRequestCall::default().abi_encode();

        let to = match target {
            TxTarget::Decryption => Some(decryption_address),
            TxTarget::Intermediary => Some(rand_address()),
            TxTarget::Creation => None,
        };
        asserter.push_success(&mock_legacy_request_tx(tx_hash, to, &calldata));

        let result = processor.fetch_calldata(tx_hash).await;
        if should_succeed {
            assert_eq!(result.unwrap(), calldata);
        } else {
            let err = result.unwrap_err();
            assert_eq!(err.kind, ProcessingErrorKind::Irrecoverable);
        }
    }

    /// Builds the mocked `eth_getTransactionByHash` response for a legacy user decryption
    /// request carrying `calldata`, sent to `to` (`None` models a contract-creation tx).
    fn mock_legacy_request_tx(
        tx_hash: FixedBytes<32>,
        to: Option<Address>,
        calldata: &[u8],
    ) -> RpcTransaction {
        serde_json::from_value(serde_json::json!({
            "hash": tx_hash,
            "nonce": "0x0",
            "blockHash": null,
            "blockNumber": null,
            "transactionIndex": null,
            "from": Address::ZERO,
            "to": to,
            "value": "0x0",
            "gasPrice": "0x0",
            "gas": "0x0",
            "input": format!("0x{}", hex::encode(calldata)),
            "v": "0x1b",
            "r": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "s": "0x0000000000000000000000000000000000000000000000000000000000000001",
            "type": "0x0"
        }))
        .unwrap()
    }

    #[test]
    fn evm_user_decryption_keeps_the_checksummed_address() {
        let address = Address::repeat_byte(0x11);
        let data = UserDecryptionExtraData::new(address, Bytes::from_static(&[0x22]));

        assert_eq!(data.client_address, address.to_checksum(None));
        assert!(data.signing_metadata.is_empty());
    }

    #[test]
    fn solana_extra_data_identifies_the_user_by_pubkey() {
        // The fixture permit is signed by `[1; 32]` for program `[7; 32]`.
        let request = connector_utils::tests::rand::solana_user_decryption_request(
            U256::from(1),
            rand_handle(),
        );
        let data = UserDecryptionExtraData::new_solana(request.permit());

        assert!(data.client_address.is_empty());
        assert_eq!(
            data.signing_metadata,
            vec![kms_grpc::kms::v1::SigningMetadata::solana(
                vec![1; 32],
                vec![7; 32]
            )]
        );
    }
}
