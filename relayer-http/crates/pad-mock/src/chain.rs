//! The host chain side: the ABI of the cleartext contracts the mock reads (`CleartextPaddedInputter`, `KMSVerifier`,
//! `ProtocolConfig`), the EIP-712 structs the KMS nodes and the user sign, and a minimal JSON-RPC client for
//! `eth_call`.

use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::sol;
use alloy::sol_types::{Eip712Domain, SolCall};
use kms_connector_api::PadDecryptionRequest as ConnectorRequest;
use serde_json::{Value, json};
use url::Url;

sol! {
    // `CleartextPaddedInputter`'s request and material types (sdk/js-sdk/contracts/src/v1.0.0/cleartext/
    // CleartextPaddedInputter.sol), field for field.

    #[derive(Debug, PartialEq)]
    struct UserDecryptPermitV2 {
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        uint256 startTimestamp;
        uint256 durationSeconds;
        bytes extraData;
    }

    #[derive(Debug, PartialEq)]
    struct PadWitness {
        address submitter;
        address consumer;
        uint64 expirationTimestamp;
        address[] decryptors;
        bytes16 nonce;
        bytes32 label;
        uint16 padLength;
    }

    #[derive(Debug, PartialEq)]
    struct PadDecryptRequestPayload {
        uint8 witnessVersion;
        uint16 prfId;
        uint64 chainId;
        PadWitness[] witnesses;
    }

    #[derive(Debug, PartialEq)]
    struct PadDecryptRequest {
        UserDecryptPermitV2 auth;
        bytes signature;
        PadDecryptRequestPayload payload;
    }

    #[derive(Debug, PartialEq)]
    struct PadShare {
        bytes32 label;
        bytes rawShares;
        uint32 partyId;
        uint32 degree;
        bytes16 sessionId;
        bytes signature;
    }

    #[derive(Debug, PartialEq)]
    struct PadDecryptResponsePayload {
        PadShare[] shares;
        bytes extraData;
    }

    #[derive(Debug, PartialEq)]
    struct CleartextPadSharesMaterial {
        bytes32 requestId;
        PadShare[] shares;
        address[] signers;
        uint32[] partyIds;
        bytes32[][] shareDigests;
        uint256 threshold;
    }

    #[derive(Debug, PartialEq)]
    struct CleartextPadResponsesMaterial {
        bytes32 requestId;
        address[] signers;
        bytes[] encryptedPayloads;
        bytes32[] transportDigests;
    }

    function getCleartextPadSharesMaterial(PadDecryptRequest request)
        external view returns (CleartextPadSharesMaterial material);
    function getCleartextPadResponsesMaterial(PadDecryptRequest request, bytes[][] shareSignatures)
        external view returns (CleartextPadResponsesMaterial responses);
    function cleartextDecryptPadPayload(bytes encryptedPayload, bytes publicKey)
        external pure returns (PadDecryptResponsePayload payload);
    // KMSVerifier (ERC-5267) and ProtocolConfig.
    function eip712Domain() external view returns (
        bytes1 fields, string name, string version, uint256 chainId, address verifyingContract, bytes32 salt,
        uint256[] extensions
    );
    function getKmsSignersForContext(uint256 contextId) external view returns (address[] signers);

    // The EIP-712 structs: the user's permit (RFC-016) and the two KMS signatures of RFC-030.

    #[derive(Debug)]
    struct UserDecryptRequestVerification {
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        uint256 startTimestamp;
        uint256 durationSeconds;
        bytes extraData;
    }

    #[derive(Debug)]
    struct PadShareVerification {
        bytes32 label;
        bytes rawShares;
        uint32 partyId;
        uint32 degree;
        bytes16 sessionId;
        bytes extraData;
    }

    #[derive(Debug)]
    struct PadDecryptResponseVerification {
        bytes32 requestId;
        bytes encryptedPayload;
    }
}

/// The connector request as `CleartextPaddedInputter` takes it: the same fields, each pad's label and length inside
/// its witness, the window as `uint256`.
pub fn contract_request(request: &ConnectorRequest) -> PadDecryptRequest {
    let payload = &request.payload;
    PadDecryptRequest {
        auth: UserDecryptPermitV2 {
            userAddress: payload.userAddress,
            publicKey: payload.publicKey.clone(),
            allowedContracts: payload.allowedContracts.clone(),
            startTimestamp: U256::from(payload.requestValidity.startTimestamp),
            durationSeconds: U256::from(payload.requestValidity.durationSeconds),
            extraData: payload.extraData.clone(),
        },
        signature: request.signature.clone(),
        payload: PadDecryptRequestPayload {
            witnessVersion: payload.witnessVersion,
            prfId: payload.prfId,
            chainId: payload.chainId,
            witnesses: payload
                .pads
                .iter()
                .map(|p| PadWitness {
                    submitter: p.witness.submitter,
                    consumer: p.witness.consumer,
                    expirationTimestamp: p.witness.expirationTimestamp,
                    decryptors: p.witness.decryptors.clone(),
                    nonce: p.witness.nonce,
                    label: p.label,
                    padLength: p.padLength,
                })
                .collect(),
        },
    }
}

/// A KMSVerifier's ERC-5267 domain with another chain id: the user signs its permit under the host chain's.
pub fn domain(reported: &eip712DomainReturn, chain_id: U256) -> Eip712Domain {
    Eip712Domain::new(
        Some(reported.name.clone().into()),
        Some(reported.version.clone().into()),
        Some(chain_id),
        Some(reported.verifyingContract),
        None,
    )
}

/// The domain the KMS nodes sign pad decryption responses under (RFC-030): `fhevm-pad-decryption`, version 1,
/// anchored on ProtocolConfig's chain id and address (`CleartextPaddedInputter._responseDomainHash`).
pub fn pad_decryption_domain(chain_id: u64, protocol_config: Address) -> Eip712Domain {
    Eip712Domain::new(
        Some("fhevm-pad-decryption".into()),
        Some("1".into()),
        Some(U256::from(chain_id)),
        Some(protocol_config),
        None,
    )
}

/// Why a JSON-RPC call failed.
#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    /// The node is unreachable or answered something that is not JSON-RPC.
    #[error("rpc transport: {0}")]
    Transport(String),
    /// The call reverted; `data` is the revert data (a custom error's selector and arguments).
    #[error("call reverted: {message} (data {data})")]
    Reverted { message: String, data: String },
    /// The result does not decode as the function's return type.
    #[error("rpc result: {0}")]
    Decode(String),
}

/// A JSON-RPC client for the few calls the mock makes.
#[derive(Clone)]
pub struct Rpc {
    http: reqwest::Client,
    url: Url,
}

impl Rpc {
    pub fn new(url: Url) -> Self {
        Self {
            http: reqwest::Client::new(),
            url,
        }
    }

    /// One JSON-RPC request; the `result`, or the `error` as `Reverted`.
    pub async fn request(&self, method: &str, params: Value) -> Result<Value, RpcError> {
        let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
        let reply: Value = self
            .http
            .post(self.url.clone())
            .json(&body)
            .send()
            .await
            .map_err(|e| RpcError::Transport(e.to_string()))?
            .json()
            .await
            .map_err(|e| RpcError::Transport(e.to_string()))?;
        if let Some(error) = reply.get("error") {
            return Err(RpcError::Reverted {
                message: error["message"].as_str().unwrap_or("?").to_owned(),
                data: error["data"].as_str().unwrap_or("none").to_owned(),
            });
        }
        reply
            .get("result")
            .cloned()
            .ok_or_else(|| RpcError::Transport(format!("no result in {reply}")))
    }

    /// `eth_call` of `call` on `to` at the latest block, decoded.
    pub async fn call<C: SolCall>(&self, to: Address, call: &C) -> Result<C::Return, RpcError> {
        let data = Bytes::from(call.abi_encode());
        let result = self
            .request("eth_call", json!([{ "to": to, "data": data }, "latest"]))
            .await?;
        let bytes: Bytes =
            serde_json::from_value(result).map_err(|e| RpcError::Decode(e.to_string()))?;
        C::abi_decode_returns(&bytes).map_err(|e| RpcError::Decode(e.to_string()))
    }

    /// The chain id and the latest block's timestamp.
    pub async fn chain_and_time(&self) -> Result<(u64, u64), RpcError> {
        let chain = self.request("eth_chainId", json!([])).await?;
        let block = self
            .request("eth_getBlockByNumber", json!(["latest", false]))
            .await?;
        Ok((quantity(&chain)?, quantity(&block["timestamp"])?))
    }
}

/// A JSON-RPC quantity (`"0x7a69"`).
fn quantity(value: &Value) -> Result<u64, RpcError> {
    value
        .as_str()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .ok_or_else(|| RpcError::Decode(format!("not a quantity: {value}")))
}

/// The KMS context and epoch the cleartext stack activates at deploy (`LocalHostBootstrap`), as v2 `extraData`.
pub fn default_extra_data() -> Bytes {
    let mut bytes = vec![0x02];
    bytes.extend_from_slice(DEFAULT_CONTEXT.as_slice());
    let mut epoch = [0u8; 32];
    epoch[0] = 0x08;
    epoch[31] = 0x01;
    bytes.extend_from_slice(&epoch);
    Bytes::from(bytes)
}

/// `INITIAL_KMS_CONTEXT_ID` of the cleartext stack.
pub const DEFAULT_CONTEXT: B256 = {
    let mut bytes = [0u8; 32];
    bytes[0] = 0x07;
    bytes[31] = 0x01;
    B256::new(bytes)
};

#[cfg(test)]
pub(crate) mod tests {
    use alloy::sol_types::SolStruct;

    use super::*;

    #[test]
    fn typehashes_match_the_cleartext_contracts() {
        // The type strings of LibKmsVerifier and LibCleartextPad.
        assert_eq!(
            UserDecryptRequestVerification::eip712_encode_type(),
            "UserDecryptRequestVerification(address userAddress,bytes publicKey,address[] allowedContracts,uint256 startTimestamp,uint256 durationSeconds,bytes extraData)"
        );
        assert_eq!(
            PadShareVerification::eip712_encode_type(),
            "PadShareVerification(bytes32 label,bytes rawShares,uint32 partyId,uint32 degree,bytes16 sessionId,bytes extraData)"
        );
        assert_eq!(
            PadDecryptResponseVerification::eip712_encode_type(),
            "PadDecryptResponseVerification(bytes32 requestId,bytes encryptedPayload)"
        );
    }

    #[test]
    fn default_extra_data_is_v2_of_the_bootstrap_context_and_epoch() {
        let zeros = "00".repeat(30);
        assert_eq!(
            default_extra_data().to_string(),
            format!("0x0207{zeros}0108{zeros}01")
        );
    }

    /// A connector request with one pad.
    pub(crate) const CONNECTOR_JSON: &str = r#"{"attestationType":"eip712-unified-user-decrypt-v1","payload":{"userAddress":"0x5555555555555555555555555555555555555555","publicKey":"0x20002000","allowedContracts":["0x3333333333333333333333333333333333333333"],"requestValidity":{"startTimestamp":1770000000,"durationSeconds":300},"extraData":"0x02","witnessVersion":0,"prfId":1,"chainId":31337,"pads":[{"label":"0x8888888888888888888888888888888888888888888888888888888888888888","witness":{"submitter":"0x5555555555555555555555555555555555555555","consumer":"0x3333333333333333333333333333333333333333","expirationTimestamp":1770604800,"decryptors":[],"nonce":"0x77777777777777777777777777777777"},"padLength":64}]},"signature":"0x6666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666"}"#;

    #[test]
    fn contract_request_keeps_every_field() {
        let connector: ConnectorRequest = serde_json::from_str(CONNECTOR_JSON).unwrap();
        let request = contract_request(&connector);
        let (p, c) = (&connector.payload, &request);
        assert_eq!(c.auth.userAddress, p.userAddress);
        assert_eq!(c.auth.publicKey, p.publicKey);
        assert_eq!(c.auth.allowedContracts, p.allowedContracts);
        assert_eq!(c.auth.startTimestamp, U256::from(1_770_000_000u64));
        assert_eq!(c.auth.durationSeconds, U256::from(300u64));
        assert_eq!(c.auth.extraData, p.extraData);
        assert_eq!(c.signature, connector.signature);
        assert_eq!(
            (c.payload.witnessVersion, c.payload.prfId, c.payload.chainId),
            (0, 1, 31337)
        );
        let (pad, cw) = (&p.pads[0], &c.payload.witnesses[0]);
        let w = &pad.witness;
        assert_eq!((cw.submitter, cw.consumer), (w.submitter, w.consumer));
        assert_eq!(cw.expirationTimestamp, w.expirationTimestamp);
        assert_eq!(cw.decryptors, w.decryptors);
        assert_eq!(
            (cw.nonce, cw.label, cw.padLength),
            (w.nonce, pad.label, pad.padLength)
        );
    }

    #[test]
    fn quantity_parses_hex() {
        assert_eq!(quantity(&json!("0x7a69")).unwrap(), 31337);
        assert!(quantity(&json!(12)).is_err());
    }
}
