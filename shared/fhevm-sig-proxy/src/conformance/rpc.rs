//! A fake Ethereum JSON-RPC serving the `ProtocolConfig` reads of the signer reader.

use super::{PROTOCOL_CONFIG, stub::serve};
use alloy::{
    primitives::{Address, U256, hex},
    sol_types::{SolCall, SolInterface},
    transports::http::reqwest::Url,
};
use bytes::Bytes;
use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::{
    IProtocolConfigBaseCalls, KmsNode, getCurrentKmsContextIdCounterCall,
    getKmsNodesForContextCall, isValidKmsContextCall,
};
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use std::sync::{Arc, Mutex};

/// One KMS context in the fake `ProtocolConfig`: Active or not, and its tx-senders.
#[derive(Clone)]
pub struct KmsContext {
    pub active: bool,
    pub tx_senders: Vec<Address>,
}

#[derive(Default)]
struct RpcState {
    contexts: Vec<KmsContext>,
    failing: bool,
}

/// An Ethereum JSON-RPC answering the `ProtocolConfig` calls of the signer reader from a list of
/// KMS contexts, or failing every call.
pub struct FakeRpc {
    pub url: Url,
    state: Arc<Mutex<RpcState>>,
}

impl FakeRpc {
    pub async fn start(contexts: Vec<KmsContext>) -> Self {
        let state = Arc::new(Mutex::new(RpcState {
            contexts,
            failing: false,
        }));
        let handler_state = state.clone();
        let addr = serve(move |request| {
            let state = handler_state.clone();
            async move { rpc(&state, request).await }
        })
        .await;
        Self {
            url: format!("http://{addr}").parse().unwrap(),
            state,
        }
    }

    pub fn set_contexts(&self, contexts: Vec<KmsContext>) {
        self.state.lock().unwrap().contexts = contexts;
    }

    pub fn set_failing(&self, failing: bool) {
        self.state.lock().unwrap().failing = failing;
    }
}

/// Context ids as `ProtocolConfig` allocates them: the type tag 0x07 in the top byte.
fn context_id(index: usize) -> U256 {
    (U256::from(7) << 248) + U256::from(index + 1)
}

async fn rpc(state: &Mutex<RpcState>, request: Request<Incoming>) -> Response<Full<Bytes>> {
    let body = request.into_body().collect().await.unwrap().to_bytes();
    let call: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let state = state.lock().unwrap();
    if state.failing {
        let mut response = Response::new(Full::default());
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return response;
    }
    assert_eq!(call["method"], "eth_call", "unexpected RPC {call}");
    let tx = &call["params"][0];
    let to: Option<Address> = tx["to"].as_str().and_then(|to| to.parse().ok());
    // Only `ProtocolConfig` at the finalized block knows the contexts.
    if to != Some(PROTOCOL_CONFIG) || call["params"][1] != "finalized" {
        let error = serde_json::json!({"jsonrpc": "2.0", "id": call["id"],
            "error": {"code": -32000, "message": format!("unexpected call {call}")}});
        return Response::new(Full::new(Bytes::from(error.to_string())));
    }
    let data = tx["input"].as_str().or(tx["data"].as_str()).unwrap();
    let context = |id: U256| {
        (0..state.contexts.len())
            .find(|&index| context_id(index) == id)
            .map(|index| &state.contexts[index])
    };
    let result = match IProtocolConfigBaseCalls::abi_decode(&hex::decode(data).unwrap()).unwrap() {
        IProtocolConfigBaseCalls::getCurrentKmsContextIdCounter(_) => {
            let counter = match state.contexts.len() {
                0 => U256::ZERO,
                n => context_id(n - 1),
            };
            Some(getCurrentKmsContextIdCounterCall::abi_encode_returns(
                &counter,
            ))
        }
        IProtocolConfigBaseCalls::isValidKmsContext(call) => {
            let active = context(call.kmsContextId).is_some_and(|c| c.active);
            Some(isValidKmsContextCall::abi_encode_returns(&active))
        }
        // As the contract: reverts unless the context is Active.
        IProtocolConfigBaseCalls::getKmsNodesForContext(call) => {
            context(call.kmsContextId).filter(|c| c.active).map(|c| {
                let nodes: Vec<KmsNode> = c
                    .tx_senders
                    .iter()
                    .map(|&tx_sender| KmsNode {
                        txSenderAddress: tx_sender,
                        ..Default::default()
                    })
                    .collect();
                getKmsNodesForContextCall::abi_encode_returns(&nodes)
            })
        }
        _ => panic!("unexpected ProtocolConfig call"),
    };
    let id = &call["id"];
    let reply = match result {
        Some(result) => {
            serde_json::json!({"jsonrpc": "2.0", "id": id, "result": hex::encode_prefixed(result)})
        }
        None => serde_json::json!({"jsonrpc": "2.0", "id": id,
            "error": {"code": 3, "message": "execution reverted: InvalidKmsContext"}}),
    };
    Response::new(Full::new(Bytes::from(reply.to_string())))
}
