# EVM → Solana Parity Map

What this document is: a per-sub-feature map of every EVM-side capability of the Zama
confidential-token + FHEVM host design onto its Solana equivalent in this PoC — or a justified
divergence. Every claim is grounded in code review of `ERC7984.sol`, `host-contracts/`,
`gateway-contracts/`, and the Solana programs + adapters
(`solana/programs/{zama-host,confidential-token,confidential-deposit-app}`,
`coprocessor/.../host-listener`, `kms-connector`).

Who it is for:

- **Zama engineers** extending the port — each section names the code entry points.
- **OpenZeppelin reviewers** — each section separates faithful parity from PoC shortcuts, and the
  [fragility list](#fragile--attention-for-a-security-pass) is the security-pass worklist.
- **Integrators porting from ERC7984** — read [Do not assume](#porting-from-erc7984-do-not-assume)
  first; it lists the EVM guarantees that do *not* hold here.

Design rationale for every divergence is recorded in
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (DD-001..DD-030) and RFC 024. Unsettled questions
live in the [open decisions ledger](#open-decisions-ledger) at the end, with owners.

## Legend

- **MET** — Solana faithfully implements the EVM semantics (sometimes stricter).
- **DIVERGENCE** — re-expressed differently because of a Solana platform constraint; intent preserved.
- **PARTIAL** — the common path is ported, a named edge is not.
- **INTENTIONAL GAP** — deliberately not ported; the section says what replaces it.
- **SCOPE** — deliberately not in this PoC; not Solana-constrained; mechanically extensible.
- **PRODUCT-OPEN** — belongs to an off-chain service or a future program by design.

## The five constraints behind every divergence

Every DIVERGENCE row below traces back to one of these Solana platform facts. Internalize them
once and the rest of the document reads as consequences:

1. **No `msg.sender`.** There is no implicit caller identity; compute identity is an explicit
   signer account, and "the app" is an explicit app-context pubkey, not a program id (program ids
   are reusable library code).
2. **No transient storage.** EVM's `allowTransient`/tstore has no primitive; transaction-local
   permission is modeled as instruction-local evidence plus CPI signer propagation (DD-008).
3. **Accounts must be pre-declared, but computed handles are unpredictable.** Handles carry
   protocol-injected entropy, so an account keyed on a handle cannot be named before the compute
   runs. This single fact forced the nonce-key ACL design (DD-001).
4. **Shallow CPI stack (~4 levels) and 64-account transaction cap.** Deep call-chains and
   per-permission accounts do not scale; designs flatten instead of nest.
5. **Storage (rent) is the dominant cost, not compute.** Account creation is expensive and
   permanent; the design minimizes accounts born per operation.

## Porting from ERC7984: do not assume

The EVM standard makes guarantees this port deliberately does not. Integrators arriving from
Solidity should check each of these against their assumptions:

- **No operator / approve / `transferFrom`.** All production transfer paths are owner-signed.
  A program that needs to move a user's funds composes via CPI with the user's signature
  propagating through (see [§6](#6-cpi-composition--confidential-deposit-app)). Delegation
  semantics for multi-transfer flows (swaps, subscriptions) are an open product question tracked
  in the ledger.
- **No `confidentialTransferAndCall` / receiver callback.** The EVM callback exists because
  contracts cannot observe incoming transfers; Solana apps drive the transfer themselves inside
  their own instruction, atomically (DD-011).
- **Multi-leg flows that EVM does in one transaction may be multiple transactions here** (e.g.
  burn → redeem is two legs, the second gated on a KMS certificate). Refund-on-reject is
  best-effort across legs, not atomic.
- **`tryIncrease` safe-math is not reproduced on credit paths.** The debit path reproduces EVM's
  `ge → sub → select` underflow guard; the recipient-credit and total-supply increase use a plain
  FHE `add`, bounded instead by the 1:1 SPL backing of the wrap model (a real `u64` mint). Fine
  for 6-decimal wrapped USDC; re-evaluate before any high-supply/18-decimal token.
- **Views are account reads.** `confidentialBalanceOf`-style calls become reads of
  `ConfidentialTokenAccount` / mint state, off-chain.
- **Events are indexing hints, never authorization** (DD-003). Nothing security-relevant may be
  derived from an event alone.

---

## 1. ACL & handles

**EVM:** `ACL.sol` — persistent `allow(handle, account)` mapping slots, deny list, transient
allow, `msg.sender`-implicit granting.
**Solana:** handles cannot key accounts (constraint 3), so ACL records are PDAs derived from
app-controlled nonce metadata, and the handle is stored *inside* the record, bound at compute time
(DD-001). Grants are append-only with an explicit grant role — stricter than EVM's implicit model.

| EVM capability | Solana equivalent | Status |
|---|---|---|
| `ACL.allow(handle, account)` | `allow_acl_subjects` — authority `Signer` + `ACL_ROLE_GRANT` + canonical PDA + deny-list + pause; append-only | **MET** (stricter) |
| `ACL.allowForDecryption(handles[])` | `allow_for_decryption` — dedicated `ACL_ROLE_PUBLIC_DECRYPT`, never set at birth (DD-005) | **MET** (per-handle) |
| `ACL.isAllowed` / `isAllowedForDecryption` | `assert_acl_record` / role checks / `AclRecord.public_decrypt` | **MET** |
| inline subjects overflow | `AclPermission` canonical overflow PDA (`MAX_ACL_SUBJECTS=8` inline) | **MET** (DD-004: layout is ABI) |
| deny list (`blockAccount`) | `set_deny_subject` + `DenySubjectRecord`, gated into grant paths | **MET** |
| pause / 2-step owner / UUPS | single `admin` signer; BPF loader owns upgrades | **DIVERGENCE** + **PRODUCT-OPEN** (see [§9](#9-admin-config--governance)) |
| `delegate/revokeForUserDecryption` | PDA per `(delegator, delegate, app)`, slot expiry, same-slot guard, wildcard-delegate rejected | **MET** |

State types, one line each (the names are close; the purposes are not):
`AclRecord` — the canonical per-handle authorization record (subjects, roles, public-decrypt flag);
`AclPermission` — overflow witness PDA for subjects beyond the inline 8;
`HandleMaterialCommitment` — ciphertext-material availability, separate from authorization (DD-006).

Handle derivation preserves the EVM byte layout (version/type/chain-id/computed-marker) **and the
keccak256 hash**; what differs is the entropy source (previous bank hash + timestamp vs blockhash)
and the embedded chain id, which keep handles chain-scoped by construction. (sha256 appears only in
the ACL nonce-key domain hash, DD-001.) Fail-closed when the prior bank hash is unavailable; the
zero-entropy fallback is compile- and chain-id-confined to the local PoC chain (DD-014, DD-015).

Code: `zama-host/src/state/{acl_record,acl_permission,deny_subject_record}.rs`,
`instructions/{allow_acl_subjects,assert_acl_record,allow_for_decryption,set_deny_subject}.rs`,
`instructions/fhe_eval/handles.rs`. Tests: `runtime-tests/tests/host_mollusk.rs`.

> **Known cost problem.** This model creates ACL accounts per transfer; at Solana rent prices that
> is on the order of $1–2 per confidential transfer — explicitly rejected as a production cost
> (guild decision, June 2026). The successor design (mutable encrypted-value "current" pointer +
> Merkle Mountain Range for historical proofs) is specified in fhevm-internal#1569 and is **not**
> in this code yet. This document describes what is built, not that plan.

## 2. Transient allow

**EVM:** `allowTransient` writes tstore, wiped at transaction end; `transferAndCall` leans on it.
**Solana:** no transient storage (constraint 2). Transaction-local permission is
instruction-local: an `AllowedLocal` value alive within one `fhe_eval`, plus signer authority
propagating through CPI within one instruction. Nothing durable is written, so there is nothing
to clean (`cleanTransientStorage` has no analog; the earlier one-shot `TransientSession` account
tier was removed — DD-008). Cross-CPI transient allowance ("oneshot accounts") was deliberately
deferred until a real DeFi flow proves the need.

## 3. Input verification

**EVM:** `InputVerification.verifyProofRequest/Response` (ZKPoK consensus) +
`FHEVMExecutor.verifyInput` — verified inputs get a *transient* allow only; persisting is the
app's explicit job.
**Solana:** identical semantics, one mechanism: the `FheEvalOperand::VerifiedInput` operand
consumed inside `fhe_eval` (the `FHE.fromExternal` analog). It re-verifies the coprocessor's
EIP-712 `CiphertextVerification` attestation in-frame via `secp256k1_recover` + threshold check,
asserts `contract_chain_id == host chain id`, and transient-allows the input for that eval only —
no persistent ACL is created (DD-007: verify ≠ allow).

Two deliberate choices worth knowing the why of:

- **Coprocessors keep their EVM secp256k1/EIP-712 keys** — verifying EIP-712 on Solana is cheap,
  and issuing coprocessors Solana-native keys would double key-management for an input path that
  the transciphering RFC (tech-spec#457) is expected to replace. Decision June 2026.
- **The standalone `verify_coprocessor_input` instruction and `InputVerifiedEvent` were removed**
  — verification is inline-only, matching EVM's "verifyInput grants transient only" shape. A
  regression test pins this (`plan_contracts.rs`).

Status: **MET**, with **SCOPE/PRODUCT-OPEN** residue (the external proof service behind the
attestation, `rejectProofResponse`) and one production blocker: the signer set is a single
hardcoded signer at threshold 1 (fragility #1, fhevm-internal#1608).

Code: `zama-host/src/instructions/input_verification.rs`, `eip712.rs`;
SDK `sdk/js-sdk/src/core/coprocessor/SolanaZkProof-p.ts`.

## 4. `fhe_eval` & FHE ops (+ HCU)

**EVM:** `FHEVMExecutor` — one op per external call, gas + `HCULimit` for per-op/tx/block metering.
**Solana:** `fhe_eval` is a bounded batched evaluator (`MAX_FHE_EVAL_OPS=16`, mixed
binary/ternary/trivial-encrypt/rand/bounded-rand steps + `VerifiedInput` operands + `AllowedLocal` transients)
— Solana-native batching that respects the CPI depth budget (constraint 4). Compute identity is an
explicit `compute_subject: Signer` (constraint 1). Compute-producing host work is routed through
`fhe_eval`; ACL, material, KMS, and admin endpoints remain separate non-compute instructions.

| Capability | Status |
|---|---|
| Add / Sub / Ge / IfThenElse(select) / Rand / RandBounded / trivialEncrypt / fromExternal | **MET** — exactly ERC7984's op footprint; the token is **op-complete** |
| Remaining opcode catalog (Mul, Div, bitwise, shifts, Eq/Ne/Gt/…, casts, Sum/IsIn) | **SCOPE** — mechanically extensible: enum + per-op type-gate row + handle hash (already op-parametrized) + coprocessor map arm + tests |
| operand ACL + scalar rule | **MET** — `ACL_ROLE_USE` per encrypted operand; scalar RHS rejects a permission witness |
| `FheType` breadth (86 variants) | **MET (partial)** — Bool/Uint8..Uint256; signed/large/string types are SCOPE |
| `HCULimit` per-op/tx/block/depth metering | **DIVERGENCE** — per-plan caps (`HostConfig::max_hcu_per_tx`, `max_hcu_depth_per_tx`, 0 = off) + Solana compute budget; **no per-block plane** (fragility #3) |

HCU framing from the guild (June 2026): HCU is fundamentally *metering* for future
pay-per-computation; the limit is a DoS stopgap. Per-instruction limits were chosen over
per-transaction; a per-block budget is under design — a naive global counter would serialize the
chain to ~85 TPS through one hot account, so the direction is per-app budgets (ledger item).

Code: `zama-host/src/instructions/fhe_eval.rs` + `fhe_eval/{admission,event_budget,event_transport,handles,hcu,preflight,walk}.rs`.
Tests: `host_mollusk.rs`, `plan_contracts.rs`.

## 5. Confidential token (ERC7984 → `confidential-token`)

A standalone program (not a Token-2022 extension — the ZK confidential-transfer extension is
unadopted and cryptographically incompatible; guild decision June 2026), modeled as a wrap of a
real SPL mint. Minting is SPL escrow + trivial-encrypt; burning releases the underlying via a
KMS-cert-gated redeem leg.

| ERC7984 capability | Solana equivalent | Status |
|---|---|---|
| `confidentialTransfer(to, euint64)` | `confidential_transfer(amount_handle)` — owner-signed; rotates both balance handles + births output ACL records | **MET** |
| `confidentialTransfer(to, external, proof)` | `confidential_transfer(amount_attestation)` — attestation consumed inline as `VerifiedInput` | **MET** (DD-007) |
| `confidentialTransferFrom` (×2, operator) | — | **INTENTIONAL GAP** — owner-signed paths only; see [Do not assume](#porting-from-erc7984-do-not-assume) |
| `confidentialTransferAndCall` (×4) | app-driven CPI composition ([§6](#6-cpi-composition--confidential-deposit-app)) | **INTENTIONAL GAP** (DD-011) |
| `setOperator` / `isOperator` | — | **INTENTIONAL GAP** — delegation semantics = open ledger item |
| `requestDiscloseEncryptedAmount` | `request_disclose_{balance,amount}` → `allow_for_decryption`; label-scoped (DD-010) | **MET** |
| `discloseEncryptedAmount(cleartext, proof)` | `disclose_{balance,amount}_secp` — ACL + material + `public_decrypt` + on-chain secp256k1 KMS cert against witness-pinned `kms_context`, consume-once | **MET** (DD-021/DD-022) |
| `confidentialTotalSupply` / `confidentialBalanceOf` / metadata views | account reads (`Mint` supply handle, `ConfidentialTokenAccount.balance_handle`) | **DIVERGENCE** (account reads) |
| `_mint` | `initialize_mint` + `wrap_usdc` (SPL escrow → trivial-encrypt → add) | **DIVERGENCE** (wrap model) |
| `_burn` | `confidential_burn` + `redeem_burned_amount_secp` (KMS-cert-gated underlying release) | **MET** + Solana-only redeem leg |
| `_update` safe-math | debit path `ge → sub → select` reproduced; credit/supply-increase plain `add` | **PARTIAL** — see [Do not assume](#porting-from-erc7984-do-not-assume) |
| `FHE.allow / allowThis` | `*_and_bind` producer paths + `allow_acl_subjects` | **MET** |
| self-transfer no-op | no handle rotation when from==to (RFC 024) | **MET** |
| token events | token-local + host events; no operator events | **PARTIAL** (DD-003) |

Persistence discipline: a transfer persists only the final balance handles and the
transferred-amount ACL record — intermediates stay transient (DD-019).

Code: `programs/confidential-token/src/instructions/`, `src/fhe.rs`, `src/state/`.
Tests: `runtime-tests/tests/token_mollusk.rs`.

## 6. CPI composition (→ `confidential-deposit-app`)

The replacement for `confidentialTransferAndCall`, shown as a working reference: a vault-style
`deposit` instruction that CPIs `confidential_token::confidential_transfer` with the depositor's
one signature propagating through — the transfer and the app's own bookkeeping are atomic in one
transaction, and the receiver *is* the caller, so the EVM callback's success-bit provenance
problem cannot arise (that provenance gap was OZ's headline finding on the earlier ported callback
flow, fhevm-internal#1591; the flow was removed by #2953).

Code: `programs/confidential-deposit-app/src/lib.rs`. Status: **DD-011 realized**; exercised by
the e2e vertical, no dedicated Mollusk suite yet.

## 7. Decrypt paths

**User decrypt — MET, via the real Gateway.** Solana is a registered Gateway host chain (bytes32
ACL id, high-bit chain id). User decrypts route through the unified Gateway V2 path with a typed
`userDecryptionRequestSolana` entrypoint (DD-012 — a reversal: an earlier native-v0 Solana-only
stack was built, then abandoned for Gateway reuse; its residual connector code is dead scaffolding
to retire). The user-facing signature is **native ed25519** (a Solana wallet signature, not
EIP-712) — re-verified per-party by the kms-connector; the KMS *return* path stays
EIP-712/secp256k1 so KMS nodes need no EdDSA support. Delegation is a PDA per
`(delegator, delegate, app)` with slot expiry; wildcard = sentinel app-context (RFC-017 parity).

**Public decrypt — MET, verified on-chain.** `request_disclose_*` creates a `DisclosureRequest`
witness PDA; `disclose_*_secp` consumes a KMS `PublicDecryptVerification` cert with full on-chain
secp256k1 threshold verification against the witness-pinned `KmsContext` singleton (DD-020/DD-021/
DD-022 — witness created before the consume; consume-once; context rotation rejects stale certs).
Same KMS trust root as EVM's `KMSVerifier`, verified with the same rigor.

Code: `zama-host/src/instructions/{allow_for_decryption,define_kms_context,delegate_for_user_decryption}.rs`,
`eip712.rs::verify_kms_public_decrypt`; SDK `sdk/js-sdk/src/solana/actions/userDecrypt.ts`,
`core/coprocessor/SolanaUserDecrypt-p.ts`; connector
`kms-worker/src/core/{event_processor/solana_user_decrypt.rs,solana_v2_fetcher.rs,solana_acl.rs}`.
Adversarial live checks: `scripts/poc/adversarial-l4.sh` (signature-substitution and
context-rotation rejection).

## 8. Event transport, listener & finality

**EVM:** logs are a trusted, durable part of execution; the listener just subscribes.
**Solana:** log delivery is provider-dependent and `emit!` is droppable, so **events are indexing
hints, never authorization** (DD-003), and the listener can rebuild every compute/ACL event from
instruction data alone:

- `solana_listener.rs` — RPC-polling transport; `solana_grpc_listener.rs` — Yellowstone gRPC
  transport (the production direction; guild decision June 2026, validated by a PoC that tracked
  CPI + PDA creation with zero emitted events); both feed the shared decoder in
  `solana_adapter.rs`.
- `solana_reconstruct.rs` — the emitless path: rebuilds events from decoded instructions + block
  context (`solana_slot_hashes.rs` streams the SlotHashes sysvar for off-chain handle
  recomputation). The e2e suite runs one leg with an EMITLESS build of `zama-host` to prove
  reconstruction is a complete event source.
- Audience split (decided, details open): Zama's own ingestion uses Yellowstone; a lean
  `emit_cpi` surface stays for third-party indexers who won't run Geyser. Which events are
  public-facing is an open ledger item.

**Finality:** decrypts must only be served for state a finalized slot committed.
`solana_finalized_account_fetcher.rs` re-fetches account state at `finalized` before enqueueing
decrypts (DD-024). The big open question — *where the finality gate sits* and wiring the Solana
poller into the EVM reorg substrate — is DD-025 / fragility #5. `drift_revert` is poller drift,
not an on-chain reorg (DD-029).

Status: **DIVERGENCE** by design (DD-003) + **PRODUCT-OPEN** (production Geyser provider, reorg
wiring).

## 9. Admin, config & governance

`HostConfig` singleton: authorities, chain id, pause, HCU caps, feature flags
(`initialize_host_config`, `set_host_pause`, `set_max_hcu_*`). Divergences from EVM:
single admin signer (no 2-step ownership), no UUPS — Solana programs upgrade natively through the
BPF loader, so no proxy pattern exists or is needed. Production governance (a Squads multisig vs.
the single Ethereum DAO used for EVM chains; whether Squads supports a Safe-style admin-module
override) is a dedicated RFC in progress — ledger item. `ProtocolConfig` / `KMSGeneration` /
`PauserSet` registry equivalents: **PRODUCT-OPEN** (no KMS keygen is needed on Solana; keys are
referenced by `key_id` only).

## 10. Gateway registry & payment

`GatewayConfig` (registry) is read off-chain by the connector; `HostConfig` carries the
Solana-side subset (authorities, coprocessor input signer, current `KmsContext`).
`ProtocolPayment` ($ZAMA fees): none — rent and tx fees only. Both **PRODUCT-OPEN**.
Kept name: the gateway entrypoint stays `verifyProofRequestSolana` (DD-030, decided after debate).

## 11. Cross-repo ABI pinning (load-bearing, easy to miss)

Connector account decoders and the coprocessor's vendored IDL are **hand-mirrored against
`zama-host/src/state/` with no compile-time link**. Guards: `scripts/check_solana_abi.py` (ABI
golden manifest), `check-zama-host-idl.sh` (IDL drift), `EVENT_VERSION` pins, and DD-004's
negative-test discipline. A same-length field reorder would still not be caught at build time —
treat any state-layout change as a cross-repo ABI break: update program checks, connector
decoders, listener constants, IDL/golden files, and docs together (see `TESTING.md` traps).

## 12. Testing & e2e surface

Automated: Mollusk suites `runtime-tests/tests/{host_mollusk,token_mollusk,plan_contracts}.rs`
(the negative-test suite is the enforcement arm of DD-004). Live vertical: `scripts/poc/`
(`clean-e2e.sh`, `full-vertical.sh` — CI-wired via `solana-e2e.yml`, including the EMITLESS
reconstruction leg; `adversarial-l4.sh` — manual-only). Details: [`TESTING.md`](./TESTING.md).

---

## Critical assessment

### Solid (faithful, often stricter than EVM)

The ACL core (explicit grant role, append-only, canonical PDAs), public-decrypt as a post-birth
release, delegation lifecycle, preserved handle byte-layout, operand-ACL discipline + scalar rule,
ABI/account-meta exactness with extensive negative tests, the owner-authorized token flows with
label-scoped disclosure, on-chain secp256k1 verification of both coprocessor attestations and KMS
certs (same trust roots as EVM), and the connector's canonical-PDA + material-binding checks.

### Fragile — attention for a security pass

PoC shortcuts, not correctness bugs; each maps to a PRODUCT-OPEN item or tracked issue.

1. **Single coprocessor input signer at threshold 1** (`HostConfig::coprocessor_signer`); the
   threshold machinery exists but is unused for inputs. → registered n-of-m set,
   fhevm-internal#1608.
2. **`poc`-gated test shims** are compiled out of production; the zero-birth-entropy fallback is
   additionally chain-id-confined (DD-014). Residual risk is build discipline only.
3. **No per-block HCU plane** — per-plan caps + compute budget only; per-app block budget under
   design (ledger).
4. **Disclosure/redemption off-chain wiring** — on-chain verification is complete; a deployed
   finalized-fetcher and KMS cert publication path still need production wiring.
5. **Solana poller not wired into the EVM reorg substrate** (DD-025) — polls at `confirmed`,
   inserts directly. Reorg correctness is the biggest open architectural gap; native-v0 residue to
   retire with it.
6. **Hand-mirrored cross-repo ABI** ([§11](#11-cross-repo-abi-pinning-load-bearing-easy-to-miss)).
7. **Material commitment is one-shot with single-authority blast radius** — a wrong seal
   permanently bricks a handle's decryptability (EVM has consensus in front of the same
   irreversibility).
8. **ACL storage cost** (~$1–2/transfer) is production-rejected — successor design tracked, not
   yet built (see [§1](#1-acl--handles)).

## Open decisions ledger

Questions this document deliberately does not answer, with owners (as of July 2026):

| Question | Direction so far | Owner / tracker |
|---|---|---|
| ACL storage successor (encrypted-value "current" + MMR history) | specified, PoC pending | Elias — fhevm-internal#1569, RFC-024 |
| Operator/approve/delegation semantics for multi-transfer flows | OZ retiring `setOperator` toward "confidential approve" on EVM; Solana analog to be co-designed | joint Zama/OZ session; Roman, Joseph-André |
| Historical-access semantics (does "current" grant history? revocable?) | punted to product | Arik |
| User-decrypt signing UX (raw sign vs SIWS envelope) | SIWS most promising, undecided | product + SDK |
| App-context identity (which pubkey stands in for `msg.sender` per app) | mint/market/vault PDA per app, opaque to KMS | design thread, tech-spec#448 |
| Per-app HCU block budget | avoids the hot-account TPS ceiling of a global counter | NikitaK, PR pending |
| Which events are public (`emit_cpi`) vs internal (Yellowstone) | keep both, split by audience | guild |
| Governance (Squads vs cross-chain DAO, admin-module override) | dedicated RFC | Joseph-André |
| Finality-gate placement / reorg substrate wiring | DD-025 (BIG OPEN) | host-listener owners |
| Transciphering input flow (replaces ZKPoK + attestations) | tech-spec#457 — track, don't wait | Panos; Nico reviewing |

## Net verdict

- **FHEVM host authorization model** (ACL, executor authorization, input verification, transient,
  decrypt release, delegation, material commitment): **faithfully ported, frequently stricter**.
- **Confidential token:** intentionally **not fully ported** — owner-signed transfer, wrap/burn/
  redeem, and disclosure cover the PoC surface; operator APIs and transfer-and-call are deliberate
  gaps replaced by Solana-idiomatic composition.
- **Every divergence is constraint-driven and documented**; nothing diverges silently.
- **SCOPE** items (opcode breadth, full type set) are mechanical extensions, not Solana limits.
- **PRODUCT-OPEN** items are the PoC→production boundary: registries, payment, signer sets,
  production event transport, reorg wiring — each tracked above.
