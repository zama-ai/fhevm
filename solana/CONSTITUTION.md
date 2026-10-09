# Solana FHEVM constitution

The rules every agent and reviewer applies to the Solana work: the code `AGENTS.md` scopes, and the Solana changes in zama-ai/kms. A PR that breaks one says so and why, and the reviewer decides.

## Goal and gate

- We harden `feature/solana` until the code can settle, then merge it to main piece by piece and commission external audits.
- "Ready" is a qualitative call: our reviewers judge the code S-tier production-ready. No scoring framework decides it.
- Before that point we prove each flow on the preview env, run as the real environment.
- Deployment follows the quarterly code freeze. If we are not ready at the freeze, public testnet moves; we do not lower the bar. After public testnet exists, every change reaches testnet before mainnet.

## Design

1. **Solana-idiomatic.** Use current Solana DevX and idioms. Do not translate EVM constructs.
2. **Blend in with EVM.** Match EVM on outcomes, security and UX; slot into the same FHEVM/KMS machinery. Read the EVM implementation and its real settings before changing a flow.
3. **Generalize EVM code only when it pays.** Make shared EVM code generic (for example coprocessor ingestion) only where that removes a Solana copy. Otherwise leave EVM alone: a file that exists on main changes on our branch only for Solana glue. Any other change to EVM code is its own named PR. No wording-only edits to main's files.
4. **One definition.** Types, contracts, clients, PDA seeds, decoders, constants, ids, limits and paths each have exactly one source, as `AGENTS.md` ("One source for everything that can change") and DD-072 define. This includes configured values: no hand-typed default for them.
5. **Simplicity and lean code.** Smallest correct change, no speculative flexibility, no pass-through wrappers, no dual paths. Every PR deletes what it makes obsolete.
6. **Public interfaces are contracts.** Stable numeric error codes with the table published in the SDK. Package manifests (dependencies, peers, exports) are checked, and every package is install-tested from its packed tarball.

## Security

7. **Every boundary has an admission rule and a negative test.** On-chain: every CPI target pinned to its program id; every account constrained by owner, key and seeds; wrong-program, wrong-mint and wrong-signer tests per instruction. Off-chain: the first service that sees a bad Solana input rejects it; nothing is coerced.
8. **Solana best practices for programs.** Verified builds whose hash equals the deployed `.so`; upgrade authority behind a multisig and a timelock; toolchains installed with checksums; `security.txt`; no new private keys without the project lead.
9. **Supply chain.** Check a dependency's exact name before adding it, install with `--ignore-scripts`, and never let an assistant add a dependency unreviewed.

## Robustness

10. **Ingestion: Yellowstone first, RPC as failover and backfill.** Losing either path, a provider outage or a missing block slows us down; it never stops or corrupts ingestion.
11. **Everything is bounded and tested at its bound.** Loops, account sizes, transaction bytes (against the real encoder), compute units, HCU, queues and catch-up windows.
12. **Forks, replays and restarts are designed in.** Read at `finalized`. Ingestion, backfill and replay are idempotent. Any value derived from slot or block context states what happens on a fork or a replay.
13. **No unhandled failure mode.** Each failure ends in a retry, an alert or a documented stop. Every alert has an owner, an escalation rule and a runbook that first checks the block is canonical. Every external dependency has a fallback agreed and tested before release.
14. **Fault tests gate releases.** Provider outage, missing block, stale RPC, reorg or skipped slot, listener catch-up, and each ingestion path failing alone.

## Evidence

15. **A test counts only if it is shown to fail.** New guards come with mutation evidence. A CI job that runs zero tests fails. Each test layer states what it cannot prove.
16. **Docs state only what code or a test pins.** Replaced designs move to DESIGN_HISTORY in the same PR.
17. **Claims carry evidence.** A resolved alert is not proof of recovery. Decisions are surfaced to the project lead, never buried in a PR.

## Working rules for the train

18. Generated code and snapshots are minted only by CI and regenerated after every rebase, never resolved by hand. One snapshot-touching PR per program at a time.
19. No backward compatibility before deployment, as `AGENTS.md` ("Pre-production") defines. When unsure whether a change needs compatibility or a fallback, ask.

## Scale

20. **Workers scale horizontally.** Adding replicas is how the system absorbs load, so no change may make a worker-type service depend on running as one replica.
    - Correctness lives in shared state: replicas take work without waiting on one another, and work done twice is idempotent.
    - A role that must run once states how it stays correct when several replicas start together.
    - An in-memory cache only saves work; losing it or splitting it across replicas changes no result.
    - A limit that protects shared capacity holds across replicas; a per-replica limit says so where it is defined.
    - Each worker exposes its backlog (how much work is pending and how old it is), so scaling and alerts follow pending work, not CPU.

    A service that must run as one replica records why next to its deployment. No change adds a new one.
