//! Blocks fetched from an archive RPC, prepared exactly like streamed ones. Each
//! `FheExecutedEvent` carries its execution's derivation context, so a block's own transactions
//! are all the listener needs: any archive can rebuild a slot the stream missed.
//!
//! As on the stream, a response is bounded per transaction, not per block. `getBlock` with
//! `transactionDetails: "accounts"` lists each transaction's signature, account keys and error,
//! without instruction data or logs ([`list_rpc_block`]). Each successful transaction naming the
//! host is then fetched alone with `getTransaction`, `encoding: "base64"` and
//! `maxSupportedTransactionVersion: 0` ([`prepare_rpc_block`]).

use anyhow::{anyhow, bail, ensure, Context, Result};
use solana_sdk::{
    message::VersionedMessage, pubkey::Pubkey, signature::Signature,
};
use solana_transaction_status_client_types::{
    option_serializer::OptionSerializer,
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    UiConfirmedBlock, UiInstruction, UiTransactionStatusMeta,
};
use zama_solana_transaction::{
    CompiledInstruction, InnerInstructionGroup, ResolvedInstruction,
};

use super::{decoded_instructions, PreparedBlock, PreparedTransaction};
use crate::solana_grpc_source::SealedBlock;

/// A block's header and the transactions of it the listener must fetch.
#[derive(Debug)]
pub(super) struct RpcBlockListing {
    pub block: SealedBlock,
    /// Each successful transaction naming the host: its index in the block and its signature.
    pub matching: Vec<(u64, Signature)>,
}

/// Lists the `getBlock` response for `slot`. As on the stream, a transaction matches when it
/// names `program`, in its static or lookup-table-loaded keys, and succeeded. A transaction's
/// index is its position in the response, which is block order.
pub(super) fn list_rpc_block(
    slot: u64,
    block: UiConfirmedBlock,
    program: &Pubkey,
) -> Result<RpcBlockListing> {
    let transactions = block.transactions.ok_or_else(|| {
        anyhow!("getBlock response for slot {slot} has no transactions")
    })?;
    let executed_transaction_count = transactions.len() as u64;
    let program_address = program.to_string();
    let mut matching = Vec::new();
    for (index, encoded) in transactions.into_iter().enumerate() {
        let meta = encoded.meta.ok_or_else(|| {
            anyhow!("transaction {index} in slot {slot} has no status meta")
        })?;
        let EncodedTransaction::Accounts(accounts) = encoded.transaction else {
            bail!(
                "transaction {index} in slot {slot} is not an account list; request transactionDetails \"accounts\""
            )
        };
        if meta.err.is_some()
            || !accounts
                .account_keys
                .iter()
                .any(|account| account.pubkey == program_address)
        {
            continue;
        }
        let signature = accounts.signatures.first().ok_or_else(|| {
            anyhow!("transaction {index} in slot {slot} has no signature")
        })?;
        let signature = signature.parse::<Signature>().with_context(|| {
            format!(
                "transaction {index} in slot {slot} has an invalid signature"
            )
        })?;
        matching.push((index as u64, signature));
    }
    Ok(RpcBlockListing {
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
        },
        matching,
    })
}

/// Prepares a listed block from the `getTransaction` response of each matching transaction, in
/// listing order.
pub(super) fn prepare_rpc_block(
    listing: RpcBlockListing,
    transactions: Vec<EncodedConfirmedTransactionWithStatusMeta>,
) -> Result<PreparedBlock> {
    let slot = listing.block.slot;
    ensure!(
        transactions.len() == listing.matching.len(),
        "slot {slot} lists {} matching transactions but {} were fetched",
        listing.matching.len(),
        transactions.len()
    );
    let mut prepared = Vec::new();
    for ((index, signature), fetched) in
        listing.matching.into_iter().zip(transactions)
    {
        ensure!(
            fetched.slot == slot,
            "getTransaction places {signature} in slot {}, getBlock in slot {slot}",
            fetched.slot
        );
        let meta = fetched.transaction.meta.ok_or_else(|| {
            anyhow!("transaction {signature} has no status meta")
        })?;
        ensure!(meta.err.is_none(), "transaction {signature} failed");
        let transaction =
            fetched.transaction.transaction.decode().ok_or_else(|| {
                anyhow!(
                    "transaction {signature} is not a sanitized base64 transaction"
                )
            })?;
        ensure!(
            transaction.signatures.first() == Some(&signature),
            "getTransaction for {signature} returned another transaction"
        );
        let instructions = resolve_rpc_transaction(&transaction.message, &meta)
            .with_context(|| format!("transaction {index} in slot {slot}"))?;
        prepared.push(PreparedTransaction {
            signature,
            index,
            instructions: decoded_instructions(instructions)?,
        });
    }
    Ok(PreparedBlock {
        block: listing.block,
        transactions: prepared,
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
        pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction,
    };
    use solana_transaction_status_client_types::{
        EncodedTransaction, TransactionBinaryEncoding, UiConfirmedBlock,
    };

    use super::super::wire_fixtures::{
        app_transaction, block_json, foreign_transaction, Compiled,
        Transaction, BLOCK_TIME,
    };
    use super::{
        list_rpc_block, prepare_rpc_block, resolve_rpc_transaction,
        RpcBlockListing,
    };
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

    /// A host program named only through a lookup table.
    fn loading_the_host(signature: u8) -> Transaction {
        let host = ZAMA_HOST.parse::<Pubkey>().unwrap().to_bytes();
        let mut transaction = foreign_transaction(signature);
        transaction.loaded_readonly = vec![host];
        transaction
    }

    /// The listing keeps what the stream's filter delivers: successful transactions naming the
    /// host, statically or through a lookup table. Each is then fetched alone.
    #[test]
    fn rebuilds_a_slot_from_get_block_and_get_transaction() {
        let failed = app_transaction(7, [1; 32]);
        let succeeded = app_transaction(8, [2; 32]);
        let loaded = loading_the_host(9);
        let block = block(
            100,
            vec![
                foreign_transaction(6).rpc_accounts_json(Value::Null),
                failed.rpc_accounts_json(
                    json!({ "InstructionError": [0, { "Custom": 1 }] }),
                ),
                succeeded.rpc_accounts_json(Value::Null),
                loaded.rpc_accounts_json(Value::Null),
            ],
        );

        let listing =
            list_rpc_block(100, block, &ZAMA_HOST.parse().unwrap()).unwrap();
        assert_eq!(listing.block.checkpoint().slot, 100);
        assert_eq!(listing.block.block_hash, hash(100));
        assert_eq!(listing.block.parent_block_hash, hash(99));
        assert_eq!(listing.block.block_time, Some(BLOCK_TIME));
        assert_eq!(listing.block.executed_transaction_count, 4);
        assert_eq!(
            listing.matching,
            vec![(2, Signature::from([8; 64])), (3, Signature::from([9; 64]))]
        );

        let prepared = prepare_rpc_block(
            listing,
            vec![succeeded.rpc_transaction(100), loaded.rpc_transaction(100)],
        )
        .unwrap();
        let [transaction, _] = &prepared.transactions[..] else {
            panic!("two host transactions")
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

    fn listed(transaction: &Transaction) -> RpcBlockListing {
        list_rpc_block(
            100,
            block(100, vec![transaction.rpc_accounts_json(Value::Null)]),
            &ZAMA_HOST.parse().unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn a_node_without_inner_instructions_is_refused() {
        let transaction = app_transaction(8, [2; 32]);
        let mut fetched =
            serde_json::to_value(transaction.rpc_transaction(100)).unwrap();
        fetched["meta"]
            .as_object_mut()
            .unwrap()
            .remove("innerInstructions");
        let error = prepare_rpc_block(
            listed(&transaction),
            vec![serde_json::from_value(fetched).unwrap()],
        )
        .unwrap_err();
        assert!(
            format!("{error:#}").contains("no inner instructions"),
            "{error:#}"
        );
    }

    /// `getTransaction` must return the listed transaction, from the listed slot.
    #[test]
    fn a_fetched_transaction_must_match_its_listing() {
        let transaction = app_transaction(8, [2; 32]);
        for (fetched, expected) in [
            (
                app_transaction(9, [2; 32]).rpc_transaction(100),
                "another transaction",
            ),
            (transaction.rpc_transaction(101), "in slot 101"),
        ] {
            let error = prepare_rpc_block(listed(&transaction), vec![fetched])
                .unwrap_err();
            assert!(format!("{error:#}").contains(expected), "{error:#}");
        }
        let error =
            prepare_rpc_block(listed(&transaction), vec![]).unwrap_err();
        assert!(format!("{error:#}").contains("were fetched"), "{error:#}");
    }

    #[test]
    fn a_full_transaction_in_the_listing_is_refused() {
        let error = list_rpc_block(
            100,
            block(100, vec![app_transaction(8, [2; 32]).rpc_json(Value::Null)]),
            &ZAMA_HOST.parse().unwrap(),
        )
        .unwrap_err();
        assert!(
            format!("{error:#}").contains("not an account list"),
            "{error:#}"
        );
    }
}
