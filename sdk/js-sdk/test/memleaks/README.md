# Memory-leak stress harness

Long-running stress loops of FHE encrypt/decrypt/serialize operations against a
real `localstack` Docker stack (not the cleartext mock), sampling process and
WASM memory to detect sustained growth. This is a **standalone script**, not a
vitest suite — there is deliberately no CI-facing pass/fail test here yet (see
[Status](#status) below).

## Why this shape

The WASM runtime (`tfhe`/`tkms`) is a per-process singleton with no re-init
path: `initTfheModule` (`src/core/modules/encrypt/module/init-p.ts`) caches
its init promise per version forever, and there is no `terminateThreadPool`.
So this can't be a create/destroy/assert-baseline-restored test — there is no
teardown to return to. It has to be a long-running, in-process steady-state
loop that watches whether memory trends toward a plateau (expected — caches
filling, thread-pool buffers, JIT warmup) or grows without bound (a leak).

`WebAssembly.Memory` can also only grow, never shrink, by spec. So "WASM
memory didn't shrink" is never itself evidence of a leak — only *sustained,
non-decaying growth proportional to operation count* is. Every scenario here
is judged by that trend, not by an absolute number (see
`support/trendDetector.ts`).

## Running it

Requires a running `localstack` Docker stack (see
`test/scripts/localstack-restart.sh`), or pass `--restart-localstack` to have
it started for you.

```sh
# from sdk/js-sdk
node test/memleaks/run.mjs --restart-localstack --scenario clientChurn --iterations 500
node test/memleaks/run.mjs --scenario clientReuse --iterations 5000
node test/memleaks/run.mjs --scenario all --duration-seconds 1800
node test/memleaks/run.mjs --help
```

`run.mjs` is a thin dispatcher: it re-execs `main.ts` via `tsx` with
`NODE_OPTIONS=--expose-gc` set, so the sampler can force a GC pass before
every measurement (without it, GC scheduling noise dominates the signal).
Output is printed as a running table and also written as JSONL under
`test/memleaks/reports/<scenario>.jsonl` for post-run inspection.

## Scenarios

Each scenario isolates a different leak surface — running them blended into
one loop would hide which one is actually responsible for any growth seen.

- **`clientReuse`** — one long-lived client, looping `encryptValues` +
  `decryptValues` against a stable on-chain handle. Targets the
  per-*operation* path (building/parsing a ciphertext list). This reads clean
  in the code (`buildWithProofPacked` frees its wasm-bindgen objects in a
  `finally`), so it's also the control group: if this one shows unbounded
  growth too, suspect the harness/detector before suspecting the SDK.

- **`clientChurn`** — evicts the global FHE-key cache and creates a fresh
  client every iteration, forcing one public-key + CRS deserialize per
  iteration. Targets `deserializeFheEncryptionPublicKey`/
  `deserializeFheEncryptionCrs` (`src/core/modules/encrypt/module/api-p.ts`),
  whose JS wrapper classes (`TfheCompactPkeCrsImpl`/`TfheCompactPublicKeyImpl`)
  never call `.free()` on their native handle — unlike `buildWithProofPacked`.
  **Two things worth knowing before reading this scenario's output:**
  - Naively creating "N fresh clients" does **not** stress this path.
    `globalFheEncryptionKeyCache` (`src/core/key/FheEncryptionKeyCache-p.ts`)
    is a process-wide singleton keyed by relayer URL with "first write wins"
    semantics — the key is deserialized once per relayer URL for the life of
    the process; every later client just awaits the same cached entry. This
    scenario calls `globalFheEncryptionKeyCache.remove(relayerUrl)` before
    every iteration (exactly what that cache's own doc comment prescribes for
    forcing a re-fetch) to actually force fresh deserialization each time.
  - Missing `.free()` is not automatically a leak here. The vendored tfhe glue
    is a `--weak-refs` wasm-bindgen build, and both `CompactPkeCrs` and
    `TfheCompactPublicKey` register themselves with a `FinalizationRegistry`
    at construction (`src/wasm/tfhe/v1.6.2/tfhe.js`), so the WASM memory *can*
    still be reclaimed once the JS wrapper is unreachable and a GC pass runs
    its pending finalizers — which is exactly what `remove()` + forced
    `global.gc()` between iterations should trigger, if the mechanism works
    end to end. Sustained growth despite that would mean something else keeps
    the old key reachable, or the finalizer path isn't actually running —
    either way a genuine finding, not a foregone conclusion.

- **`roundtrip`** — full encrypt → submit tx → wait for receipt →
  user-decrypt → public-decrypt cycle against the real `FHETest` contract.
  Exercises the ethers provider/signer/contract/tx-lifecycle code paths the
  pure client-side scenarios never touch. Far fewer iterations given per-tx
  latency and localstack throughput.
  **This is the one scenario that showed real growth in a real run**:
  `tfheMemory` grew at an *accelerating* rate (4.4KB/iter → 115.8KB/iter over
  40 iterations) — unlike `rss`/`external`, WASM memory can't produce that
  pattern from sampling-phase noise, so this was worth investigating. A
  source-level pass ruled out a JS-side cache keyed by plaintext value, any
  tfhe-module involvement in `decryptPublicValues` (it's tkms-only), and any
  code path that loops back into the encrypt module after `tx.wait()`. The
  leading (not yet independently verified) explanation is WASM-side allocator
  fragmentation: `roundtrip` is also the only scenario that encrypts a
  *different* plaintext value every iteration (`clearValue = counter % 256`)
  rather than the same fixed value(s) every time — see `valueChurn` below,
  which isolates exactly that variable.

- **`valueChurn`** — one long-lived client, looping `encryptValue` with a
  different uint8 value every iteration (`counter % 256`). No transaction, no
  decrypt — added specifically to isolate the "varying plaintext" variable
  from everything else `roundtrip` does, and runs much faster since it skips
  the tx-wait/decrypt relayer round-trips. If the allocator-fragmentation
  hypothesis is right, growth here should decelerate/plateau once the value
  cycle repeats past iteration 256 (every allocation shape has already been
  seen once); a genuine unbounded leak wouldn't care about that boundary.

- **`permitChurn`** — the tkms-side counterpart to `valueChurn`. One
  long-lived client, looping a *fresh* `generateTransportKeyPair()` +
  `signLegacyDecryptionPermit()` + `signUnifiedDecryptionPermit()` every
  iteration — no transaction, no relayer decrypt call. `roundtrip`
  deliberately signs one permit in its `setup()` and reuses it for every
  iteration (realistic session behavior, and to keep this leak surface out of
  the encrypt/tx/decrypt measurement); this scenario isolates exactly what
  that choice leaves untested: does repeated ML-KEM transport-keypair
  generation and EIP-712 permit signing (both the legacy V1 and unified V2
  permit paths) leak tkms WASM memory on its own? All three operations are
  purely local (no network I/O), so
  this should run much faster per iteration than any relayer-bound scenario.

- **`providerChurn`** — not a WASM/FHE test. Creates an ephemeral
  `ethers.JsonRpcProvider` + signer every iteration, does one read call, and
  discards them. Isolates listener/socket/interval leaks in the ethers layer
  itself, independent of anything FHE-specific — a distinct and common leak
  class that would be invisible inside the other scenarios. Also mirrors how
  `test/fheTest/setup-ethers.ts` itself builds providers (fresh
  `JsonRpcProvider` per config, never `.destroy()`-ed).

## Reading the output

Each scenario prints a running table (iteration, elapsed time, RSS + delta,
tfhe/tkms WASM memory + delta, cumulative GC count) and a trend summary at the
end:

```
--- clientChurn: trend summary ---
  rss            plateauing    first-half 12.0KB/iter -> second-half 0.4KB/iter  (peak 3.1MB above baseline)
  tfheMemory     ⚠ GROWING     first-half 8.0KB/iter -> second-half 7.6KB/iter  (peak 4.2MB above baseline)
```

A metric is classified `growing` when its second-half-of-the-run growth rate
hasn't meaningfully decayed from the first half (or exceeds the absolute
ceiling) — see `support/trendDetector.ts` for the exact rule.

**Only `tfheMemory`/`tkmsMemory` gate the exit code** (plus an absolute
ceiling breach on any metric). `rss`/`heapUsed`/`external`/`arrayBuffers` are
still computed and printed — real runs against localstack showed them
oscillating by tens of MB per iteration (a big transient allocation during
each iteration's work, mostly reclaimed by GC before the next one), and a
two-half linear fit over that kind of sawtooth data can show a spurious slope
purely from which points happen to land near a peak vs. a trough, regardless
of how the noise floor is tuned. `WebAssembly.Memory` only ever grows, so
`tfheMemory`/`tkmsMemory` don't have a trough to be out of phase with — a
sustained climb there is real, which is why they're the metrics that actually
fail the run. The process-level metrics are printed as
`(informational — process-level, does not gate)` for exactly this reason:
worth a glance, not a verdict.

## Status

**v1 is a standalone script only — there is no vitest smoke test or CI wiring
yet, and that's deliberate.** The trend-detection thresholds in
`support/trendDetector.ts` (warmup window, growth-ratio tolerance, absolute
ceiling) are placeholders. Shipping them as a CI-facing gate before they've
been validated against real observed runs would either flake on GC/warmup
noise or give false confidence that nothing was checked. The plan is: run
this manually (or as a scheduled job) first, tune the thresholds against real
data, and only then extract a CI smoke test.

Also out of scope for v1: the `localstack_v11`/`v12`/`v13` version matrix (only
the latest `localstack` chain is targeted), and a viem variant (the leak
surface — the core WASM modules — is client-library-agnostic; ethers is what
`test/multi-wasm` already exercises for round-trips).
