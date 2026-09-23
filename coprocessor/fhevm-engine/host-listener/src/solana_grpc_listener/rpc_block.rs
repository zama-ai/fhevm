//! Blocks fetched with `getBlock`, prepared exactly like streamed ones. Each `FheExecutedEvent`
//! carries its execution's derivation context, so a block's own transactions are all the
//! listener needs: any archive can rebuild a slot the stream missed.
//!
//! Expects the response of `getBlock` with `encoding: "base64"`, `transactionDetails: "full"`
//! and `maxSupportedTransactionVersion: 0`.

use anyhow::{anyhow, bail, Context, Result};
use solana_sdk::message::VersionedMessage;
use solana_transaction_status_client_types::{
    option_serializer::OptionSerializer, UiConfirmedBlock, UiInstruction,
    UiTransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction, InnerInstructionGroup, ResolvedInstruction,
};

use super::{decoded_instructions, PreparedBlock, PreparedTransaction};
use crate::solana_grpc_source::SealedBlock;

/// Prepares the `getBlock` response for `slot`. Failed transactions are skipped, as on the
/// stream; a transaction's index is its position in the response, which is block order.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "archive catch-up (fhevm-internal#2085) is its first production caller"
    )
)]
pub(super) fn prepare_rpc_block(
    slot: u64,
    block: UiConfirmedBlock,
) -> Result<PreparedBlock> {
    let transactions = block.transactions.ok_or_else(|| {
        anyhow!("getBlock response for slot {slot} has no transactions")
    })?;
    let matching_transaction_count = transactions.len();
    let mut prepared = Vec::new();
    for (index, encoded) in transactions.into_iter().enumerate() {
        let meta = encoded.meta.ok_or_else(|| {
            anyhow!("transaction {index} in slot {slot} has no status meta")
        })?;
        if meta.err.is_some() {
            continue;
        }
        let transaction = encoded.transaction.decode().ok_or_else(|| {
            anyhow!(
                "transaction {index} in slot {slot} is not a sanitized base64 transaction"
            )
        })?;
        let signature = *transaction.signatures.first().ok_or_else(|| {
            anyhow!("transaction {index} in slot {slot} has no signature")
        })?;
        let instructions = resolve_rpc_transaction(&transaction.message, &meta)
            .with_context(|| format!("transaction {index} in slot {slot}"))?;
        prepared.push(PreparedTransaction {
            signature,
            index: index as u64,
            instructions: decoded_instructions(instructions)?,
        });
    }
    Ok(PreparedBlock {
        block: SealedBlock {
            slot,
            block_hash: decode_hash(&block.blockhash)
                .context("getBlock blockhash")?,
            parent_slot: block.parent_slot,
            parent_block_hash: decode_hash(&block.previous_blockhash)
                .context("getBlock previousBlockhash")?,
            block_time: block.block_time,
            block_height: block.block_height,
            executed_transaction_count: matching_transaction_count as u64,
            transactions: Vec::new(),
        },
        transactions: prepared,
        matching_transaction_count,
    })
}

fn resolve_rpc_transaction(
    message: &VersionedMessage,
    meta: &UiTransactionStatusMeta,
) -> Result<Vec<ResolvedInstruction>> {
    let static_keys = message
        .static_account_keys()
        .iter()
        .map(|key| key.to_bytes())
        .collect::<Vec<_>>();
    let (loaded_writable_keys, loaded_readonly_keys) = match &meta
        .loaded_addresses
    {
        OptionSerializer::Some(loaded) => (
            decode_keys(&loaded.writable)?,
            decode_keys(&loaded.readonly)?,
        ),
        _ if message
            .address_table_lookups()
            .is_some_and(|lookups| !lookups.is_empty()) =>
        {
            bail!("transaction uses address lookup tables but its meta has no loaded addresses")
        }
        _ => (Vec::new(), Vec::new()),
    };
    let top_level = message
        .instructions()
        .iter()
        .map(|instruction| CompiledInstruction {
            program_id_index: usize::from(instruction.program_id_index),
            account_indices: instruction
                .accounts
                .iter()
                .map(|index| usize::from(*index))
                .collect(),
            data: instruction.data.clone(),
            stack_height: None,
        })
        .collect();
    // A node that did not record inner instructions would hide every CPI into the host.
    let OptionSerializer::Some(inner) = &meta.inner_instructions else {
        bail!("transaction meta has no inner instructions")
    };
    let inner_groups = inner
        .iter()
        .map(|group| {
            Ok(InnerInstructionGroup {
                top_level_index: usize::from(group.index),
                instructions: group
                    .instructions
                    .iter()
                    .map(|instruction| {
                        let UiInstruction::Compiled(instruction) = instruction
                        else {
                            bail!("inner instruction is parsed; request base64 encoding")
                        };
                        Ok(CompiledInstruction {
                            program_id_index: usize::from(
                                instruction.program_id_index,
                            ),
                            account_indices: instruction
                                .accounts
                                .iter()
                                .map(|index| usize::from(*index))
                                .collect(),
                            data: bs58::decode(&instruction.data)
                                .into_vec()
                                .context("inner instruction data is not base58")?,
                            stack_height: instruction.stack_height,
                        })
                    })
                    .collect::<Result<_>>()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    zama_solana_transaction::resolve_transaction(
        &static_keys,
        &loaded_writable_keys,
        &loaded_readonly_keys,
        top_level,
        inner_groups,
    )
    .map_err(anyhow::Error::from)
}

fn decode_keys(keys: &[String]) -> Result<Vec<[u8; 32]>> {
    keys.iter().map(|key| decode_hash(key)).collect()
}

fn decode_hash(value: &str) -> Result<[u8; 32]> {
    let bytes = bs58::decode(value)
        .into_vec()
        .with_context(|| format!("{value} is not base58"))?;
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow!("{value} is {} bytes, expected 32", bytes.len()))
}

#[cfg(test)]
mod tests {
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
    use solana_transaction_status_client_types::{
        EncodedTransaction, TransactionBinaryEncoding, UiConfirmedBlock,
    };
    use yellowstone_grpc_proto::prelude::{
        CompiledInstruction as GrpcCompiledInstruction, InnerInstruction,
        InnerInstructions, Message as GrpcMessage, TransactionStatusMeta,
    };

    use super::{prepare_rpc_block, resolve_rpc_transaction};
    use crate::solana_grpc_listener::fhe_execute_acl_tests::{
        encoded_execution, reconstruct, with_events, ZAMA_HOST,
    };
    use crate::solana_grpc_listener::resolve_transaction_instructions;
    use crate::solana_reconstruct::DecodedInstruction;

    mod shared_fixtures {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../solana/test-fixtures/transaction_decoding.rs"
        ));
    }
    use shared_fixtures::{
        fixture_key, transaction_decoding_fixtures, ExpectedInstruction,
        ExpectedOutcome,
    };

    /// A compiled instruction as the fixtures and both wire formats describe it.
    struct Compiled {
        program_id_index: u8,
        accounts: Vec<u8>,
        data: Vec<u8>,
        stack_height: Option<u32>,
    }

    /// One transaction, written once and rendered in both wire formats.
    struct Transaction {
        signature: [u8; 64],
        static_keys: Vec<[u8; 32]>,
        loaded_writable: Vec<[u8; 32]>,
        loaded_readonly: Vec<[u8; 32]>,
        top_level: Vec<Compiled>,
        inner_groups: Vec<(u8, Vec<Compiled>)>,
    }

    impl Transaction {
        /// `getBlock`'s JSON for the transaction, spelled as the RPC wire format.
        fn rpc_json(&self, err: Value) -> Value {
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

        fn grpc(&self) -> (GrpcMessage, TransactionStatusMeta) {
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
                        program_id_index: u32::from(
                            instruction.program_id_index,
                        ),
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
    }

    fn block_json(slot: u64, transactions: Vec<Value>) -> UiConfirmedBlock {
        serde_json::from_value(json!({
            "previousBlockhash": bs58::encode([4; 32]).into_string(),
            "blockhash": bs58::encode([5; 32]).into_string(),
            "parentSlot": slot - 1,
            "transactions": transactions,
            "blockTime": 1_700_000_000,
            "blockHeight": slot - 2,
        }))
        .expect("getBlock JSON")
    }

    /// Resolves `transaction` from its `getBlock` JSON. The bytes are decoded without
    /// sanitizing, as the fixtures include shapes no ledger holds; production decoding
    /// sanitizes first (`rebuilds_a_slot_from_get_block_alone`).
    fn rpc_resolution(
        transaction: &Transaction,
    ) -> anyhow::Result<Vec<zama_solana_transaction::ResolvedInstruction>> {
        let block = block_json(9, vec![transaction.rpc_json(Value::Null)]);
        let encoded = block.transactions.unwrap().pop().unwrap();
        let EncodedTransaction::Binary(blob, TransactionBinaryEncoding::Base64) =
            &encoded.transaction
        else {
            panic!("base64 transaction")
        };
        let decoded: VersionedTransaction =
            bincode::deserialize(&BASE64_STANDARD.decode(blob).unwrap())
                .unwrap();
        resolve_rpc_transaction(&decoded.message, &encoded.meta.unwrap())
    }

    #[test]
    fn shared_transaction_decoding_contract() {
        let compiled =
            |instruction: &shared_fixtures::CompiledInstructionFixture| {
                Compiled {
                    program_id_index: u8::try_from(
                        instruction.program_id_index,
                    )
                    .unwrap(),
                    accounts: instruction.accounts.clone(),
                    data: instruction.data.clone(),
                    stack_height: instruction.stack_height,
                }
            };
        let keys =
            |tags: &[u8]| tags.iter().copied().map(fixture_key).collect();
        for fixture in transaction_decoding_fixtures() {
            let transaction = Transaction {
                signature: [1; 64],
                static_keys: keys(&fixture.static_account_tags),
                loaded_writable: keys(&fixture.loaded_writable_account_tags),
                loaded_readonly: keys(&fixture.loaded_readonly_account_tags),
                top_level: fixture.top_level.iter().map(compiled).collect(),
                inner_groups: fixture
                    .inner_groups
                    .iter()
                    .map(|group| {
                        (
                            u8::try_from(group.index).unwrap(),
                            group.instructions.iter().map(compiled).collect(),
                        )
                    })
                    .collect(),
            };
            let decoded = rpc_resolution(&transaction);
            match &fixture.expected {
                ExpectedOutcome::Accept { instructions } => {
                    let actual = decoded
                        .unwrap_or_else(|error| {
                            panic!("{}: {error:#}", fixture.name)
                        })
                        .into_iter()
                        .map(|instruction| ExpectedInstruction {
                            program: instruction.program_id,
                            accounts: instruction.accounts,
                            data: instruction.data,
                            top_level_index: u32::try_from(
                                instruction.top_level_index,
                            )
                            .unwrap(),
                            stack_height: instruction.stack_height,
                        })
                        .collect::<Vec<_>>();
                    let expected = instructions
                        .iter()
                        .map(|instruction| instruction.resolve())
                        .collect::<Vec<_>>();
                    assert_eq!(actual, expected, "{}", fixture.name);
                }
                ExpectedOutcome::Reject => {
                    assert!(decoded.is_err(), "{}", fixture.name);
                }
            }
        }
    }

    /// An app program's top-level instruction CPIs into `fhe_execute`, which emits its event
    /// through a self-CPI, as on chain.
    fn app_transaction(signature: u8, plaintext: [u8; 32]) -> Transaction {
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

    #[test]
    fn rebuilds_a_slot_from_get_block_alone() {
        let failed = app_transaction(7, [1; 32]);
        let succeeded = app_transaction(8, [2; 32]);
        let block = block_json(
            100,
            vec![
                failed.rpc_json(
                    json!({ "InstructionError": [0, { "Custom": 1 }] }),
                ),
                succeeded.rpc_json(Value::Null),
            ],
        );

        let prepared = prepare_rpc_block(100, block).unwrap();
        assert_eq!(prepared.block.checkpoint().slot, 100);
        assert_eq!(prepared.block.block_hash, [5; 32]);
        assert_eq!(prepared.block.parent_block_hash, [4; 32]);
        assert_eq!(prepared.block.block_time, Some(1_700_000_000));
        assert_eq!(prepared.matching_transaction_count, 2);
        let [transaction] = &prepared.transactions[..] else {
            panic!("the failed transaction is skipped")
        };
        assert_eq!(transaction.signature, Signature::from([8; 64]));
        assert_eq!(transaction.index, 1);

        let (message, meta) = succeeded.grpc();
        assert_eq!(
            rpc_resolution(&succeeded).unwrap(),
            resolve_transaction_instructions(&message, &meta).unwrap(),
            "getBlock and the stream resolve the same instructions"
        );

        let reconstructed = reconstruct(&transaction.instructions).unwrap();
        assert!(reconstructed.check_failures.is_empty());
        assert!(matches!(
            &reconstructed.records[..],
            [crate::solana_adapter::SolanaHostRecord::TrivialEncrypt(op)] if op.plaintext == [2; 32]
        ));
    }

    #[test]
    fn a_node_without_inner_instructions_is_refused() {
        let mut transaction = app_transaction(8, [2; 32]).rpc_json(Value::Null);
        transaction["meta"]
            .as_object_mut()
            .unwrap()
            .remove("innerInstructions");
        let error = prepare_rpc_block(100, block_json(100, vec![transaction]))
            .unwrap_err();
        assert!(
            format!("{error:#}").contains("no inner instructions"),
            "{error:#}"
        );
    }
}
