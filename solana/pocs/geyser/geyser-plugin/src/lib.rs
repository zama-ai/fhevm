//! A minimal Geyser plugin that tracks two things for a single target program:
//!
//!   1. **CPI calls** to the program — detected from the inner instructions of
//!      each notified transaction (any inner instruction whose program is the
//!      target program is, by definition, a cross-program invocation).
//!   2. **PDAs the program creates / owns** — detected from account update
//!      notifications whose `owner` equals the target program id.
//!
//! The plugin is configured via the JSON file passed to the validator with
//! `--geyser-plugin-config`. Beyond the mandatory `libpath`, it reads:
//!
//! ```json
//! {
//!   "libpath": "/abs/path/libgeyser_tracker_plugin.dylib",
//!   "program_id": "H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8",
//!   "log_path": "/abs/path/geyser-events.log"   // optional; stderr if omitted
//! }
//! ```

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::str::FromStr;
use std::sync::Mutex;

use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaTransactionInfoVersions,
    Result as PluginResult,
};
use serde::Deserialize;
use solana_message::compiled_instruction::CompiledInstruction;
use solana_pubkey::Pubkey;

#[derive(Debug, Deserialize)]
struct PluginConfig {
    /// Base58 program id to track.
    program_id: String,
    /// Optional path to append human-readable events to. Defaults to stderr.
    #[serde(default)]
    log_path: Option<String>,
}

#[derive(Debug, Default)]
pub struct GeyserTracker {
    program_id: Option<Pubkey>,
    /// Sink for events; `None` => write to stderr.
    log_file: Mutex<Option<File>>,
    /// Pubkeys already seen owned by the program, so we can distinguish the
    /// first observation ("create") from later ones ("update").
    seen_pdas: Mutex<HashSet<[u8; 32]>>,
}

impl GeyserTracker {
    fn emit(&self, line: &str) {
        log::info!(target: "geyser_tracker", "{line}");
        let mut guard = self.log_file.lock().unwrap();
        if let Some(file) = guard.as_mut() {
            let _ = writeln!(file, "{line}");
            let _ = file.flush();
        } else {
            eprintln!("[geyser-tracker] {line}");
        }
    }

    /// Inspect a transaction for direct calls and CPI calls to the target
    /// program, given the fully-resolved account key list (static keys plus any
    /// address-lookup-table loaded addresses), the top-level instructions, and
    /// the status meta carrying inner instructions.
    fn inspect_resolved(
        &self,
        slot: u64,
        signature: &str,
        account_keys: &[Pubkey],
        top_instructions: &[CompiledInstruction],
        meta: &solana_transaction_status::TransactionStatusMeta,
    ) {
        let Some(target) = self.program_id else {
            return;
        };

        let program_at = |idx: usize| -> Option<Pubkey> { account_keys.get(idx).copied() };
        let mut involved = false;

        // Top-level (direct) invocations. `ix.data` is the raw instruction
        // payload exactly as the client submitted it (for an Anchor program:
        // the 8-byte discriminator followed by the borsh-encoded args).
        for ix in top_instructions {
            if program_at(ix.program_id_index as usize) == Some(target) {
                involved = true;
                self.emit(&format!(
                    "DIRECT  call  slot={slot} sig={signature} program={target} \
                     data_len={} data={} decoded={}",
                    ix.data.len(),
                    to_hex(&ix.data),
                    decode_instruction(&ix.data)
                ));
            }
        }

        // Inner instructions == cross-program invocations.
        if let Some(inner_groups) = meta.inner_instructions.as_ref() {
            for group in inner_groups {
                for inner in &group.instructions {
                    let ix = &inner.instruction;
                    if program_at(ix.program_id_index as usize) == Some(target) {
                        involved = true;
                        let depth = inner
                            .stack_height
                            .map(|h| h.to_string())
                            .unwrap_or_else(|| "?".to_string());
                        self.emit(&format!(
                            "CPI     call  slot={slot} sig={signature} program={target} \
                             top_ix={} stack_height={depth} data_len={} data={} decoded={}",
                            group.index,
                            ix.data.len(),
                            to_hex(&ix.data),
                            decode_instruction(&ix.data)
                        ));
                    }
                }
            }
        }

        if involved {
            self.report_result(slot, signature, meta);
        }
    }

    /// Emit the execution result that came back for a transaction that touched
    /// the target program: status, compute units, any program return data, and
    /// the program log lines.
    fn report_result(
        &self,
        slot: u64,
        signature: &str,
        meta: &solana_transaction_status::TransactionStatusMeta,
    ) {
        let status = match &meta.status {
            Ok(()) => "Ok".to_string(),
            Err(e) => format!("Err({e:?})"),
        };
        let cu = meta
            .compute_units_consumed
            .map(|c| c.to_string())
            .unwrap_or_else(|| "?".to_string());
        self.emit(&format!(
            "RESULT  slot={slot} sig={signature} status={status} compute_units={cu} fee={}",
            meta.fee
        ));

        if let Some(rd) = meta.return_data.as_ref() {
            self.emit(&format!(
                "RETURN  sig={signature} program={} data_len={} data={} decoded={}",
                rd.program_id,
                rd.data.len(),
                to_hex(&rd.data),
                decode_return(&rd.data)
            ));
        }

        if let Some(logs) = meta.log_messages.as_ref() {
            for line in logs {
                self.emit(&format!("LOG     sig={signature} {line}"));
            }
        }
    }

    /// Inspect an account update; report it when the program owns the account.
    fn inspect_account(&self, slot: u64, pubkey: &[u8], owner: &[u8], lamports: u64, data: &[u8]) {
        let Some(target) = self.program_id else {
            return;
        };
        if owner != target.as_ref() {
            return;
        }

        let pk = Pubkey::try_from(pubkey)
            .map(|p| p.to_string())
            .unwrap_or_else(|_| "<invalid>".to_string());

        let mut key = [0u8; 32];
        let is_new = if pubkey.len() == 32 {
            key.copy_from_slice(pubkey);
            self.seen_pdas.lock().unwrap().insert(key)
        } else {
            false
        };

        let verb = if is_new { "PDA   create" } else { "PDA   update" };
        self.emit(&format!(
            "{verb} slot={slot} pubkey={pk} owner={target} lamports={lamports} \
             data_len={} decoded={}",
            data.len(),
            decode_account(data)
        ));
    }
}

impl GeyserPlugin for GeyserTracker {
    fn name(&self) -> &'static str {
        "geyser-tracker-plugin"
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> PluginResult<()> {
        let raw = std::fs::read_to_string(config_file).map_err(|e| {
            GeyserPluginError::ConfigFileReadError {
                msg: format!("failed to read config {config_file}: {e}"),
            }
        })?;
        let config: PluginConfig = serde_json::from_str(&raw).map_err(|e| {
            GeyserPluginError::ConfigFileReadError {
                msg: format!("failed to parse config json: {e}"),
            }
        })?;

        let program_id = Pubkey::from_str(&config.program_id).map_err(|e| {
            GeyserPluginError::ConfigFileReadError {
                msg: format!("invalid program_id {:?}: {e}", config.program_id),
            }
        })?;
        self.program_id = Some(program_id);

        if let Some(path) = config.log_path.as_ref() {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| GeyserPluginError::ConfigFileReadError {
                    msg: format!("failed to open log_path {path}: {e}"),
                })?;
            *self.log_file.lock().unwrap() = Some(file);
        }

        self.emit(&format!("LOADED  tracking program {program_id}"));
        Ok(())
    }

    fn on_unload(&mut self) {
        self.emit("UNLOAD  plugin stopping");
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        // Startup replays the whole snapshot; we only care about live writes.
        if is_startup {
            return Ok(());
        }
        match account {
            ReplicaAccountInfoVersions::V0_0_1(a) => {
                self.inspect_account(slot, a.pubkey, a.owner, a.lamports, a.data)
            }
            ReplicaAccountInfoVersions::V0_0_2(a) => {
                self.inspect_account(slot, a.pubkey, a.owner, a.lamports, a.data)
            }
            ReplicaAccountInfoVersions::V0_0_3(a) => {
                self.inspect_account(slot, a.pubkey, a.owner, a.lamports, a.data)
            }
        }
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> PluginResult<()> {
        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(tx) => {
                if tx.is_vote {
                    return Ok(());
                }
                let msg = tx.transaction.message();
                let keys: Vec<Pubkey> = msg.account_keys().iter().copied().collect();
                self.inspect_resolved(
                    slot,
                    &tx.signature.to_string(),
                    &keys,
                    msg.instructions(),
                    tx.transaction_status_meta,
                );
            }
            ReplicaTransactionInfoVersions::V0_0_2(tx) => {
                if tx.is_vote {
                    return Ok(());
                }
                let msg = tx.transaction.message();
                let keys: Vec<Pubkey> = msg.account_keys().iter().copied().collect();
                self.inspect_resolved(
                    slot,
                    &tx.signature.to_string(),
                    &keys,
                    msg.instructions(),
                    tx.transaction_status_meta,
                );
            }
            ReplicaTransactionInfoVersions::V0_0_3(tx) => {
                if tx.is_vote {
                    return Ok(());
                }
                // VersionedTransaction is not address-resolved; rebuild the full
                // key list as static keys ++ ALT writable ++ ALT readonly, which
                // is the canonical index order instructions reference.
                let meta = tx.transaction_status_meta;
                let mut keys: Vec<Pubkey> = tx.transaction.message.static_account_keys().to_vec();
                keys.extend(meta.loaded_addresses.writable.iter().copied());
                keys.extend(meta.loaded_addresses.readonly.iter().copied());
                self.inspect_resolved(
                    slot,
                    &tx.signature.to_string(),
                    &keys,
                    tx.transaction.message.instructions(),
                    meta,
                );
            }
        }
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

// Anchor discriminators copied from target/idl/geyser.json. Anchor prefixes
// every instruction with an 8-byte discriminator and every account with one
// too, both followed by borsh-encoded fields.
const IX_WRITE_DATA: [u8; 8] = [211, 152, 195, 131, 83, 179, 248, 77];
const IX_INITIALIZE: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
const ACC_DATA_ACCOUNT: [u8; 8] = [85, 240, 182, 158, 76, 7, 18, 233];

/// Read a borsh string (u32 LE length prefix + utf8 bytes) at `off`, returning
/// the decoded string and the offset just past it.
fn read_borsh_string(buf: &[u8], off: usize) -> Option<(String, usize)> {
    let len = u32::from_le_bytes(buf.get(off..off + 4)?.try_into().ok()?) as usize;
    let start = off + 4;
    let bytes = buf.get(start..start + len)?;
    Some((String::from_utf8_lossy(bytes).into_owned(), start + len))
}

/// Decode a geyser-program instruction payload into a readable form by matching
/// the 8-byte Anchor discriminator.
fn decode_instruction(data: &[u8]) -> String {
    let Some(disc) = data.get(0..8) else {
        return "<no discriminator>".to_string();
    };
    let body = &data[8..];
    match <[u8; 8]>::try_from(disc).unwrap() {
        IX_WRITE_DATA => {
            let value = body
                .get(0..8)
                .and_then(|b| b.try_into().ok())
                .map(u64::from_le_bytes);
            let message = read_borsh_string(body, 8).map(|(s, _)| s);
            match (value, message) {
                (Some(v), Some(m)) => format!("write_data{{value={v}, message={m:?}}}"),
                _ => "write_data{<truncated>}".to_string(),
            }
        }
        IX_INITIALIZE => "initialize{}".to_string(),
        _ => "<unknown instruction>".to_string(),
    }
}

/// Decode a program-owned account's data into a readable form. Currently knows
/// the `DataAccount` layout (authority: pubkey, value: u64, message: string,
/// bump: u8).
fn decode_account(data: &[u8]) -> String {
    let Some(disc) = data.get(0..8) else {
        return "<empty>".to_string();
    };
    let body = &data[8..];
    match <[u8; 8]>::try_from(disc).unwrap() {
        ACC_DATA_ACCOUNT => {
            let authority = body.get(0..32).and_then(|b| Pubkey::try_from(b).ok());
            let value = body
                .get(32..40)
                .and_then(|b| b.try_into().ok())
                .map(u64::from_le_bytes);
            let message = read_borsh_string(body, 40);
            match (authority, value, message) {
                (Some(a), Some(v), Some((m, next))) => {
                    let bump = body.get(next).copied();
                    format!(
                        "DataAccount{{authority={a}, value={v}, message={m:?}, bump={}}}",
                        bump.map(|b| b.to_string()).unwrap_or_else(|| "?".to_string())
                    )
                }
                _ => "DataAccount{<truncated>}".to_string(),
            }
        }
        _ => "<unknown account>".to_string(),
    }
}

/// Decode the geyser program's return data. `write_data` returns
/// `WriteResult { value: u64, bump: u8 }` (borsh => 9 bytes, no discriminator).
fn decode_return(data: &[u8]) -> String {
    if data.len() == 9 {
        let value = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let bump = data[8];
        return format!("WriteResult{{value={value}, bump={bump}}}");
    }
    "<unknown return>".to_string()
}

/// Lowercase hex encoding of a byte slice.
fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Entry point the validator looks up via `dlsym` after loading the library.
///
/// # Safety
/// The validator owns the returned box and frees it through the same trait
/// object pointer on shutdown.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserTracker::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
