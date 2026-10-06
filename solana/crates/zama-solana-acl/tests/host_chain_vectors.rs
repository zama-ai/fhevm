//! The DD-052 chain-type vectors: generated here from `zama_solana_acl::host_chain` and compared
//! against the committed `solana/test-fixtures/host-chain/host_chain_v1.json`. The file is a pin,
//! not a source: other implementations check their own values against it. Rewrite it with
//! `ZAMA_UPDATE_HOST_CHAIN_VECTORS=1 cargo test -p zama-solana-acl --test host_chain_vectors`; a
//! changed byte changes which chain every existing handle names.
//!
//! Every 64-bit number is a decimal string so a TypeScript reader cannot lose precision. The
//! `constants` are all decimal strings, as Anchor writes `#[constant]` values into the IDL.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use zama_solana_acl::host_chain::{
    chain_type_byte, handle_chain_id, is_evm_host_chain_id, is_solana_host_chain_id,
    solana_host_chain_id, CHAIN_TYPE_SHIFT, CLUSTER_TAG_MASK, EVM_CHAIN_TYPE, SOLANA_CHAIN_TYPE,
};

const UPDATE_ENV: &str = "ZAMA_UPDATE_HOST_CHAIN_VECTORS";

fn vector_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures/host-chain/host_chain_v1.json")
}

fn chain_id_entry(chain_id: u64) -> Value {
    json!({
        "chain_id": chain_id.to_string(),
        "chain_type_byte": chain_type_byte(chain_id),
        "is_evm": is_evm_host_chain_id(chain_id),
        "is_solana": is_solana_host_chain_id(chain_id),
    })
}

fn handle_entry(chain_id: u64) -> Value {
    let mut handle = [0xab; 32];
    handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
    json!({
        "handle": format!("0x{}", hex::encode(handle)),
        "chain_id": handle_chain_id(&handle).to_string(),
    })
}

fn generate() -> Value {
    let unknown_type = 0x0200_0000_0000_0001;
    let solana_host_chain_ids: Vec<Value> = [
        0,
        1,
        12345,
        CLUSTER_TAG_MASK,
        CLUSTER_TAG_MASK + 1,
        u64::MAX,
    ]
    .into_iter()
    .map(|cluster_tag| {
        json!({
            "cluster_tag": cluster_tag.to_string(),
            "chain_id": solana_host_chain_id(cluster_tag).to_string(),
        })
    })
    .collect();
    let chain_ids: Vec<Value> = [
        1,
        31_337,
        11_155_111,
        CLUSTER_TAG_MASK,
        solana_host_chain_id(1),
        solana_host_chain_id(12345),
        unknown_type,
        u64::MAX,
    ]
    .into_iter()
    .map(chain_id_entry)
    .collect();
    let handles: Vec<Value> = [31_337, solana_host_chain_id(12345), unknown_type]
        .into_iter()
        .map(handle_entry)
        .collect();
    json!({
        "schema": "zama-host-chain-vectors/v1",
        "regenerate_with":
            "ZAMA_UPDATE_HOST_CHAIN_VECTORS=1 cargo test -p zama-solana-acl --test host_chain_vectors",
        "constants": {
            "evm_chain_type": EVM_CHAIN_TYPE.to_string(),
            "solana_chain_type": SOLANA_CHAIN_TYPE.to_string(),
            "chain_type_shift": CHAIN_TYPE_SHIFT.to_string(),
            "cluster_tag_mask": CLUSTER_TAG_MASK.to_string(),
        },
        "solana_host_chain_ids": solana_host_chain_ids,
        "chain_ids": chain_ids,
        "handles": handles,
    })
}

#[test]
fn committed_vectors_match_the_host_chain_module() {
    let path = vector_path();
    let rendered = serde_json::to_string_pretty(&generate()).unwrap() + "\n";
    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &rendered).unwrap();
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        committed,
        rendered,
        "{} differs from the host_chain module; regenerate it with {UPDATE_ENV}=1",
        path.display()
    );
}
