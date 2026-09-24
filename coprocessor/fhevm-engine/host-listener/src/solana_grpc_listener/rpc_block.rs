//! Blocks fetched with `getBlock`, prepared exactly like streamed ones. Each `FheExecutedEvent`
//! carries its execution's derivation context, so a block's own transactions are all the
//! listener needs: any archive can rebuild a slot the stream missed.
//!
//! Expects the response of `getBlock` with `encoding: "base64"`, `transactionDetails: "full"`
//! and `maxSupportedTransactionVersion: 0`.

use anyhow::{anyhow, bail, Context, Result};
use solana_sdk::{message::VersionedMessage, pubkey::Pubkey};
use solana_transaction_status_client_types::{
    option_serializer::OptionSerializer, UiConfirmedBlock, UiInstruction,
    UiTransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction, InnerInstructionGroup, ResolvedInstruction,
};

use super::{decoded_instructions, PreparedBlock, PreparedTransaction};
use crate::solana_grpc_source::SealedBlock;

/// Prepares the `getBlock` response for `slot`. As on the stream, only transactions naming
/// `program` count as matching, and failed ones are then skipped. A transaction's index is its
/// position in the response, which is block order.
pub(super) fn prepare_rpc_block(
    slot: u64,
    block: UiConfirmedBlock,
    program: &Pubkey,
) -> Result<PreparedBlock> {
    let transactions = block.transactions.ok_or_else(|| {
        anyhow!("getBlock response for slot {slot} has no transactions")
    })?;
    let executed_transaction_count = transactions.len() as u64;
    let program_address = program.to_string();
    let mut matching_transaction_count = 0;
    let mut prepared = Vec::new();
    for (index, encoded) in transactions.into_iter().enumerate() {
        let meta = encoded.meta.ok_or_else(|| {
            anyhow!("transaction {index} in slot {slot} has no status meta")
        })?;
        let transaction = encoded.transaction.decode().ok_or_else(|| {
            anyhow!(
                "transaction {index} in slot {slot} is not a sanitized base64 transaction"
            )
        })?;
        // Yellowstone's `account_include` matches static and lookup-table-loaded keys alike.
        let names_program = transaction
            .message
            .static_account_keys()
            .contains(program)
            || matches!(&meta.loaded_addresses, OptionSerializer::Some(loaded)
                    if loaded.writable.contains(&program_address)
                        || loaded.readonly.contains(&program_address));
        if !names_program {
            continue;
        }
        matching_transaction_count += 1;
        if meta.err.is_some() {
            continue;
        }
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
            executed_transaction_count,
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
    use solana_sdk::{signature::Signature, transaction::VersionedTransaction};
    use solana_transaction_status_client_types::{
        EncodedTransaction, TransactionBinaryEncoding, UiConfirmedBlock,
    };

    use super::super::wire_fixtures::{
        app_transaction, block_json, foreign_transaction, Compiled,
        Transaction, BLOCK_TIME,
    };
    use super::{prepare_rpc_block, resolve_rpc_transaction};
    use crate::solana_grpc_listener::resolve_transaction_instructions;
    use crate::solana_grpc_listener::test_support::{reconstruct, ZAMA_HOST};

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

    fn hash(slot: u64) -> [u8; 32] {
        [slot as u8; 32]
    }

    fn block(slot: u64, transactions: Vec<Value>) -> UiConfirmedBlock {
        block_json(slot, slot - 1, hash, transactions)
    }

    /// Resolves `transaction` from its `getBlock` JSON. The bytes are decoded without
    /// sanitizing, as the fixtures include shapes no ledger holds; production decoding
    /// sanitizes first (`rebuilds_a_slot_from_get_block_alone`).
    fn rpc_resolution(
        transaction: &Transaction,
    ) -> anyhow::Result<Vec<zama_solana_transaction::ResolvedInstruction>> {
        let block = block(9, vec![transaction.rpc_json(Value::Null)]);
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

    /// Both wire formats resolve every shared fixture as it expects.
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
            let (message, meta) = transaction.grpc();
            for (wire, decoded) in [
                ("gRPC", resolve_transaction_instructions(&message, &meta)),
                ("getBlock", rpc_resolution(&transaction)),
            ] {
                match &fixture.expected {
                    ExpectedOutcome::Accept { instructions } => {
                        let actual = decoded
                            .unwrap_or_else(|error| {
                                panic!("{} ({wire}): {error:#}", fixture.name)
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
                        assert_eq!(
                            actual, expected,
                            "{} ({wire})",
                            fixture.name
                        );
                    }
                    ExpectedOutcome::Reject => {
                        assert!(decoded.is_err(), "{} ({wire})", fixture.name);
                    }
                }
            }
        }
    }

    #[test]
    fn rebuilds_a_slot_from_get_block_alone() {
        let failed = app_transaction(7, [1; 32]);
        let succeeded = app_transaction(8, [2; 32]);
        let block = block(
            100,
            vec![
                foreign_transaction(6).rpc_json(Value::Null),
                failed.rpc_json(
                    json!({ "InstructionError": [0, { "Custom": 1 }] }),
                ),
                succeeded.rpc_json(Value::Null),
            ],
        );

        let prepared =
            prepare_rpc_block(100, block, &ZAMA_HOST.parse().unwrap()).unwrap();
        assert_eq!(prepared.block.checkpoint().slot, 100);
        assert_eq!(prepared.block.block_hash, hash(100));
        assert_eq!(prepared.block.parent_block_hash, hash(99));
        assert_eq!(prepared.block.block_time, Some(BLOCK_TIME));
        assert_eq!(prepared.block.executed_transaction_count, 3);
        assert_eq!(
            prepared.matching_transaction_count, 2,
            "the stream's account filter counts the failed transaction, not the foreign one"
        );
        let [transaction] = &prepared.transactions[..] else {
            panic!("the failed transaction is skipped")
        };
        assert_eq!(transaction.signature, Signature::from([8; 64]));
        assert_eq!(transaction.index, 2);

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
        let error = prepare_rpc_block(
            100,
            block(100, vec![transaction]),
            &ZAMA_HOST.parse().unwrap(),
        )
        .unwrap_err();
        assert!(
            format!("{error:#}").contains("no inner instructions"),
            "{error:#}"
        );
    }
}
