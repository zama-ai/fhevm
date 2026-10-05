//! Transactions written once and rendered both as `getBlock` JSON and as Yellowstone messages,
//! so the two transports can be held to the same result.

use serde_json::{json, Value};
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::prelude::{
    CompiledInstruction as GrpcCompiledInstruction, InnerInstruction,
    InnerInstructions, Message as GrpcMessage, SubscribeUpdateTransaction,
    SubscribeUpdateTransactionInfo, Transaction as GrpcTransaction,
    TransactionStatusMeta,
};

use anchor_lang::InstructionData;
use solana_transaction_status_client_types::{
    EncodedConfirmedTransactionWithStatusMeta, UiConfirmedBlock,
};
use zama_host::state::{
    ExecutionResultRef, FheExecuteArgs, FheExecuteEffect, FheExecuteStep,
};
use zama_host::FheExecutedEvent;

use super::test_support::ZAMA_HOST;

pub(crate) const BLOCK_TIME: i64 = 1_700_000_000;

/// A compiled instruction as the fixtures and both wire formats describe it.
pub(crate) struct Compiled {
    pub(crate) program_id_index: u8,
    pub(crate) accounts: Vec<u8>,
    pub(crate) data: Vec<u8>,
    pub(crate) stack_height: Option<u32>,
}

/// One transaction, written once and rendered in both wire formats.
pub(crate) struct Transaction {
    pub(crate) signature: [u8; 64],
    pub(crate) static_keys: Vec<[u8; 32]>,
    pub(crate) loaded_writable: Vec<[u8; 32]>,
    pub(crate) loaded_readonly: Vec<[u8; 32]>,
    pub(crate) top_level: Vec<Compiled>,
    pub(crate) inner_groups: Vec<(u8, Vec<Compiled>)>,
}

impl Transaction {
    /// `getBlock`'s JSON for the transaction, spelled as the RPC wire format with `encoding:
    /// "json"`: a legacy transaction, or a v0 one when it loads accounts from a lookup table.
    pub(crate) fn rpc_json(&self, err: Value) -> Value {
        let has_lookups = !self.loaded_writable.is_empty()
            || !self.loaded_readonly.is_empty();
        let keys = |keys: &[[u8; 32]]| {
            keys.iter()
                .map(|key| bs58::encode(key).into_string())
                .collect::<Vec<_>>()
        };
        let instructions = |instructions: &[Compiled]| {
            instructions
                .iter()
                .map(|instruction| {
                    json!({
                        "programIdIndex": instruction.program_id_index,
                        "accounts": instruction.accounts,
                        "data": bs58::encode(&instruction.data).into_string(),
                        "stackHeight": instruction.stack_height,
                    })
                })
                .collect::<Vec<_>>()
        };
        let mut message = json!({
            "header": {
                "numRequiredSignatures": 1,
                "numReadonlySignedAccounts": 0,
                "numReadonlyUnsignedAccounts": 0,
            },
            "accountKeys": keys(&self.static_keys),
            "recentBlockhash": bs58::encode([9; 32]).into_string(),
            "instructions": instructions(&self.top_level),
        });
        if has_lookups {
            message["addressTableLookups"] = json!([{
                "accountKey": bs58::encode([0xAA; 32]).into_string(),
                "writableIndexes": (0..self.loaded_writable.len() as u8).collect::<Vec<_>>(),
                "readonlyIndexes": (0..self.loaded_readonly.len() as u8).collect::<Vec<_>>(),
            }]);
        }
        let status = if err.is_null() {
            json!({ "Ok": null })
        } else {
            json!({ "Err": err })
        };
        json!({
            "transaction": {
                "signatures": [bs58::encode(self.signature).into_string()],
                "message": message,
            },
            "meta": {
                "err": err,
                "status": status,
                "fee": 5000,
                "preBalances": [],
                "postBalances": [],
                "innerInstructions": self.inner_groups.iter().map(|(index, group)| json!({
                    "index": index,
                    "instructions": instructions(group),
                })).collect::<Vec<_>>(),
                "logMessages": [],
                "preTokenBalances": [],
                "postTokenBalances": [],
                "rewards": [],
                "loadedAddresses": {
                    "writable": keys(&self.loaded_writable),
                    "readonly": keys(&self.loaded_readonly),
                },
                "computeUnitsConsumed": 1000,
            },
            "version": if has_lookups { json!(0) } else { json!("legacy") },
        })
    }

    /// `getBlock`'s JSON for the transaction with `transactionDetails: "accounts"`: its
    /// signatures, account keys and status, without instructions or logs.
    pub(crate) fn rpc_accounts_json(&self, err: Value) -> Value {
        let full = self.rpc_json(err.clone());
        let keys = |keys: &[[u8; 32]], source: &str| {
            keys.iter()
                .map(|key| {
                    json!({
                        "pubkey": bs58::encode(key).into_string(),
                        "writable": false,
                        "signer": false,
                        "source": source,
                    })
                })
                .collect::<Vec<_>>()
        };
        let account_keys = [
            keys(&self.static_keys, "transaction"),
            keys(&self.loaded_writable, "lookupTable"),
            keys(&self.loaded_readonly, "lookupTable"),
        ]
        .concat();
        json!({
            "transaction": {
                "signatures": [bs58::encode(self.signature).into_string()],
                "accountKeys": account_keys,
            },
            "meta": {
                "err": err,
                "status": full["meta"]["status"],
                "fee": 5000,
                "preBalances": [],
                "postBalances": [],
            },
            "version": full["version"],
        })
    }

    /// `getTransaction`'s response for the successful transaction in `slot`.
    pub(crate) fn rpc_transaction(
        &self,
        slot: u64,
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let mut response = self.rpc_json(Value::Null);
        response["slot"] = json!(slot);
        response["blockTime"] = json!(BLOCK_TIME);
        serde_json::from_value(response).expect("getTransaction JSON")
    }

    pub(crate) fn grpc(&self) -> (GrpcMessage, TransactionStatusMeta) {
        let message = GrpcMessage {
            account_keys: self
                .static_keys
                .iter()
                .map(|key| key.to_vec())
                .collect(),
            instructions: self
                .top_level
                .iter()
                .map(|instruction| GrpcCompiledInstruction {
                    program_id_index: u32::from(instruction.program_id_index),
                    accounts: instruction.accounts.clone(),
                    data: instruction.data.clone(),
                })
                .collect(),
            versioned: true,
            ..Default::default()
        };
        let meta = TransactionStatusMeta {
            inner_instructions: self
                .inner_groups
                .iter()
                .map(|(index, instructions)| InnerInstructions {
                    index: u32::from(*index),
                    instructions: instructions
                        .iter()
                        .map(|instruction| InnerInstruction {
                            program_id_index: u32::from(
                                instruction.program_id_index,
                            ),
                            accounts: instruction.accounts.clone(),
                            data: instruction.data.clone(),
                            stack_height: instruction.stack_height,
                        })
                        .collect(),
                })
                .collect(),
            loaded_writable_addresses: self
                .loaded_writable
                .iter()
                .map(|key| key.to_vec())
                .collect(),
            loaded_readonly_addresses: self
                .loaded_readonly
                .iter()
                .map(|key| key.to_vec())
                .collect(),
            ..Default::default()
        };
        (message, meta)
    }

    /// The transaction as Yellowstone streams it, successful, at `index` in `slot`.
    pub(crate) fn grpc_update(
        &self,
        slot: u64,
        index: u64,
    ) -> SubscribeUpdateTransaction {
        SubscribeUpdateTransaction {
            transaction: Some(self.grpc_info(index)),
            slot,
        }
    }

    /// The transaction as a Yellowstone transaction update carries it, at `index`.
    pub(crate) fn grpc_info(
        &self,
        index: u64,
    ) -> SubscribeUpdateTransactionInfo {
        let (message, meta) = self.grpc();
        SubscribeUpdateTransactionInfo {
            signature: self.signature.to_vec(),
            is_vote: false,
            transaction: Some(GrpcTransaction {
                signatures: vec![self.signature.to_vec()],
                message: Some(message),
            }),
            meta: Some(meta),
            index,
        }
    }
}

/// An app program's top-level instruction CPIs into `fhe_execute`, which emits its event
/// through a self-CPI, as on chain.
pub(crate) fn app_transaction(
    signature: u8,
    plaintext: [u8; 32],
) -> Transaction {
    app_calling_host(signature, execute_args(plaintext, None), vec![])
}

/// [`app_transaction`], storing its result into `store` and allowing `key`: one leaf, the
/// store's `previous_leaf_count`-th.
pub(crate) fn storing_app_transaction(
    signature: u8,
    plaintext: [u8; 32],
    store: [u8; 32],
    key: [u8; 32],
    previous_leaf_count: u64,
) -> Transaction {
    // Static key indices of the account filling the host's fixed accounts, and of the store.
    const FIXED: u8 = 4;
    const STORE: u8 = 5;
    let mut transaction = app_calling_host(
        signature,
        execute_args(plaintext, Some((key, previous_leaf_count))),
        [FIXED; zama_host::FHE_EXECUTE_FIXED_ACCOUNTS]
            .into_iter()
            .chain([STORE])
            .collect(),
    );
    transaction.static_keys.extend([[0; 32], store]);
    transaction
}

/// A transaction of another program, which the stream's account filter leaves out.
pub(crate) fn foreign_transaction(signature: u8) -> Transaction {
    Transaction {
        signature: [signature; 64],
        static_keys: vec![[1; 32], [8; 32]],
        loaded_writable: vec![],
        loaded_readonly: vec![],
        top_level: vec![Compiled {
            program_id_index: 1,
            accounts: vec![0],
            data: vec![1],
            stack_height: None,
        }],
        inner_groups: vec![],
    }
}

/// A trivial encryption of `plaintext`, optionally stored with one allowed key.
fn execute_args(
    plaintext: [u8; 32],
    stored: Option<([u8; 32], u64)>,
) -> FheExecuteArgs {
    FheExecuteArgs {
        execution_store_index: 0,
        effects: stored
            .map(|(_, previous_leaf_count)| FheExecuteEffect {
                result: ExecutionResultRef {
                    step_index: 0,
                    output_index: 0,
                },
                store_index: 0,
                previous_leaf_count,
                slot: None,
                allow_indexes: vec![0],
                make_public: false,
                grants: vec![],
            })
            .into_iter()
            .collect(),
        returned_results: vec![],
        account_count: u8::from(stored.is_some()),
        dictionary: stored.map(|(key, _)| key).into_iter().collect(),
        steps: vec![FheExecuteStep::TrivialEncrypt {
            plaintext,
            fhe_type: 5,
        }],
    }
}

/// The bytes of the event CPI a host emits for an execution whose step results are `results`.
pub(crate) fn event_cpi_data(results: Vec<[u8; 32]>) -> Vec<u8> {
    let event = FheExecutedEvent {
        version: zama_host::EVENT_VERSION,
        previous_bank_hash: [0x44; 32],
        unix_timestamp: BLOCK_TIME,
        results,
        seeds: vec![],
    };
    anchor_lang::event::EVENT_IX_TAG_LE
        .iter()
        .copied()
        .chain(anchor_lang::Event::data(&event))
        .collect()
}

/// `execution_accounts` index the transaction's static keys. The execution's one result is
/// `plaintext`, read as a handle: the follower carries handles, it does not derive them.
fn app_calling_host(
    signature: u8,
    args: FheExecuteArgs,
    execution_accounts: Vec<u8>,
) -> Transaction {
    let host: [u8; 32] = ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes();
    let FheExecuteStep::TrivialEncrypt { plaintext, .. } = args.steps[0] else {
        panic!("a trivial encryption")
    };
    let execution_data = zama_host::instruction::FheExecute { args }.data();
    let event_data = event_cpi_data(vec![plaintext]);
    Transaction {
        signature: [signature; 64],
        // Payer, app program, host program, event authority.
        static_keys: vec![[1; 32], [2; 32], host, [3; 32]],
        loaded_writable: vec![[6; 32]],
        loaded_readonly: vec![],
        top_level: vec![Compiled {
            program_id_index: 1,
            accounts: vec![0, 4],
            data: vec![0xA0],
            stack_height: None,
        }],
        inner_groups: vec![(
            0,
            vec![
                Compiled {
                    program_id_index: 2,
                    accounts: execution_accounts,
                    data: execution_data,
                    stack_height: Some(2),
                },
                Compiled {
                    program_id_index: 2,
                    accounts: vec![3],
                    data: event_data,
                    stack_height: Some(3),
                },
            ],
        )],
    }
}

/// `getBlock`'s JSON for a block at `slot` whose parent is `parent`, hashed with `hash`.
pub(crate) fn block_json(
    slot: u64,
    parent: u64,
    hash: impl Fn(u64) -> [u8; 32],
    transactions: Vec<Value>,
) -> UiConfirmedBlock {
    serde_json::from_value(json!({
        "previousBlockhash": bs58::encode(hash(parent)).into_string(),
        "blockhash": bs58::encode(hash(slot)).into_string(),
        "parentSlot": parent,
        "transactions": transactions,
        "blockTime": BLOCK_TIME,
        "blockHeight": slot - 2,
    }))
    .expect("getBlock JSON")
}
