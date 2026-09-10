# Protocol invariants — Solana fhevm (POC)

Last synced: 2026-09-10.

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

Scope note: this register covers the Solana feature branch: `zama-host`, the `zama-fhe` SDK, the host-listener
reconstruction path and its leaf record, the KMS connector's Solana pipeline, and the reference confidential-token and
confidential-batcher applications. Vocabulary follows GLOSSARY.md: execution, dictionary, State, slot, allow, result
grant, scratch, application.

---

# Part I — What the system guarantees

## A. Confidentiality & privacy

**1. [HOLDS]** Plaintext values never appear on-chain: the chain stores handles
and access state; ciphertexts live only in the coprocessor.

**2. [HOLDS]** A failed confidential transfer is indistinguishable on-chain from
a successful one (the execution moves an encrypted zero; there is no failure
branch to observe).

**3. [ANTI]** Participation, timing, touched accounts, instruction shapes, and
execution structure are all public.

**4. [ANTI]** Allows (who may decrypt which handle) are public: every allow is
sealed from instruction data, and the leaf record republishes it.

## B. Handles & access state

**5. [HOLDS]** A State slot changes only through an `fhe_execute` output. No instruction accepts a caller-chosen handle
into a slot; `make_state_handle_public` seals a leaf for the handle a slot already holds.

**6. [HOLDS]** A slot write states what the slot holds now: the exact current handle, or nothing for a first write. A
State output also states the State's leaf count as the builder saw it. If either is stale the whole execution fails
(`PreviousStateMismatch`), so two writers cannot lose an update. The new handle's allows are declared on the write; the
old handle's leaves stay sealed.

**7. [HOLDS]** Every encrypted State lives at `["encrypted-state", program, authority, scope]` and stores those identity fields plus its canonical bump. Creation proves that authority is a PDA of program. Readers rederive the address and validate the stored shape. Slot keys are not address seeds.

**8. [HOLDS]** Sealed history (the MMR) is append-only: a handle sealed public
stays provable after any number of later updates.

**9. [RETIRED]** The instruction that removed a viewer went with the stored list
(RFC 035, DD-048): allows are sealed on the write and never removed. A handle with no allows and
no public leaf is undecryptable by everyone, which is its author's choice, not
a stranding — the next write declares the next handle's allows.

**10. [HOLDS]** Every allow the host seals passes the deny list when it is enabled. A State output, with or without a
slot write, and `make_state_handle_public` both require the application's record `["deny-scope", program, scope]` to be
present at its canonical address and not denied (`DenyRecordMissing`, `ScopeDenied`). With the list disabled no record
may be passed. An `fhe_execute` checks every application whose State it reads or writes, and the application of every
grant's initiating State. Naming a consumer State in a new grant only validates that State's identity; the consumer's
application is checked by the execution that consumes the grant. A denied application can therefore neither use a grant
nor receive a State write from another program's execution. The list names applications, not keys (DD-048): a denied key
can still be allowed by a clean application, and user-decryption delegation is a separate access path with no deny
check.

**11. [HOLDS]** Only the State authority writes its slots or appends permissions: it signs State creation, State outputs
and `make_state_handle_public`, and it is a PDA of the State's `program` (#7). A viewer is not a co-admin — an allow
grants decrypt and nothing else, and later new permissions on history-only handles are deferred to #2007 (DD-048).
Confidential-token ships owner-gated wrappers that `invoke_signed` as the **token-account** PDA
(`allow_balance_viewers`, which re-writes the balance onto a handle allowed to the viewers, and
`make_token_account_handle_public`); the mint authority has the same pair for the total supply, signed as the
total-supply authority PDA (`allow_total_supply_viewers`, `make_total_supply_handle_public`). (fhevm-internal#1862 #13;
RFC 035.) Related token/Host lifecycle guardrails are:

- **11b [HOLDS].** `make_state_handle_public` requires the signer to equal `EncryptedState.authority` and the handle to
  be the current handle in the named slot. A viewer cannot directly publish through this authority-only instruction;
  history-only publication and EVM-style re-sharing are deferred to #2007. The deny list is consulted for the value's
  application, because sealing a public leaf is an allow (#10). Confidential-token owner/mint-authority wrappers
  validate the exact state field and sign as the token-account/total-supply PDA. (fhevm-internal#1862.)
- **11c [HOLDS].** Each confidential token account may have exactly one pending burn, stored at
  `["pending-burn", mint, token_account]`. A second burn is rejected before FHE execution until `redeem_burned_amount`
  or `cancel_pending_burn` closes the account and returns its rent to the owner. Parallel burns for one token account
  are deliberately deferred; applications can aggregate an amount or use separate app-owned token accounts.
- **11d [HOLDS].** `cancel_pending_burn` requires the pending burned handle to equal the State’s current burned-amount
  slot handle. A stale or mismatched pending burn cannot restore value.
- **11e [HOLDS].** `cancel_pending_burn` restores both confidential balance and encrypted `total_supply` (mirrors wrap's
  dual add; undoes burn's dual sub). Redeem does not restore encrypted supply — it exits via underlying payout.
  (fhevm-internal#1862 review P1.)
- **11f [HOLDS].** Host pause (`HostConfig.paused`) gates token cash-out / disclose paths that call
  `assert_host_config_allows_token_response` (redeem, disclose). Opening a burn / cancelling a pending burn still
  requires a live FHE path through the host; there is no separate token-level pause. No registry / observer / on-chain
  gov surface in this PoC (out of scope; zama-ai/fhevm-internal#1634).

**62. [HOLDS]** Compute permission is a signature, never a proof. Reading a slot requires its State authority's
signature and the exact current handle. Using a result produced by another execution in the same transaction requires a
grant for that exact handle and consumer State in the canonical scratch account, plus the consumer authority's
signature. Scratch is opened by the initiating State's authority, its grants are created only as outputs of the
execution that produced the handle, and it is closed by the matching final top-level instruction, which refunds the
payer recorded at opening; a failed close rolls back the transaction. Decrypt permission is separate: it needs an MMR
leaf and is never implied by compute authority. In the batcher, each JoinRecord is the authority of its participant's
contribution State, scoped to the batch. Pinned by `transient_mollusk.rs`
(`transient_result_accepts_the_exact_granted_handle_and_consumer_state`, `transient_result_rejects_an_ungranted_handle`,
`transient_result_rejects_the_wrong_consumer_state`, `transient_result_requires_the_consumer_state_authority_signature`,
`scratch_cannot_close_before_the_final_instruction`, `nested_scratch_close_rolls_back_the_whole_transaction`).

**64. [ANTI]** A grant limits who may compute with a handle inside one transaction. It does not limit what that
computation may reveal: the consumer's output can be written to a slot, allowed to any key or made public, and those
leaves outlive the scratch account. A producer that grants a result trusts the consumer's program with the information
in it.

**53. [ANTI]** `make_state_handle_public` is not idempotent. Sealing a handle that is already sealed appends a second
leaf committing to the same `(account, handle)` fact: it authorizes nothing the first leaf did not, and its cost is
bounded — peaks are one per set bit of `leaf_count`, capped at `MAX_MMR_PEAKS` (64) — and funded by the caller's own
payer. Guarding it on-chain would need the account to remember which handle is sealed, which is new `EncryptedState`
state in four consumers; a state-free guard could only read back the last leaf when `leaf_count` is odd, so the same
call would be accepted or rejected by parity. Pinned by
`mollusk_make_handle_public_twice_appends_an_equivalent_leaf`.

## C. Execution

**12. [HOLDS]** An execution is atomic. Step count, account count, return selection, account table, preflight and deny
checks run before the walk; step semantics are validated during the walk. A failure at any point rolls back the entire
transaction, including earlier slot writes, sealed leaves and the rand nonce.

**65. [HOLDS]** Return data carries handles, never permission. An execution names the results to return as an ordered
list of `(step_index, output_index)` pairs, at most 32 (the 1,024-byte return-data limit), and the host copies exactly
those handles in that order, repeats included. An empty list still sets empty return data after the event CPIs, so an
event CPI's return data cannot reach the caller. An out-of-range step or a nonzero output index is rejected before the
walk (`InvalidReturnSelection`); current operators have one output. The consumer reads a returned handle by position and
still needs a grant or its own authority to compute with it (#62). Pinned by
`mollusk_fhe_execute_returns_selected_handles_in_order_with_repeats`,
`mollusk_fhe_execute_rejects_invalid_return_selection_before_execution`, and the SDK test
`selected_result_stays_paired_with_its_execution_and_checks_return_data`.

**13. [HOLDS]** Every dictionary index is bounds-checked by all three consumers
(program, SDK, listener); an unreferenced dictionary entry rejects the
execution.

**15. [HOLDS]** Every op/type combination that validation accepts also has a
metering cost row, so a step that passed validation can never abort because
its cost is unknown. It does not work the other way round, deliberately:
some combinations have a price but are still rejected by validation.

**16. [HOLDS]** An execution containing a rand step must pass the host's
`RandNonce` singleton (`["rand-nonce"]`; `FheExecuteRandNonceMissing`
otherwise) and advances it; the nonce is bound into every rand seed, so two
executions can never derive the same seed, whatever they persist (DD-043).
The nonce is host state, never caller-supplied, so a caller cannot steer it.

**17. [HOLDS]** `account_count` declared inside the instruction data must equal the number of remaining accounts
actually delivered.

**18. [HOLDS]** A transient result from one SDK builder cannot be used in another. `FheExecution::build` gives each
builder a distinct `'id` lifetime, which its transient results carry; mixing them is a compile error. Stored slot
operands are independent of a builder. The host also checks producer-index bounds because callers can construct
instruction arguments without the SDK. Pinned by the `compile_fail` doctest on `FheExecution::build`.
(fhevm-internal#1859 §4.)

**61. [ANTI]** `FheExecution::build` does not guarantee that the host's CPI fits the host's heap or compute budget. The
builder's typed limits (#54) cover the app's own heap. The host has a separate 32 KiB heap, and what it allocates
depends on the live State size, the number of MMR peaks and the permissions sealed per output, none of which the builder
can see. The gap is measurable: `shared_audience_state_outputs_fit_the_builder_at_full_depth` admits 32 slot outputs
with the same eight viewers and a public leaf, while the runtime sweep `fhe_execute_boundary/allow_heavy_public_creates`
succeeds at 24 such outputs and exhausts the host heap at 25. These are shape measurements, not an output cap. No
host-side admission model exists; an app validates its shapes against the sweeps and budgets the whole transaction. Why
no allocator was shipped is DD-046 (fhevm-internal#1872).

## D. Entry & exit trust

**19. [HOLDS]** A verified input is consumed only with a threshold-valid
coprocessor attestation that names the calling program and the host chain id.

**20. [HOLDS]** Verified inputs grant nothing persistent: they are usable only
inside the carrying execution; persistence requires an explicit output with its
own allows.

**21. [HOLDS]** Public cleartext is accepted on-chain only through
`verify_public_decrypt`: a KMS threshold certificate **and** an MMR
inclusion proof that the exact handle was sealed public.

**22. [HOLDS]** Certificate binding chain: signed `extra_data` → context id → canonical KmsContext PDA → signer set.
Empty or version-0 `extra_data` selects the current context; version 1 is exactly 33 bytes and carries the 32-byte id.
Solana version 4 is exactly 65 bytes: version, context id, then the State address used to route the decrypt request.
Version 3 is rejected. The verifier authenticates the context, handle and cleartext through the certificate and
independently verifies the exact handle's public leaf against the supplied State's current peaks. It does not require
that State to equal the routing address in `extra_data`. Destroying a context invalidates its certificates; rotation
alone invalidates none.

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
delegation record for the encrypted State's authority and the delegator's wildcard row in the deciding snapshot. Either
row authorizes the delegate if it is live at that slot: not revoked, not expired, and not written after the observation.
A dead row cannot veto a live one. The connector then requires the delegator's allow leaf
(`kms-worker/src/core/solana/delegation.rs`). The relayer refuses dead rows advisorily before the gateway fee (#50).
Delegation emits no event; readers read the record (DD-044).

## E. Reconstruction & off-chain services

**28. [HOLDS]** Handles the listener re-derives are byte-identical to the
on-chain ones, because the listener imports the program's own derivation
functions and argument types rather than reimplementing them (fixtures and
the e2e derivation check this too).

**29. [HOLDS]** Every transaction is independently interpretable: replay from
instruction bytes alone reconstructs full history with zero account reads
(updates echo the previous handle and declare the new handle's allows).

**30. [HOLDS]** The leaf record can stop a decrypt from happening but can
never be what allows one: the KMS connector verifies every proof against
the peaks it read on chain itself, fans out to every configured
coprocessor and merges (a proof beats no proof, more history beats less),
and rejects a client-supplied proof outright. A compromised or lagging
record fails or delays decrypts; it cannot authorize one (DD-048).

**31. [HOLDS]** Coprocessor scheduling is decoupled from authorization: eager
scheduling can waste compute on a minority fork; it can never release
plaintext.

**32. [GAP]** No reorg unwind on the listener path; minority-fork work is never
rolled back (safe only because of #31).

**33. [RISK]** Nothing pins a deployed program build to the listener build; the
shared-crate identicality guarantee (#28) silently assumes matching
versions.

## F. Admin, config & custody

**35. [HOLDS]** Only the configured admin can change HostConfig; every change stamps `updated_slot` and emits a config
event. The event always goes out through the event CPI, so it lands in the transaction's inner instructions, which an
RPC provider cannot truncate the way it can truncate logs. A reader therefore sees an admin change without replaying
instruction data to find one (DD-044). The event only makes the change visible: authorization still comes from account
state, never from event bytes.

**36. [HOLDS]** `HostConfig.paused` freezes both halves of the plaintext path: the production-shaped host instructions
(`fhe_execute`, `make_state_handle_public`, `delegate_for_user_decryption`, and the token cash-out paths of 11f), and
connector user decryption — the KMS connector's authorization reads the `HostConfig` PDA in the account read it already
makes and refuses while paused (transiently: the same request authorizes once the pause is lifted). One switch, on the
host, and no gateway-side pause is involved. The connector decodes the singleton through `zama-solana-acl`'s shared
decoder, the same crate the program's own `shared_crate_decoder_reads_what_the_program_serializes` test pins against its
serializer, so the switch cannot be disarmed by layout drift. User abort levers stay open while paused, deliberately:
`revoke_permits` takes no config account at all, and `revoke_delegation_for_user_decryption` is not pause-gated. This
asymmetry is deliberate. A pause stops the connector from serving delegated decryptions, so a delegator who could not
revoke would be left with grants they can neither use nor withdraw, and a lever the operator can switch off is not the
user's lever. Not gated: `verify_public_decrypt` (DD-040, already-sealed leaves reveal nothing new) and the admin
setters, pause included.

**37. [HOLDS]** HCU enforcement ships disabled (unrestricted defaults) and is opt-in per knob. `u64::MAX` means
unlimited; `0` is rejected for per-tx limits and means ban untrusted applications only for the block cap. When both
compared limits are finite and the block cap is nonzero, setters enforce `block cap ≥ max per tx ≥ max depth`.

**38. [ASSUMPTION]** The host admin key is a single trusted key. This is a POC:
there is no multisig and no timelock. The initial admin must be the BPF
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
it does not control. The block cap is enforced by the program in
`fhe_execute` before the execution walk; the meter account is only a
counter.

**41. [ANTI]** HCU block budgets do not impose a program-wide limit. Each `(program, scope)` has its own per-slot
budget, and a program chooses its scopes freely. The host proves the State authority belongs to `program` (#40), but
that does not prevent the program from creating more scopes and funding their meters. This provides separate budgets
for cooperating application instances. The shipped configuration disables the cap (#37).
All steps are charged to the default authority's application, including work on States admitted by additional signing
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

## G. Decrypt authorization (gateway, relayer, KMS)

**42. [HOLDS]** Every KMS party's connector independently re-verifies the
user's ed25519 signature over the full request — identity, handles,
allowed scopes, validity window, and nonce. The
relayer and gateway are transport; neither can alter who asks or for what.

**43. [ANTI]** The user-decrypt nonce is not dedup-enforced on-chain or in the
connector; replay is bounded only by the request validity window (EVM
parity).

**44. [ANTI]** An empty `allowedScopes` list means permissive mode: the permit
is not scoped to an application, and opens the signer's own handles and
every delegation the signer holds. Scoping (at most seven `(program,
 scope)` pairs) is opt-in per permit and is tested per entry against the
account's own pair, never against a request field.

**63. [ANTI]** Revoking permits does not invalidate a permit signed earlier with a future start time. `revoke_permits`
stores the later of the user's previous `PermitInvalidation` watermark and the current clock. The connector rejects a permit only when its
signed `start_timestamp` is below that watermark; an absent watermark reads as zero. A future-start permit remains
unusable until its window opens, but can then authorize decryption despite the earlier revocation. Permits have no
individual on-chain record to revoke. EVM's `ACL.invalidateDecryptionSignaturesBefore` also rejects future watermarks
with `InvalidationTimestampInTheFuture`. Pinned by the connector authorization vector
`future-start-permit-outliving-a-revocation` and the host test
`first_revocation_creates_the_account_and_records_the_clock`.

**45. [HOLDS]** The connector authorizes against the canonical EncryptedState PDA, program-owned, rederived from the seeds the account carries,
using the same compiled `zama_solana_acl` code the on-chain program runs
(decode, seeds, MMR verification, both authorize functions). The leaf proof
comes from the coprocessors' leaf record (`POST /v1/solana/leaf-proofs`,
API key), never from the client, and is verified against the peaks of the
account the connector read itself (`kms-worker/src/core/solana/`).

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

**55. [HOLDS]** `disclose_secp` binds a certificate to this token program, the mint as scope, the canonical State of one
token account or of the total supply, the exact handle and the certified cleartext, then emits `HandleDisclosedEvent`
with those fields. The event carries no slot key and no token kind; which operation produced the handle is known from
that operation's own event. A public leaf stays usable after the slot moves on, so an old handle can be disclosed at any
time. Pinned by `mollusk_disclose_secp_is_idempotent_no_replay_marker`.

**56. [HOLDS]** An underlying mint's owner pins one token program. Wrap and
redeem require that program to own the underlying mint and both token
accounts. Classic Token and extension-free Token-2022 are supported.
Token-2022 mint extensions are rejected unless explicitly allowlisted;
today none are allowlisted. Token accounts allow only `ImmutableOwner`.

**57. [HOLDS]** Frozen underlying token accounts cannot wrap, confidential-transfer, burn, or
redeem. Transfer checks both owners' associated token accounts (`from_ata`/`to_ata`); burn
checks the burner's (`owner_ata`). Uninitialized at that ATA address is treated as not frozen,
so the mirror does not reach a holder with no canonical ATA (funded by confidential transfer
only) or one who closed and recreated an empty frozen ATA; see DD-045 and fhevm-internal#1981.
Wrap and redeem keep checking the
accounts they actually move. Cancel-pending-burn is not freeze-gated. Token-2022 transfer-fee, transfer-hook,
non-transferable, and confidential-transfer behavior cannot be inherited
accidentally because those mint extensions fail closed under #56.

**58. [HOLDS]** Only `ConfidentialMint.authority` can re-write the encrypted
total supply onto a handle with new viewers (`allow_total_supply_viewers`)
or seal its handle public (`make_total_supply_handle_public`). The wrapper
signs the Host CPI as the canonical total-supply authority PDA; callers
cannot substitute another State, State authority, slot key, or scope.

**59. [HOLDS]** `ConfidentialMint.authority` is the wrapper's policy authority.
It is distinct from the authority that can upgrade the Zama Host program.
Future governance may own the mint authority without acquiring Host upgrade
power; no governance or authority-rotation mechanism is implied here.

**60. [HOLDS]** A dispatched confidential batch can be cancelled by its join
mint's `ConfidentialMint.authority` while the burn is pending. This is the wrapper policy
authority from #59, not the Zama Host upgrade authority. Cancellation restores the batch's confidential join balance and
encrypted total supply, closes the pending burn, and moves the batch to the
refund-only `Refunding` state. That state accepts user quits but rejects new
joins, dispatch, settlement, and repeated cancellation, so recovery from failed KMS or vault settlement requires that authority’s cooperation. Redeem and cancellation cannot consume the same pending burn twice.

---

# Part II — Sizes, limits, and how it is run

Nothing here is a safety promise. These entries record sizes, limits, and how
the system is operated. They change when we resize something or swap tooling,
not when the threat model changes.

## I. Sizes, limits, and operations

**14. [HOLDS]** Executions are capped at 32 steps. Packet size and CU cost depend on the shape; the cap does not imply a
fit in a 1,232-byte transaction or 200k CU. The runtime snapshots pin, for example, a 32-step dependent chain at 461
instruction-data bytes and 60,597 CU, and 32 private slot writes at 2,901 bytes and 123,784 CU. The 30-output
wide-audience shape costs 281,645 CU. These are host-instruction measurements, before transaction overhead and
application CPIs; see `runtime-tests/cost-snapshots/fhe_execute_boundary.json`.

**34. [OPERATIONAL]** Reconstruction fixtures compile only under
`--features solana-grpc,solana-reconstruct`; coverage exists only where CI
passes those flags.

**47. [RETIRED]** The standalone proof service is gone (RFC 035, DD-048). The
leaf record lives in each coprocessor's host listener, served behind an API
key; the connector fans out to every configured coprocessor, so one behind
or unreachable cannot sink a request another can serve. Authorization was
never its to give (#30).

**48. [HOLDS]** Settle transactions at production KMS thresholds fit one packet
only as v0 + one address lookup table; a legacy settle never fits. Both
directions are pinned by tests.

**50. [OPERATIONAL]** The relayer's ACL preflight covers EVM host chains and,
advisorily, Solana delegated entries: a delegation row that is dead at the
slot of the read (absent, revoked, expired) is refused before the gateway
fee (`relayer/src/host/solana_delegation_precheck.rs`); every ambiguity of
data passes. A direct Solana entry is not pre-checked — its authorization
is an allow leaf the connector fetches, and there is no cheaper reading of
it — so an unauthorized one is rejected by the KMS connectors after the
gateway fee is paid. This does not affect authorization (#42, #45); for
the POC we accept that a rejected request can still cost a fee, and that
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

State creation is a separate instruction. Execution can grow existing States and top up rent; there is no per-result
account-creation cap. `FheExecution::cost` reports packet bytes, tallied heap, and instruction-trace bounds. The
worst-case trace includes a possible rent transfer per State output and lazy meter creation. The caller must add
State/scratch creation, final scratch close and all other app CPIs when budgeting the transaction.

Counting-allocator tests in `solana/crates/zama-fhe/src/heap_budget/` compare requested build/packet and invoke-table
bytes with their tallies across the current shape frontier. `the_tally_never_crosses_the_budget_even_transiently` also
checks rejection paths. `print_build_frontier_grid` prints the current measurements; those tests are the source of truth
for the admission frontier.

The host heap remains a separate limit (#61). Runtime sweeps cover wide audiences, reductions and mature history. In the
committed snapshots, updates across States with 8, 32 and 64 MMR peaks reach 15, 7 and 4 steps, respectively; 60-operand
reductions reach 4. These shape measurements do not guarantee that an arbitrary composition fits.

## J. Roadmap

**39. [RETIRED]** App-layer invariants were folded into this register rather
than split into a second source of truth (#55–#60).
