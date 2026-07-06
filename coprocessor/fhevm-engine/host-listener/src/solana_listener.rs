//! Live driver that polls a Solana validator for `zama-host` Anchor CPI events
//! and ingests them into the coprocessor database.
//!
//! This is the Solana counterpart of the EVM listener in [`crate::cmd`]: where the
//! EVM path subscribes to logs over a websocket, the Solana path polls
//! `getSignaturesForAddress(zama_host)` for new transactions, pulls each
//! transaction's inner instructions, and decodes the `emit_cpi!` self-invocations
//! into [`SolanaHostEvent`]s. Decoding and database insertion are shared with the
//! in-process integration path via [`crate::solana_adapter`]; only the transport
//! (real RPC vs. LiteSVM) differs, so the two stay behaviorally identical.

#[cfg(feature = "solana-reconstruct")]
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    UiInstruction, UiMessage, UiTransactionEncoding,
};
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::database::tfhe_event_propagate::Database;
use crate::solana_adapter::{
    decode_solana_transaction_events, insert_solana_events,
    solana_transaction_id, SolanaBlockMeta, SolanaHostEvent,
};

/// `getSignaturesForAddress` returns at most this many entries per call.
const SIGNATURES_PAGE_LIMIT: usize = 1_000;

/// Runtime configuration for the live Solana listener loop.
#[derive(Clone, Debug)]
pub struct SolanaListenerConfig {
    /// JSON-RPC endpoint of the validator (e.g. `http://127.0.0.1:8899`).
    pub rpc_url: String,
    /// `zama-host` program whose CPI events are ingested.
    pub program_id: Pubkey,
    /// Delay between polls once the listener has caught up.
    pub poll_interval: Duration,
    /// Commitment used for both signature enumeration and transaction fetches.
    pub commitment: CommitmentConfig,
}

/// Runs the polling loop until `cancel` fires.
///
/// The cursor is the newest signature already ingested; each poll fetches every
/// signature strictly newer than it (paginating backwards), then ingests them
/// oldest-first so dependence chains see operands before their results.
/// Insertion is idempotent (`ON CONFLICT DO NOTHING`), so a restart that replays
/// already-seen transactions is harmless.
pub async fn run(
    db: &Database,
    rpc: &RpcClient,
    config: &SolanaListenerConfig,
    cancel: CancellationToken,
) -> Result<()> {
    info!(
        program_id = %config.program_id,
        rpc_url = %config.rpc_url,
        "Starting Solana host listener"
    );

    let mut cursor: Option<Signature> = None;
    #[cfg(feature = "solana-reconstruct")]
    let mut encrypted_value_tracker =
        crate::solana_reconstruct::EncryptedValueLineageTracker::new();
    let mut ticker = tokio::time::interval(config.poll_interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                info!("Solana host listener cancelled");
                return Ok(());
            }
            _ = ticker.tick() => {}
        }

        let signatures = match fetch_new_signatures(rpc, config, cursor).await {
            Ok(signatures) => signatures,
            Err(err) => {
                error!(error = %err, "Failed to fetch new signatures; retrying next tick");
                continue;
            }
        };

        for signature in signatures {
            match ingest_signature(
                db,
                rpc,
                config,
                &signature,
                #[cfg(feature = "solana-reconstruct")]
                &mut encrypted_value_tracker,
            )
            .await
            {
                Ok(()) => cursor = Some(signature),
                Err(err) => {
                    // Stop advancing the cursor on the first failure so the next
                    // poll retries this signature rather than skipping it.
                    error!(signature = %signature, error = %err, "Failed to ingest transaction");
                    break;
                }
            }
        }
    }
}

/// Returns every confirmed signature for the program strictly newer than `cursor`,
/// ordered oldest-first. Pages backwards from newest until it reaches `cursor` or
/// exhausts history.
async fn fetch_new_signatures(
    rpc: &RpcClient,
    config: &SolanaListenerConfig,
    cursor: Option<Signature>,
) -> Result<Vec<Signature>> {
    let mut collected: Vec<Signature> = Vec::new();
    let mut before: Option<Signature> = None;

    loop {
        let page = rpc
            .get_signatures_for_address_with_config(
                &config.program_id,
                GetConfirmedSignaturesForAddress2Config {
                    before,
                    until: cursor,
                    limit: Some(SIGNATURES_PAGE_LIMIT),
                    commitment: Some(config.commitment),
                },
            )
            .await
            .context("get_signatures_for_address_with_config")?;

        if page.is_empty() {
            break;
        }

        let page_len = page.len();
        for status in &page {
            // Skip transactions that failed on-chain: they emitted no committed events.
            if status.err.is_some() {
                continue;
            }
            let signature = Signature::from_str(&status.signature)
                .with_context(|| {
                    format!("parse signature {}", status.signature)
                })?;
            collected.push(signature);
            before = Some(signature);
        }

        if page_len < SIGNATURES_PAGE_LIMIT {
            break;
        }
    }

    // Newest-first across pages -> oldest-first for ingestion.
    collected.reverse();
    if !collected.is_empty() {
        debug!(count = collected.len(), "Fetched new signatures");
    }
    Ok(collected)
}

/// Fetches one transaction, decodes its `zama-host` CPI events, and inserts them.
async fn ingest_signature(
    db: &Database,
    rpc: &RpcClient,
    config: &SolanaListenerConfig,
    signature: &Signature,
    #[cfg(feature = "solana-reconstruct")]
    encrypted_value_tracker: &mut crate::solana_reconstruct::EncryptedValueLineageTracker,
) -> Result<()> {
    let confirmed = rpc
        .get_transaction_with_config(
            signature,
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Json),
                commitment: Some(config.commitment),
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .with_context(|| format!("get_transaction {signature}"))?;

    let (events, block) = extract_host_events_with_tracker(
        &confirmed,
        &config.program_id,
        #[cfg(feature = "solana-reconstruct")]
        encrypted_value_tracker,
    )
    .with_context(|| {
        format!("decode CPI events for transaction {signature}")
    })?;

    if events.is_empty() {
        return Ok(());
    }

    let transaction_id = solana_transaction_id(signature.as_ref());
    let mut tx = db
        .new_transaction()
        .await
        .context("open database transaction")?;
    let stats =
        insert_solana_events(db, &mut tx, events, transaction_id, block)
            .await
            .context("insert_solana_events")?;
    tx.commit().await.context("commit database transaction")?;

    info!(
        signature = %signature,
        slot = block.block_number,
        tfhe_events = stats.tfhe_events,
        acl_events = stats.acl_events,
        inserted_rows = stats.inserted_rows,
        "Ingested Solana host events"
    );
    Ok(())
}

/// Decodes the `zama-host` events carried by a confirmed transaction and derives
/// its block metadata.
///
/// A host event reaches us by one of two transports, chosen per emitting frame:
/// `emit_cpi!` puts the event bytes in an inner instruction that self-invokes the
/// program, while `emit!` writes them as a `Program data:` log line. A large
/// `fhe_eval` frame (event count above the CPI cap, e.g. `confidential_burn`)
/// uses log transport, so a CPI-only decoder silently drops its whole compute
/// graph. We gather both the inner instructions and the log messages and hand
/// them to [`decode_solana_transaction_events`], the shared decoder that strips
/// the CPI/Anchor discriminators, scopes log lines to this program, and rejects a
/// transaction that mixes the two transports.
pub fn extract_host_events(
    confirmed: &EncodedConfirmedTransactionWithStatusMeta,
    program_id: &Pubkey,
) -> Result<(Vec<SolanaHostEvent>, SolanaBlockMeta)> {
    #[cfg(feature = "solana-reconstruct")]
    {
        let mut tracker =
            crate::solana_reconstruct::EncryptedValueLineageTracker::new();
        extract_host_events_with_tracker(confirmed, program_id, &mut tracker)
    }
    #[cfg(not(feature = "solana-reconstruct"))]
    {
        extract_host_events_with_tracker(confirmed, program_id)
    }
}

fn extract_host_events_with_tracker(
    confirmed: &EncodedConfirmedTransactionWithStatusMeta,
    program_id: &Pubkey,
    #[cfg(feature = "solana-reconstruct")]
    encrypted_value_tracker: &mut crate::solana_reconstruct::EncryptedValueLineageTracker,
) -> Result<(Vec<SolanaHostEvent>, SolanaBlockMeta)> {
    let block = block_meta(confirmed)?;
    let meta = confirmed
        .transaction
        .meta
        .as_ref()
        .ok_or_else(|| anyhow!("transaction has no metadata"))?;

    let account_keys = account_keys(confirmed)?;
    let program_id = program_id.to_string();

    // CPI transport: each `emit_cpi!` event is an inner instruction of this
    // program; collect (program, data) pairs and let the decoder filter + decode.
    let mut cpi_instructions: Vec<(String, Vec<u8>)> = Vec::new();
    if let OptionSerializer::Some(inner) = &meta.inner_instructions {
        for group in inner {
            for instruction in &group.instructions {
                let UiInstruction::Compiled(compiled) = instruction else {
                    continue;
                };
                let program = account_keys
                    .get(compiled.program_id_index as usize)
                    .ok_or_else(|| {
                        anyhow!(
                            "program_id_index {} out of range",
                            compiled.program_id_index
                        )
                    })?;
                let data = bs58::decode(&compiled.data)
                    .into_vec()
                    .context("base58-decode inner instruction data")?;
                cpi_instructions.push((program.clone(), data));
            }
        }
    }

    #[cfg(feature = "solana-reconstruct")]
    let all_instructions =
        decoded_transaction_instructions(confirmed, &account_keys)?;

    // Log transport: `emit!` events arrive as `Program data:` log lines. Do not
    // early-return when there are no inner instructions — a log-only transaction
    // (every large eval frame) carries all its events here.
    let logs: Vec<String> = match &meta.log_messages {
        OptionSerializer::Some(logs) => logs.clone(),
        OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
    };

    let events = decode_solana_transaction_events(
        &logs,
        cpi_instructions
            .iter()
            .map(|(program, data)| (program.as_str(), data.as_slice())),
        &program_id,
    )
    .context("decode host events")?;

    #[cfg(feature = "solana-reconstruct")]
    let events = {
        let mut events = events;
        events.extend(
            crate::solana_reconstruct::decode_encrypted_value_fetch_events(
                &all_instructions,
                &program_id,
                encrypted_value_tracker,
            ),
        );
        events
    };

    Ok((events, block))
}

#[cfg(feature = "solana-reconstruct")]
fn decoded_transaction_instructions(
    confirmed: &EncodedConfirmedTransactionWithStatusMeta,
    account_keys: &[String],
) -> Result<Vec<crate::solana_reconstruct::DecodedInstruction>> {
    let account_key_bytes = account_keys
        .iter()
        .map(|key| {
            Pubkey::from_str(key)
                .map(|pubkey| pubkey.to_bytes())
                .with_context(|| format!("parse account key {key}"))
        })
        .collect::<Result<Vec<_>>>()?;

    let resolve_accounts = |idxs: &[u8]| -> Vec<[u8; 32]> {
        idxs.iter()
            .filter_map(|&index| account_key_bytes.get(index as usize).copied())
            .collect()
    };

    let decode_compiled =
        |top_level_index: u32,
         is_inner: bool,
         program_id_index: u8,
         data: &str,
         accounts: &[u8]|
         -> Result<crate::solana_reconstruct::DecodedInstruction> {
            let program = account_keys
                .get(program_id_index as usize)
                .cloned()
                .unwrap_or_default();
            let data = bs58::decode(data)
                .into_vec()
                .context("base58-decode instruction data")?;
            Ok(crate::solana_reconstruct::DecodedInstruction {
                program,
                data,
                accounts: resolve_accounts(accounts),
                top_level_index,
                is_inner,
            })
        };

    let mut top_level_by_index = HashMap::new();
    let top_level_len = match &confirmed.transaction.transaction {
        EncodedTransaction::Json(ui_tx) => match &ui_tx.message {
            UiMessage::Raw(raw) => {
                for (index, instruction) in raw.instructions.iter().enumerate()
                {
                    top_level_by_index.insert(
                        index as u32,
                        decode_compiled(
                            index as u32,
                            false,
                            instruction.program_id_index,
                            &instruction.data,
                            &instruction.accounts,
                        )?,
                    );
                }
                raw.instructions.len()
            }
            UiMessage::Parsed(parsed) => {
                for (index, instruction) in
                    parsed.instructions.iter().enumerate()
                {
                    let UiInstruction::Compiled(compiled) = instruction else {
                        continue;
                    };
                    top_level_by_index.insert(
                        index as u32,
                        decode_compiled(
                            index as u32,
                            false,
                            compiled.program_id_index,
                            &compiled.data,
                            &compiled.accounts,
                        )?,
                    );
                }
                parsed.instructions.len()
            }
        },
        _ => bail!("expected a JSON-encoded transaction"),
    };

    let mut inner_by_index: HashMap<u32, Vec<_>> = HashMap::new();
    if let Some(meta) = &confirmed.transaction.meta {
        if let OptionSerializer::Some(inner) = &meta.inner_instructions {
            for group in inner {
                let mut decoded = Vec::new();
                for instruction in &group.instructions {
                    let UiInstruction::Compiled(compiled) = instruction else {
                        continue;
                    };
                    decoded.push(decode_compiled(
                        group.index as u32,
                        true,
                        compiled.program_id_index,
                        &compiled.data,
                        &compiled.accounts,
                    )?);
                }
                inner_by_index.insert(group.index as u32, decoded);
            }
        }
    }

    let mut ordered = Vec::new();
    for index in 0..top_level_len as u32 {
        if let Some(instruction) = top_level_by_index.remove(&index) {
            ordered.push(instruction);
        }
        if let Some(inner) = inner_by_index.remove(&index) {
            ordered.extend(inner);
        }
    }
    ordered.extend(top_level_by_index.into_values());
    ordered.extend(inner_by_index.into_values().flatten());
    Ok(ordered)
}

/// Full ordered account-key list for a JSON-encoded transaction: static keys from
/// the message followed by address-table-loaded writable then readonly keys, which
/// is the order `program_id_index` indexes into.
fn account_keys(
    confirmed: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<Vec<String>> {
    let mut keys = match &confirmed.transaction.transaction {
        EncodedTransaction::Json(ui_tx) => match &ui_tx.message {
            UiMessage::Raw(raw) => raw.account_keys.clone(),
            UiMessage::Parsed(parsed) => parsed
                .account_keys
                .iter()
                .map(|k| k.pubkey.clone())
                .collect(),
        },
        _ => bail!("expected a JSON-encoded transaction"),
    };

    if let Some(meta) = &confirmed.transaction.meta {
        if let OptionSerializer::Some(loaded) = &meta.loaded_addresses {
            keys.extend(loaded.writable.iter().cloned());
            keys.extend(loaded.readonly.iter().cloned());
        }
    }

    Ok(keys)
}

/// Builds [`SolanaBlockMeta`] from the confirmed transaction's slot and block time.
fn block_meta(
    confirmed: &EncodedConfirmedTransactionWithStatusMeta,
) -> Result<SolanaBlockMeta> {
    let block_time = confirmed
        .block_time
        .ok_or_else(|| anyhow!("transaction has no block_time"))?;
    let datetime = OffsetDateTime::from_unix_timestamp(block_time)
        .with_context(|| format!("invalid block_time {block_time}"))?;
    Ok(SolanaBlockMeta {
        block_number: confirmed.slot,
        block_timestamp: PrimitiveDateTime::new(
            datetime.date(),
            datetime.time(),
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::{
        anchor_event_discriminator, ANCHOR_EVENT_IX_TAG_LE, EVENT_VERSION,
    };
    use crate::solana_adapter::SolanaMappedEvent;

    const ZAMA_HOST: &str = "6rQaBev7B67LrQW7nJPBhJYt7rHSK38DWuz1LdiQRFcf";
    const FOREIGN: &str = "11111111111111111111111111111111";

    /// Mirrors the on-chain `emit_cpi!` byte layout: CPI sentinel tag, event
    /// discriminator, then the borsh payload.
    fn anchor_event_cpi(name: &str, payload: Vec<u8>) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&ANCHOR_EVENT_IX_TAG_LE);
        encoded.extend_from_slice(&anchor_event_discriminator(name));
        encoded.extend_from_slice(&payload);
        encoded
    }

    /// A `TrivialEncryptEvent` payload: version, padded scalar, to_type, result.
    fn trivial_encrypt_payload() -> Vec<u8> {
        let mut payload = vec![EVENT_VERSION];
        payload.extend_from_slice(&[9; 32]); // input handle
        payload.extend_from_slice(&[0; 31]); // scalar high bytes
        payload.push(7); // scalar low byte
        payload.push(5); // to_type (euint64)
        payload.extend_from_slice(&[8; 32]); // result handle
        payload
    }

    /// Builds a confirmed-transaction fixture in the exact JSON shape returned by
    /// `getTransaction(..., encoding=json)`, with one inner instruction whose
    /// program is `program_index` and whose data is `cpi_bytes`.
    fn confirmed_tx(
        cpi_bytes: &[u8],
        program_index: u8,
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let data = bs58::encode(cpi_bytes).into_string();
        let json = format!(
            r#"{{
              "slot": 42,
              "blockTime": 1700000000,
              "transaction": {{
                "signatures": ["{FOREIGN}"],
                "message": {{
                  "header": {{
                    "numRequiredSignatures": 1,
                    "numReadonlySignedAccounts": 0,
                    "numReadonlyUnsignedAccounts": 1
                  }},
                  "accountKeys": ["{FOREIGN}", "{ZAMA_HOST}"],
                  "recentBlockhash": "{FOREIGN}",
                  "instructions": []
                }}
              }},
              "meta": {{
                "err": null,
                "status": {{ "Ok": null }},
                "fee": 5000,
                "preBalances": [],
                "postBalances": [],
                "innerInstructions": [
                  {{
                    "index": 0,
                    "instructions": [
                      {{ "programIdIndex": {program_index}, "accounts": [1], "data": "{data}", "stackHeight": 2 }}
                    ]
                  }}
                ]
              }}
            }}"#
        );
        serde_json::from_str(&json)
            .expect("valid confirmed-transaction fixture")
    }

    #[test]
    fn extracts_zama_host_cpi_event_from_inner_instruction() {
        let bytes =
            anchor_event_cpi("TrivialEncryptEvent", trivial_encrypt_payload());
        let program_id = Pubkey::from_str(ZAMA_HOST).unwrap();

        let (events, block) =
            extract_host_events(&confirmed_tx(&bytes, 1), &program_id).unwrap();

        assert_eq!(block.block_number, 42);
        assert_eq!(events.len(), 1);
        let SolanaMappedEvent::Tfhe(_) =
            crate::solana_adapter::map_solana_event(
                events.into_iter().next().unwrap(),
            )
        else {
            panic!("expected a TFHE-mapped event");
        };
    }

    /// Builds a fixture whose `zama-host` events arrive via log transport
    /// (`emit!`): no compute inner instructions, just the program invoke/success
    /// brackets and the `Program data:` lines carrying the events.
    fn confirmed_tx_with_logs(
        log_lines: &[String],
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let logs = log_lines
            .iter()
            .map(|line| format!("{line:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        let json = format!(
            r#"{{
              "slot": 42,
              "blockTime": 1700000000,
              "transaction": {{
                "signatures": ["{FOREIGN}"],
                "message": {{
                  "header": {{ "numRequiredSignatures": 1, "numReadonlySignedAccounts": 0, "numReadonlyUnsignedAccounts": 1 }},
                  "accountKeys": ["{FOREIGN}", "{ZAMA_HOST}"],
                  "recentBlockhash": "{FOREIGN}",
                  "instructions": []
                }}
              }},
              "meta": {{
                "err": null,
                "status": {{ "Ok": null }},
                "fee": 5000,
                "preBalances": [],
                "postBalances": [],
                "innerInstructions": [],
                "logMessages": [{logs}]
              }}
            }}"#
        );
        serde_json::from_str(&json)
            .expect("valid confirmed-transaction fixture")
    }

    /// A large eval frame (e.g. `confidential_burn`) switches from `emit_cpi!` to
    /// `emit!`, so its events arrive only as `Program data:` log lines. The
    /// listener must decode them or it silently drops the whole compute graph —
    /// the regression that left burned-amount handles unmaterialized.
    #[test]
    fn extracts_zama_host_event_from_log_transport() {
        use base64::engine::general_purpose::STANDARD as BASE64;
        use base64::Engine;

        let mut event_bytes =
            anchor_event_discriminator("TrivialEncryptEvent").to_vec();
        event_bytes.extend_from_slice(&trivial_encrypt_payload());
        let logs = vec![
            format!("Program {ZAMA_HOST} invoke [1]"),
            format!("Program data: {}", BASE64.encode(&event_bytes)),
            format!("Program {ZAMA_HOST} success"),
        ];
        let program_id = Pubkey::from_str(ZAMA_HOST).unwrap();

        let (events, block) =
            extract_host_events(&confirmed_tx_with_logs(&logs), &program_id)
                .unwrap();

        assert_eq!(block.block_number, 42);
        assert_eq!(
            events.len(),
            1,
            "log-transport event must be decoded, not dropped"
        );
    }

    #[test]
    fn ignores_inner_instruction_from_other_program() {
        let bytes =
            anchor_event_cpi("TrivialEncryptEvent", trivial_encrypt_payload());
        let program_id = Pubkey::from_str(ZAMA_HOST).unwrap();

        // program_index 0 is the foreign account key, not zama-host.
        let (events, _) =
            extract_host_events(&confirmed_tx(&bytes, 0), &program_id).unwrap();

        assert!(events.is_empty());
    }

    #[test]
    fn block_meta_uses_slot_and_block_time() {
        let bytes =
            anchor_event_cpi("TrivialEncryptEvent", trivial_encrypt_payload());
        let program_id = Pubkey::from_str(ZAMA_HOST).unwrap();

        let (_, block) =
            extract_host_events(&confirmed_tx(&bytes, 1), &program_id).unwrap();

        let expected =
            OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        assert_eq!(
            block.block_timestamp,
            PrimitiveDateTime::new(expected.date(), expected.time())
        );
    }
}
