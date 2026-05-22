//! RFC-012 EIP-712 signature verification for unified user-decryption requests.
//!
//! Implements the `ecrecover` → ERC-1271 fallback specified in RFC-012.
//! The EIP-712 typed-data struct is the unified `UserDecryptRequestVerification` from RFC-016
//! we declare both it and `IERC1271` inline because the unified struct is intentionally
//! absent from `Decryption.sol` (signature verification is off-chain) and `IERC1271` is not
//! part of any in-tree binding.

use IERC1271::isValidSignatureCall;
use alloy::{
    primitives::{Address, B256, Bytes, FixedBytes, Signature, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol,
    sol_types::{Eip712Domain, SolCall, SolStruct},
    transports::RpcError,
};
use fhevm_gateway_bindings::decryption::IDecryption::UserDecryptionRequestPayload;
use thiserror::Error;

sol! {
    /// ERC-1271 signature validation interface (OpenZeppelin v5.1).
    /// Magic value `0x1626ba7e = bytes4(keccak256("isValidSignature(bytes32,bytes)"))`.
    interface IERC1271 {
        function isValidSignature(bytes32 hash, bytes signature) external view returns (bytes4 magicValue);
    }

    /// RFC-016 unified EIP-712 typed-data struct for user-decryption requests.
    /// Field names and order MUST match the SDK's signing payload exactly.
    struct UserDecryptRequestVerification {
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        uint256 startTimestamp;
        uint256 durationSeconds;
        bytes extraData;
    }
}

/// Default EIP-712 domain `name` for the Gateway `Decryption` contract — used by
/// [`default_user_decrypt_domain`] when the caller has only the contract address.
//
// `dead_code` allow: in-tree `DecryptionProcessor` builds its `Eip712Domain` directly from
// `self.domain` (which carries deployment-specific name/version), so it doesn't go through
// the default constructor. Kept exposed for downstream consumers that lack an
// `Eip712DomainMsg` and want the canonical defaults.
pub const DEFAULT_DOMAIN_NAME: &str = "Decryption";
/// Default EIP-712 domain `version` for the Gateway `Decryption` contract.
pub const DEFAULT_DOMAIN_VERSION: &str = "1";
/// ERC-1271 magic return value: `bytes4(keccak256("isValidSignature(bytes32,bytes)"))`.
pub const ERC1271_MAGIC_VALUE: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];

#[derive(Debug, Error)]
pub enum Erc1271Error {
    #[error("ecrecover signer mismatch and userAddress {0} has no contract code on host chain")]
    EoaMismatchNoCode(Address),
    #[error("empty signature is only valid for contracts; userAddress {0} has no contract code")]
    EmptySigOnEoa(Address),
    #[error("ERC-1271 isValidSignature returned non-magic value {1} for userAddress {0}")]
    WrongMagic(Address, FixedBytes<4>),
    #[error(
        "ERC-1271 isValidSignature reverted or returned malformed data for userAddress {0}: {1}"
    )]
    Rejected(Address, String),
    #[error("RPC transport error during ERC-1271 verification: {0}")]
    Transport(String),
}

/// Default `Eip712Domain` builder for callers that only have the host `contractsChainId` and
/// the Gateway `Decryption` contract address. Uses [`DEFAULT_DOMAIN_NAME`] and
/// [`DEFAULT_DOMAIN_VERSION`].
///
/// Callers that already hold the full domain (name/version may differ per deployment) should
/// pass their own `Eip712Domain` directly to [`compute_user_decrypt_digest`] instead.
///
/// `chain_id` is the host `contractsChainId` (extracted from the handle), **not** the Gateway
/// chain id used for KMS gRPC requests.
pub fn default_user_decrypt_domain(chain_id: u64, verifying_contract: Address) -> Eip712Domain {
    Eip712Domain {
        name: Some(DEFAULT_DOMAIN_NAME.into()),
        version: Some(DEFAULT_DOMAIN_VERSION.into()),
        chain_id: Some(U256::from(chain_id)),
        verifying_contract: Some(verifying_contract),
        salt: None,
    }
}

/// Maps the runtime ABI payload onto the unified EIP-712 signing struct and computes the
/// EIP-712 digest the user signed, against the supplied `domain`.
pub fn compute_user_decrypt_digest(
    payload: &UserDecryptionRequestPayload,
    domain: &Eip712Domain,
) -> B256 {
    UserDecryptRequestVerification::from(payload).eip712_signing_hash(domain)
}

/// Try to recover the EOA signer address from a 65-byte signature. Returns `None` for any
/// other length or unparsable signature so the caller can fall through to ERC-1271.
fn try_ecrecover(digest: &B256, signature: &[u8]) -> Option<Address> {
    if signature.len() != 65 {
        return None;
    }
    let sig = Signature::from_raw(signature).ok()?;
    sig.recover_address_from_prehash(digest).ok()
}

/// Gas-bounded static call to `IERC1271(addr).isValidSignature(digest, signature)`.
///
/// Returns `Ok(())` iff returndata length ≥ 32 and the leading bytes4 equals the canonical
/// magic value `0x1626ba7e`. Otherwise:
/// - empty returndata → `EmptySigOnEoa` / `EoaMismatchNoCode` (depending on whether the
///   original signature was empty). At the EVM level, `STATICCALL` to a no-code address
///   succeeds with empty returndata, so this is the dominant signal that `addr` is an EOA.
///   A non-compliant fallback that returns empty bytes also lands here — same rejection
///   outcome, slightly inaccurate error message; not worth a second RPC to disambiguate.
/// - returndata length 1..32 → `Rejected` (non-compliant ABI return);
/// - leading bytes don't match magic → `WrongMagic`;
/// - `isValidSignature` call reverted → `Rejected`;
/// - any other RPC-layer error → `Transport`.
async fn check_erc1271_signature<P: Provider>(
    provider: &P,
    addr: Address,
    digest: B256,
    signature: &[u8],
    gas_limit: u64,
) -> Result<(), Erc1271Error> {
    let call = isValidSignatureCall {
        hash: digest,
        signature: Bytes::copy_from_slice(signature),
    };
    let tx = TransactionRequest::default()
        .to(addr)
        .input(call.abi_encode().into())
        .gas_limit(gas_limit);

    let returndata = provider.call(tx).await.map_err(|err| match err {
        RpcError::ErrorResp(e) => {
            if let Some(revert_data) = e.as_revert_data() {
                // A revert is interpreted as an invalid signature
                Erc1271Error::Rejected(
                    addr,
                    format!("isValidSignature call reverted: {revert_data}"),
                )
            } else {
                Erc1271Error::Transport(e.to_string())
            }
        }
        _ => Erc1271Error::Transport(err.to_string()),
    })?;

    if returndata.is_empty() {
        return Err(if signature.is_empty() {
            Erc1271Error::EmptySigOnEoa(addr)
        } else {
            Erc1271Error::EoaMismatchNoCode(addr)
        });
    }

    // Solidity ABI-encodes `bytes4` as a full 32-byte word, left-aligned with zero padding.
    // A non-compliant fallback function (or a proxy without `isValidSignature`) may return
    // fewer bytes — RFC-012 and OpenZeppelin's `SignatureChecker` require us to reject those
    // before pattern-matching the leading bytes.
    if returndata.len() < 32 {
        return Err(Erc1271Error::Rejected(
            addr,
            format!("returndata length {} < 32", returndata.len()),
        ));
    }

    let magic = FixedBytes::<4>::from_slice(&returndata[..4]);
    if magic.0 == ERC1271_MAGIC_VALUE {
        Ok(())
    } else {
        Err(Erc1271Error::WrongMagic(addr, magic))
    }
}

/// Verify the EIP-712 signature on a user-decryption request per RFC-012.
///
/// 1. If `signature` is non-empty: try `ecrecover`. If it recovers `claimed_signer`, accept.
/// 2. Otherwise (empty signature, mismatch, or unparsable signature): static-call
///    `IERC1271(claimed_signer).isValidSignature(digest, signature)` with a gas cap. Accept
///    iff the call returns the canonical magic value `0x1626ba7e`. Empty returndata is
///    interpreted as "no code at `claimed_signer`" — a single RPC handles both the
///    smart-account path and the EOA-mismatch reject.
pub async fn verify_signature<P: Provider>(
    provider: &P,
    claimed_signer: Address,
    digest: B256,
    signature: &[u8],
    gas_limit: u64,
) -> Result<(), Erc1271Error> {
    if !signature.is_empty()
        && let Some(recovered) = try_ecrecover(&digest, signature)
        && recovered == claimed_signer
    {
        return Ok(());
    }

    check_erc1271_signature(provider, claimed_signer, digest, signature, gas_limit).await
}

impl From<&UserDecryptionRequestPayload> for UserDecryptRequestVerification {
    fn from(value: &UserDecryptionRequestPayload) -> Self {
        UserDecryptRequestVerification {
            userAddress: value.userAddress,
            publicKey: value.publicKey.clone(),
            allowedContracts: value.allowedContracts.clone(),
            startTimestamp: value.requestValidity.startTimestamp,
            durationSeconds: value.requestValidity.durationSeconds,
            extraData: value.extraData.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Bytes, U256},
        providers::{ProviderBuilder, mock::Asserter},
        rpc::json_rpc::ErrorPayload,
        signers::{SignerSync, local::PrivateKeySigner},
    };
    use fhevm_gateway_bindings::decryption::IDecryption::RequestValiditySeconds;
    use serde_json::value::RawValue;
    use std::borrow::Cow;

    const TEST_CHAIN_ID: u64 = 12345;

    fn dummy_payload(user_address: Address) -> UserDecryptionRequestPayload {
        UserDecryptionRequestPayload {
            userAddress: user_address,
            publicKey: Bytes::from(vec![1, 2, 3, 4]),
            allowedContracts: vec![Address::from([0xAB; 20])],
            requestValidity: RequestValiditySeconds {
                startTimestamp: U256::from(1_700_000_000_u64),
                durationSeconds: U256::from(86_400_u64),
            },
            extraData: Bytes::from(vec![0x42]),
            signature: Bytes::default(),
        }
    }

    /// Build a (signer, request payload, digest, signature_bytes) tuple where the signature is
    /// a valid 65-byte ECDSA signature over the EIP-712 digest.
    fn signed_payload(
        gateway_addr: Address,
    ) -> (
        PrivateKeySigner,
        UserDecryptionRequestPayload,
        B256,
        Vec<u8>,
    ) {
        let signer = PrivateKeySigner::random();
        let payload = dummy_payload(signer.address());
        let domain = default_user_decrypt_domain(TEST_CHAIN_ID, gateway_addr);
        let digest = compute_user_decrypt_digest(&payload, &domain);
        let sig = signer.sign_hash_sync(&digest).unwrap();
        let bytes = sig.as_bytes().to_vec();
        assert_eq!(bytes.len(), 65, "ECDSA signature should be 65 bytes");
        (signer, payload, digest, bytes)
    }

    fn mock_provider(asserter: Asserter) -> impl Provider {
        ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter)
    }

    #[tokio::test]
    async fn eoa_valid_signature_zero_rpc() {
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let gateway = Address::from([0xCA; 20]);
        let (signer, _, digest, sig) = signed_payload(gateway);

        verify_signature(&provider, signer.address(), digest, &sig, 100_000)
            .await
            .unwrap();

        // Asserter has no queued responses; if `verify_signature` had made any RPC, the mock
        // would have returned an error. Reaching `Ok(())` proves the EOA fast path was used.
    }

    #[tokio::test]
    async fn eoa_mismatch_no_code_rejects() {
        // STATICCALL to a no-code address returns success with empty returndata at the EVM
        // level — that's our "this address is an EOA" signal.
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        // 65 bytes of a "valid-looking but wrong" signature: ecrecover may recover a random
        // address, which won't match the claimed signer. Any 65 bytes works.
        let sig = vec![0x11_u8; 65];

        asserter.push_success(&Bytes::default());

        let err = verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::EoaMismatchNoCode(_)));
    }

    #[tokio::test]
    async fn empty_sig_on_eoa_rejects() {
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);

        asserter.push_success(&Bytes::default());

        let err = verify_signature(&provider, claimed, digest, &[], 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::EmptySigOnEoa(_)));
    }

    #[tokio::test]
    async fn erc1271_valid_magic_accepts() {
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0x11_u8; 65];

        // isValidSignature → magic value (32-byte left-aligned)
        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&returndata);

        verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn erc1271_wrong_magic_rejects() {
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0x11_u8; 65];

        // 32 bytes of zeros — wrong magic
        asserter.push_success(&[0u8; 32]);

        let err = verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::WrongMagic(..)));
    }

    /// Build a JSON-RPC error payload that mimics what an Ethereum node sends when the call
    /// reverts: a message containing "revert" plus a `data` field whose string value parses
    /// as hex bytes. This is the shape `ErrorPayload::as_revert_data` looks for.
    fn revert_payload(revert_data_hex: &str) -> ErrorPayload {
        let raw = RawValue::from_string(format!("\"{revert_data_hex}\"")).unwrap();
        ErrorPayload::internal_error_with_message_and_obj(Cow::Borrowed("execution reverted"), raw)
    }

    #[tokio::test]
    async fn erc1271_revert_rejects() {
        // A node's revert response carries hex-encoded revert data in the `data` field; that
        // pattern is what `as_revert_data` looks for. Treat it as a deterministic rejection.
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0x11_u8; 65];

        asserter.push_failure(revert_payload("0xdeadbeef"));

        let err = verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::Rejected(_, _)));
    }

    #[tokio::test]
    async fn erc1271_non_revert_rpc_error_maps_to_transport() {
        // A generic JSON-RPC error with no revert data (rate limit, node syncing, transport
        // glitch, …) is recoverable — we can't tell whether it's a contract-level rejection.
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0x11_u8; 65];

        asserter.push_failure_msg("rate limit exceeded");

        let err = verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::Transport(_)));
    }

    #[tokio::test]
    async fn erc1271_short_returndata_rejects() {
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0x11_u8; 65];

        // Only 4 bytes returned — a non-compliant fallback function
        asserter.push_success(&ERC1271_MAGIC_VALUE);

        let err = verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap_err();
        assert!(matches!(err, Erc1271Error::Rejected(_, _)));
    }

    #[tokio::test]
    async fn empty_sig_smart_account_accepts() {
        // Safe `approveHash` flow: empty signature, contract returns magic
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);

        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&returndata);

        verify_signature(&provider, claimed, digest, &[], 100_000)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn unparsable_signature_falls_through_to_erc1271() {
        // A 1-byte "signature" cannot be ecrecovered — must fall through to ERC-1271.
        let asserter = Asserter::new();
        let provider = mock_provider(asserter.clone());
        let claimed = Address::from([0xDE; 20]);
        let digest = B256::from([0u8; 32]);
        let sig = vec![0xFF_u8];

        let mut returndata = [0u8; 32];
        returndata[..4].copy_from_slice(&ERC1271_MAGIC_VALUE);
        asserter.push_success(&returndata);

        verify_signature(&provider, claimed, digest, &sig, 100_000)
            .await
            .unwrap();
    }
}
