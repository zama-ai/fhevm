# Solana rotation-leaf indexer

A standalone service that serves **MMR inclusion proofs** for off-chain
historical / public confidential-balance decrypt. A client/SDK queries it
**before** signing a decrypt request; the KMS only verifies the proof.

It indexes the `zama-host` program's encrypted-value-ACL lineage instructions
via the [Carbon](https://github.com/sevenlabs-hq/carbon) framework, reconstructs
each lineage's ordered MMR leaf list through `zama_solana_acl::lineage`, and
exposes inclusion proofs over HTTP/JSON.

## How it works

- **Listen** — Carbon's `RpcTransactionCrawler` datasource at `finalized`
  (`getSignaturesForAddress` backfill + forward poll). Works against a plain
  `solana-test-validator`; no Geyser required. (Seam in `Cargo.toml` /
  `pipeline/mod.rs` to swap in `carbon-yellowstone-grpc-datasource` later.)
- **Decode** — a hand-written `InstructionDecoder` (`src/decoder.rs`) peels the
  8-byte Anchor discriminator and Borsh-decodes the four EV-ACL instructions
  (`initialize` / `rotate` / `allow_subjects` / `mark_public`). The four
  instructions are **absent from the checked-in IDL**
  (`coprocessor/fhevm-engine/host-listener/idl/zama_host.json`), so carbon-cli
  codegen cannot produce them — see the TODO atop `decoder.rs` for the
  re-codegen step once they are exported.
- **Reconstruct** — `src/lineage/state.rs` maintains per-lineage
  `(current_handle, current_subjects)`. On `rotate` it emits
  `LineageEvent::Rotation { old_handle, subjects_before_rotation }` (the
  post-`allow` snapshot, taken BEFORE the rotate applies) then advances; on
  `mark_public` it emits `MarkedPublic { handle }`. Events are persisted, then
  proofs are built with `zama_solana_acl::lineage::{reconstruct, build_proof}`,
  cross-checked against the live account via `build_verified_proof`.
- **Cursor** — Carbon persists nothing. The last-processed signature/slot lives
  in `indexer_cursor`, advanced in the **same** Postgres transaction as each
  event insert (gap-free resume). The cursor seeds the crawler's `until`.

The lineage key is the on-chain PDA (`accounts[2]` on every EV-ACL
instruction). Only `initialize` carries the API's `value_key`, so the
`value_key -> PDA` mapping is recorded on `initialize`; the value_key-keyed API
resolves through it.

## API

```
POST /build_proof
  req: {"value_key": "<hex 32 bytes>", "leaf_index": <u64>}
  200: {"mmr_proof_bytes": "<hex>", "leaf_count": <u64>, "verified": <bool>}
       mmr_proof_bytes = 1 mode-prefix byte (0x01 historical if the leaf at
       leaf_index came from a Rotation, 0x02 public if from MarkedPublic)
       ‖ Borsh(MmrProof). verified=false when the on-chain cross-check could
       not be performed.
  404: {"label":"lineage_not_found", ...}
  422: {"label":"leaf_index_out_of_range", ...}

GET /lineage/{value_key}/leaf?subject=<hex>&handle=<hex>
  200: {"leaf_index": <u64>, "leaf_count": <u64>}
  404: {"label":"lineage_not_found" | "leaf_not_found", ...}

GET /liveness -> 200 "ok"
GET /healthz  -> 200 "ok" (503 on DB-ping failure)
GET /version  -> 200 {"version": "<semver>", "git_sha": "<str>"}
```

Prometheus metrics are served on a separate port (`APP_METRICS__ENDPOINT`).

## Configuration

`config/default.toml`, overlaid by `APP_<SECTION>__<FIELD>` env vars
(double-underscore nesting):

| Variable | Default | Meaning |
| --- | --- | --- |
| `APP_SOLANA__RPC_URL` | mainnet-beta | JSON-RPC endpoint the crawler polls |
| `APP_SOLANA__PROGRAM_ID` | `6Atbv…Fgtu` (mainnet) | `zama-host` program id |
| `APP_SOLANA__COMMITMENT` | `finalized` | crawl/verify commitment |
| `APP_SOLANA__POLL_INTERVAL` | `5s` | forward-poll interval |
| `APP_SOLANA__BACKFILL_BATCH` | `100` | `getSignaturesForAddress` page size |
| `APP_HTTP__ENDPOINT` | `0.0.0.0:8080` | HTTP API bind addr |
| `APP_METRICS__ENDPOINT` | `0.0.0.0:9090` | Prometheus bind addr |
| `APP_DATABASE__URL` | local indexer DB | Postgres DSN |

## Build & test

```sh
# offline build (uses the committed .sqlx cache; no DB needed)
SQLX_OFFLINE=true cargo build --workspace

# tests need the migrated DB for the HTTP integration test
createdb -h localhost -U postgres indexer
export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/indexer
cargo run -p indexer-migrate
cargo test -p indexer

# regenerate the offline cache after changing any SQL
cargo sqlx prepare --workspace
```

## Local verify loop (integration; not in the unit gate)

1. `createdb -h localhost -U postgres indexer`
   `export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/indexer`
2. `cargo run -p indexer-migrate`   # applies migrations
3. Start the validator with the local `zama_host` program. The local keypair
   pubkey at `solana/target/deploy/zama_host-keypair.json` differs from mainnet;
   use it as `APP_SOLANA__PROGRAM_ID`:
   ```sh
   PID=$(solana address -k ../solana/target/deploy/zama_host-keypair.json)
   solana-test-validator --reset \
     --bpf-program "$PID" ../solana/target/deploy/zama_host.so
   ```
4. Drive a lineage `initialize -> allow -> rotate -> mark_public` against the
   validator (via the existing zama-host anchor tests or a small client script)
   so the indexer observes the four instructions.
5. Run the indexer:
   ```sh
   APP_SOLANA__RPC_URL=http://127.0.0.1:8899 \
   APP_SOLANA__PROGRAM_ID="$PID" \
   APP_SOLANA__COMMITMENT=finalized \
   cargo run -p indexer
   ```
6. Query it:
   ```sh
   curl -s localhost:8080/build_proof \
     -H 'content-type: application/json' \
     -d '{"value_key":"<hex from the initialize tx>","leaf_index":0}'
   curl -s "localhost:8080/lineage/<value_key_hex>/leaf?subject=<hex>&handle=<hex>"
   ```
7. **Cross-check**: `build_verified_proof` fetches the live PDA via
   `getAccountInfo` and asserts the reconstructed `(peaks, leaf_count)` equal the
   on-chain values — a mismatch surfaces as `PeaksDiverged`, proving the
   reconstruction matches the chain. Restart the indexer and confirm it resumes
   from `indexer_cursor.last_signature` without reprocessing.

## Docker

```sh
docker compose -f docker/docker-compose.yaml up --build
```

Brings up Postgres, the one-shot `indexer-migrate` sidecar, then the indexer
(HTTP `:8080`, metrics `:9090`).

## Why a standalone workspace

Carbon 1.0.0 pulls the Solana 3.x client ecosystem; `kms-connector` is pinned to
Solana 2.x. This crate is its **own** `[workspace]` with its **own** `Cargo.lock`
(a sibling of `relayer/` and `kms-connector/`, not a member of any parent
workspace), so the two graphs stay isolated. `zama-solana-acl` is depended on by
path with **no** version skew — it is pure `borsh` + `[u8; 32]` with zero Solana
deps, composing with Carbon's Solana types across a clean byte boundary.
