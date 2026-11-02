//! One mock KMS connector per KMS signer key, each serving `POST /v1/pad-decrypt` (the relayer's temporary DTO) from
//! the cleartext `CleartextPaddedInputter`: the contract runs the RFC-030 checks and builds what every node signs;
//! the node signs its own share digests and its transport digest with its key, like the SDK's cleartext mock does
//! for all nodes at once. The `FhevmSig` header is not verified.

use std::net::SocketAddr;
use std::sync::Arc;

use alloy::primitives::{Address, B256, Bytes};
use alloy::signers::SignerSync;
use alloy::signers::local::PrivateKeySigner;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use kms_connector_api::{
    ErrorCode, ErrorResponse, PAD_DECRYPTION_ROUTE, PadDecryptionRequest, PadDecryptionResponse,
};
use tokio::net::TcpListener;
use tokio::task::JoinSet;
use tracing::{info, warn};

use crate::chain::{
    Rpc, RpcError, contract_request, getCleartextPadResponsesMaterialCall,
    getCleartextPadSharesMaterialCall,
};

/// Why a node could not answer; mapped to the connector's error codes.
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error(transparent)]
    Rpc(#[from] RpcError),
    /// The node's key is not among the KMS signers of the request's context.
    #[error("{0} is not a KMS signer of the request's context")]
    NotASigner(Address),
    /// The contract answered material without this node's entry.
    #[error("material has no entry for node {0}")]
    Material(usize),
    #[error("signing failed: {0}")]
    Sign(String),
}

impl NodeError {
    /// A revert is the request's fault (final); an unreachable chain is transient.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Rpc(RpcError::Reverted { .. }) => ErrorCode::Unprocessable,
            Self::Rpc(RpcError::Transport(_)) => ErrorCode::UpstreamTransient,
            Self::Rpc(RpcError::Decode(_))
            | Self::NotASigner(_)
            | Self::Material(_)
            | Self::Sign(_) => ErrorCode::Unknown,
        }
    }
}

/// One mock KMS node: its signing key and the chain it reads.
pub struct Node {
    pub name: String,
    pub key: PrivateKeySigner,
    pub rpc: Rpc,
    pub padded_inputter: Address,
}

impl Node {
    /// Flow 1 to 4 of `CleartextPaddedInputter`, for this node only: the material, this node's share signatures,
    /// its encrypted payload and transport digest, its transport signature.
    pub async fn answer(
        &self,
        request: &PadDecryptionRequest,
    ) -> Result<PadDecryptionResponse, NodeError> {
        let request = contract_request(request);
        let material = self
            .rpc
            .call(
                self.padded_inputter,
                &getCleartextPadSharesMaterialCall {
                    request: request.clone(),
                },
            )
            .await?;
        let me = self.key.address();
        let index = material
            .signers
            .iter()
            .position(|s| *s == me)
            .ok_or(NodeError::NotASigner(me))?;
        let digests = material
            .shareDigests
            .get(index)
            .ok_or(NodeError::Material(index))?;
        let mine = digests
            .iter()
            .map(|d| self.sign(d))
            .collect::<Result<Vec<_>, _>>()?;
        // The contract needs a row per node but only reads this node's into this node's payload.
        let rows = vec![mine; material.signers.len()];
        let responses = self
            .rpc
            .call(
                self.padded_inputter,
                &getCleartextPadResponsesMaterialCall {
                    request,
                    shareSignatures: rows,
                },
            )
            .await?;
        let payload = responses
            .encryptedPayloads
            .get(index)
            .ok_or(NodeError::Material(index))?;
        let digest = responses
            .transportDigests
            .get(index)
            .ok_or(NodeError::Material(index))?;
        Ok(PadDecryptionResponse {
            decryption_id: responses.requestId,
            encrypted_payload: payload.clone(),
            signature: self.sign(digest)?,
        })
    }

    /// A 65-byte `r ‖ s ‖ v` signature of `hash`, `v` in {27, 28}.
    fn sign(&self, hash: &B256) -> Result<Bytes, NodeError> {
        let signature = self
            .key
            .sign_hash_sync(hash)
            .map_err(|e| NodeError::Sign(e.to_string()))?;
        Ok(Bytes::from(signature.as_bytes().to_vec()))
    }
}

/// The node's router: `POST /v1/pad-decrypt` only.
pub fn router(node: Arc<Node>) -> Router {
    Router::new()
        .route(PAD_DECRYPTION_ROUTE, post(pad_decrypt))
        .with_state(node)
}

/// `200` with the node's response, or the connector's error body at the code's status.
async fn pad_decrypt(State(node): State<Arc<Node>>, body: axum::body::Bytes) -> Response {
    let request: PadDecryptionRequest = match serde_json::from_slice(&body) {
        Ok(request) => request,
        Err(e) => return error(ErrorCode::Malformed, e.to_string(), None),
    };
    let id = request.id();
    let pads = request.payload.pads.len();
    match node.answer(&request).await {
        Ok(response) => {
            info!(node = %node.name, %id, pads, "pad decryption answered");
            Json(response).into_response()
        }
        Err(e) => {
            warn!(node = %node.name, %id, pads, error = %e, "pad decryption failed");
            error(e.code(), e.to_string(), Some(id))
        }
    }
}

/// The connector's error body at `code.http_status()`.
fn error(code: ErrorCode, message: String, id: Option<B256>) -> Response {
    let status =
        StatusCode::from_u16(code.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(ErrorResponse::new(code, message, id))).into_response()
}

/// Serves `nodes[i]` on `addresses[i]` until one server stops.
pub async fn serve(nodes: Vec<Node>, addresses: Vec<SocketAddr>) -> std::io::Result<()> {
    let mut servers = JoinSet::new();
    for (node, address) in nodes.into_iter().zip(addresses) {
        let listener = TcpListener::bind(address).await?;
        info!(node = %node.name, signer = %node.key.address(), %address, "mock KMS connector listening");
        servers.spawn(async move { axum::serve(listener, router(Arc::new(node))).await });
    }
    match servers.join_next().await {
        Some(Ok(result)) => result,
        Some(Err(e)) => Err(std::io::Error::other(e)),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{Signature, U256};
    use alloy::sol_types::{SolCall, SolValue};
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use super::*;
    use crate::chain::tests::CONNECTOR_JSON;
    use crate::chain::{CleartextPadResponsesMaterial, CleartextPadSharesMaterial};

    /// A stand-in chain: answers the two material calls by selector, records nothing.
    async fn fake_chain(signers: Vec<Address>) -> (Url, tokio::task::JoinHandle<()>) {
        let app = Router::new().route(
            "/",
            post(move |Json(body): Json<Value>| {
                let signers = signers.clone();
                async move {
                    let data: Bytes = serde_json::from_value(body["params"][0]["data"].clone()).unwrap();
                    let selector: [u8; 4] = data[..4].try_into().unwrap();
                    let result = if selector == getCleartextPadSharesMaterialCall::SELECTOR {
                        CleartextPadSharesMaterial {
                            requestId: B256::repeat_byte(0x99),
                            shares: vec![],
                            signers: signers.clone(),
                            partyIds: vec![1, 2],
                            shareDigests: vec![vec![B256::repeat_byte(0x01)], vec![B256::repeat_byte(0x02)]],
                            threshold: U256::from(2),
                        }
                        .abi_encode()
                    } else if selector == getCleartextPadResponsesMaterialCall::SELECTOR {
                        let call = getCleartextPadResponsesMaterialCall::abi_decode(&data).unwrap();
                        // One row per node, each of the node's single share.
                        assert_eq!(call.shareSignatures.len(), 2);
                        assert_eq!(call.shareSignatures[1].len(), 1);
                        CleartextPadResponsesMaterial {
                            requestId: B256::repeat_byte(0x99),
                            signers,
                            encryptedPayloads: vec![Bytes::from_static(&[0xa0]), Bytes::from_static(&[0xa1])],
                            transportDigests: vec![B256::repeat_byte(0x10), B256::repeat_byte(0x11)],
                        }
                        .abi_encode()
                    } else {
                        return Json(json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": 3, "message": "execution reverted", "data": "0xdeadbeef" } }));
                    };
                    Json(json!({ "jsonrpc": "2.0", "id": 1, "result": Bytes::from(result) }))
                }
            }),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        (
            url,
            tokio::spawn(async move { axum::serve(listener, app).await.unwrap() }),
        )
    }

    use url::Url;

    fn node(key: PrivateKeySigner, url: Url) -> Node {
        Node {
            name: "kms_1".to_owned(),
            key,
            rpc: Rpc::new(url),
            padded_inputter: Address::repeat_byte(0xde),
        }
    }

    async fn post_request(node: Node, body: &str) -> (StatusCode, Value) {
        let response = router(Arc::new(node))
            .oneshot(
                Request::post(PAD_DECRYPTION_ROUTE)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_owned()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, serde_json::from_slice(&bytes).unwrap())
    }

    #[tokio::test]
    async fn node_answers_its_own_entry_signed_with_its_key() {
        let (me, other) = (PrivateKeySigner::random(), PrivateKeySigner::random());
        let (url, _chain) = fake_chain(vec![other.address(), me.address()]).await;
        let (status, body) = post_request(node(me.clone(), url), CONNECTOR_JSON).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let response: PadDecryptionResponse = serde_json::from_value(body).unwrap();
        assert_eq!(response.decryption_id, B256::repeat_byte(0x99));
        assert_eq!(response.encrypted_payload, Bytes::from_static(&[0xa1]));
        let signature = Signature::try_from(response.signature.as_ref()).unwrap();
        assert_eq!(
            signature
                .recover_address_from_prehash(&B256::repeat_byte(0x11))
                .unwrap(),
            me.address()
        );
        assert!(matches!(response.signature[64], 27 | 28));
    }

    #[tokio::test]
    async fn unknown_signer_bad_body_and_revert_map_to_connector_errors() {
        let (url, _chain) =
            fake_chain(vec![Address::repeat_byte(1), Address::repeat_byte(2)]).await;
        let (status, body) = post_request(
            node(PrivateKeySigner::random(), url.clone()),
            CONNECTOR_JSON,
        )
        .await;
        assert_eq!(
            (status.as_u16(), body["code"].as_str().unwrap()),
            (500, "unknown")
        );
        assert!(body["decryptionId"].is_string());

        let (status, body) = post_request(node(PrivateKeySigner::random(), url), "{}").await;
        assert_eq!(
            (status.as_u16(), body["code"].as_str().unwrap()),
            (400, "malformed")
        );

        let revert = NodeError::Rpc(RpcError::Reverted {
            message: "x".into(),
            data: "0x".into(),
        });
        assert_eq!(revert.code(), ErrorCode::Unprocessable);
        let down = NodeError::Rpc(RpcError::Transport("refused".into()));
        assert_eq!(down.code(), ErrorCode::UpstreamTransient);
    }

    #[tokio::test]
    async fn unreachable_chain_is_transient() {
        let url = Url::parse("http://127.0.0.1:1").unwrap();
        let (status, body) =
            post_request(node(PrivateKeySigner::random(), url), CONNECTOR_JSON).await;
        assert_eq!(
            (status.as_u16(), body["code"].as_str().unwrap()),
            (502, "upstream_transient")
        );
        assert_eq!(body["retryable"], true);
    }
}
