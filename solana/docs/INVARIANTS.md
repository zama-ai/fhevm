# Protocol invariants — Solana fhevm

Last synced: 2026-09-24.

Every entry carries a stable number and a tag. Numbers are never reused: an
entry that dies is retired in place. The tags:

- **[HOLDS]** — designed guarantee. Each entry names the tests that pin it, or says why it holds by construction.
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
holds is a size or a limit — #14, #48, #54 and #66 are all of that kind. An [ANTI] never
can: it is a guarantee explicitly withheld, so a reader who skips it walks away
assuming the opposite. Numbers are stable across both parts and never reused, so an
entry that moves between them keeps its number.

Scope note: this register covers the Solana feature branch: `zama-host`, the `zama-fhe` SDK, the host-listener
reconstruction path and its leaf record, the KMS connector's Solana pipeline, and the reference confidential-token and
confidential-batcher applications. Vocabulary follows GLOSSARY.md: execution, dictionary, Store, slot, allow, result
grant, transient store, application.

---

# Part I — What the system guarantees

## A. Confidentiality & privacy

**1. [HOLDS]** Plaintext values never appear on-chain: the chain stores handles
and access state; ciphertexts live only in the coprocessor.
Holds by construction: a confidential value reaches the host and token programs only as a handle, and the one
instruction that accepts a cleartext is `verify_public_decrypt`, for a handle already sealed public (#21). Values public
by design, such as trivially encrypted constants and the SPL amounts of wrap and redeem, are outside this entry.

**2. [HOLDS]** A failed confidential transfer is indistinguishable on-chain from
a successful one (the execution moves an encrypted zero; there is no failure
branch to observe).
Holds by construction for the chain state: no instruction receives the amount, so nothing on chain can depend on it.
Pinned for the encrypted zero by `mollusk_overdrawn_confidential_transfer_succeeds_and_moves_an_encrypted_zero`: a
transfer that overdraws the sender succeeds and, once evaluated, leaves both balances unchanged.

**3. [ANTI]** Participation, timing, touched accounts, instruction shapes, and
execution structure are all public.

**4. [ANTI]** Allows (who may decrypt which handle) are public: every allow is
sealed from instruction data, and the leaf record republishes it.

## B. Handles & access state

**5. [HOLDS]** A Store slot changes only through an `fhe_execute` output. No instruction accepts a caller-chosen handle
into a slot; `make_store_handle_public` seals a leaf for the handle a slot already holds.
Holds by construction: a Store output names an execution result (`ExecutionResultRef`), never a handle, and the
handles an instruction does carry are only compared with the slot (#6, #11b). No test writes a chosen handle, because
no argument could carry one.

**6. [HOLDS]** A slot write states what the slot holds now: the exact current handle, or nothing for a first write. A
Store output also states the Store's leaf count as the builder saw it. If either is stale the whole execution fails
(`PreviousStoreMismatch`), so two writers cannot lose an update. The new handle's allows are declared on the write; the
old handle's leaves stay sealed.
Pinned by `mollusk_fhe_execute_rejects_stale_previous_handle`, `mollusk_fhe_execute_rejects_stale_previous_leaf_count`
and `mollusk_make_store_handle_public_rejects_stale_leaf_count`.

**7. [HOLDS]** Every encrypted store lives at `["encrypted-state", program, authority, scope]` and stores those identity fields plus its canonical bump. Creation proves that authority is a PDA of program. Readers rederive the address and validate the stored shape. Slot keys are not address seeds.
Pinned by `mollusk_create_encrypted_store_rejects_wallet_authority`,
`mollusk_create_encrypted_store_rejects_seeds_from_another_program`, `mollusk_make_store_handle_public_rejects_wrong_stored_bump`
and the state test `canonical_validation_rejects_wrong_bump_address_and_duplicate_slots`.

**8. [HOLDS]** Sealed history (the MMR) is append-only: a handle sealed public
stays provable after any number of later updates.
Pinned by `mollusk_historical_proof_round_trip_after_two_updates`, `mollusk_verify_public_decrypt_survives_update_after_seal`
and the MMR test `every_leaf_verifies_and_tampering_fails`.

**9. [RETIRED]** The instruction that removed a viewer went with the stored list
(RFC 035, DD-048): allows are sealed on the write and never removed. A handle with no allows and
no public leaf is undecryptable by everyone, which is its author's choice, not
a stranding — the next write declares the next handle's allows.

**10. [HOLDS]** Every allow the host seals passes the deny list when it is enabled. A Store output, with or without a
slot write, and `make_store_handle_public` both require the application's record `["deny-scope", program, scope]` to be
present at its canonical address and not denied (`DenyRecordMissing`, `ScopeDenied`). With the list disabled no record
may be passed. An `fhe_execute` checks every application whose Store it reads or writes, and its explicit producing Store's application. Naming a consumer Store in a new grant only validates that Store's identity; the consumer's
application is checked by the execution that consumes the grant. A denied application can therefore neither use a grant
nor receive a Store write from another program's execution. The list names applications, not keys (DD-048): a denied key
can still be allowed by a clean application, and user-decryption delegation is a separate access path with no deny
check.
Pinned by `mollusk_denied_application_cannot_write_or_seal_but_its_sibling_scope_can`,
`mollusk_deny_list_requires_exactly_the_applications_record`,
`mollusk_fhe_execute_denies_a_write_under_an_additional_authority_into_a_denied_application`, and the token tests
`mollusk_transfer_from_value_checks_every_application_deny_record` and
`mollusk_burn_from_value_checks_every_application_deny_record`.

**11. [HOLDS]** Only the Store authority writes its slots or appends permissions: it signs Store creation, Store outputs
and `make_store_handle_public`, and it is a PDA of the Store's `program` (#7). A viewer is not a co-admin — an allow
grants decrypt and nothing else, and later new permissions on history-only handles are deferred to #2007 (DD-048).
Confidential-token ships owner-gated wrappers that `invoke_signed` as the **token-account** PDA
(`allow_balance_viewers`, which re-writes the balance onto a handle allowed to the viewers, and
`make_token_account_handle_public`); the mint authority has the same pair for the total supply, signed as the
total-supply authority PDA (`allow_total_supply_viewers`, `make_total_supply_handle_public`). (fhevm-internal#1862 #13;
RFC 035.) Pinned by `only_the_admin_changes_trust_roots_and_only_an_authority_changes_its_store`, which checks over
random instruction sequences that no Store's bytes change unless the authority it records signed; the planted bug
`runtime-tests/planted-bugs/h2-fhe-execute-accepts-an-unsigned-witness.patch` must make it fail. This covers the default
build; the preview-only `admin-sweep` build adds `close_owned_accounts`, which lets the upgrade authority close any
Store (`AUTHORITY.md`). The token wrappers are
pinned by `mollusk_owner_allows_balance_viewers`, `mollusk_non_owner_cannot_allow_balance_viewers`,
`mollusk_mint_authority_allows_total_supply_viewers` and `mollusk_non_mint_authority_cannot_allow_total_supply_viewers`.
Related token/Host lifecycle guardrails are:

- **11b [HOLDS].** `make_store_handle_public` requires the signer to equal `EncryptedStore.authority` and the handle to
  be the current handle in the named slot. A viewer cannot directly publish through this authority-only instruction;
  history-only publication and EVM-style re-sharing are deferred to #2007. The deny list is consulted for the value's
  application, because sealing a public leaf is an allow (#10). Confidential-token owner/mint-authority wrappers
  validate the exact state field and sign as the token-account/total-supply PDA. (fhevm-internal#1862.) Pinned by
  `mollusk_make_handle_public_rejects_wrong_expected_handle` and
  `mollusk_make_handle_public_rejects_signer_that_is_not_the_store_authority`.
- **11c [HOLDS].** Each confidential token account may have exactly one pending burn, stored at
  `["pending-burn", mint, token_account]`. A second burn is rejected before FHE execution until `redeem_burned_amount`
  or `cancel_pending_burn` closes the account and returns its rent to the owner. Parallel burns for one token account
  are deliberately deferred; applications can aggregate an amount or use separate app-owned token accounts. Pinned by
  `mollusk_confidential_burn_is_sequential_until_cancelled`.
- **11d [HOLDS].** `cancel_pending_burn` requires the pending burned handle to equal the Store’s current burned-amount
  slot handle. A stale or mismatched pending burn cannot restore value. Pinned by
  `mollusk_cancel_pending_burn_rejects_stale_current_handle_atomically`.
- **11e [HOLDS].** `cancel_pending_burn` restores both confidential balance and encrypted `total_supply` (mirrors wrap's
  dual add; undoes burn's dual sub). Redeem does not restore encrypted supply — it exits via underlying payout.
  (fhevm-internal#1862 review P1.) Pinned by `mollusk_cancel_pending_burn_restores_balance_and_supply` and
  `mollusk_redeem_current_pending_burn_then_rejects_double_settlement`, which checks that redeem leaves the supply Store
  untouched.
- **11f [HOLDS].** The host's pause flags (#36) stop the token; there is no separate token-level pause. Redeem and
  disclose stop with `public_decrypt`, through the host's `verify_public_decrypt`. Opening a burn and cancelling one
  stop with `execution`, through `fhe_execute`, and opening a burn also with `verified_inputs`, since its amount is a
  verified input. No registry / observer / on-chain gov surface yet (zama-ai/fhevm-internal#1634). Pinned by
  `mollusk_redeem_rejected_when_host_paused`, `mollusk_disclose_secp_rejected_when_host_paused`,
  `mollusk_burn_and_cancel_are_refused_by_a_paused_host` and `mollusk_burn_is_refused_while_verified_inputs_are_paused`.

**68. [ASSUMPTION]** A program never passes a PDA it signs with, a Store authority or a delegator, as a signer to a
program it does not trust. zama-host accepts whatever that PDA signs. A program given the PDA as a signer, and any
program it calls in turn, can write the PDA's Stores, make their handles public and delegate its decryption rights.
zama-host cannot enforce this, so the audits of each application check it (fhevm-internal#2068, fhevm-internal#2070).

**62. [HOLDS]** Compute permission is a signature, never a proof. Reading a slot requires its Store authority's
signature and the exact current handle. Each execution names a canonical producing Store; its authority must sign.
Every produced result, including unstored intermediates, is usable by that Store for the rest of the transaction.
Another Store needs an explicit exact-handle grant and its authority's signature at consumption. Merely returning
bytes, sharing the payer or appearing as an additional signer grants no permission.

Every FHE call requires the same host-owned transient store PDA `["transient", payer]`. A signed top-level open creates it;
the exact final top-level close refunds its recorded payer. Open and every FHE call validate that final close;
a second context, reopening or early/nested closure fails. The payer is a rent role, independent of Store authority.
A failed transaction rolls back all writes. Decrypt permission remains a separate exact-handle MMR leaf.
In the batcher, each JoinRecord controls its participant's contribution Store, scoped to the batch.
Pinned by `mollusk_fhe_execute_rejects_read_of_a_value_whose_authority_did_not_sign`,
`producer_reuses_its_result_across_calls_with_transaction_origin_and_depth`,
`transient_result_rejects_missing_grants_and_wrong_handle_or_consumer`, `active_workspace_cannot_be_reopened`,
`result_journal_capacity_is_shared_across_calls_and_fails_atomically`,
`transient_store_cannot_close_before_the_final_instruction` and
`transient_store_is_created_and_closed_atomically_including_prefunded_addresses`.

**64. [ANTI]** A grant limits who may compute with a handle inside one transaction. It does not limit what that
computation may reveal: the consumer's output can be written to a slot, allowed to any key or made public, and those
leaves outlive the transient store account. A producer that grants a result trusts the consumer's program with the information
in it.

**53. [ANTI]** `make_store_handle_public` is not idempotent. Sealing a handle that is already sealed appends a second
leaf committing to the same `(account, handle)` fact: it authorizes nothing the first leaf did not, and its cost is
bounded — peaks are one per set bit of `leaf_count`, capped at `MAX_MMR_PEAKS` (64) — and funded by the caller's own
payer. Guarding it on-chain would need the account to remember which handle is sealed, which is new `EncryptedStore`
state in four consumers; a state-free guard could only read back the last leaf when `leaf_count` is odd, so the same
call would be accepted or rejected by parity. Pinned by
`mollusk_make_handle_public_twice_appends_an_equivalent_leaf`.

## C. Execution

**12. [HOLDS]** An execution is atomic. Step count, account count, return selection, account table, preflight and deny
checks run before the walk; step semantics are validated during the walk. A failure at any point rolls back the entire
transaction, including earlier slot writes, sealed leaves and the rand nonce.
Pinned by `mollusk_transaction_later_failure_rolls_back_created_public_output`,
`nested_transient_store_close_rolls_back_the_whole_transaction` and `two_slots_share_history_and_stale_slot_writes_roll_back`.
The rollback is the runtime's, so the rand nonce needs no test of its own.

**65. [HOLDS]** Return data carries handles, never permission. An execution names the results to return as an ordered
list of `(step_index, output_index)` pairs, at most 32 (the 1,024-byte return-data limit), and the host copies exactly
those handles in that order, repeats included. An empty list still sets empty return data after the event CPIs, so an
event CPI's return data cannot reach the caller. An out-of-range step or a nonzero output index is rejected before the
walk (`InvalidReturnSelection`); current operators have one output. The consumer reads a returned handle by position and
still needs a grant or its own authority to compute with it (#62). Pinned by
`mollusk_fhe_execute_returns_selected_handles_in_order_with_repeats`,
`mollusk_fhe_execute_rejects_invalid_return_selection_before_execution`, and the SDK test
`selected_result_stays_paired_with_its_execution_and_checks_return_data`.

**67. [HOLDS]** Operand origin is derived by the host, never declared by the caller. Before an operand-bearing result
is derived, each encrypted operand gets boundary bit 1 only if its handle was not produced earlier in this transaction.
Scalars get bit 0. The big-endian 256-bit mask enters the handle preimage; input position 0 uses the least-significant
bit. The listener reconstructs the same ordered transaction membership. An earlier transaction in the same block is
still a boundary; `EarlierStep`, slot reload and transient store grant witnesses cannot choose a different origin.
Pinned by `producer_reuses_its_result_across_calls_with_transaction_origin_and_depth`. The mask enters the handle
preimage, so the listener's side is pinned by its re-derivation check (#28) passing in
`fhe_execute_walk_chains_transient_handles`.

**13. [HOLDS]** Every dictionary index is bounds-checked by all three consumers
(program, SDK, listener); an unreferenced dictionary entry rejects the
execution.
Pinned by `mollusk_fhe_execute_rejects_a_dictionary_index_past_the_dictionary`,
`mollusk_fhe_execute_rejects_an_unreferenced_dictionary_entry`, the SDK tests
`finish_rejects_dictionary_index_past_dictionary_end` and `finish_rejects_dictionary_entry_no_step_references`, and
the listener test `rejects_store_output_dictionary_overflow`.

**15. [HOLDS]** Every op/type combination that validation accepts also has a
metering cost row, so a step that passed validation can never abort because
its cost is unknown. It does not work the other way round, deliberately:
some combinations have a price but are still rejected by validation.
Pinned by the eight `*_hcu_covers_every_validated_*` tests in
`programs/zama-host/src/instructions/fhe_execute/hcu/tests.rs`, one per operator family.

**16. [HOLDS]** An execution containing a rand step must pass the host's
`RandNonce` singleton (`["rand-nonce"]`; `FheExecuteRandNonceMissing`
otherwise) and advances it; the nonce is bound into every rand seed, so two
executions can never derive the same seed, whatever they persist (DD-043).
The nonce is host state, never caller-supplied, so a caller cannot steer it.
Pinned by `mollusk_fhe_execute_rand_without_nonce_account_is_rejected`,
`mollusk_fhe_execute_nonce_account_without_rand_is_rejected`,
`mollusk_fhe_execute_rand_consumes_the_nonce_and_never_repeats_a_seed`,
`mollusk_fhe_execute_rand_rejects_non_canonical_nonce_account` and `rand_seed_is_distinct_across_every_uniqueness_axis`.

**17. [HOLDS]** `account_count` declared inside the instruction data must equal the number of remaining accounts
actually delivered.
Pinned by `mollusk_fhe_execute_extra_remaining_account_still_rejected_with_block_cap` and
`mollusk_fhe_execute_missing_remaining_account_rejected`.

**18. [HOLDS]** A transient result from one SDK builder cannot be used in another. `FheExecution::build` gives each
builder a distinct `'id` lifetime, which its transient results carry; mixing them is a compile error. Stored slot
operands are independent of a builder. The host also checks producer-index bounds because callers can construct
instruction arguments without the SDK. Pinned by the `compile_fail` doctest on `FheExecution::build`.
(fhevm-internal#1859 §4.)

**61. [ANTI]** `FheExecution::build` does not guarantee that the host's CPI fits the host's heap or compute budget. The
builder's typed limits (#54) cover the app's own heap. The host has a separate 32 KiB heap, and what it allocates
depends on the live Store size, the number of MMR peaks and the permissions sealed per output, none of which the builder
can see. The gap is measurable: `the_builder_admits_mature_updates_the_host_heap_cannot_run` builds 16 updates, each
to its own Store with 8 MMR peaks and the same eight viewers, while the runtime sweep
`fhe_execute_boundary/mature_updates_peaks_8` runs 15 and exhausts the host heap at 16. The builder admits 22 such
updates at any peak count; the host runs 7 at 32 peaks. These are shape measurements, not an output cap. No
host-side admission model exists; an app validates its shapes against the sweeps and budgets the whole transaction. Why
no allocator was shipped is DD-046 (fhevm-internal#1872).

## D. Entry & exit trust

**19. [HOLDS]** A verified input is consumed only with a threshold-valid
coprocessor attestation that names the calling program and the host chain id.
Pinned by `verifies_full_coprocessor_input_flow`, `mollusk_confidential_transfer_rejects_attestation_user_mismatch` and
`mollusk_confidential_transfer_rejects_attestation_contract_mismatch`.

**20. [HOLDS]** Verified inputs grant nothing persistent: they are usable only
inside the carrying execution; persistence requires an explicit output with its
own allows.
Holds by construction: a verified input is an operand of one execution, and an execution persists only its declared
Store outputs (#5). No test tries to persist one otherwise, because no instruction could.

**21. [HOLDS]** Public cleartext is accepted on-chain only through
`verify_public_decrypt`: a KMS threshold certificate **and** an MMR
inclusion proof that the exact handle was sealed public.
Pinned by `mollusk_verify_public_decrypt_returns_handle_and_cleartext`,
`mollusk_verify_public_decrypt_rejects_handle_proof_mismatch`, `mollusk_verify_public_decrypt_rejects_historical_only_leaf`
and `mollusk_verify_public_decrypt_rejects_sub_threshold_signatures`.

**22. [HOLDS]** Certificate binding chain: signed `extra_data` → context id → canonical KmsContext PDA → signer set.
Empty or version-0 `extra_data` selects the current context; version 1 is exactly 33 bytes and carries the 32-byte id.
Solana version 4 is exactly 65 bytes: version, context id, then the Store address used to route the decrypt request.
Version 3 is rejected. The verifier authenticates the context, handle and cleartext through the certificate and
independently verifies the exact handle's public leaf against the supplied Store's current peaks. It does not require
that Store to equal the routing address in `extra_data`. Destroying a context invalidates its certificates; rotation
alone invalidates none.
Pinned by `extract_kms_context_id_mirrors_evm_extractcontextid`,
`mollusk_verify_public_decrypt_accepts_v4_extra_data_routed_through_another_store`,
`mollusk_verify_public_decrypt_rejects_non_canonical_kms_context`, `mollusk_verify_public_decrypt_rejects_context_account_mismatch`,
`mollusk_redeem_rejects_destroyed_kms_context` and `mollusk_redeem_accepts_live_rotated_out_kms_context`.

**23. [ASSUMPTION]** The coprocessor and KMS committees are honest at their
thresholds, and their EVM signing keys are not compromised.

**24. [ANTI]** `verify_public_decrypt` does not prevent replay. Each application must track whether it has already
acted on a verified result. The token program creates one `PendingBurn` per token account at burn time and closes it
when redeeming or cancelling, returning its rent to the owner. This application-owned account prevents a second
settlement of the same burn; verification alone does not. Pinned by
`mollusk_redeem_current_pending_burn_then_rejects_double_settlement`,
`mollusk_two_sequential_burns_each_redeemable_exactly_once`, and `mollusk_disclose_secp_is_idempotent_no_replay_marker`.
(fhevm-internal#1859 §5; fhevm-internal#1862 Wave 2.)

**25. [ANTI]** The verifier accepts any _live_ context. Demanding the _current_ context is caller policy, exercised
through the returned context id; it is not enforced by the verifier.

**26. [RISK]** `cleartext` is one 32-byte word ("today's results fit"): an FHE
type outgrowing it changes the certificate format, the entrypoint
signature, and the return layout together.

**27. [HOLDS]** A delegated user-decryption entry names the delegator as its allowed key. The KMS connector reads the
delegation record for the encrypted store's authority and the delegator's wildcard row in the deciding snapshot. Either
row authorizes the delegate if it is live at that slot: not revoked, not expired, and not written after the observation.
A dead row cannot veto a live one. The connector then requires the delegator's allow leaf
(`kms-worker/src/core/solana/delegation.rs`). The relayer refuses dead rows advisorily before the gateway fee (#50).
Delegation emits no event; readers read the record (DD-044). A wallet delegator must call
`delegate_for_user_decryption` as a top-level instruction (`WalletDelegationThroughCpi`). A wallet's signature reaches
every CPI of the transaction it signed, so without that rule any program the user calls could delegate the user's
decryption rights. A PDA delegator may delegate through CPI: only its own program can sign for it, and #68 covers
where that program may pass it. Pinned by `a_wallet_grant_forwarded_through_another_program_is_rejected`
and `a_vault_pda_grants_a_delegation_via_cpi` (fhevm-internal#2084).

## E. Reconstruction & off-chain services

**28. [HOLDS]** The handles the listener stores are the ones the host
emitted in each execution's `FheExecutedEvent`, so its computation rows, leaves
and allowed handles match the chain even if its own derivation drifts. It
re-derives every handle as a check, with the program's own derivation functions
and argument types and the followed program id (`--program-id`) rather than the
crate's compiled `declare_id!`. A step that does not re-derive is held back as a
terminal error, which ends its dependents too, and raises an alarm; the rest of
the block is ingested (DD-056). The check detects a listener bug, not a lying
provider, which can forge the event and the transaction consistently.
Pinned by `reports_a_wrong_emitted_handle_without_substituting_it` and `rejects_an_event_that_describes_other_steps`.

**29. [HOLDS]** Every transaction is independently interpretable: its
instructions and inner instructions, including each execution's event,
reconstruct its history with zero account reads and no sysvar state (updates
echo the previous handle and declare the new handle's allows). A block from
`getBlock` prepares into the same input as one from the stream.
Pinned by the reconstruction walks such as `fhe_execute_walk_chains_transient_handles`, which take only transaction
data, and by `rebuilds_a_slot_from_get_block_alone` and `shared_transaction_decoding_contract`.

**30. [HOLDS]** The leaf record can stop a decrypt from happening but can
never be what allows one: the KMS connector verifies every proof against
the peaks it read on chain itself, fans out to every configured
coprocessor and merges (a proof beats no proof, more history beats less),
and rejects a client-supplied proof outright. A compromised or lagging
record fails or delays decrypts; it cannot authorize one (DD-048).
Pinned by `matches_on_chain_append_and_authorizes`, `one_serving_coprocessor_carries_a_request_the_others_cannot`,
`a_record_behind_the_chain_is_retried_not_refused`. A client cannot supply a proof: the request wire
(`SolanaUserDecryptRequestWire`) has no proof field, and `the_decoder_is_strict` rejects trailing bytes.

**31. [HOLDS]** Coprocessor scheduling is decoupled from authorization: eager
scheduling can waste compute on a minority fork; it can never release
plaintext.
Pinned by `compute_is_eager_regardless_of_same_tx_allow_signal` and
`unrelated_allow_handle_does_not_affect_eager_compute_result`.

**32. [GAP]** No reorg unwind on the listener path; minority-fork work is never
rolled back (safe only because of #31). The operator repair of DD-056 does not
unwind a fork either: it replays the same slots, and a replayed write must
reproduce the leaves recorded for it or the listener stops.

**33. [RISK]** Nothing pins a deployed program build to the listener build.
#28 now takes the followed program id as an input, so a listener compiled
for one `declare_id!` can still derive another deployment's handles.
Instruction layout and decoder types still assume matching crate revisions.
#28's check catches a decoder drift in the steps, since every decoded step
field feeds its handle. Two drifts stay silent: one in the effects (allows,
Store slots, make public), which shape leaves rather than handles, and one in
the adapter that maps a checked step to the tfhe-worker's operation, which runs
after the check (its own unit tests pin that mapping).

## F. Admin, config & custody

**35. [HOLDS]** Only the configured admin can change HostConfig, except that an enabled pauser can set pause flags
(#36); every change stamps `updated_slot` and emits a host
event (`HostConfigUpdatedEvent`, or `NewKmsContextEvent` when `define_kms_context` moves the current context). The event always goes out through the event CPI, so it lands in the transaction's inner instructions, which an
RPC provider cannot truncate the way it can truncate logs. A reader therefore sees an admin change without replaying
instruction data to find one (DD-044). The event only makes the change visible: authorization still comes from account
state, never from event bytes.
Pinned by `only_the_admin_changes_trust_roots_and_only_an_authority_changes_its_store`, which checks over random
instruction sequences that `HostConfig`, the KMS contexts and the deny, HCU trust and pauser records change only in a
transaction the admin signed, apart from a signer holding an enabled pauser record adding pause flags,
and that every `HostConfig` change stamps the current slot and emits an event CPI. Every host instruction is drawn,
with its admin or Store-authority role also filled by keys that lack it, signing or not, and a host account type the
property does not classify fails it. The planted bugs `runtime-tests/planted-bugs/h1-unpause-skips-assert-admin.patch`
and `h1-pause-accepts-a-withdrawn-pauser.patch` must make it fail (`scripts/check-planted-bugs.sh`). This covers the default build; the preview-only `admin-sweep`
build lets the upgrade authority close `HostConfig` and the KMS contexts (`AUTHORITY.md`).

**36. [HOLDS]** `HostConfig.paused` holds one flag per host area (DD-058). `execution` stops `fhe_execute`;
`verified_inputs` stops `fhe_execute` steps that consume a `VerifiedInput`; `acl_writes` stops `create_encrypted_store`,
`make_store_handle_public` and `delegate_for_user_decryption`; `public_decrypt` stops `verify_public_decrypt`. A flag
stops only its own area. Any signer with an enabled `PauserRecord` sets flags; only the admin clears them, and only the
admin creates, enables or disables pauser records. A wallet pauser must call `pause` as a top-level instruction
(`WalletPauseThroughCpi`), as a wallet delegator must delegate (#27): otherwise any program the pauser calls could pause
the host. A PDA pauser, such as a Squads vault, may pause through CPI. Admin setters are never paused, and `revoke_permits` takes no config
account, so it runs under every flag. `revoke_delegation_for_user_decryption` is not paused yet; the delegation-record
change gives it the `acl_writes` gate, as EVM's `revokeDelegationForUserDecryption` is `whenNotPaused`.
Pinned by `mollusk_each_pause_flag_stops_only_its_area`, `mollusk_only_the_public_decrypt_flag_stops_verify_public_decrypt`,
the token tests of 11f, `mollusk_a_pauser_pauses_and_only_the_admin_unpauses`, `mollusk_only_an_enabled_pauser_pauses`,
`mollusk_a_wallet_pause_forwarded_through_another_program_is_rejected`, `mollusk_a_vault_pda_pauses_through_cpi`,
`mollusk_only_the_admin_sets_pausers`, `a_revocation_while_paused_succeeds` and, over random sequences, the H1 property
of #35. The flags do not reach decryption, and the KMS connector does not read `HostConfig`. Gateway ingress has
its own pause: `Decryption.sol` `whenNotPaused` covers every request entry point, the Solana `userDecryptionRequest`
included. HTTP decryption has no pause on either chain, as on EVM.

**37. [HOLDS]** HCU enforcement ships disabled (unrestricted defaults) and is opt-in per knob. `u64::MAX` means
unlimited; `0` is rejected for per-tx limits and means ban untrusted applications only for the block cap. When both
compared limits are finite and the block cap is nonzero, setters enforce `block cap ≥ max per tx ≥ max depth`.
Total and critical-path depth accumulate across all calls in the transaction’s shared transient store, including calls from
different applications. Each application block meter is charged only the cost of its own execution. Repeated handle
occurrences retain the maximum depth for that handle; changing its operand witness cannot reset its depth.
Pinned by `mollusk_initialize_host_config_defaults_block_cap_to_unrestricted`, `mollusk_set_max_hcu_setters_reject_zero`,
`mollusk_set_max_hcu_per_tx_rejects_above_block_cap_band`, `mollusk_set_hcu_block_cap_at_max_per_tx_boundary_is_accepted`,
`mollusk_set_hcu_block_cap_below_max_per_tx_is_rejected` and
`mollusk_fhe_execute_same_application_accumulates_across_payers_and_authorities_and_trips_cap`.

**38. [ASSUMPTION]** The host admin key is a single trusted key. There is no
multisig and no timelock yet (fhevm-internal#1634). The initial admin must be the BPF
upgrade authority (`ProgramData.upgrade_authority_address`). After init,
`set_admin` rotates in one instruction: the current admin signs; a new
keypair must co-sign; a new PDA skips co-sign only when it is off-curve
and already owned by a program (not the System Program), so a mistyped
empty key cannot become admin. A program whose upgrade authority has been
burned (`upgrade_authority_address == None`) cannot initialize. Production
governance is tracked separately (fhevm-internal#1634).

**40. [HOLDS]** An application cannot self-trust: HCU trust records are
written only by the admin, live at `["hcu-trusted", program, scope]`, and
a caller can neither point at another application's record (address
check) nor forge one (program-owned PDA, admin-gated write). The
application is `(program, scope)` with `program` verified from the output
authority (#7, DD-039/DD-047), so a caller cannot claim a trusted program
it does not control. `fhe_execute` validates the trust witness and charges the application meter after its execution walk; exceeding
the cap rolls back the transaction. The meter account is only a counter.
Pinned by `mollusk_set_hcu_app_trusted_rejects_wrong_admin`, `mollusk_set_hcu_app_trusted_rejects_wrong_record_pda`
and `mollusk_fhe_execute_wrong_pda_trust_witness_is_rejected`, and over random sequences by the admin property of #35.

**41. [ANTI]** HCU block budgets do not impose a program-wide limit. Each `(program, scope)` has its own per-slot
budget, and a program chooses its scopes freely. The host proves the Store authority belongs to `program` (#40), but
that does not prevent the program from creating more scopes and funding their meters. This provides separate budgets
for cooperating application instances. The shipped configuration disables the cap (#37).
All steps are charged to the default authority's application, including work on Stores admitted by additional signing
authorities (#62). Pinned by `mollusk_fhe_execute_per_app_meters_are_isolated_under_uniform_cap` and
`mollusk_fhe_execute_same_application_accumulates_across_payers_and_authorities_and_trips_cap`.

**51. [HOLDS]** The optional HCU accounts on `fhe_execute` can arrive in four states, and every state that could hand
out more budget fails closed:

- **Present, program-owned, well-formed** — used. An `hcu_trusted_app_record` with `trusted == true` bypasses the cap;
  `hcu_block_meter` charges the application's per-slot budget.
- **Absent (`None`)** — the untrusted default. An application that is supposed to be metered but omits its meter is
  rejected, not left unmetered.
- **Present at the canonical PDA but never created** (system-owned, empty) — harmless. The application is simply
  untrusted or unused. A squatted meter that does hold data is rejected when `charge` lazily creates it.
- **Present at the wrong PDA, or program-owned but malformed** — the execution is rejected outright.

Pinned by `mollusk_fhe_execute_trusted_witness_bypasses_and_creates_no_meter`,
`mollusk_fhe_execute_untrusted_missing_meter_fails_closed`, `mollusk_fhe_execute_prefunded_empty_meter_is_created_not_griefed`,
`mollusk_fhe_execute_squatted_meter_with_data_is_rejected`, `mollusk_fhe_execute_wrong_pda_trust_witness_is_rejected`
and `mollusk_fhe_execute_malformed_trust_witness_is_rejected`.

## G. Decrypt authorization (gateway, relayer, KMS)

**42. [HOLDS]** Every KMS party's connector independently re-verifies the
user's ed25519 signature over the full request — identity, handles,
allowed scopes, validity window, and nonce. The
relayer and gateway are transport; neither can alter who asks or for what.
Pinned by `every_vector_behaves_as_declared`, `every_wire_field_reaches_the_canonical_bytes` (every request field
changes the signed bytes) and `a_field_the_relayer_changed_fails_the_signature` (the connector refuses a permit whose
key, window or routing the relayer changed).

**43. [ANTI]** The user-decrypt nonce is not dedup-enforced on-chain or in the
connector; replay is bounded only by the request validity window (EVM
parity).

**44. [ANTI]** An empty `allowedScopes` list means permissive mode: the permit
is not scoped to an application, and opens the signer's own handles and
every delegation the signer holds. Scoping (at most seven `(program,
 scope)` pairs) is opt-in per permit and is tested per entry against the
account's own pair, never against a request field.

**63. [ANTI]** Revoking permits does not invalidate a permit signed earlier with a future start time. `revoke_permits`
stores the later of the user's previous `PermitInvalidation` watermark and the current clock. The connector's revocation
check rejects permits whose signed `start_timestamp` is below that watermark; an absent watermark reads as zero. A future-start permit remains
unusable until its window opens, but can then authorize decryption despite the earlier revocation. Permits have no
individual on-chain record to revoke. EVM's `ACL.invalidateDecryptionSignaturesBefore` also rejects future watermarks
with `InvalidationTimestampInTheFuture`. Pinned by the connector authorization vector
`future-start-permit-outliving-a-revocation` and the host test
`first_revocation_creates_the_account_and_records_the_clock`.

**45. [HOLDS]** The connector authorizes against the canonical EncryptedStore PDA, program-owned, rederived from the seeds the account carries,
using the same compiled `zama_solana_acl` code the on-chain program runs
(decode, seeds, MMR verification, both authorize functions). The leaf proof
comes from the coprocessors' leaf record (`POST /v1/solana/leaf-proofs`,
API key), never from the client, and is verified against the peaks of the
account the connector read itself (`kms-worker/src/core/solana/`).
Pinned by `an_encrypted_store_whose_fields_derive_another_address_is_rejected`,
`an_encrypted_store_with_an_altered_bump_is_rejected` and `on_chain_account_decoder_reads_layout`.

**46. [RISK]** The connector's ACL reads use confirmed (not finalized)
commitment, and this component is the authorization gate. The choice is
deliberate and documented at the site (`kms-worker/src/core/solana/snapshot.rs`
module doc: a grant observed on a supermajority-confirmed fork is
sufficient authorization even if that fork is exceptionally rolled back).
This entry records that choice as accepted at the protocol level.

**49. [ASSUMPTION]** The coprocessor's EVM-shaped event rows carry a zeroed
`caller` for every Solana transaction (the 32-byte program does not fit
the 20-byte field and is discarded). This is safe if and only if
nothing downstream ever derives authorization, quotas, or identity from
`caller` on Solana rows — authorization lives in the KMS connector (#42,
#45). Any feature reading `caller` from these rows must branch on the
chain type first.

## H. Reference confidential applications

**55. [HOLDS]** `disclose_secp` binds a certificate to this token program, the mint as scope, the canonical Store of one
token account or of the total supply, the exact handle and the certified cleartext, then emits `HandleDisclosedEvent`
with those fields. The event carries no slot key and no token kind; which operation produced the handle is known from
that operation's own event. A public leaf stays usable after the slot moves on, so an old handle can be disclosed at any
time. Pinned by `mollusk_disclose_secp_is_idempotent_no_replay_marker`.

**56. [HOLDS]** An underlying mint's owner pins one token program. Wrap and
redeem require that program to own the underlying mint and both token
accounts. Classic Token and extension-free Token-2022 are supported.
Token-2022 mint extensions are rejected unless explicitly allowlisted;
today none are allowlisted. Token accounts allow only `ImmutableOwner`.
Pinned by `mollusk_wrap_rejects_classic_program_for_token_2022_accounts`, `mollusk_wrap_rejects_token_2022_mint_extensions`,
`mollusk_wrap_rejects_token_2022_account_extensions` and `mollusk_wrap_usdc_rejects_wrong_underlying_mint`.

**57. [HOLDS]** Each token operation rejects a frozen underlying account that it checks: transfer checks
both owners' canonical ATAs (`from_ata`/`to_ata`), burn checks the owner's ATA (`owner_ata`), and wrap
and redeem check the SPL source and destination accounts they move. Redeem does not recheck the
burner's ATA and may pay a third party. An uninitialized canonical ATA is treated as not frozen;
an empty frozen ATA can be closed and recreated unfrozen. These are account-level checks, not a
persistent holder denylist; see DD-045 and fhevm-internal#1981 for the unresolved launch decision.
Cancel-pending-burn has no issuer-freeze check. Token-2022 transfer-fee, transfer-hook,
non-transferable, and confidential-transfer behavior cannot be inherited
accidentally because those mint extensions fail closed under #56.
Pinned by `mollusk_confidential_transfer_rejects_frozen_sender_ata`, `mollusk_confidential_transfer_rejects_frozen_recipient_ata`,
`mollusk_confidential_burn_rejects_frozen_owner_ata`, `mollusk_redeem_rejects_frozen_token_2022_destination` and
`mollusk_wrap_rejects_frozen_token_2022_source`.

**58. [HOLDS]** Only `ConfidentialMint.authority` can re-write the encrypted
total supply onto a handle with new viewers (`allow_total_supply_viewers`)
or seal its handle public (`make_total_supply_handle_public`). The wrapper
signs the Host CPI as the canonical total-supply authority PDA; callers
cannot substitute another Store, Store authority, slot key, or scope.
Pinned by `mollusk_mint_authority_allows_total_supply_viewers`, `mollusk_mint_authority_seals_total_supply` and
`mollusk_non_mint_authority_cannot_allow_total_supply_viewers`.

**59. [HOLDS]** `ConfidentialMint.authority` is the wrapper's policy authority.
It is distinct from the authority that can upgrade the Zama Host program.
Future governance may own the mint authority without acquiring Host upgrade
power; no governance or authority-rotation mechanism is implied here.
Holds by construction: the token program checks only `ConfidentialMint.authority` for mint-authority actions and reads
neither the Host upgrade authority nor `HostConfig.admin`.

**60. [HOLDS]** A dispatched confidential batch can be cancelled by its join
mint's `ConfidentialMint.authority` while the burn is pending. This is the wrapper policy
authority from #59, not the Zama Host upgrade authority. Cancellation restores the batch's confidential join balance and
encrypted total supply, closes the pending burn, and moves the batch to the
refund-only `Refunding` state. That state accepts user quits but rejects new
joins, dispatch, settlement, and repeated cancellation, so recovery from failed KMS or vault settlement requires that authority’s cooperation. Redeem and cancellation cannot consume the same pending burn twice.
Pinned by `mollusk_cancel_dispatch_restores_burn_and_allows_refunds` and
`mollusk_redeem_current_pending_burn_then_rejects_double_settlement`.

---

# Part II — Sizes, limits, and how it is run

Nothing here is a safety promise. These entries record sizes, limits, and how
the system is operated. They change when we resize something or swap tooling,
not when the threat model changes.

## I. Sizes, limits, and operations

**14. [HOLDS]** Executions are capped at 32 steps. Packet size and CU cost depend on the shape; the cap does not imply a
fit in a 1,232-byte transaction or 200k CU. `runtime-tests/cost-snapshots/fhe_execute_boundary.json` pins each shape's
instruction-data bytes and CU at its largest passing size. These are host-instruction measurements, before transaction
overhead and application CPIs.
Pinned by `rejects_more_than_max_ops`, `cost_snapshot_fhe_execute_max_steps` and `cost_snapshot_boundary_sweeps`.

**34. [OPERATIONAL]** Reconstruction fixtures compile only under
`--features solana-grpc,solana-reconstruct`; coverage exists only where CI
passes those flags.

**47. [RETIRED]** The standalone proof service is gone (RFC 035, DD-048). The
leaf record lives in each coprocessor's host listener, served behind an API
key; the connector fans out to every configured coprocessor, so one behind
or unreachable cannot sink a request another can serve. Authorization was
never its to give (#30).

**48. [HOLDS]** Settle transactions at production KMS thresholds fit one packet
only as v0 + one address lookup table; a legacy settle never fits. Pinned by
`settle_transaction_size_needs_v0_lookup_table_and_fits` and `redeem_settle_transaction_size_needs_v0_lookup_table_and_fits`.

**50. [OPERATIONAL]** The relayer's ACL preflight covers EVM host chains and,
advisorily, Solana delegated entries: a delegation row that is dead at the
slot of the read (absent, revoked, expired) is refused before the gateway
fee (`relayer/src/host/solana_delegation_precheck.rs`); every ambiguity of
data passes. A direct Solana entry is not pre-checked — its authorization
is an allow leaf the connector fetches, and there is no cheaper reading of
it — so an unauthorized one is rejected by the KMS connectors after the
gateway fee is paid. This does not affect authorization (#42, #45); for
now we accept that a rejected request can still cost a fee, and that
this leaves room for spam.

**52. [OPERATIONAL]** Every batch gets its own settle address lookup table, and
the demo runs the full table lifecycle: create + extend at `open_batch`
(chunked so no extend can exceed the transaction wire limit), deactivate
immediately after settlement, close once the ~513-slot deactivation
cooldown has elapsed, refunding rent to the keeper. Deactivate and close
are best-effort rent hygiene — a failure never fails a settlement, and the
close crank retries on the next batch preparation. One composition function
fills the table and compresses against it, so provisioned and consumed
membership cannot diverge (`solana/demo-dapp/src/vault`).

**54. [HOLDS]** `FheExecution::build` enforces three typed resource ceilings:

- **Steps** — at most the host's `MAX_FHE_EXECUTION_STEPS` (32), or `TooManySteps`.
- **CPI packet** — serialized instruction data, including its discriminator, fits `CPI_INSTRUCTION_DATA_LIMIT` (10 KiB),
  or `ExceedsCpiInstructionDataLimit`. A transaction's 1,232-byte wire limit is separate: a compact app instruction can
  construct a larger host CPI.
- **Build heap** — allocations requested by build, packet serialization, account resolution and invoke tables stay
  within `BUILD_HEAP_BUDGET_BYTES` (24 KiB), or `ExceedsBuildHeapBudget`. `HeapBudget` charges allocations before they
  occur. The remaining `APP_HEAP_RESERVE_BYTES` (8 KiB) covers allocator padding, Anchor deserialization and the app's
  own allocations; an app needing more must leave additional headroom.

Store creation is a separate instruction. Execution can grow existing Stores and top up rent; there is no per-result
account-creation cap. `FheExecution::cost` reports packet bytes, tallied heap, and instruction-trace bounds. The
worst-case trace includes a possible rent transfer per Store output and lazy meter creation. The caller must add
Store/transient store creation, final transient store close and all other app CPIs when budgeting the transaction.

Counting-allocator tests in `solana/crates/zama-fhe/src/heap_budget/` compare requested build/packet and invoke-table
bytes with their tallies across the current shape frontier. `the_tally_never_crosses_the_budget_even_transiently` also
checks rejection paths. `print_build_frontier_grid` prints the current measurements; those tests are the source of truth
for the admission frontier.

The host heap remains a separate limit (#61). Runtime sweeps cover wide audiences, reductions and mature history. In the
committed snapshots, updates across Stores with 8, 32 and 55 MMR peaks reach 16, 7 and 4 steps, respectively;
maximum-width sums reach 6. These shape measurements do not guarantee that an arbitrary composition fits.

**66. [HOLDS]** TransientStore has fixed storage for 112 result occurrences and 32 explicit grants (10,168 bytes including
discriminator). Repeated handles count as occurrences to preserve step/output references. Each execution admits at most
32 steps and 32 effects; return selection admits 32 handles, including repeated selections. Capacity overflow fails
atomically. SBF capacity is not packet capacity: application CPIs can construct payloads larger than the outer 1,232-byte
transaction. The SDK heap model, runtime shape sweeps and packet-fit tests measure these separate limits.
Pinned by `result_journal_capacity_is_shared_across_calls_and_fails_atomically` and
`maximum_result_grants_fit_one_execution_and_leave_no_account`.

**39. [RETIRED]** App-layer invariants were folded into this register rather
than split into a second source of truth (#55–#60).
