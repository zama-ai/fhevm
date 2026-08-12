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

The register has two parts. **Part I** is what the system promises. An auditor,
an integrator, or whoever picks this up next can rely on every entry in it, and
breaking one of them is a security or correctness bug. **Part II** is how the
system is built and run: sizes, limits, and operational notes. All of it is
true; the sizes and limits are pinned by tests, and the [OPERATIONAL] entries are
notes about how we run the system rather than properties anything enforces.
Nothing in Part II is a promise about safety, so you can skip it without skipping
anything you have to trust. A [HOLDS] entry can sit in Part II when the thing it
holds is a size or a limit — #14, #48 and #54 are all of that kind. An [ANTI] never
can: it is a guarantee explicitly withheld, so a reader who skips it walks away
assuming the opposite. Numbers are stable across both parts and never reused, so an
entry that moves between them keeps its number.

Scope note: this register covers the Solana feature branch: `zama-host`, the
`zama-fhe` SDK, the host-listener reconstruction path, the proof service, and the
reference confidential-token and confidential-batcher applications. Vocabulary
follows GLOSSARY.md (execution, dictionary, persistent, update, encrypted value
ID…).

---

# Part I — What the system guarantees

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
11. **[HOLDS]** Subject-list mutation is gated like persistent create/update:
    `allow_subjects` and `remove_subject` require the signer to equal
    `EncryptedValue.encrypted_value_account_authority` (the app-owned account
    identity). Decrypt subjects are not co-admins. Confidential-token ships
    owner-gated CPI wrappers that `invoke_signed` as the **token-account** PDA
    (`allow_token_account_subjects` / `remove_token_account_subject`). The mint
    authority can rotate total-supply subjects through wrappers that
    `invoke_signed` as the total-supply authority PDA. (Closed the former flat-admin
    ANTI; fhevm-internal#1862 #13.)
    Related token/Host lifecycle guardrails are:
    - **11b [HOLDS].** `make_handle_public` requires the signer to equal
      `EncryptedValue.encrypted_value_account_authority`. A decrypt subject cannot
      make a handle public and the grant deny-list is not consulted because no
      subject is being granted. Confidential-token owner/mint-authority wrappers
      validate the exact state field and sign as the token-account/total-supply PDA.
      (fhevm-internal#1862.)
    - **11c [HOLDS].** Each confidential token account may have exactly one pending
      burn, stored at `["pending-burn", mint, token_account]`. A second burn is
      rejected before FHE execution until `redeem_burned_amount` or
      `cancel_pending_burn` closes the account and returns its rent to the owner.
      Parallel burns for one token account are deliberately deferred; applications
      can aggregate an amount or use separate app-owned token accounts.
    - **11d [HOLDS].** `cancel_pending_burn` requires the pending burned handle to equal
      the burned-amount encrypted value account's current handle. A stale or mismatched
      pending burn cannot restore value.
    - **11e [HOLDS].** `cancel_pending_burn` restores both confidential balance and
      encrypted `total_supply` (mirrors wrap's dual add; undoes burn's dual sub).
      Redeem does not restore encrypted supply — it exits via underlying payout.
      (fhevm-internal#1862 review P1.)
    - **11f [HOLDS].** Host pause (`HostConfig.paused`) gates token cash-out /
      disclose paths that call `assert_host_config_allows_token_response`
      (redeem, disclose). Opening a burn / cancelling a pending burn still requires a live
      FHE path through the host; there is no separate token-level pause. No
      registry / observer / on-chain gov surface in this PoC (out of scope;
      zama-ai/fhevm-internal#1634).

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

## C. Execution

12. **[HOLDS]** An execution is atomic: preflight validates the whole of it —
    indexes, accounts, types, costs — before any state is touched; a failing
    execution mutates nothing.
13. **[HOLDS]** Every dictionary index is bounds-checked by all four consumers
    (program, SDK, proof service, listener); an unreferenced dictionary entry
    rejects the execution.
15. **[HOLDS]** Every op/type combination that validation accepts also has a
    metering cost row, so a step that passed validation can never abort because
    its cost is unknown. It does not work the other way round, deliberately:
    some combinations have a price but are still rejected by validation.
16. **[HOLDS]** An execution containing a rand step must bind a persistent output.
    That output can be claimed only once per execution
    (`ExecutionAccountTable::claim_persistent_output`), which is what stops two
    executions from deriving the same seed.
17. **[HOLDS]** `account_count` declared inside the instruction data must equal
    the accounts actually delivered (the execution's bytes are self-describing).
18. **[HOLDS]** Values from two different builders cannot be mixed into one
    execution: [`FheExecution::build`] hands each builder an invariant `'id` lifetime
    that its transient values carry, so a foreign value is a compile error
    rather than a runtime check. It replaced a runtime scope tag that was inert
    on SBF (writable statics are forbidden on-chain, so every builder in a
    program shared one scope number). Persistent operands are deliberately
    identity-free — a stored value belongs to no builder. Pinned by the
    `compile_fail` doctest on `FheExecution::build`. One runtime check remains, the
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
    who needs act-once state, what shape it takes, and why it cannot ship as
    shared code (Anchor derives account ownership from the program that
    declares the type, so the state must live in the app) — is stated at the
    verifier instruction and demonstrated by `redeem_burned_amount`'s
    open-at-burn/close-at-redeem-or-cancel `PendingBurn` account (one per token
    account; rent returns to the owner on close, unlike a forever write-once marker).
    Pinned from both sides by
    `mollusk_redeem_current_pending_burn_then_rejects_double_settlement`,
    `mollusk_two_sequential_burns_each_redeemable_exactly_once`, and
    `mollusk_disclose_secp_is_idempotent_no_replay_marker`.
    (fhevm-internal#1859 §5; fhevm-internal#1862 Wave 2.)
25. **[ANTI]** The verifier accepts any *live* context. Demanding the *current*
    context is caller policy, exercised through the returned context id; it is
    not enforced by the verifier.
26. **[RISK]** `cleartext` is one 32-byte word ("today's results fit"): an FHE
    type outgrowing it changes the certificate format, the entrypoint
    signature, and the return layout together.
27. **[GAP]** User-decryption delegation records have no consumer yet —
    gateway/KMS payloads do not carry them (stated in the account's own docs).
    The KMS connector's `verify_delegation` is written and unit-tested but
    nothing in the request path calls it. The event that used to be emitted on
    delegation was removed for the same reason (DD-044): the reader being built
    reads the record, not an event.

## E. Reconstruction & off-chain services

28. **[HOLDS]** Handles the listener re-derives are byte-identical to the
    on-chain ones, because the listener imports the program's own derivation
    functions and argument types rather than reimplementing them (fixtures and
    the e2e derivation check this too).
29. **[HOLDS]** Every transaction is independently interpretable: replay from
    instruction bytes alone reconstructs full history with zero account reads
    (updates echo the previous handle and subjects).
30. **[HOLDS]** The proof service can stop a decrypt from happening but can
    never be what allows one, because the KMS re-verifies every proof against
    live confirmed on-chain peaks. A compromised proof service fails decrypts;
    it cannot authorize one.
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
    stamps `updated_slot` and emits a config event. The event always goes out
    through the event CPI, so it lands in the transaction's inner instructions,
    which an RPC provider cannot truncate the way it can truncate logs. A reader
    therefore sees an admin change without replaying instruction data to find
    one (DD-044). Seeing it is all this buys: authorization still comes from
    account state, never from event bytes.
36. **[HOLDS]** Pause blocks all production-shaped instructions.
37. **[HOLDS]** HCU enforcement ships disabled (unrestricted defaults) and is
    opt-in per knob; `u64::MAX` is the single "unlimited" sentinel on every
    knob (`0` is rejected on the per-tx limits and means "ban untrusted apps"
    only on the block cap). When finite, the ordering invariant
    `block cap ≥ max per tx ≥ max depth` is enforced at set time.
38. **[ASSUMPTION]** The host admin key is a single trusted key. This is a POC:
    there is no multisig and no timelock.
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
51. **[HOLDS]** The optional HCU accounts on `fhe_execute` can arrive in four
    states, and every state that could hand out more budget fails closed:
    - **Present, program-owned, well-formed** — used. An
      `hcu_trusted_app_record` with `trusted == true` bypasses the cap;
      `hcu_block_meter` charges the subject's per-slot budget.
    - **Absent (`None`)** — the untrusted default. A subject that is supposed to
      be metered but omits its meter is rejected, not left unmetered.
    - **Present at the canonical PDA but never created** (system-owned, empty) —
      harmless. The subject is simply untrusted or unused. A squatted meter that
      does hold data is rejected when `charge` lazily creates it.
    - **Present at the wrong PDA, or program-owned but malformed** — the
      execution is rejected outright.

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
    records that choice as accepted at the protocol level.
49. **[ASSUMPTION]** The coprocessor's EVM-shaped event rows carry a zeroed
    `caller` for every Solana transaction (the 32-byte compute subject does
    not fit the 20-byte field and is discarded). This is safe if and only if
    nothing downstream ever derives authorization, quotas, or identity from
    `caller` on Solana rows — authorization lives in the KMS connector (#42,
    #45). Any feature reading `caller` from these rows must branch on the
    chain type first.

## H. Reference confidential applications

55. **[HOLDS]** Token disclosure binds the requested kind to the complete
    confidential-token state field: mint domain, canonical encrypted value
    account address, encrypted value account authority, encrypted value label,
    and current or historically sealed handle. A valid certificate for another
    field in the same mint cannot be relabelled in the emitted disclosure event.
56. **[HOLDS]** An underlying mint's owner pins one token program. Wrap and
    redeem require that program to own the underlying mint and both token
    accounts. Classic Token and extension-free Token-2022 are supported.
    Token-2022 mint extensions are rejected unless explicitly allowlisted;
    today none are allowlisted. Token accounts allow only `ImmutableOwner`.
57. **[HOLDS]** Frozen underlying token accounts cannot enter or leave the
    wrapper through wrap or redeem. Token-2022 transfer-fee, transfer-hook,
    non-transferable, and confidential-transfer behavior cannot be inherited
    accidentally because those mint extensions fail closed under #56.
58. **[HOLDS]** Only `ConfidentialMint.authority` can add or remove subjects on
    the encrypted total supply. The wrapper signs the Host CPI as the canonical
    total-supply authority PDA; callers cannot substitute another encrypted
    value account, encrypted value account authority, label, or domain.
59. **[HOLDS]** `ConfidentialMint.authority` is the wrapper's policy authority.
    It is distinct from the authority that can upgrade the Zama Host program.
    Future governance may own the mint authority without acquiring Host upgrade
    power; no governance or authority-rotation mechanism is implied here.
60. **[HOLDS]** A dispatched confidential batch can be cancelled by its join
    mint's `ConfidentialMint.authority` while the burn is pending. This is the wrapper policy
    authority from #59, not the Zama Host upgrade authority. Cancellation restores the batch's confidential join balance and
    encrypted total supply, closes the pending burn, and moves the batch to the
    refund-only `Refunding` state. That state accepts user quits but rejects new
    joins, dispatch, settlement, and repeated cancellation, so a failed KMS or
    vault settlement cannot trap participant funds or reuse a burned amount.

---

# Part II — Sizes, limits, and how it is run

Nothing here is a safety promise. These entries record sizes, limits, and how
the system is operated. They change when we resize something or swap tooling,
not when the threat model changes.

## H. Sizes, limits, and operations

14. **[HOLDS]** The maximum execution (32 steps) fits one 1,232-byte packet and the
    default 200k CU budget; both bounds are pinned by tests.
34. **[OPERATIONAL]** Reconstruction fixtures compile only under
    `--features solana-grpc,solana-reconstruct`; coverage exists only where CI
    passes those flags.
47. **[OPERATIONAL]** The proof service runs single-replica and
    unauthenticated in the POC. An outage stalls decrypts; it can never
    authorize one (#30).
48. **[HOLDS]** Settle transactions at production KMS thresholds fit one packet
    only as v0 + one address lookup table; a legacy settle never fits. Both
    directions are pinned by tests.
50. **[OPERATIONAL]** The relayer's ACL preflight covers EVM host chains only.
    An unauthorized Solana request is rejected by the KMS connectors, after
    the gateway fee is paid. This does not affect authorization (#42, #45); for
    the POC we accept that a rejected request can still cost a fee, and that
    this leaves room for spam.
52. **[OPERATIONAL]** Every batch gets its own settle address lookup table, and
    the demo runs the full table lifecycle: create + extend at `open_batch`
    (chunked so no extend can exceed the transaction wire limit), deactivate
    immediately after settlement, close once the ~513-slot deactivation
    cooldown has elapsed, refunding rent to the keeper. Deactivate and close
    are best-effort rent hygiene — a failure never fails a settlement, and the
    close crank retries on the next batch preparation. One composition function
    fills the table and compresses against it, so provisioned and consumed
    membership cannot diverge (`solana/demo-dapp/src/vault`).
54. **[HOLDS]** Lowering an execution never copies the builder's intern tables,
    and the tables reserve their per-execution bound up front, so growth never
    strands outgrown buffers on the never-freeing bump region. What has to fit
    the entrypoint's fixed 32 KB bump heap is the whole instruction, not just
    the build: after the build the CPI helper stamps the account count into
    `FheExecuteArgs` in place and borsh-serializes the packet once into a
    right-sized buffer. The reservation is a fixed cost of roughly 10 KB paid
    by every build regardless of size — the price of the maximum fitting — so
    a small execution requests more than it strictly uses, with the whole
    region still far away. Measured for steps that each write a persistent
    output:
    - **16 steps** — 14,536 bytes (13,558 building, 978 for the packet).
    - **24 steps** — 17,888 bytes.
    - **32 steps** (the largest execution the host accepts) — 24,280 bytes:
      fits, with ~8.5 KB of slack.

    Account resolution and Anchor's own account deserialization sit on top of
    those figures — roughly the ~8.5 KB the 32-step maximum leaves. There is
    one step ceiling, the host's `MAX_FHE_EXECUTION_STEPS`, on-chain and off:
    the separate on-chain constant was deleted because the maximum is measured
    to fit, byte-counted by `solana/crates/zama-fhe/src/heap_budget.rs` (whose
    regression test fails if the fit is lost) and proven under SBF by the
    at-cap dep-chain specimen, whose Mollusk test extends a 32-link chain built
    entirely on-chain. The measured worst shape is itself unreachable: the host
    stops every-step-persistent executions at 21 steps on the non-extendable
    instruction trace (`fhe_execute_boundary/all_private_creates`), so the fit
    is asserted for a build strictly heavier than anything an app can execute. The rest of the table is measurement, printed by that
    file's `print_measurement_table` (`#[ignore]`d — run it with
    `--ignored --nocapture`), so read the intermediate rows as the last
    measurement rather than as a bound.

## I. Roadmap

39. **[RETIRED]** App-layer invariants were folded into this register rather
    than split into a second source of truth (#55–#60).
