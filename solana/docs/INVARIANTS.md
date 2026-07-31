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
register later. Vocabulary follows GLOSSARY.md (batch, dictionary, persistent,
update, encrypted value ID…).

---

# Part I — Load-bearing invariants

## A. Confidentiality & privacy

1. **[HOLDS]** Plaintext values never appear on-chain: the chain stores handles
   and access state; ciphertexts live only in the coprocessor.
2. **[HOLDS]** A failed confidential transfer is indistinguishable on-chain from
   a successful one (the batch moves an encrypted zero; there is no failure
   branch to observe).
3. **[ANTI]** Participation, timing, touched accounts, instruction shapes, and
   batch structure are all public.
4. **[ANTI]** Subject lists (who is allowed on a value) are public.

## B. Handles & access state

5. **[HOLDS]** Handles enter or replace persistent state **only** as
   `fhe_execute` outputs; no instruction accepts a caller-chosen handle into
   persistent state.
6. **[HOLDS]** Updating a persistent value requires echoing its exact current
   handle and subject list; a stale echo fails the whole batch (no lost-update).
7. **[HOLDS]** Every value account lives at the canonical PDA of its encrypted
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

## C. Batch execution

12. **[HOLDS]** A batch is atomic: preflight validates the entire batch —
    indexes, accounts, types, costs — before any state is touched; a failing
    batch mutates nothing.
13. **[HOLDS]** Every dictionary index is bounds-checked by all four consumers
    (program, SDK, proof service, listener); an unreferenced dictionary entry
    rejects the batch.
15. **[HOLDS]** HCU inverse conformance: every op/type combination validation
    admits has a metering cost row — a validated step can never abort on
    unknown cost. (The converse is fail-closed: some priced combinations are
    rejected by validation.)
16. **[HOLDS]** A batch containing a rand step must bind a persistent output.
    The bound output anchors a compulsorily fresh seed, so a seed is never
    reused across batches.
18. **[RISK]** On-chain, values from two different builders mixed into one batch
    are caught only by index bounds (the builder scope tag is inert on SBF).
    A compile-time builder brand is planned in the SDK rework.

## D. Entry & exit trust

19. **[HOLDS]** A verified input is consumed only with a threshold-valid
    coprocessor attestation that names the calling app and the host chain id.
20. **[HOLDS]** Verified inputs grant nothing persistent: they are usable only
    inside the carrying batch; persistence requires an explicit output with its
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
    each consuming app owns its own act-once state machine. A shared helper
    is tracked in fhevm-internal#1859; until it exists this is a
    predictable-bug surface for integrators.
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
    on real values (unanchored batches are rejected under a finite cap).

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
45. **[HOLDS]** The connector authorizes against the canonical value-account
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

14. **[HOLDS]** The maximum batch (32 steps) fits one 1,232-byte packet and the
    default 200k CU budget; both bounds are pinned by tests.
17. **[HOLDS]** `account_count` declared inside the instruction data must equal
    the accounts actually delivered (the batch bytes are self-describing).
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

## N. Roadmap

39. **[V2]** App-layer register (confidential token, batcher lifecycle, ALT
    construction) — separate section once the protocol register stabilizes.
