# Blue/Green Upgrade Runbook (devnet & testnet)

## Purpose & scope

This is the step-by-step procedure for infra and QA to run a Blue/Green (B/G) coprocessor upgrade from **v0.14.0 to v0.15** on **devnet** and **testnet**. 

Mental model: the live **Blue** stack keeps serving while a parallel **Green** stack replays a recent window of blocks in dry-run, writes into its own Postgres schema, and publishes a per-block state hash to S3. When every operator publishes the same hash (consensus), an atomic cutover promotes Green and retires Blue. If they do not agree in time, the round rolls back and Blue keeps running - no downtime either way.

End-to-end flow:

1. Apply DB migrations (0.14 -> 0.15) on every operator **first**.
2. Deploy the Green stack (it auto-detects dry-run mode and tails in paused mode).
3. Send the upgrade proposal (one block window per host chain + a gateway start).
4. Watch the dry-run reach consensus across all operators.
5. Automatic cutover on agreement, or automatic rollback on timeout.

The only difference between devnet and testnet is step 3: devnet broadcasts the proposal directly with a deployer key; testnet goes through the DAO.

## Key concepts & terms

| Term | Meaning |
| --- | --- |
| Blue (BCS) | The current live stack (v0.14.0). Serves all traffic throughout the upgrade. |
| Green (GCS) | The incoming stack (v0.15). Runs the dry-run in parallel, writes to the `gcs-<version>` schema. |
| `consensus_version` | Column in the `versioning` table that **gates the upgrade**. A service runs in Green/dry-run mode when its compiled version is strictly newer than `versioning.consensus_version`. Blue baseline = `1`. |
| `stack_version` | The human-readable live version string (e.g. `v0.14.0`), bumped at cutover alongside `consensus_version`. |
| dry-run | Green replays `[start_block, end_block]` with full compute, uploads state hashes to S3, sends no host-chain transactions. |
| window | Per host chain: `[start_block, end_block]` of blocks Green replays and operators must agree on. The gateway track has its own `gw_start_block`. |
| consensus | For a block, every operator uploaded the **same** state hash to S3. One agreed block per track is enough to anchor. |
| cutover | The atomic transaction that promotes Green to live and drops the Green schema. |
| rollback | On a consensus timeout: Green rows go `PAUSED/failed`, the Green schema is dropped and recreated empty, Blue keeps running. |
| per-operator sovereignty | Each operator runs its own controller + detector and decides from its own DB + S3 + chain. No central coordinator. |

The upgrade is driven by one on-chain `proposeCoprocessorUpgrade` call; everything else happens per operator.

## Roles & responsibilities

| Stage | Infra | QA |
| --- | --- | --- |
| Pre-flight | Confirm fleet healthy, all operators on 0.14.0 | Confirm a baseline functional test passes (encrypt/decrypt) |
| DB migrations | Run the migration image on every operator DB | Verify migration success (schema + `consensus_version` present) |
| Deploy Green | Deploy the v0.15 image to every operator | Confirm Green pods are up and in paused/dry-run mode |
| Proposal | Send the proposal (devnet: broadcast; testnet: DAO) | Generate in-window host traffic so the dry-run has real work |
| Dry-run | Watch operator logs/health | Watch `upgrade_state` + S3 convergence across operators |
| Cutover | Confirm version bump + schema drop on every operator | Run post-cutover functional + decryption tests |


## Pre-flight checklist

Do all of this before touching anything. All operators must start from the same clean baseline.

- Fleet is healthy: every operator's host-listener, gw-listener, tfhe-worker, sns-worker, zkproof-worker, transaction-sender are Running.
- Every operator is on **v0.14.0** (`SELECT stack_version, consensus_version FROM versioning;` -> `v0.14.0` / `1` or no column yet).
- No upgrade is already in progress: `SELECT * FROM upgrade_state WHERE status = 'in_progress';` returns no rows on any operator.
- Baseline functional test passes: an encrypt + public/user decrypt round-trips on the current/Blue stack.
- The target Green image tag is known and pullable (e.g. `v0.15.0`).

If any operator fails a check, stop and fix it first. A split baseline (one operator ahead) is the main way a round goes wrong.

## Step 1 - Apply DB migrations (0.14 -> 0.15), FIRST

**Why before Green:** v0.15 adds columns and tables that Green reads from the moment it boots (for example `versioning.consensus_version`, added by `20260819120000_add_consensus_version_to_versioning.sql`, default `1`). The Green upgrade-controller also creates its `gcs-<version>` schema as `LIKE public.*`, so the public tables must already have the new shape. If Green starts before migrations, it crashes or builds a wrong schema.
 

**Order:** migrate all operators before deploying Green to any of them. 0.14 stays fully compatible with the migrated schema (the new columns are additive with defaults), so Blue keeps running normally after migration - this is safe to do ahead of the deploy.

**Verify (per operator):**

```
-- consensus_version column exists and is at the Blue baseline
SELECT stack_version, consensus_version FROM versioning;   -- expect v0.14.0 / 1
-- latest migrations are recorded
SELECT version, description FROM _sqlx_migrations ORDER BY version DESC LIMIT 5;
```

## Step 2 - Deploy the Green stack

Deploy the **v0.15** coprocessor image to every operator, as the Green fleet (upgrade-controller, consensus-detector, host-listener, gw-listener, tfhe-worker, sns-worker, zkproof-worker, transaction-sender). On k8s this is the `-gcs-` deployment set per operator.

**No flag selects dry-run mode - it is derived.** At startup each service compares its compiled `CONSENSUS_PROTOCOL_VERSION` to `versioning.consensus_version`. Strictly newer -> it runs as Green (dry-run); equal -> it would be live. Because migrations left `consensus_version = 1` and the v0.15 binary is newer, Green comes up in dry-run automatically.

**What to expect right after deploy (no proposal yet):**

- Green pods are Running. The Green upgrade-controller has created the `gcs-<version>` schema (empty `LIKE` copies).
- Blue is untouched and still serving. The two stacks share one Postgres; Blue writes `public`, Green writes `gcs-<version>`.

**Verify (per operator):**

```
SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs-%';   -- the Green schema exists
```

and check Green pod logs for the resolved mode (it logs `gcs_mode` resolved from `versioning`). Do not send a proposal until Green is up on **all** operators.

## Step 3 - Send the upgrade proposal

> For the contracts-side detail of this step (task reference, parameters, and the DAO flow), see the host-contracts [Coprocessor upgrade runbook](https://github.com/zama-ai/fhevm/blob/9957e9c44ac34f97e37aaf05c0b72f8f2e5bd125/host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md).

The proposal is one on-chain `ProtocolConfig.proposeCoprocessorUpgrade(proposalId, softwareVersion, chainUpgradeWindows[], gwStartBlock)`. Use the host-contracts tasks - they sample block times per chain, compute one `[startBlock, endBlock]` per host chain, and **pin `gwStartBlock` to the current gateway tip** (do not hand-pick block numbers).

**Devnet (no DAO) - broadcast directly:**

```
npx hardhat task:proposeCoprocessorUpgrade \
  --environment devnet \
  --start-time <ISO8601 UTC, e.g. 2026-10-02T12:00:00Z> \
  --duration 30m \
  --buffer 2m \
  --proposal-id <positive int, unique per attempt> \
  --software-version v0.15.0 \
  --network <host-network>
```

- `--duration` is the window length; `--buffer` is the lead between now and `startBlock` (the task **refuses to broadcast** if `startBlock` is too close to the tip - raise the buffer if it does).
- Pick a **new** `--proposal-id` for every attempt; the contract does not enforce uniqueness and re-using one after a rollback confuses the FSM.
- QA note: a cutover needs real in-window work to reach unanimity, so start generating host traffic (encrypt/transfer) right after broadcast. A quiet window still anchors via the synthetic op, but real traffic is the meaningful test.

**Testnet (DAO path):** build the calldata and route it through the DAO instead of broadcasting:

```
npx hardhat task:buildProposeCoprocessorUpgradeCalldata --environment testnet --start-time <ISO8601 UTC, e.g. 2026-10-02T12:00:00Z> --duration 30m --buffer <DAO-lead> --proposal-id <int> --software-version v0.15.0 --network <host-network>
```

Submit the printed Aragon calldata as the DAO proposal; `gwStartBlock` is still pinned to the gateway tip, and the DAO signing lead keeps `startBlock` in the past by the time it executes.

**Preview-env shortcut:** `ci/preview-env/scripts/deploy/propose-coprocessor-upgrade.sh` runs the devnet task from a host-contracts pod as the ACL owner and then asserts the `upgrade_state` / `versioning` rows. Drive it with env vars (`NAMESPACE`, `NB_COPROCESSOR`, `HOST_HTTP`, `GATEWAY_HTTP`, `HOST_CHAIN_ID`, `GCS_VERSION`, `START_LEAD_SECS`, `WINDOW_DURATION`, `PROPOSAL_ID`); `PROPOSE_DRY_RUN=true` prints the window report without broadcasting.

### Because `gwStartBlock` is pinned to the gateway tip

Two consequences follow from fixing `gwStartBlock` to the tip before the proposal is signed:

- **Draining the `[gw_start_block, *]` input backlog across the cutover is safe.** Input-handle derivation is deterministic and the inserts are idempotent (`input_handles` merges on `handle` with `ON CONFLICT DO NOTHING`), and `gw_listener_last_block` is merged GCS-wins at cutover, so the promoted stack resumes from Green's cursor and keeps scanning forward. Nothing in the window is dropped at the boundary.
- **The Green `gw-listener` must be deployed and started at or before `gw_start_block`.** The `gcs` schema is created structure-only (`LIKE ... INCLUDING ALL`, no data), so its watermark starts empty; on first boot the listener begins at the current tip and does **not** rewind. A listener started after `gw_start_block` therefore skips `[gw_start_block, boot_tip)` and sees only a partial backlog. If operators boot at different times, they ingest different real inputs and the Gateway consensus track diverges. Starting *before* `gw_start_block` is harmless - pre-window proofs (`bn < gw_start_block`) are skipped in GCS mode, so the listener just reaches the window with a continuous watermark.

## Step 4 - Monitor the dry-run

After the proposal activates, Green starts the dry-run: it replays host-chain work and the synthetic Gateway input in a shadow `gcs` schema and uploads one `state_hash` per block to S3. Consensus anchors on the first block where **all operators** uploaded a byte-identical hash, on both tracks (host chains + Gateway inputs). Nothing switches yet - Blue is still live.

**What to watch (per operator):**

- `upgrade_state` row: `state` should walk `UpgradeActivated` -> `DryRunStarted`, `status = in_progress`. The flip to `DryRunStarted` needs every host chain ready, so a lagging chain holds the whole fleet here.
- `SELECT gw_consensus_reached, host_consensus_reached FROM upgrade_state` - both must reach `true`. `gw_consensus_reached` flips only after every operator has uploaded the **same** Gateway `state_hash` to S3, same for host.
- S3: one `state_hash` object per anchored block under each operator's bucket. Empty/no-work blocks publish nothing by design, so gaps are expected.
- Logs: `GCS: building synthetic Gateway input` (one per operator) and `GCS: inserted synthetic Gateway input` confirm the Gateway track has something to anchor on.

## Step 5 - Cutover

Cutover is **automatic** once both tracks reach consensus inside the window - you do not run a command to switch. Each operator's upgrade-controller, seeing `host_consensus_reached` and `gw_consensus_reached` both `true`, runs `execute_cutover` under an exclusive advisory lock: it merges the `gcs` (green) rows into `public` green-wins, drops the `gcs` schema, bumps `versioning.consensus_version` to the new protocol version, and emits the `EVENT_STACK_VERSION_UPGRADED` notify. Every service's stack-version listener then flips `gcs_mode` and the new version is live.

**Operator sovereignty:** each operator cuts over on its own. That is intended - consensus already proved they agree block-for-block. But it means a straggler (an operator that was down during the window and returns after `end_block`) <comment id='c58a2b5d-a0ce'/>can cut over alone and split the fleet; keep all operators healthy through the whole window.

**What to verify after cutover:**

- `SELECT consensus_version FROM versioning` returns the new value  on every operator. In v14->v15 upgrade proposal, the consensus_version must be 2 after cutover.
- `upgrade_state.status = 'completed'`, `state` terminal.
- The `gcs` schema is gone (`\dn` shows no `gcs`).  For debugging purposes, the previous gcs schema has been retained as precutover.
- v0.15.0 Service logs show `gcs_mode` flipped and no `assert_not_retired` / write-guard stop spam.
- v0.14.0 Service logs errors indicating that it's been retired.

If some operators cut over and others do not inside a reasonable margin, treat it as a split: stop the lagging operators, do not let them rejoin on the old version, and go to rollback/recovery. There will be CLI tool that will facilitate the healing process.

## Step 6 - Retire & scale down Blue (BCS)

Cutover flips the version but does **not** stop the old Blue fleet - the BCS pods keep running. Their stack-version listener puts them into no-op mode: every write path is guarded (it can no longer touch the DB), so they do no useful work and just log info-level `... skipping ... on retired stack` lines on each cycle. They still consume CPU, memory, and - importantly - **DB connection-pool slots** against the shared Postgres, so leaving them up wastes resources and eats into `max_connections` for no reason.

Once cutover is verified healthy on **every** operator (checks above), scale the Blue fleet to zero. Blue cannot resume on its own anyway - the schema was merged and `consensus_version` bumped, so a return to the old version requires the rollback/recovery procedure, not just restarting these pods.

- k8s: scale each operator's Blue (non-`-gcs-`) deployment set to 0 replicas (host-listener, gw-listener, tfhe-worker, sns-worker, zkproof-worker, transaction-sender, consensus-detector, upgrade-controller).
- compose/box: stop the Blue coprocessor containers for each operator.

Leave shared infrastructure (Postgres, object-store, gateway/host nodes) running - only the retired BCS coprocessor services are scaled down. The now-live Green (`-gcs-`) fleet keeps serving.

## Rollback & recovery

Two different situations, two different actions. Know which one you are in before touching anything.

**A) Roll back the dry-run (before cutover).** Green is in `UpgradeActivated`/`DryRunStarted` and you found divergence or want to abort. Blue is untouched and stays live, so this is safe. The controller's `rollback_dry_run` retires the green stack and drops the `gcs` schema; 

**B) Recover after faulty cutover (v15 is live but wrong).**

**TBD**

## Troubleshooting

| Symptom | Likely cause | What to do |
| --- | --- | --- |
| Stuck in `UpgradeActivated`, never reaches `DryRunStarted` | One host chain not ready; the flip needs **all** chains ready | Check each chain's listener is caught up; a lagging chain holds the whole fleet |
| `gw_consensus_reached` never true | No Gateway input to anchor on, or operators uploaded different Gateway `state_hash` | Confirm `GCS: inserted synthetic Gateway input` logged on every operator; then compare Gateway `state_hash` in S3 |
| "Missing input to compute transaction" after cutover | The cutover transaction failed to merge | Needs a deeper investigation |
| Proposal refuses to broadcast | `startBlock` too close to the tip for the buffer | Raise `--buffer` / `START_LEAD_SECS` and re-run with a new `--proposal-id` |
 
 
