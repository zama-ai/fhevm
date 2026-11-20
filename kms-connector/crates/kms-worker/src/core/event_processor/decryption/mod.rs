//! Decryption processing, with one submodule per decryption flow.

pub(super) mod pad;
mod public;
mod user;

use crate::core::{
    config::Config,
    event_processor::{CiphertextManager, HostRpcClient, RequestCheckError, RequestCheckKind},
};
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol_types::Eip712Domain,
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::Eip712DomainMsg;
use sqlx::types::chrono::Utc;
use std::collections::HashMap;

#[derive(Clone)]
/// The struct responsible of processing incoming decryption requests.
pub struct DecryptionProcessor<GP: Provider, HP: Provider> {
    /// The EIP712 domain of the `Decryption` contract.
    domain: Eip712DomainMsg,

    /// The instance of the `Decryption` contract used to check decryption were not already done.
    decryption_contract: DecryptionInstance<GP>,

    /// The instances of the host chains `ACL` contracts used to check the decryption ACL.
    host_clients: HashMap<u64, HostRpcClient<HP>>,

    /// The entity used to verify and collect the ciphertexts of decryption requests.
    ciphertext_manager: CiphertextManager<GP>,

    /// Gas cap for the `IERC1271.isValidSignature` static call (RFC-012).
    erc1271_gas_limit: u64,

    /// The maximum number of handles a single decryption request can contain.
    max_handles_per_decryption: usize,
}

impl<GP, HP> DecryptionProcessor<GP, HP>
where
    GP: Provider + Clone + 'static,
    HP: Provider,
{
    pub fn new(
        config: &Config,
        gateway_provider: GP,
        host_clients: HashMap<u64, HostRpcClient<HP>>,
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
            host_clients,
            ciphertext_manager,
            erc1271_gas_limit: config.erc1271_gas_limit,
            max_handles_per_decryption: config.max_handles_per_decryption.get(),
        }
    }

    /// Rejects decryption requests containing more handles than the configured limit.
    pub fn check_handle_count(&self, handle_count: usize) -> Result<(), RequestCheckError> {
        if handle_count > self.max_handles_per_decryption {
            return Err(RequestCheckError::irrecoverable(
                RequestCheckKind::HandleLimit,
                ErrorCode::Unprocessable,
                anyhow!(
                    "Request contains {handle_count} handles, exceeding the limit of {}",
                    self.max_handles_per_decryption
                ),
            ));
        }
        Ok(())
    }

    /// Returns the client of the host chain `chain_id`.
    fn host_client(&self, chain_id: u64) -> Result<&HostRpcClient<HP>, RequestCheckError> {
        self.host_clients.get(&chain_id).ok_or_else(|| {
            RequestCheckError::recoverable(
                RequestCheckKind::Acl,
                ErrorCode::UpstreamTransient,
                anyhow!("No ACL contract config found for chain id {chain_id}"),
            )
        })
    }

    /// The EIP-712 domain of user decryption signatures for the host chain `chain_id`.
    ///
    /// Name, version and verifying contract are the Gateway `Decryption` ones of `self.domain`
    /// (validated at startup), but the chain id is the host one: `self.domain` targets KMS gRPC
    /// requests, while users sign for the host chain.
    fn user_decrypt_domain(&self, chain_id: u64) -> Eip712Domain {
        Eip712Domain {
            name: Some(self.domain.name.clone().into()),
            version: Some(self.domain.version.clone().into()),
            chain_id: Some(U256::from(chain_id)),
            verifying_contract: Some(*self.decryption_contract.address()),
            salt: None,
        }
    }
}

/// Checks the user-signed permit fields shared by user decryption v2 and pad decryption:
/// - the current time is within the validity window `[start, start + duration]`
/// - `user_address` is not in `allowed_contracts`
fn check_permit(
    user_address: Address,
    allowed_contracts: &[Address],
    start: U256,
    duration: U256,
) -> Result<(), RequestCheckError> {
    let now = U256::from(Utc::now().timestamp() as u64);
    let end = start.saturating_add(duration);
    if now < start {
        return Err(RequestCheckError::recoverable(
            RequestCheckKind::Signature,
            ErrorCode::UserSignatureRejected,
            anyhow!("Request not yet valid: now {now} < startTimestamp {start}"),
        ));
    }
    if now > end {
        return Err(RequestCheckError::irrecoverable(
            RequestCheckKind::Signature,
            ErrorCode::UserDecryptionExpired,
            anyhow!("Request validity window expired: now {now} > end {end}"),
        ));
    }
    if allowed_contracts.contains(&user_address) {
        return Err(RequestCheckError::irrecoverable(
            RequestCheckKind::Signature,
            ErrorCode::Unprocessable,
            anyhow!("userAddress {user_address} is listed in allowedContracts"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::{ProcessingError, ProcessingErrorKind};
    use alloy::{
        primitives::{Address, B256},
        providers::{ProviderBuilder, mock::Asserter},
    };
    use connector_utils::types::handle::extract_chain_id_from_handle;
    use fhevm_host_bindings::acl::ACL;
    use rstest::rstest;
    use std::num::NonZeroUsize;

    pub(super) const CHAIN_ID: u64 = 12345;

    pub(super) enum ExpectedOutcome {
        Ok,
        Recoverable,
        Irrecoverable,
    }

    pub(super) fn assert_kind(result: &Result<(), ProcessingError>, expected: &ExpectedOutcome) {
        match expected {
            ExpectedOutcome::Ok => unreachable!(),
            ExpectedOutcome::Recoverable => {
                assert_eq!(
                    result.as_ref().unwrap_err().kind,
                    ProcessingErrorKind::Recoverable
                )
            }
            ExpectedOutcome::Irrecoverable => {
                assert_eq!(
                    result.as_ref().unwrap_err().kind,
                    ProcessingErrorKind::Irrecoverable
                )
            }
        }
    }

    /// A processor with a host chain client for the chain of `handle`.
    pub(super) fn setup_test_processor(
        asserter: Asserter,
        handle: B256,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>> {
        let chain_id = extract_chain_id_from_handle(&handle).unwrap();
        setup_test_processor_with_config(asserter, chain_id, Config::default())
    }

    pub(super) fn setup_test_processor_with_config(
        asserter: Asserter,
        chain_id: u64,
        config: Config,
    ) -> DecryptionProcessor<impl Provider + Clone + use<>, impl Provider + use<>> {
        let mock_provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let host_clients = HashMap::from([(
            chain_id,
            HostRpcClient::new(
                chain_id,
                ACL::new(Address::default(), mock_provider.clone()),
            ),
        )]);
        let ciphertext_manager = CiphertextManager::for_test(mock_provider.clone());
        DecryptionProcessor::new(&config, mock_provider, host_clients, ciphertext_manager)
    }

    #[rstest]
    #[case::below_limit(1, ExpectedOutcome::Ok)]
    #[case::at_limit(2, ExpectedOutcome::Ok)]
    #[case::above_limit(3, ExpectedOutcome::Irrecoverable)]
    #[test]
    fn check_handle_count(#[case] handle_count: usize, #[case] expected: ExpectedOutcome) {
        let config = Config {
            max_handles_per_decryption: NonZeroUsize::new(2).unwrap(),
            ..Default::default()
        };
        let decryption_processor =
            setup_test_processor_with_config(Asserter::new(), CHAIN_ID, config);

        let result = decryption_processor
            .check_handle_count(handle_count)
            .map_err(RequestCheckError::record);

        match expected {
            ExpectedOutcome::Ok => result.unwrap(),
            _ => assert_kind(&result, &expected),
        }
    }
}
