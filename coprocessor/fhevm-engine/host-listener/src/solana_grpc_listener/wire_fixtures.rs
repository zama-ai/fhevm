//! Transactions written once and rendered both as `getBlock` JSON and as Yellowstone messages,
//! so the two transports can be held to the same result.

use base64::{prelude::BASE64_STANDARD, Engine};
use serde_json::{json, Value};
use solana_sdk::{
    hash::Hash,
    message::{
        compiled_instruction::CompiledInstruction as SdkCompiledInstruction,
        v0::{self, MessageAddressTableLookup},
        MessageHeader, VersionedMessage,
    },
    pubkey::Pubkey,
    signature::Signature,
    transaction::VersionedTransaction,
};
use yellowstone_grpc_proto::prelude::{
    CompiledInstruction as GrpcCompiledInstruction, InnerInstruction,
    InnerInstructions, Message as GrpcMessage, SubscribeUpdateTransactionInfo,
    Transaction as GrpcTransaction, TransactionStatusMeta,
};

use super::test_support::{encoded_execution, with_events, ZAMA_HOST};
use crate::solana_reconstruct::DecodedInstruction;

/// A compiled instruction as the fixtures and both wire formats describe it.
pub(super) struct Compiled {
    pub(super) program_id_index: u8,
    pub(super) accounts: Vec<u8>,
    pub(super) data: Vec<u8>,
    pub(super) stack_height: Option<u32>,
}

/// One transaction, written once and rendered in both wire formats.
pub(super) struct Transaction {
    pub(super) signature: [u8; 64],
    pub(super) static_keys: Vec<[u8; 32]>,
    pub(super) loaded_writable: Vec<[u8; 32]>,
    pub(super) loaded_readonly: Vec<[u8; 32]>,
    pub(super) top_level: Vec<Compiled>,
    pub(super) inner_groups: Vec<(u8, Vec<Compiled>)>,
}

impl Transaction {
    /// `getBlock`'s JSON for the transaction, spelled as the RPC wire format.
    pub(super) fn rpc_json(&self, err: Value) -> Value {
        let has_lookups = !self.loaded_writable.is_empty()
            || !self.loaded_readonly.is_empty();
        let message = v0::Message {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 0,
            },
            account_keys: self
                .static_keys
                .iter()
                .map(|key| Pubkey::new_from_array(*key))
                .collect(),
            recent_blockhash: Hash::new_from_array([9; 32]),
            instructions: self
                .top_level
                .iter()
                .map(|instruction| SdkCompiledInstruction {
                    program_id_index: instruction.program_id_index,
                    accounts: instruction.accounts.clone(),
                    data: instruction.data.clone(),
                })
                .collect(),
            address_table_lookups: if has_lookups {
                vec![MessageAddressTableLookup {
                    account_key: Pubkey::new_from_array([0xAA; 32]),
                    writable_indexes: (0..self.loaded_writable.len() as u8)
                        .collect(),
                    readonly_indexes: (0..self.loaded_readonly.len() as u8)
                        .collect(),
                }]
            } else {
                vec![]
            },
        };
        let transaction = VersionedTransaction {
            signatures: vec![Signature::from(self.signature)],
            message: VersionedMessage::V0(message),
        };
        let keys = |keys: &[[u8; 32]]| {
            keys.iter()
                .map(|key| bs58::encode(key).into_string())
                .collect::<Vec<_>>()
        };
        let status = if err.is_null() {
            json!({ "Ok": null })
        } else {
            json!({ "Err": err })
        };
        json!({
            "transaction": [
                BASE64_STANDARD.encode(bincode::serialize(&transaction).unwrap()),
                "base64"
            ],
            "meta": {
                "err": err,
                "status": status,
                "fee": 5000,
                "preBalances": [],
                "postBalances": [],
                "innerInstructions": self.inner_groups.iter().map(|(index, instructions)| json!({
                    "index": index,
                    "instructions": instructions.iter().map(|instruction| json!({
                        "programIdIndex": instruction.program_id_index,
                        "accounts": instruction.accounts,
                        "data": bs58::encode(&instruction.data).into_string(),
                        "stackHeight": instruction.stack_height,
                    })).collect::<Vec<_>>(),
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
            "version": 0,
        })
    }

    pub(super) fn grpc(&self) -> (GrpcMessage, TransactionStatusMeta) {
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

    /// The transaction as a successful entry of a Yellowstone block, at `index`.
    pub(super) fn grpc_info(
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
pub(super) fn app_transaction(
    signature: u8,
    plaintext: [u8; 32],
) -> Transaction {
    let host: [u8; 32] = ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes();
    let execution = DecodedInstruction {
        program: ZAMA_HOST.to_owned(),
        data: encoded_execution(zama_host::state::FheExecuteArgs {
            execution_store_index: 0,
            effects: vec![],
            returned_results: vec![],
            account_count: 0,
            dictionary: vec![],
            steps: vec![zama_host::state::FheExecuteStep::TrivialEncrypt {
                plaintext,
                fhe_type: 5,
            }],
        }),
        accounts: vec![],
        top_level_index: 0,
        is_inner: true,
    };
    let [execution, event] = &with_events([execution])[..] else {
        panic!("one execution, one event")
    };
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
                    accounts: vec![],
                    data: execution.data.clone(),
                    stack_height: Some(2),
                },
                Compiled {
                    program_id_index: 2,
                    accounts: vec![3],
                    data: event.data.clone(),
                    stack_height: Some(3),
                },
            ],
        )],
    }
}
