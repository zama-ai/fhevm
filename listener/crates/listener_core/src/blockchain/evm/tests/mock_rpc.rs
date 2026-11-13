//! In-process mock JSON-RPC node serving one real BSC testnet block
//! (`fixtures/bsc_testnet_block_142386396.json`). Shared by the fetcher and
//! listener tests.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{Value, json};

const FIXTURE: &str = include_str!("fixtures/bsc_testnet_block_142386396.json");

pub struct MockNode {
    pub url: String,
    pub block_hash: alloy::primitives::B256,
    pub block_number: u64,
    pub parent_hash: alloy::primitives::B256,
    pub by_hash_calls: Arc<AtomicUsize>,
    /// `eth_getTransactionReceipt` lookups, single or inside a batch.
    pub tx_receipt_calls: Arc<AtomicUsize>,
}

struct MockState {
    block: Value,
    receipts: Value,
    block_by_hash_null: bool,
    tx_receipt_null: bool,
    by_hash_calls: Arc<AtomicUsize>,
    tx_receipt_calls: Arc<AtomicUsize>,
}

/// Start a mock node. `receipt_block_hash` overrides every receipt's
/// `blockHash`; `block_by_hash_null` makes `eth_getBlockByHash` return null.
pub async fn start(receipt_block_hash: Option<&str>, block_by_hash_null: bool) -> MockNode {
    start_inner(receipt_block_hash, block_by_hash_null, false).await
}

/// Start a mock node whose `eth_getTransactionReceipt` (single or batched) always
/// returns null: a tx of the block was dropped by a reorg.
pub async fn start_with_null_tx_receipts() -> MockNode {
    start_inner(None, false, true).await
}

async fn start_inner(
    receipt_block_hash: Option<&str>,
    block_by_hash_null: bool,
    tx_receipt_null: bool,
) -> MockNode {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture JSON");
    let block = fixture["block"].clone();
    let mut receipts = fixture["receipts"].clone();
    if let Some(hash) = receipt_block_hash {
        for r in receipts.as_array_mut().expect("receipts array") {
            r["blockHash"] = json!(hash);
        }
    }
    let block_hash = block["hash"].as_str().unwrap().parse().unwrap();
    let block_number = u64::from_str_radix(
        block["number"].as_str().unwrap().trim_start_matches("0x"),
        16,
    )
    .unwrap();
    let parent_hash = block["parentHash"].as_str().unwrap().parse().unwrap();
    let by_hash_calls = Arc::new(AtomicUsize::new(0));
    let tx_receipt_calls = Arc::new(AtomicUsize::new(0));
    let state = Arc::new(MockState {
        block,
        receipts,
        block_by_hash_null,
        tx_receipt_null,
        by_hash_calls: by_hash_calls.clone(),
        tx_receipt_calls: tx_receipt_calls.clone(),
    });

    let app = Router::new().route("/", post(handle)).with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    MockNode {
        url: format!("http://{addr}"),
        block_hash,
        block_number,
        parent_hash,
        by_hash_calls,
        tx_receipt_calls,
    }
}

async fn handle(State(state): State<Arc<MockState>>, Json(req): Json<Value>) -> Json<Value> {
    // JSON-RPC batch (batched receipt strategies): answer each entry.
    if let Some(batch) = req.as_array() {
        return Json(Value::Array(
            batch.iter().map(|r| respond(&state, r)).collect(),
        ));
    }
    Json(respond(&state, &req))
}

fn respond(state: &MockState, req: &Value) -> Value {
    let result = match req["method"].as_str().unwrap_or_default() {
        "eth_blockNumber" => state.block["number"].clone(),
        "eth_getBlockByNumber" => state.block.clone(),
        "eth_getBlockByHash" => {
            state.by_hash_calls.fetch_add(1, Ordering::SeqCst);
            if state.block_by_hash_null {
                Value::Null
            } else {
                state.block.clone()
            }
        }
        "eth_getBlockReceipts" => state.receipts.clone(),
        "eth_getTransactionReceipt" => {
            state.tx_receipt_calls.fetch_add(1, Ordering::SeqCst);
            if state.tx_receipt_null {
                Value::Null
            } else {
                let tx_hash = &req["params"][0];
                state
                    .receipts
                    .as_array()
                    .and_then(|rs| rs.iter().find(|r| &r["transactionHash"] == tx_hash))
                    .cloned()
                    .unwrap_or(Value::Null)
            }
        }
        other => panic!("mock node: unexpected method {other}"),
    };
    json!({ "jsonrpc": "2.0", "id": req["id"].clone(), "result": result })
}
