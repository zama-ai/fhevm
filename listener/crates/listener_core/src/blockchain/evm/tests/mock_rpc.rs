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
    pub by_hash_calls: Arc<AtomicUsize>,
}

struct MockState {
    block: Value,
    receipts: Value,
    block_by_hash_null: bool,
    by_hash_calls: Arc<AtomicUsize>,
    executor_parent_reorg: bool,
}

/// Start a mock node. `receipt_block_hash` overrides every receipt's
/// `blockHash`; `block_by_hash_null` makes `eth_getBlockByHash` return null.
pub async fn start(receipt_block_hash: Option<&str>, block_by_hash_null: bool) -> MockNode {
    start_inner(receipt_block_hash, block_by_hash_null, false).await
}

/// An orphaned empty block still passes block/receipt consistency checks, but
/// its executor-version parent has been replaced by another canonical block.
pub async fn start_with_executor_parent_reorg() -> MockNode {
    start_inner(None, false, true).await
}

async fn start_inner(
    receipt_block_hash: Option<&str>,
    block_by_hash_null: bool,
    executor_parent_reorg: bool,
) -> MockNode {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture JSON");
    let mut block = fixture["block"].clone();
    let mut receipts = fixture["receipts"].clone();
    if executor_parent_reorg {
        block["transactions"] = json!([]);
        receipts = json!([]);
    }
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
    let by_hash_calls = Arc::new(AtomicUsize::new(0));
    let state = Arc::new(MockState {
        block,
        receipts,
        block_by_hash_null,
        by_hash_calls: by_hash_calls.clone(),
        executor_parent_reorg,
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
        by_hash_calls,
    }
}

async fn handle(State(state): State<Arc<MockState>>, Json(req): Json<Value>) -> Json<Value> {
    let result = match req["method"].as_str().unwrap_or_default() {
        "eth_getBlockByNumber" if state.executor_parent_reorg => {
            let mut canonical = state.block.clone();
            canonical["number"] = req["params"][0].clone();
            canonical["hash"] = json!(alloy::primitives::B256::repeat_byte(0x77));
            canonical
        }
        "eth_getBlockByNumber" => state.block.clone(),
        "eth_call" if state.executor_parent_reorg => {
            return Json(json!({"jsonrpc":"2.0", "id":req["id"],
                "error":{"code":-32000,"message":"hash is not currently canonical"}}));
        }
        "eth_getBlockByHash" => {
            state.by_hash_calls.fetch_add(1, Ordering::SeqCst);
            if state.block_by_hash_null {
                Value::Null
            } else {
                state.block.clone()
            }
        }
        "eth_getBlockReceipts" => state.receipts.clone(),
        other => panic!("mock node: unexpected method {other}"),
    };
    Json(json!({ "jsonrpc": "2.0", "id": req["id"].clone(), "result": result }))
}
