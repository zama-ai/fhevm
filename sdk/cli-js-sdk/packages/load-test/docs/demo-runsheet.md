# Relayer v2 - Yet another load test tool

**Where:** `cd sdk/cli-js-sdk/packages/load-test`

**Prereq:** `.env` has a funded `MNEMONIC` (+ RPC for the target network). Relayer URLs are passed per run below.

**Scenario:** `scenarios/sprint-demo-scenario.json` — open model, 2 rps × 30s, split evenly across input-proof, public-decrypt (k=2), user-decrypt (~21 requests/flow).

The report lands in `runs/<timestamp>-sprint-demo-scenario/` (`report.md` / `report.json`).

---

## A — Testnet (single relayer)

### Beat 1 — What's already prepared

```bash
pnpm load-test --network testnet pool inspect
```

### Beat 2 — Plan it: what would this scenario need? (no generation or on-chain writes)

```bash
pnpm load-test --network testnet suite plan ./suites/sprint-demo.json
```

### Beat 3 — Prepare any deficit (skip if plan is all ✅)

```bash
pnpm load-test --network testnet pool add --flow input-proof --count 25 --types uint64
pnpm load-test --network testnet pool add --flow public-decrypt --count 7
pnpm load-test --network testnet pool add --flow user-decrypt --count 4
# …or one shot: pnpm load-test --network testnet suite run ./suites/sprint-demo.json --prepare-only
```

### Beat 4 — The actual run

```bash
pnpm load-test --network testnet run ./scenarios/sprint-demo-scenario.json --skip-readiness
```

### Beat 5 — The payoff: read the verdict

```bash
pnpm load-test render runs/<the-timestamp-dir>   # or just open report.md
```

- **Verdict** — one line: healthy/degraded, error count, the bottleneck stage, whether backlog drained.
- **Diagnosis** — dominant stage as % of e2e, saturated limiters, and a recommendation that says _what not to bother tuning_.

---

## B — Devnet (paired dispatch: two relayers, A vs B)

Pool prep is relayer-independent — prepare the **devnet** pools once, then run
against both URLs. The report shows paired A/B columns (submit/e2e latency,
per-flow deltas). SDK-driven decrypt flows use the same claimed handles but run
independent target journeys; they are not identical request-byte replays.

```bash
# status + plan (devnet)
pnpm load-test --network devnet pool inspect
pnpm load-test --network devnet suite plan ./suites/sprint-demo.json

# prepare devnet pools (funded devnet MNEMONIC + devnet RPC via --rpc-url or SEPOLIA_RPC_URL)
pnpm load-test --network devnet pool add --flow input-proof --count 25 --types uint64
pnpm load-test --network devnet pool add --flow public-decrypt --count 7
pnpm load-test --network devnet pool add --flow user-decrypt --count 4
# …or one shot: pnpm load-test --network devnet suite run ./suites/sprint-demo.json --prepare-only

# run against BOTH relayers (A = primary, B = candidate)
pnpm load-test --network devnet \
  --relayer-url https://relayer-zws-dev.diplodocus-boa.ts.net \
  --relayer-b   https://relayer-zws-dev-v2.diplodocus-boa.ts.net \
  run ./scenarios/sprint-demo-scenario.json \
  --skip-readiness
```

SDK-native decrypt targets must expose `/v2`; alpha.8 does not provide a public
custom-route seam for these journeys. Custom API prefixes remain available to
raw HTTP flows. If health lives elsewhere, add `--skip-readiness`.

---

## Notes

- Single-use pools (input-proof, public-decrypt) are consumed per run — bump `--count` if you'll rehearse several times. `user-decrypt` handles are reusable.
- `pnpm load-test` works from this package dir and from the workspace root (`sdk/cli-js-sdk`).

- **"Does it verify correctness?"** Yes it gates correctness; a wrong-but-fast relayer fails the run.
- **"How does it know the right answer?"** Handles are recomputed locally; decrypt cleartexts checked vs known plaintexts; sigs recovered against the on-chain KMS signer set with threshold enforced.
- **"How is it wired to CI?"** Nightly `suite run standard` with committed baselines; non-zero exit on threshold breach or regression → fails the build.
- **"Open vs closed vs drain?"** open = fixed arrivals (capacity), closed = fixed clients (SLA/latency), drain = fixed backlog (does it clear correctly).
