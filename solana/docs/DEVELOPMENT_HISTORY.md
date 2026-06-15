# How the Solana FHEVM PoC Was Built

> Partially superseded (reconciliation, June 2026). This history predates the reconciliation chapter:
> the "native-v0 KMS path that does not reuse EVM gateway routing" and the "single Ed25519 input
> authority" notes below were reversed/removed — input binding is now on-chain secp256k1 over the
> coprocessor attestation, and decrypt reuses the Gateway V2 / EVM stack. See `DESIGN_DECISIONS.md`
> DD-007, DD-012, DD-020–DD-030 for the current design.

This is the story behind the code: what we set out to do, the order we did it in, the
hard-won lessons, and an honest snapshot of what is finished versus deliberately left for
later. If you just want the *what* and *why* of the design, read
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) and [`EVM_PARITY.md`](./EVM_PARITY.md). This
file is the *how we got here* — the part that usually evaporates once the scratch notes are
deleted.

## What this PoC is

Zama's FHEVM lets smart contracts compute on encrypted values. It already exists on EVM as a
set of Solidity contracts (an ACL, an executor, input/KMS verifiers) plus a confidential token
standard (ERC-7984). The job here was to bring that design to **Solana** — honestly, not by
transliteration. Solana has no `msg.sender`, no contract-storage maps, no transient storage, and
a shallow CPI stack, so the same *ideas* had to be re-expressed in Solana's account model.

Two Anchor programs do the work:

- **`zama-host`** — the protocol layer: ACL records, handle derivation, the FHE "executor"
  instructions, input binding, transient capabilities, public-decrypt release, delegation, and
  ciphertext-material commitments. This is the chain-native source of truth a KMS would verify.
- **`confidential-token`** — an ERC-7984-style app that keeps its token logic local and calls
  `zama-host` (by CPI) for everything FHE-related.

Plus adapters that have to track the host's on-chain shapes: a **coprocessor host-listener** that
decodes host events, and a **KMS connector** that verifies host ACL/material witnesses.

## The story, in two chapters

### Chapter 1 — Build it, then try hard to break it

The first long push built both programs and then spent most of its energy *attacking* them. The
guiding belief: in a system where a missed check silently grants someone the ability to decrypt
another user's balance, the negative tests matter more than the happy path.

That push closed ~176 tracked issues. Almost none of them were "add a feature." The overwhelming
majority were "this account could be malformed / this meta could be extra / this authority isn't
really allowed / this handle's metadata isn't actually checked — prove the program rejects it."
The result is a program surface where the account layout, the witness shapes, and the role rules
are treated as a strict ABI, with a rejection test for each way they can be abused.

Midway through, the runtime test harness itself was migrated to **Mollusk**. That migration is what
grew `host_mollusk` and `token_mollusk` into the suites you see today.

### Chapter 2 — Prove it actually matches the EVM design

The second push was less about new code and more about *evidence*. Chapter 1 had hardened the
programs thoroughly but never finished the one deliverable that ties it all together: a
capability-by-capability map from the EVM side to the Solana side, with a clear-eyed note on what
diverges and why.

That map is [`EVM_PARITY.md`](./EVM_PARITY.md). Producing it meant reading `ERC7984.sol`, the
host contracts, and the gateway contracts against the Solana instructions and confirming each
mapping in code. The only actual code change this chapter needed was a one-line build fix:
`anchor build` was failing a program-id keypair check, resolved with `anchor keys sync` (the IDs
the programs now declare match the deploy keypairs; the coprocessor's vendored IDL `address` was
updated to match). Everything else was verification and documentation.

## The hardening catalog — what those ~176 issues actually bought

Grouped by theme, so the count means something. Each theme is enforced in code *and* guarded by
rejection tests; the design rationale for most of them is in `DESIGN_DECISIONS.md`.

- **ACL authorization core.** Keyed-nonce ACL records (`PDA("acl-record", nonce_key, sequence)`
  with the handle stored *inside*, never used as a seed). `is_allowed(handle, subject)` parity.
  Append-only with no revocation in v0. The load-bearing security rule: **there is no generic
  `bind(handle, subjects)` instruction** — only an authorized producer path can mint a fresh ACL
  record, and `allow` can only *append* subjects when the authority already holds the grant role.
- **Handle birth = authorized producers.** Every way a new handle appears (input verification,
  trivial encrypt, random, FHE op output) creates and binds its canonical ACL record atomically,
  authorized by the signer that produced it.
- **Account-meta / witness exactness is ABI.** Reject trailing metas, wrong lengths, non-canonical
  PDAs, wrong bumps, malformed "unused" slots, duplicate subjects, and executable placeholders.
  The same tuple must mean the same thing to the program, the listener, the KMS verifier, and the
  tests — so all four move together.
- **Role bits, not a single boolean.** `USE / GRANT / PUBLIC_DECRYPT / COMPUTE` are distinct. A
  subject that may compute on a handle is *not* automatically allowed to make it publicly
  decryptable — public-decrypt release is its own role and happens *after* birth, never at it.
- **Transient allowance is real state, not "transient storage."** A one-shot capability account
  that must prove a matching `create_transient_session` earlier *in the same transaction*. Same-slot
  expiry is only defense-in-depth — same slot is not the same transaction.
- **Material commitment is separate from ACL.** "Who may use this handle" (ACL) and "is the
  ciphertext available and bound to a key" (material) are different questions. Public decryptability
  needs both: the public-decrypt flag *and* sealed, committed material. Commitment is one-shot.
- **Delegation is durable witness state.** Grant/revoke/reactivate preserve the row, reject
  same-slot double-updates, and bump a counter. `[0xff; 32]` is reserved as the *wildcard
  app-context* row key and is explicitly rejected as a *delegate identity*.
- **Token fidelity to ERC-7984.** Self-transfer is a no-op (no wasted historical handles). Transfer
  amounts are owner-scoped, while payer is only a rent/fee account; operator/delegated transfer
  parity is intentionally not preserved. Disclosure is label-scoped (a generic "amount" path can't
  be used to disclose a balance or total supply). Minting is modeled as wrapping a real SPL token;
  burn has a matching underlying-redeem leg.
- **Transfer-and-call is split.** A single-instruction callback blows the SBF heap/CPI budget, so
  it became hook → prepare → finalize, with explicit hook causality and one-shot replay markers.
- **Operator-specific FHE type gates.** Add/Sub follow the LHS type, Ge returns Bool, unbounded
  random excludes some types, bounded random excludes others — replicating the EVM executor's
  per-operator type rules rather than a single permissive set.
- **Native-v0 KMS path.** A Solana-native request/response model (Ed25519 over keccak-tagged
  domains, replay keys, threshold response certificates, finality recheck) that deliberately does
  *not* reuse the EVM gateway routing. Fully implemented and unit-tested; not yet wired into a
  running binary (see status below).
- **Config/admin guards.** `HostConfig` rejects zero chain-id and zero authorities; pause blocks
  grants; flag setters are idempotent and emit transition events only on real changes.

## Where things stand today

### Solid

The ACL core, handle derivation and byte-layout, transient capabilities, delegation lifecycle,
material commitment, operand-ACL discipline, the selected ERC7984-inspired owner-authorized token
surface, and the account-meta ABI are implemented, faithful to the useful EVM invariants (often
*stricter*), and backed by an extensive rejection-test suite. The missing ERC7984 operator/delegated
transfer surface is intentional Solana API design, not unfinished parity work.

### PoC scope — deliberately minimal, not Solana-limited

The FHE executor ships exactly the ops the confidential token needs (add, sub, compare, select,
trivial-encrypt, random, input-verify). The broader opcode catalog (mul, div, bitwise, shifts,
min/max, cast, …) is *not* a Solana constraint — it's mechanically extensible by following the
existing op pattern (enum + per-op type-gate table + the handle hash already takes the op + a
coprocessor map arm + tests). It was left out because the token never exercises it.

### Product-open — someone else's job, on purpose

These belong to off-chain services or future programs by design (see the "Open Product Decisions"
in `DESIGN_DECISIONS.md` and §5–6 of `EVM_PARITY.md`): live KMS Core dispatch and the native-v0
transport/publisher wiring; a production Yellowstone/Geyser event provider with finality/replay;
the real threshold input-verifier (today's on-chain input and disclosure checks trust a single
Ed25519 authority — fine for a PoC, not for mainnet); on-chain KMS/gateway config, keygen, and
payment; and durable archival/compaction of ACL/material state. None of these are correctness
bugs in what's implemented — they're the expected gap between a PoC and a deployment.

## If you're picking this up next

A sensible production-hardening order: threshold input + on-chain decrypt-signature verification;
compute-cost metering (the EVM `HCULimit` plane has no on-chain analogue here yet); wire the
native-v0 transport + publisher + a real event-ingestion provider; force-off (ideally compile-out)
the test/mock instruction gates for mainnet; and replace the hand-mirrored, version-pinned adapter
ABI with a compile-time-shared crate so a layout change in `zama-host` can't silently desync the
listener and KMS decoders.

## Map of the docs

- `DESIGN_DECISIONS.md` — the stable *why* (DD-001…DD-013) + open product decisions.
- `EVM_PARITY.md` — capability-by-capability EVM→Solana map + the solid/fragile assessment.
- `TRANSIENT_ALLOW.md` — the transient-allowance design in depth.
- `TESTING.md` — test layout, Mollusk runtime coverage, how to run, and the traps.
- this file — the history, the hardening catalog, and the status snapshot.
