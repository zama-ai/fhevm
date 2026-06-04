# Geyser Tracker Plugin

A minimal [Agave Geyser](https://docs.anza.xyz/validator/geyser) plugin that watches a **single target program** on a local `solana-test-validator` and prints, in human‑readable form, everything that program does:

- **Calls into the program** — both direct (top‑level) invocations and CPI (cross‑program) invocations, with the instruction payload decoded.
- **PDAs the program creates / owns** — every account write owned by the program, with the account contents decoded.
- **The result of each transaction** — status, compute units, program return data, and the raw program logs.

This PoC lives at `solana/pocs/geyser/` in the fhevm repo and is **fully self-contained**: it has its own Cargo workspace, `Anchor.toml`, and `rust-toolchain.toml`, and is deliberately **not** a member of the parent `solana/` workspace (the plugin is a native dylib pinned to the validator's ABI, and the demo programs would otherwise pollute the main Anchor build).

It is the companion to two Anchor programs in this PoC:

| Program | ID | Role |
|---|---|---|
| `geyser` (`programs/geyser`) | `H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8` | **The tracked program.** Has one instruction, `write_data(value, message)`, that creates/updates a PDA `[b"data", authority]` and returns `WriteResult { value, bump }` as return data. |
| `caller` (`programs/caller`) | `4RsnoEwKPWbZg4Z6NUGaqP355SvtGWjjUFqEdmEGiFAB` | A thin proxy whose `proxy_write` instruction **CPIs into** `geyser::write_data`. Used to exercise CPI detection. The plugin does **not** track this program. |

---

## How Geyser plugins work

A Geyser plugin is a **native dynamic library** (`.dylib` on macOS, `.so` on Linux) that the validator loads at startup via `--geyser-plugin-config <file.json>`. The validator then streams real‑time events into the plugin through trait callbacks:

```
                         ┌──────────────────────────────────────────┐
   solana-test-validator │  (banking stage / accounts db / ledger)  │
                         └───────────────┬──────────────────────────┘
                                         │ dlopen(libpath) + dlsym("_create_plugin")
                                         ▼
                         ┌──────────────────────────────────────────┐
                         │        GeyserTracker (this plugin)        │
                         │  update_account(..)   ← every account write
                         │  notify_transaction(..) ← every executed tx
                         └───────────────┬──────────────────────────┘
                                         │ filter to target program, decode
                                         ▼
                              geyser-events.log  (+ stderr)
```

The validator owns the plugin instance for its whole lifetime. The plugin only **observes** — it never modifies state.

---

## Layout

```
solana/pocs/geyser/             # the PoC root — its own Anchor + Cargo workspace
├── Anchor.toml                 # localnet config for the two demo programs
├── Cargo.toml                  # workspace: programs/* only (NOT the plugin)
├── programs/
│   ├── geyser/                 # the tracked demo program (write_data → PDA + return data)
│   └── caller/                 # thin proxy that CPIs into geyser (exercises CPI detection)
└── geyser-plugin/              # ← you are here
    ├── Cargo.toml              # standalone crate (own [workspace]) so `anchor build` never compiles it for SBF
    ├── src/lib.rs              # the entire plugin
    ├── config/
    │   └── geyser-config.json  # the --geyser-plugin-config file
    ├── run-validator.sh        # builds programs + plugin, launches the validator with both programs
    ├── invoke-write-data.js    # client: calls geyser::write_data directly        → DIRECT event
    ├── invoke-via-cpi.js       # client: calls caller::proxy_write (CPIs in)       → CPI event
    └── geyser-events.log       # where decoded events are written (created at runtime, gitignored)
```

---

## Core parts of the plugin

All of the following lives in [`src/lib.rs`](src/lib.rs).

### 1. `GeyserTracker` — the plugin state

```rust
pub struct GeyserTracker {
    program_id: Option<Pubkey>,        // the program we track (from config)
    log_file: Mutex<Option<File>>,     // output sink; stderr if no log_path
    seen_pdas: Mutex<HashSet<[u8;32]>>, // remembers PDAs to tell "create" from "update"
}
```

The callback methods take `&self`, so mutable state (`seen_pdas`, the log handle) lives behind a `Mutex`.

### 2. `_create_plugin` — the FFI entry point

```rust
#[no_mangle]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin { ... }
```

The validator looks this symbol up with `dlsym` right after loading the library, calls it to get a boxed trait object, and owns it from then on. This is the one mandatory export.

### 3. `impl GeyserPlugin for GeyserTracker` — the callbacks

| Method | When the validator calls it | What we do |
|---|---|---|
| `on_load(config_file, _)` | once, at startup | read `geyser-config.json`, parse the target `program_id` and optional `log_path`, open the log, emit `LOADED` |
| `update_account(account, slot, is_startup)` | every account write | skip the startup snapshot replay; for live writes, forward to `inspect_account` |
| `notify_transaction(tx, slot)` | every executed transaction | skip votes; resolve the account‑key list; forward to `inspect_resolved` |
| `account_data_notifications_enabled` | once | return `true` (otherwise account updates aren't sent) |
| `transaction_notifications_enabled` | once | return `true` (otherwise transactions aren't sent) |
| `on_unload` | at shutdown | emit `UNLOAD` |

> ⚠️ The two `*_notifications_enabled` methods have **no default** — returning `true` is what makes the validator send account/transaction data at all.

The replica structs are **versioned** (`V0_0_1` / `V0_0_2` / `V0_0_3`); the plugin matches all variants. The newest transaction variant (`V0_0_3`) hands over a `VersionedTransaction` whose addresses aren't resolved, so we rebuild the full key list as `static_keys ++ ALT_writable ++ ALT_readonly` — the canonical index order that `program_id_index` references.

### 4. `inspect_resolved` — call (CPI / DIRECT) detection

Given the resolved account keys, the top‑level instructions, and the transaction meta:

- **DIRECT** — a *top‑level* instruction whose program is the target. The client invoked the program itself.
- **CPI** — an *inner* instruction (`meta.inner_instructions`) whose program is the target. Another program invoked it; an inner instruction is a cross‑program invocation by definition. We also report `stack_height` (call depth) and `top_ix` (which top‑level instruction it descended from).

For each match it logs the raw instruction `data` (hex) **and** the decoded form. If the program was touched at all, it then calls `report_result`.

### 5. `report_result` — the transaction outcome

Emitted once per transaction that touched the target program, read from `TransactionStatusMeta`:

- **RESULT** — `status` (`Ok` / `Err(..)`), `compute_units`, `fee`.
- **RETURN** — present only if the transaction set return data (`meta.return_data`); reports the producing program, raw bytes, and decoded value. (Solana keeps only the *last* `set_return_data` of a transaction.)
- **LOG** — every line from `meta.log_messages` (the program's `msg!` output).

### 6. `inspect_account` — PDA tracking

A program "creates" PDAs that it then owns. So on each live account write we compare `owner` to the target program id; if it matches, the account is one of the program's PDAs. The first time we see a given pubkey we log **PDA create**, afterwards **PDA update** (tracked via `seen_pdas`). The account data is decoded too.

### 7. The decoders — `decode_instruction`, `decode_account`, `decode_return`

Anchor prefixes every instruction and every account with an 8‑byte **discriminator**, followed by borsh‑encoded fields. The decoders match those discriminators (copied from `target/idl/geyser.json`) and parse the bodies:

| Decoder | Input | Knows how to decode |
|---|---|---|
| `decode_instruction` | instruction `data` | `write_data { value: u64, message: String }`, `initialize {}` |
| `decode_account` | program‑owned account data | `DataAccount { authority: pubkey, value: u64, message: String, bump: u8 }` |
| `decode_return` | `meta.return_data` | `WriteResult { value: u64, bump: u8 }` (9 bytes, no discriminator) |

Unknown discriminators fall back to `<unknown ...>`; the raw hex is always kept, so nothing is lost. `read_borsh_string` (u32 LE length prefix + utf8) is the shared helper.

> These discriminators are hardcoded for this POC. If you change the program's instruction/account signatures, update the constants near the bottom of `lib.rs` (or extend the plugin to load `geyser.json` at startup).

---

## Event reference

Every line is `<TYPE>  <fields...>` written to `geyser-events.log` (and stderr). Types:

| Type | Source | Meaning |
|---|---|---|
| `LOADED` | `on_load` | plugin started; shows the tracked program |
| `PDA create` / `PDA update` | `inspect_account` | a program‑owned account was written (first time vs. subsequent) |
| `DIRECT` | `inspect_resolved` | program invoked as a top‑level instruction |
| `CPI` | `inspect_resolved` | program invoked via cross‑program invocation (inner instruction) |
| `RESULT` | `report_result` | transaction status + compute units + fee |
| `RETURN` | `report_result` | program return data (decoded) |
| `LOG` | `report_result` | a raw program log line |
| `UNLOAD` | `on_unload` | plugin stopping |

**Mental model:** `PDA …` is the *effect*, `DIRECT`/`CPI` is the *input* (and how the program was reached), `RESULT`/`RETURN` is the *output*.

---

## Configuration

`config/geyser-config.json`:

```json
{
  "libpath": "/abs/path/.../target/release/libgeyser_tracker_plugin.dylib",
  "program_id": "H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8",
  "log_path": "/abs/path/.../geyser-events.log"
}
```

| Field | Required | Meaning |
|---|---|---|
| `libpath` | yes (by the validator) | absolute path to the built dylib |
| `program_id` | yes (by the plugin) | base58 id of the program to track |
| `log_path` | no | file to append events to; if omitted, events go to the validator's stderr |

Point `program_id` at any program to track it — **no rebuild required**.

> ⚠️ The validator requires `libpath` (and the plugin `log_path`) to be **absolute**, so the committed `geyser-config.json` is machine-specific. After cloning or moving the repo, rewrite both paths to your checkout before running.

---

## Build & run

Prerequisites: `solana-cli` / `solana-test-validator` (Agave **3.1.12**), `anchor` 1.0.x, Rust, Node.js, a funded wallet at `~/.config/solana/id.json`.

The invoke scripts need `@solana/web3.js` — `node_modules` is not committed, so install once from the PoC root:

```bash
cd solana/pocs/geyser && yarn install
```

### One command

```bash
cd solana/pocs/geyser/geyser-plugin
./run-validator.sh
```

This builds both Anchor programs (SBF), builds the plugin (native release dylib), and launches `solana-test-validator` with both programs loaded and the plugin attached.

### Then, in another terminal, exercise it

```bash
# direct call → DIRECT event
node invoke-write-data.js 42 "direct-write"

# call through the caller program → CPI event
node invoke-via-cpi.js 7777 "cpi-write"

# watch decoded events
tail -f geyser-events.log
```

### Example output

```
LOADED  tracking program H4Yc3MugAkJk2FEjLCfCr2J28hgMXzipJaSLq1Sa2SP8

PDA   create slot=44 pubkey=HsTpJg… owner=H4Yc… lamports=1705200 data_len=117 decoded=DataAccount{authority=HsnUWj…, value=42, message="direct-write", bump=254}
DIRECT  call  slot=44 sig=5rCH… program=H4Yc… data_len=32 data=d398…7772697465 decoded=write_data{value=42, message="direct-write"}
RESULT  slot=44 sig=5rCH… status=Ok compute_units=11927 fee=5000
RETURN  sig=5rCH… program=H4Yc… data_len=9 data=2a00000000000000fe decoded=WriteResult{value=42, bump=254}
LOG     sig=5rCH… Program log: Wrote value=42 message="direct-write" into PDA HsTpJg…

PDA   update slot=46 pubkey=HsTpJg… owner=H4Yc… … decoded=DataAccount{… value=7777, message="cpi-write" …}
CPI     call  slot=46 sig=2Kqi… program=H4Yc… top_ix=0 stack_height=2 data_len=29 data=d398…6370692d7772697465 decoded=write_data{value=7777, message="cpi-write"}
RESULT  slot=46 sig=2Kqi… status=Ok compute_units=13829 fee=5000
RETURN  sig=2Kqi… program=H4Yc… data_len=9 data=611e000000000000fe decoded=WriteResult{value=7777, bump=254}
```

Note the second transaction enters through `caller`, yet the program is reported as **CPI** with `stack_height=2`, and `caller`'s id appears only inside the captured `LOG` lines — never as a tracked event.

---

## Version pinning (important)

The plugin and the validator exchange Rust structs across the dynamic‑library boundary, so the types must be **ABI‑identical**. `Cargo.toml` pins the plugin to exactly the validator's versions:

```toml
agave-geyser-plugin-interface = "=3.1.12"   # matches `solana-test-validator --version`
solana-transaction            = "^3.0.2"
solana-transaction-status     = "=3.1.12"
```

If you upgrade the validator, bump these to match, or the plugin may fail to load or read garbage.

---

## Caveats

- **POC scope.** Discriminators and struct layouts are hardcoded for the `geyser` program. Decoding is best‑effort; unknown data is shown as hex with an `<unknown ...>` tag.
- **Single program.** Only one `program_id` is tracked. (Extending to a set is straightforward — swap the `Option<Pubkey>` for a `HashSet<Pubkey>`.)
- **`return_data` is last‑writer‑wins.** Solana stores only the final `set_return_data` of a transaction; if multiple programs set it, only the last is visible in `meta.return_data`.
- **macOS produces `.dylib`; Linux produces `.so`.** Update `libpath` accordingly.
