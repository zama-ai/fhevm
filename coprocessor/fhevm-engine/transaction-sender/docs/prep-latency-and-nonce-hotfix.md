# tx-sender: nonce sequencing + static gas limit

> The new static-gas feature explored here has been deferred from v0.13. The
> final binary estimates every attempt and does not expose `--gas-limit`.
> This document is historical rationale, not a rollout recipe.

> Historical rationale for the original two commits. The amended implementation and release
> gates are documented in [the minimal v0.13 plan](minimal-v013-hotfix-plan.md).
> Use that plan for deployment and validation: static gas now applies only to first attempts,
> retries estimate, and nonce lookup has its own bounded phase. The timeout recommendations,
> unconditional zero-estimate expectation, and OOG classifier below are superseded; the
> incident attribution has not been independently established by a controlled load experiment.

Branch `fix/txn-sender-nonce-and-static-gas`, on top of `release/0.13.x`.

Two commits:

1. `fix(txn-sender): correct nonce sequencing and stop aborting sibling sends` — the existing
   hotfix, unmodified.
2. `feat(txn-sender): optional static gas limit and separate gas-estimation timeout` — removes
   `eth_estimateGas` from the send path and unties the two timeouts.

Both address the same incident from opposite ends. **Validation has not been run** — this branch
type-checks (`cargo check -p transaction-sender --tests`) but the reproduction needs a machine
that can carry a real backlog. See §5.

---

## 1. What happened

On testnet, 2026-09-08, two of five coprocessors (`luganodes`, `artifact`) fell to **0.3–2.2 %**
of peer commit throughput and stayed **4 h and 17 h behind** for hours, leaving the 3/5 quorum
with **zero margin** — three healthy senders against a threshold of three.

The measured chain of causation:

```
transient send failures
  → verify-proof eligible set grows past the read batch limit (128)
  → up to 128 concurrent tasks each run eth_estimateGas on ONE shared WS provider
  → "prep" latency (row selected → signing) inflates 0.05 s → 5–6 s p50, 19 s p90
  → add-ciphertext throughput collapses to ≈ inflow
  → the backlog cannot drain; the node never rejoins current consensus rounds
```

Two independent defects fed it:

- **Nonce sequencing.** The provider reset its nonce manager on *any* send error, and the manager
  seeded from `latest` (mined only). A client-side timeout therefore re-seeded from a view that
  excluded the transaction just broadcast and reissued the same nonce → `nonce too low` /
  `already known`. Conversely a send that failed before reaching the mempool left a hole, and
  higher nonces already broadcast were rejected as `nonce too high`, stalling the sequence.
- **Sibling cancellation.** `execute()` ended in `res??`, returning early and dropping the
  `JoinSet`, cancelling in-flight sends whose transactions were already in the mempool while
  their `retry_count` was never incremented — on-chain work with no DB record.

Field evidence for the second: one node was observed committing **64 transactions on-chain while
logging 0 successes**, and its on-chain count consistently exceeded its `addCiphertext txn
succeeded` count.

### Evidence for the prep bottleneck

Prep p50 tracks verify-path volume monotonically across four nodes **and** across a before/after
transition on two of them — the latter being a natural experiment in which nothing else changed:

| node | verify attempts/h | prep p50 | prep p90 |
|---|---|---|---|
| p2p (healthy throughout) | 115 | **0.046 s** | 0.313 s |
| luganodes **before** its verify queue drained | 1,403 | **5.248 s** | 19.10 s |
| luganodes **after** | 116 | **0.082 s** | 14.24 s |
| artifact **before** | 1,710 | **5.951 s** | 19.27 s |
| artifact **after** | 207 | **0.488 s** | 9.84 s |

When each node's verify queue emptied, prep collapsed and its add-path drain jumped from **5 %**
to **124 %** and then **266 %** of real time. User-visible effect while degraded:
attempt→on-chain **p50 24–30 s, p99 55–78 s**, against **<1 s** on healthy peers.

Ruled out by measurement, not assumption:

- **gas underpricing** — all five senders identical (`maxFeePerGas` 0.020 gwei,
  `maxPriorityFeePerGas` 0, median `transactionIndex` 1);
- **per-account inclusion throttling** — the chain packed the affected nodes' consecutive nonces
  into the same or next block in 67/85 and 39/51 cases, i.e. faster than the healthy nodes;
- **on-chain nonce gaps** — zero `gaps > 1` in the included sequences of all five senders.

---

## 2. The fix and why this shape

### 2.1 Static gas limit (`--gas-limit`) — the primary change

`overprovision_gas_limit` **already** skips estimation when gas is pre-set, and `gas: Option<u64>`
was already threaded `main` → `TransactionSender::new` → both operations. The only call site
passed `None`, and no flag existed:

```rust
let new_gas = match txn.gas {
    Some(existing_gas) => Some(existing_gas),          // no RPC call
    None => Some(self.provider.estimate_gas(txn.clone()).await?),
}
.map(overprovision);                                    // multiplier applies to BOTH branches
```

So the change is a flag plus one argument. When set, **`eth_estimateGas` leaves the send path
entirely** and prep becomes local work — the contention that drove the incident cannot form.

**`--gas-limit` is a BASE value; `--gas-limit-overprovision-percent` still applies on top.**
Suggested `250000`, which at the deployed 300 % yields a 750,000 limit against a worst observed
real cost of **235,158** (the threshold-th, consensus-finalising add — the expensive branch).
Over-provisioning is close to free: unused gas is refunded and GW blocks run ~0.00 % full.

**Default is unset**, i.e. byte-for-byte the previous behaviour. This is opt-in per operator.

**Deliberately deferred:** no automatic re-estimation or OOG fallback. A contract-cost increase
above the static base would cause out-of-gas. Mitigated for now by (a) the 3× multiplier,
(b) leaving the flag unset by default, (c) the mandatory OOG check in §5. A periodic off-path
re-estimate is the natural follow-up if the flag becomes the default.

### 2.2 Separate gas-estimation timeout (`--gas-estimation-timeout-secs`)

The hotfix bounds `estimate_gas` with `send_txn_sync_timeout_secs`. Those two want **opposite**
values:

- the **send** timeout is held across the serialised nonce sequence, so it must be **small** — at
  the 16 s some operators were running, and the observed 280–331 send failures/h, the mutex would
  be blocked **124 %–147 %** of every hour and throughput would collapse;
- **estimation** measured **5–6 s p50 / 19 s p90** on a loaded node, so a 4 s cap would fail more
  than half of attempts and nothing would drain.

One parameter cannot satisfy both. Split, default **20 s**. When `--gas-limit` is set this is
inert, since no estimation occurs — which is why §2.1 also *unblocks* the hotfix rather than
merely coexisting with it.

### 2.3 Interaction with the hotfix

The hotfix's `--max-inflight-sends` semaphore exists because "one task per selected row runs
`estimate_gas` concurrently". With `--gas-limit` there is no `estimate_gas`, so the permit
effectively guards only the send — which the nonce sequence already serializes. **Keep it** (it
still bounds task and memory growth) but it is no longer the load-bearing control, and it is not a
substitute for §2.1.

---

## 3. Changes

| file | change |
|---|---|
| `src/bin/transaction_sender.rs` | `--gas-limit: Option<u64>`, `--gas-estimation-timeout-secs` (default 20); pass `conf.gas_limit` instead of `None` |
| `src/config.rs` | `gas_estimation_timeout_secs` (default 20) |
| `src/nonce_managed_provider.rs` | `send_sync_with_overprovision` takes a separate `gas_estimation_timeout` |
| `src/ops/{add_ciphertext,verify_proof}.rs` | pass it through |
| `tests/overprovision_gas_limit_tests.rs` | `static_gas_limit_bypasses_estimation` |
| `charts/coprocessor/values.yaml` | document both flags |

`ConfigSettings` gained a field; every existing test literal uses `..Default::default()`, so all
25 existing tests are unaffected.

---

## 4. Deployment

Recommended per operator, **all three together**:

```yaml
txSender:
  extraArgs:
    - --gas-limit=250000
    - --send-txn-sync-timeout-secs=4        # or 6; NOT 16
    - --verify-proof-resp-batch-limit=10
```

⚠ **`--send-txn-sync-timeout-secs` must not remain at 16 s** with the hotfix — see §2.2. Some
operators are currently running 16 s.
⚠ `--send-txn-sync-timeout-secs` and `--txn-receipt-timeout-secs` are **aliases for the same clap
option**; passing both is a **fatal** startup error (`cannot be used multiple times`). Edit the
existing entry, do not append a second one.
⚠ Do **not** reduce `--verify-proof-resp-batch-limit` below 10 to "protect" the add path. Field
result: draining the verify queue quickly is what *released* the add path on both nodes; throttling
verify would have prolonged the outage.

---

## 5. Validation requirements

Everything below looks healthy at idle. **The reproduction must be run under a real backlog.**

### 5.1 Reproduce the failure on the unpatched build (baseline)

1. Bring up a coprocessor against a gateway you can throttle or slow.
2. Create a verify-proof backlog **larger than `--verify-proof-resp-batch-limit`** (128 by default)
   — e.g. pause the tx-sender while proofs accumulate, then resume.
3. Expected baseline signature, all of which were observed in the field:
   - `Selected rows to process` returning **`rows_count = 128`** continuously;
   - `Processing verified proof` **attempts ÷ distinct `zk_proof_id` ≫ 1** (9–20× observed);
   - prep p50 **> 5 s** (measure per §5.3);
   - add-path on-chain throughput ≈ inflow or below, so the backlog does not drain;
   - `nonce too low` / `nonce too high` present;
   - on-chain transactions with **no corresponding DB transition** (the `res??` defect).

### 5.2 Pass criteria on this branch

Under the **same** induced backlog:

| # | metric | pass |
|---|---|---|
| 1 | prep p50 | **< 0.5 s** while the verify queue is deep (baseline 5–6 s) |
| 2 | add-path on-chain throughput | **within 10 %** of an unaffected peer |
| 3 | backlog drain | commit lag behind the reference sender **strictly decreasing** |
| 4 | `gas estimation timeout` | **0** occurrences |
| 5 | out-of-gas | **0** — classify every failed tx from its receipt; `gasUsed/gasLimit ≥ 0.95` is OOG |
| 6 | realized multiplier | record `gasLimit/gasUsed` on successes; expect ≈3.0 with `--gas-limit=250000` |
| 7 | nonce errors | `nonce too high` **0**; `nonce too low` materially reduced |
| 8 | DB/chain consistency | **every** mined transaction has a DB transition (no orphans) |
| 9 | dropped proofs | `Max retries reached for proof` **0** |

### 5.3 How to measure prep latency (there is no metric yet)

⚠ **The tx-sender exposes no prep or gas-estimation metric.** Every figure in §1 was obtained by
bracketing `Adding ciphertext` → the AWS SDK's `BusinessMetric` warning, which happens to be
emitted inside the `request_sign_digest` span. That marker is accidental and would vanish if
`aws-runtime` is upgraded to silence those warnings.

For this validation, either use that bracket (AWS-KMS signer only), or — better — **add a
`coprocessor_txn_sender_gas_estimation_seconds` histogram and a prep-phase histogram** as part of
the exercise. Do **not** measure from `Processing transaction`: that timestamp precedes gas
estimation and signing, so on a loaded node the delta silently includes queuing. Measuring from
it produced a spurious "timeout = 16 s" reading during the incident.

Also widen `coprocessor_host_txn_latency_seconds` and `coprocessor_zkproof_txn_latency_seconds`:
their default `0.1:60.0:0.1` range put every value that mattered (p50 78 s, max 364 s) into the
single `+Inf` bucket.

### 5.4 Regression guard

Alert on **prep p50 > 1 s**. It is the earliest indicator of this failure mode and led the
throughput collapse by hours. The existing `action = REVIEW` alarm did **not** fire during the
incident (0 in 24 h on all nodes) because the affected path increments a different counter, so it
cannot be relied on here.
