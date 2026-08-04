# Protocol invariants — Solana fhevm (POC)

Every entry carries a stable number and a tag. Numbers are never reused: an
entry that dies is retired in place. The tags:

- **[HOLDS]** — designed guarantee.
- **[OPERATIONAL]** — maintained by ops/monitoring, not enforced on-chain.
- **[ASSUMPTION]** — an external trust assumption the system depends on.
- **[ANTI]** — an explicit _non_-guarantee (commonly assumed; not promised).
- **[GAP]** — intended but **not currently enforced** (known deficiency).
- **[RISK]** — accepted or unresolved risk the current design does not fully remove.
- **[V2]** — planned, not yet built.
- **[RETIRED]** — withdrawn with the feature it described; number stays.

The register has two parts. **Part I** holds the load-bearing invariants: an
auditor, integrator, or handover reader must be able to rely on every entry,
and a violation of any of them is a security or correctness incident.
**Part II** holds pinned engineering facts and operational posture: true and
test-pinned, but a reader who skips Part II misses no trust property.
Numbers are stable across both parts and never reused.

Scope note: this register covers the **protocol layer** of the Solana feature
branch — `zama-host`, the `zama-fhe` SDK, the host-listener reconstruction path,
and the proof service. The app layer (confidential token, batcher) gets its own
register later. Vocabulary follows GLOSSARY.md (execution, dictionary, persistent,
update, encrypted value ID…).

---

# Part I — Load-bearing invariants

## A. Confidentiality & privacy

1. **[HOLDS]** Plaintext values never appear on-chain: the chain stores handles
   and access state; ciphertexts live only in the coprocessor.
2. **[HOLDS]** A failed confidential transfer is indistinguishable on-chain from
   a successful one (the execution moves an encrypted zero; there is no failure
   branch to observe).
3. **[ANTI]** Participation, timing, touched accounts, instruction shapes, and
   execution structure are all public.
4. **[ANTI]** Subject lists (who is allowed on a value) are public.

## B. Handles & access state

5. **[HOLDS]** Handles enter or replace persistent state **only** as
   `fhe_execute` outputs; no instruction accepts a caller-chosen handle into
   persistent state.
6. **[HOLDS]** Updating a persistent value requires echoing its exact current
   handle and subject list; a stale echo fails the whole execution (no lost-update).
7. **[HOLDS]** Every encrypted value account lives at the canonical PDA of its encrypted
   value ID. The ID is recomputed from the account's seeds rather than stored,
   so an account cannot claim a different identity.
8. **[HOLDS]** Sealed history (the MMR) is append-only: a handle sealed public
   stays provable after any number of later updates.
9. **[HOLDS]** `remove_subject` cannot leave a value with zero subjects.
10. **[HOLDS]** Every subject newly granted membership on an encrypted value
    clears the grant deny-list when it is enabled, on all three membership
    paths: `fhe_execute` persistent create, persistent update (added subjects), and
    `allow_subjects`. Subjects already stored are exempt. Scope is value
    membership only: user-decryption delegation is a separate access path
    with no deny check (a denied key can still be delegated to by a clean
    subject) — that boundary belongs to the delegation/permit rework.
    (Closed the former create-path gap; fhevm-internal#1859 §3-S1.)
11. **[ANTI]** No on-chain roles: any current subject may allow further
    subjects, remove others (not the last), or make the value public.
    Membership is flat by design; apps that need owner/spender-style
    distinctions must enforce them in the app program before granting.

## C. Execution

12. **[HOLDS]** An execution is atomic: preflight validates the whole of it —
    indexes, accounts, types, costs — before any state is touched; a failing
    execution mutates nothing.
13. **[HOLDS]** Every dictionary index is bounds-checked by all four consumers
    (program, SDK, proof service, listener); an unreferenced dictionary entry
    rejects the execution.
15. **[HOLDS]** HCU inverse conformance: every op/type combination validation
    admits has a metering cost row — a validated step can never abort on
    unknown cost. (The converse is fail-closed: some priced combinations are
    rejected by validation.)
16. **[HOLDS]** An execution containing a rand step must bind a persistent output.
    The bound output anchors a compulsorily fresh seed, so a seed is never
    reused across executions.
18. **[HOLDS]** Values from two different builders cannot be mixed into one
    execution: [`FheExecution::build`] hands each builder an invariant `'brand` lifetime
    that its transient values carry, so a foreign value is a compile error
    rather than a runtime check. It replaced a runtime scope tag that was inert
    on SBF (writable statics are forbidden on-chain, so every builder in a
    program shared one scope number). Persistent operands are deliberately
    brand-free — a stored value belongs to no builder. Pinned by the
    `compile_fail` doctest on `FheExecution::build`; the surviving runtime guard is the
    producer-index bounds check, which protects the wire against hand-built
    args (fhevm-internal#1859 §4).

## D. Entry & exit trust

19. **[HOLDS]** A verified input is consumed only with a threshold-valid
    coprocessor attestation that names the calling app and the host chain id.
20. **[HOLDS]** Verified inputs grant nothing persistent: they are usable only
    inside the carrying execution; persistence requires an explicit output with its
    own access list.
21. **[HOLDS]** Public cleartext is accepted on-chain only through
    `verify_public_decrypt`: a KMS threshold certificate **and** an MMR
    inclusion proof that the exact handle was sealed public.
22. **[HOLDS]** Certificate binding chain: signed extra_data → context id →
    canonical KmsContext PDA → signer set. Destroying a context invalidates
    every certificate it issued; rotation alone invalidates none.
23. **[ASSUMPTION]** The coprocessor and KMS committees are honest at their
    thresholds, and their EVM signing keys are not compromised.
24. **[ANTI]** `verify_public_decrypt` provides no act-once/replay protection;
    each consuming app owns its own act-once state machine. The audited rule —
    who needs a marker, what shape it takes, and why the marker itself cannot
    ship as shared code (Anchor derives account ownership from the program that
    declares the type, so the marker must live in the app) — is stated at the
    verifier instruction and demonstrated by `redeem_burned_amount`'s
    per-`(mint, handle)` write-once marker. Pinned from both sides by
    `mollusk_redeem_historical_burned_handle_after_supersession_then_rejects_double_redeem`,
    `mollusk_two_concurrent_burns_each_redeemable_exactly_once`, and
    `mollusk_disclose_secp_is_idempotent_no_replay_marker`.
    (fhevm-internal#1859 §5.)
25. **[ANTI]** The verifier accepts any *live* context. Demanding the *current*
    context is caller policy, exercised through the returned context id; it is
    not enforced by the verifier.
26. **[RISK]** `cleartext` is one 32-byte word ("today's results fit"): an FHE
    type outgrowing it changes the certificate format, the entrypoint
    signature, and the return layout together.
27. **[GAP]** User-decryption delegation records have no consumer yet —
    gateway/KMS payloads do not carry them (stated in the account's own docs).

## E. Reconstruction & off-chain services

28. **[HOLDS]** Listener-re-derived handles are byte-identical to on-chain
    handles, by construction: the listener imports the program's own derivation
    functions and argument types (pinned further by fixtures and the e2e lane).
29. **[HOLDS]** Every transaction is independently interpretable: replay from
    instruction bytes alone reconstructs full history with zero account reads
    (updates echo the previous handle and subjects).
30. **[HOLDS]** The proof service is availability-critical but never an
    authorization anchor: the KMS re-verifies every proof against live
    confirmed on-chain peaks. A bad or compromised proof service can fail a
    decrypt; it can never wrongly authorize one.
31. **[HOLDS]** Coprocessor scheduling is decoupled from authorization: eager
    scheduling can waste compute on a minority fork; it can never release
    plaintext.
32. **[GAP]** No reorg unwind on the listener path; minority-fork work is never
    rolled back (safe only because of #31).
33. **[RISK]** Nothing pins a deployed program build to the listener build; the
    shared-crate identicality guarantee (#28) silently assumes matching
    versions.

## F. Admin, config & custody

35. **[HOLDS]** Only the configured admin can change HostConfig; every change
    stamps `updated_slot` and emits a config event.
36. **[HOLDS]** Pause blocks all production-shaped instructions.
37. **[HOLDS]** HCU enforcement ships disabled (unrestricted defaults) and is
    opt-in per knob; `u64::MAX` is the single "unlimited" sentinel on every
    knob (`0` is rejected on the per-tx limits and means "ban untrusted apps"
    only on the block cap). When finite, the ordering invariant
    `block cap ≥ max per tx ≥ max depth` is enforced at set time.
38. **[ASSUMPTION]** The host admin key is a single trusted key (POC posture;
    no multisig, no timelock).
40. **[HOLDS]** A compute subject cannot self-trust: HCU trust records are
    written only by the admin, live at a PDA derived from the subject they
    trust, and a caller can neither point at another subject's record (address
    check) nor forge one (program-owned PDA, admin-gated write). The block cap
    is enforced by the program in `fhe_execute` before the execution walk; the
    meter account is only a counter.
41. **[ANTI]** HCU block budgets are per compute subject, not per organization:
    a caller controlling N allowed subjects has N per-slot budgets. The
    multiplier is bounded by grant control — each subject must first be allowed
    on real values (unanchored executions are rejected under a finite cap).
51. **[HOLDS]** The optional HCU accounts on `fhe_execute` are four-state, and
    every state that could grant more budget fails closed: present +
    program-owned + well-formed ⇒ used (`hcu_trusted_app_record: trusted ==
    true` bypasses the cap; `hcu_block_meter` charges the subject's per-slot
    budget); absent (`None`) ⇒ the untrusted default (a metered subject that
    omits its meter is rejected, not unmetered); present at the canonical PDA
    but never created (system-owned, empty) ⇒ benign — the subject is simply
    untrusted/unused, and a squatted meter with data is rejected when `charge`
    lazily creates it; present at the wrong PDA, or program-owned but
    malformed ⇒ the execution is rejected outright.

## G. Decrypt authorization (gateway, relayer, KMS)

42. **[HOLDS]** Every KMS party's connector independently re-verifies the
    user's ed25519 signature over the full request — identity, handles,
    allowed domains, validity window, nonce, and the evidence tail. The
    relayer and gateway are transport; neither can alter who asks or for what.
43. **[ANTI]** The user-decrypt nonce is not dedup-enforced on-chain or in the
    connector; replay is bounded only by the request validity window (EVM
    parity).
44. **[ANTI]** An empty `allowedAclDomainKeys` list means permissive mode: the
    request is not domain-scoped. Scoping is opt-in per request.
45. **[HOLDS]** The connector authorizes against the canonical encrypted-value-account
    PDA, program-owned, using the same compiled `zama_solana_acl` code the
    on-chain program runs (decode, MMR verification, all three authorize
    functions).
46. **[RISK]** The connector's ACL reads use confirmed (not finalized)
    commitment, and this component is the authorization gate. The choice is
    deliberate and documented at the site (`solana_v2_fetcher.rs` module doc:
    a grant observed on a supermajority-confirmed fork is sufficient
    authorization even if that fork is exceptionally rolled back). This entry
    blesses it at the protocol level.
49. **[ASSUMPTION]** The coprocessor's EVM-shaped event rows carry a zeroed
    `caller` for every Solana transaction (the 32-byte compute subject does
    not fit the 20-byte field and is discarded). This is safe if and only if
    nothing downstream ever derives authorization, quotas, or identity from
    `caller` on Solana rows — authorization lives in the KMS connector (#42,
    #45). Any feature reading `caller` from these rows must branch on the
    chain type first.

---

# Part II — Pinned bounds & operational posture

A reader who skips this part misses no trust property. Entries here are
test-pinned engineering facts and operations notes; they change with sizing
or tooling decisions, not with the threat model.

14. **[HOLDS]** The maximum execution (32 steps) fits one 1,232-byte packet and the
    default 200k CU budget; both bounds are pinned by tests.
17. **[HOLDS]** `account_count` declared inside the instruction data must equal
    the accounts actually delivered (the execution's bytes are self-describing).
34. **[OPERATIONAL]** Reconstruction fixtures compile only under
    `--features solana-grpc,solana-reconstruct`; coverage exists only where CI
    passes those flags.
47. **[OPERATIONAL]** The proof service runs single-replica and
    unauthenticated in the POC. It is availability-critical but never
    authorization-critical (#30).
48. **[HOLDS]** Settle transactions at production KMS thresholds fit one packet
    only as v0 + one address lookup table; a legacy settle never fits. Both
    directions are pinned by tests.
50. **[OPERATIONAL]** The relayer's ACL preflight covers EVM host chains only.
    An unauthorized Solana request is rejected by the KMS connectors, after
    the gateway fee is paid. Authorization is unaffected (#42, #45); the
    fee/spam surface is accepted for the POC.
52. **[OPERATIONAL]** Every batch gets its own settle address lookup table, and
    the demo runs the full table lifecycle: create + extend at `open_batch`
    (chunked so no extend can exceed the transaction wire limit), deactivate
    immediately after settlement, close once the ~513-slot deactivation
    cooldown has elapsed, refunding rent to the keeper. Deactivate and close
    are best-effort rent hygiene — a failure never fails a settlement, and the
    close crank retries on the next batch preparation. One composition function
    fills the table and compresses against it, so provisioned and consumed
    membership cannot diverge (`solana/demo-dapp/src/vault`).
53. **[ANTI]** `make_handle_public` is not idempotent. Sealing a handle that is
    already sealed appends a second leaf committing to the same
    `(account, handle)` fact: it authorizes nothing the first leaf did not, and
    its cost is bounded — peaks are one per set bit of `leaf_count`, capped at
    `MAX_MMR_PEAKS` (64) — and funded by the caller's own payer. Guarding it
    on-chain would need the account to remember which handle is sealed, which is
    new `EncryptedValue` state in four consumers; a state-free guard could only
    read back the last leaf when `leaf_count` is odd, so the same call would be
    accepted or rejected by parity. Pinned by
    `mollusk_make_handle_public_twice_appends_an_equivalent_leaf`.
54. **[HOLDS]** Lowering an execution never copies the builder's intern tables, so an
    app program pays a few hundred heap bytes per step. What has to fit Anchor's
    32 KB default bump heap is the whole instruction, not just the build: the
    region is never freed, and after the build the CPI helper deep-clones
    `FheExecuteArgs` and borsh-serializes the packet. Measured for steps that
    each write a persistent output: 16 steps request 19,454 bytes and are the
    documented budget, 24 request 32,158 and clear the region by only 610 bytes,
    28 do not fit, and the maximum 32-step execution (41,726) has to be built
    off-chain or by a program with its own allocator. Account resolution and
    Anchor's own account deserialization are on top of those figures, which is
    why the budget is 16 rather than 24. The SDK enforces that budget on-chain
    (`MAX_ON_CHAIN_EXECUTION_STEPS`, lifted by the `raised-heap` feature for a program
    that installs its own allocator) so a program past it gets
    `TooManyStepsForDefaultHeap` instead of an allocator abort with no error of
    its own. Pinned by `solana/crates/zama-fhe/src/heap_budget.rs`, which takes
    its step count from that constant.

## N. Roadmap

39. **[V2]** App-layer register (confidential token, batcher lifecycle, ALT
    construction) — separate section once the protocol register stabilizes.
