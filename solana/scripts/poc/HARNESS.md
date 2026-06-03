# Solana PoC — Phase 0 harness

The acceptance harness the autonomous build loop rides on. Goal pinned by
[`fhevm-internal#1494`](https://github.com/zama-ai/fhevm-internal/issues/1494).
Branch: `test/solana-e2e` (off `feature/solana`).

## Guardrails (non-negotiable)

- Scope = the #1494 work items only.
- RFC-021 / RFC-024 and `feature/solana` are **read-only** starting points. On conflict the **#1494 plan wins** — record the divergence, never edit an RFC.
- **No** writes to RFCs, PRs, or issues. **No `git push`.** Local commits only.
- **No hack / no glue**: `run-oracle.sh` is the source of truth; never weaken a test or the oracle to go green. Honest-pass or park.

## The acceptance ladder

| Level | What | How | Runs |
|---|---|---|---|
| **L0** | form + build | `check-form.sh`, rustfmt, `clippy -D warnings`, `anchor build` + IDL sync | `run-oracle.sh` |
| **L1** | regression + unit | full `cargo test --workspace` (Mollusk floor + new tests) | `run-oracle.sh` |
| **L2** | per-item contract | the item's own test (e.g. handle byte-equals coprocessor keccak) | `<item>.oracle` |
| **L3** | integration slice | input→transfer→compute, decrypt, disclose/redeem on the live side-stack + dual-backend rig agree | needs docker (below) |

L0+L1+L2 are pure-Rust and run **now** — they gate the bulk of #1494 with no docker.
L3 needs the live env.

## Run it

```bash
cd solana
bash scripts/poc/check-form.sh         # the form gate alone (no toolchain needed)
bash scripts/poc/run-oracle.sh         # L0+L1 (needs the Solana/Anchor toolchain)
SKIP_BUILD=1 bash scripts/poc/run-oracle.sh   # skip anchor build for pure off-chain Rust
```

`check-form.sh` is the deterministic backpressure: known oversized files are
grandfathered in `form-allow.txt` but may **not grow**; no new file may exceed
500 lines; no shortcut/glue markers (`unwrap`/`panic`/`TODO`/`dbg`/`#[allow]`)
in changed Rust; the tests/oracle themselves may not be edited; no silent deps.
Verified baseline-green and that it bites (catches new oversized files + growth).

## Side-stack (L3, needs the live env)

`fhevm-cli up` brings the **real** backend unchanged (gateway / coprocessor /
KMS / relayer / DB). Alongside it:

```bash
SOLANA_VALIDATOR_IMAGE=<solana-test-validator 2.1.0 image> \
  docker compose -f scripts/poc/docker-compose.solana.yml up -d
```

Then deploy + bootstrap (the **first build work-item**, not yet implemented —
deliberately not faked here): `anchor deploy` the programs, then an init client
sends `initialize_host_config{chain_id: <high-bit>}` → mint → token accounts →
wrap, and the `solana_adapter` host-listener is pointed at the same coprocessor
DB. The loop owns the lifecycle and may `down -v` + re-up to reset (capped).

## Autonomous driver (Track 2)

`driver.workflow.mjs` — a single frugal background loop over the #1494 queue:
implement (worktree) → self-test `run-oracle.sh` → adversarial verify
(default-refuted, multi-lens, no-hack) → local commit; park honest-hard items;
break the circuit only for cheat-required / undecided-architecture /
env-unrecoverable / scope-conflict. Resumable via `resumeFromRunId`.

Launch only on explicit go, with `budget.total` set under the session window:

```
Workflow({ scriptPath: "solana/scripts/poc/driver.workflow.mjs" })   // + budget at launch
```

Token discipline: deterministic gates run in shell via `run-oracle.sh` (≈0
reasoning tokens); only implement + verify spend tokens; cheaper model on the
mechanical lenses; bounded retries; `budget.remaining()` stops the loop clean
with a resume point.
