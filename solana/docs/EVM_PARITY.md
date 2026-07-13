# EVM → Solana Parity Note

This note maps each EVM-side capability of the Zama confidential-token + FHEVM host design to
its Solana equivalent in this PoC, or records a justified divergence. It is grounded in direct
code review of `ERC7984.sol`, `host-contracts/`, `gateway-contracts/`, and the Solana programs +
adapters (`fhevm/solana/programs/{zama-host,confidential-token,confidential-deposit-app}`,
`fhevm/coprocessor/.../host-listener`, `fhevm/kms-connector`).

Legend:

- **MET** — Solana faithfully implements the EVM semantics (sometimes stricter).
- **DIVERGENCE** — re-expressed differently because of a Solana platform constraint (account model,
  no `msg.sender`, no transient storage, no `ecrecover`/EIP-712, shallow CPI stack, compute-budget
  vs gas/HCU). The intent is preserved.
- **SCOPE** — deliberately not implemented in this PoC; not Solana-constrained, mechanically
  extensible; not required by the confidential token.
- **PRODUCT-OPEN** — belongs to an off-chain service or a future program by design (KMS Core,
  Gateway registry, keygen, payment, live transport).

Design rationale for the divergences below is recorded in
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (DD-001..DD-035) and RFC 024.

---

## 1. ERC7984 confidential token → `confidential-token`

| ERC7984 capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `confidentialTransfer(to, euint64)` | holder transfers an already-allowed handle; `require isAllowed(amount, sender)` | `confidential_transfer(amount_handle)` — owner-signed; owner-scoped amount ACL; rotates sender/recipient balance handles + births output ACL records | **MET** |
| `confidentialTransfer(to, externalEuint64, inputProof)` | transfer a freshly verified external input | `confidential_transfer(amount_attestation)` — the attestation is consumed inline as the `fhe_eval` `FheEvalOperand::VerifiedInput` operand (the Solana `FHE.fromExternal` analog): verified in-frame, transient-allowed, no persistent input ACL | **MET** (DD-007: verify ≠ allow; caller-is-contract enforced via `attestation.contract_address == compute_subject`) |
| `confidentialTransferFrom(from,to,euint64)` | operator transfer; `require isOperator` + `isAllowed`; `allowTransient(transferred, sender)` | no Solana production equivalent | **INTENTIONAL GAP** — operator/delegated transfer APIs were removed to simplify authority and reduce attack surface |
| `confidentialTransferFrom(from,to,externalEuint64,proof)` | operator transfer of external input | no Solana production equivalent | **INTENTIONAL GAP** — the owner-authorized `confidential_transfer(amount_attestation)` fromExternal path is the supported route |
| `confidentialTransferAndCall(...)` ×4 | transfer then call `onConfidentialTransferReceived`; refund `select(success,0,sent)`; `transferred = sent - refund`; transient-allow to sender | not ported; a receiving app exposes its own instruction that CPIs `confidential_transfer` with the user as sole signer (see `confidential-deposit-app`) | **INTENTIONAL GAP** (DD-011) — the callback is an EVM workaround for contracts not observing incoming transfers; Solana propagates signer authority through CPI. Token-2022 transfer hooks are rejected as a substitute (veto-only, not receiver callbacks) |
| `setOperator(operator, until)` | time-bounded operator approval (`uint48` deadline); `OperatorSet` event | no Solana production equivalent | **INTENTIONAL GAP** — no operator rows or operator events |
| `isOperator(holder, spender)` | `holder==spender \|\| now <= until` | no Solana production equivalent | **INTENTIONAL GAP** — holder self-authority is handled by owner-signed paths |
| `requestDiscloseEncryptedAmount(euint64)` | `makePubliclyDecryptable` + event; `require isAllowed(amount, sender)` | `request_disclose_balance` / `request_disclose_amount` → host `make_handle_public` (seals a `PublicDecryptLeaf` into the `EncryptedValue` MMR, DD-032); emits `*DisclosureRequestedEvent` | **MET** (DD-010: label-scoped — balance vs amount paths distinct) |
| `discloseEncryptedAmount(euint64, cleartext, proof)` | `FHE.checkSignatures` + `AmountDisclosed` event | `disclose_balance_secp` / `disclose_amount_secp(cleartext, proof)` — authorize the **witness-pinned** handle by its `EncryptedValue` public-decrypt MMR proof (the sealed leaf is permanent, so a disclosure survives its lineage being superseded during the KMS round-trip — never reads the live handle, DD-036) + gateway `CiphertextCommits` materiality (DD-031) + on-chain secp256k1 KMS-cert verification against the witness-pinned `kms_context` (consume-once) | **MET** (DD-021/DD-022: mirrors EVM `KMSVerifier` — same threshold cert verifies on both sides) |
| `confidentialTotalSupply()` view | encrypted total supply handle | one stable total-supply `EncryptedValue` lineage (born in `initialize_mint`; superseded by `confidential_burn`, DD-032) — read off-chain | **MET** (DIVERGENCE: account read, not a view call) |
| `confidentialBalanceOf(account)` view | encrypted balance handle | `ConfidentialTokenAccount.balance_encrypted_value` points to the stable `EncryptedValue` lineage whose `current_handle` is read off-chain | **MET** (DIVERGENCE: account read) |
| `name/symbol/decimals(=6)/contractURI` views | metadata | mint/app config; `wrap_usdc` ties decimals to the underlying SPL mint | **DIVERGENCE** (app config / off-chain reads) |
| `_mint(to, amount)` | increase total supply + credit | `initialize_mint` (total-supply lineage birth) + `wrap_usdc` (escrow SPL USDC → trivial-encrypt → add to balance) + `initialize_token_account` (zero-balance lineage birth) | **DIVERGENCE** — minting modeled as SPL wrapping (RFC 024 wrap flow; real SPL-token boundary) |
| `_burn(from, amount)` | decrease balance + total supply | `confidential_burn` (supersede balance + total-supply lineages) + `redeem_burned_amount_secp` through a `request_burn_redemption` witness (release underlying from vault — KMS-cert gated, public-decrypt gated) | **MET** + Solana-only redeem leg (no ERC7984 analogue; wrapper needs underlying release) |
| `_update` safe-math (`tryIncrease`/`tryDecrease`, `select`, allow/allowThis) | overflow/underflow-safe FHE balance update, conditional transfer of `select(success, amount, 0)` | `confidential-token/src/fhe.rs` + `fhe_eval` Binary/Ternary steps (add/sub/ge/select — the standalone `fhe_binary_op*`/`fhe_ternary_op*` instructions were removed, DD-032) + output binding into `EncryptedValue` | **PARTIAL** — `tryDecrease` reproduced (transfer/burn debit: `ge` → `sub` → `select(success, candidate, balance)`); `tryIncrease` NOT reproduced — the wrap/mint total-supply increase and the recipient credit use a plain `add`. Total-supply overflow is instead bounded by the 1:1 SPL backing (a real `u64` mint), per the `_mint` DIVERGENCE above. Output ACL via authorized producer paths |
| `FHE.allow / allowThis` | durable ACL grant to user / contract | output-binding producer paths (create or supersede an `EncryptedValue` with subjects, DD-032) + `allow_subjects` (append) | **MET** |
| `FHE.allowTransient(transferred, sender)` | transaction-local grant | instruction-local `AllowedLocal` value within one `fhe_eval`, plus CPI signer propagation within one instruction | **DIVERGENCE** (no tstore; DD-008) |
| `ConfidentialTransfer` / `OperatorSet` / `AmountDisclose(d/Requested)` events | indexing | token-local events; the `EncryptedValue` ACL lifecycle itself emits no events by design (DD-033), reconstructed instead by instruction-replay indexing; no operator events | **PARTIAL** (DD-003: events are indexing hints, not authorization) |
| self-transfer | (EVM updates regardless) | `confidential_transfer` no-op when from==to (no handle rotation, no output ACL) | **MET** (RFC 024 explicit; avoids useless historical handles) |

ERC7984's FHE op footprint is exactly **add / sub / compare(ge) / select / trivial-encrypt / rand /
fromExternal** — all implemented. The confidential token is therefore **op-complete**.

---

## 2. FHEVM host (`host-contracts/`) → `zama-host`

| EVM unit | Capability | Solana equivalent | Status |
|---|---|---|---|
| `ACL.allow(handle, account)` | persistent grant; caller must be allowed | `allow_subjects` — authority `Signer` must already be in the `EncryptedValue.subjects` allowed set + canonical PDA + deny-list + pause; append-only and idempotent, capped at `MAX_ENCRYPTED_VALUE_SUBJECTS=8` (DD-032) | **MET** |
| `ACL.allowForDecryption(handles[])` | mark publicly decryptable | `make_handle_public` — any allowed subject can seal an exact-handle `PublicDecryptLeaf` into the MMR; that handle's publicness is then **permanent** and survives a later durable-output supersession (append-only leaf, the KMS honors historical public leaves), while staying exact-handle scoped so a superseding handle is not itself public (DD-005's live flag superseded, DD-032) | **MET** (DIVERGENCE: per-handle) |
| `ACL.allowTransient(handle, account)` | tx-local grant | no durable analog; instruction-local `AllowedLocal` within one `fhe_eval`, plus CPI signer propagation within one instruction | **DIVERGENCE** (no tstore; DD-008) |
| `ACL.isAllowed(handle, account)` | transient OR persistent | live path: canonical `EncryptedValue` PDA + `current_handle` match + subject membership; historical path: MMR `HistoricalAccessLeaf` inclusion proof against live confirmed peaks (DD-032) | **MET** |
| `ACL.isAllowedForDecryption(handle)` | public-decrypt flag | exact-handle `PublicDecryptLeaf` MMR inclusion proof (no live flag — DD-032) | **MET** (DIVERGENCE: proof, not a stored flag) |
| `ACL.persistAllowed` / `allowedTransient` | read pair | inline subject lookup on `EncryptedValue.subjects`; transient = instruction-local `AllowedLocal` | **MET** / **DIVERGENCE** |
| `ACL.cleanTransientStorage()` | wipe tx transient (AA bundling) | nothing to reclaim — `AllowedLocal` values are instruction-scoped and never persisted (the one-shot `TransientSession` account tier was removed, DD-008) | **DIVERGENCE** (no durable transient state) |
| `ACL.delegate/revokeForUserDecryption` | user-decrypt delegation lifecycle | `delegate_for_user_decryption` / `revoke_...` — PDA per `(delegator,delegate,app)`, slot-based expiry, same-slot double-update guard, wildcard-delegate rejected | **MET** |
| `ACL` deny list (`blockAccount`/`isAccountDenied`) | owner deny list | `set_deny_subject` + `DenySubjectRecord`, gated into grant paths | **MET** |
| `ACL` pause/owner/UUPS | pauser role, 2-step ownership, upgrade | single `admin` (`set_host_pause` etc.) | **DIVERGENCE** (admin signer) + **PRODUCT-OPEN** (BPF upgrade authority handles program upgrade) |
| `FHEVMExecutor.fheAdd/fheSub` | binary add/sub | `fhe_eval` Binary step, op=Add/Sub (the standalone `fhe_binary_op*` instructions were removed; `fhe_eval` is the only compute path, DD-032) | **MET** |
| `FHEVMExecutor.fheGe` | ≥ comparison → ebool | `fhe_eval` Binary step, op=Ge | **MET** |
| `FHEVMExecutor.fheIfThenElse(select)` | ternary; ebool control, branch type-checked | `fhe_eval` Ternary step, op=IfThenElse (the standalone `fhe_ternary_op*` instruction was removed, DD-032) | **MET** |
| `FHEVMExecutor.fheRand/fheRandBounded` | random / bounded random | `fhe_eval` Rand and RandBounded steps (pow2+≤max bound check); the standalone `fhe_rand_and_bind`/`fhe_rand_bounded_and_bind` instructions were removed | **MET** (DIVERGENCE: slot/bankhash+output-binding seed vs counterRand+blockhash) |
| `FHEVMExecutor.trivialEncrypt` | plaintext → ct handle | `fhe_eval` TrivialEncrypt step (the standalone `trivial_encrypt_and_bind` instruction was removed, DD-032) | **MET** |
| `FHEVMExecutor.verifyCiphertext` (input) | verify signed/proved input, allowTransient | `fhe_eval` `FheEvalOperand::VerifiedInput` operand — in-frame secp256k1 recover + threshold-check of the coprocessor EIP-712 `CiphertextVerification` attestation, asserts `contract_chain_id == host chain id`; transient-allowed, no persistent ACL | **MET** (DD-007: verify ≠ allow; same coprocessor EIP-712 trust root as EVM) |
| `FHEVMExecutor` batched/expression compute | — | `fhe_eval` (bounded mixed-step eval; `MAX_FHE_EVAL_OPS=16`) | **MET** (Solana-native batching to limit CPI depth; DD-008; mixed binary/ternary/unary/trivial-encrypt/rand/rand-bounded/sum/isIn/mulDiv steps with `AllowedLocal` transients, and `VerifiedInput` operands) |
| operand ACL + scalar rule | encrypted operand needs ACL; scalar exempt | `EncryptedValue` subject membership check per operand (DD-032); scalar RHS rejects a permission witness | **MET** (RFC 024 scalar rule) |
| handle byte-layout (ver/type/chainid/computed-marker) + entropy-seeded derivation | symbolic-exec handle; no per-output nonce (only `fheRand`'s global `counterRand`, folded into the rand seed) | identical layout; `hash(domain, op, operands, programID, chainid, prev_bank_hash, ts, context_id, op_index)`. Durable outputs use the same base handle as local outputs; `value_key` remains only the `EncryptedValue` PDA seed (DD-015). | **MET** (DIVERGENCE: sha256/keccak vs keccak, bankhash vs blockhash → handles not cross-chain-interoperable) |
| compute identity = `msg.sender` | implicit caller | explicit `compute_subject: Signer` checked `is_signer && is_allowed` | **DIVERGENCE** (no `msg.sender`; RFC 024 compute-signer) |
| `FHEVMExecutor` op breadth: Mul/Div/Rem/BitAnd/Or/Xor/Shl/Shr/Rotl/Rotr/Eq/Ne/Gt/Lt/Le/Min/Max, Neg/Not, cast, Sum/IsIn, MulDiv | full opcode catalog | implemented in `fhe_eval` as the binary catalog, unary ops, `Sum`, `IsIn`, and `MulDiv`; constrained by the supported `FheType` set below | **MET** for the modeled operator surface; remaining breadth is type support, not dispatch shape |
| `HCULimit` (per-op/tx/block/depth homomorphic-compute caps) | gas-like metering | `HostConfig::max_hcu_per_tx` / `max_hcu_depth_per_tx` summed over an `fhe_eval` plan (`0` = off), plus Solana compute-budget + op-count/collection caps | **DIVERGENCE** (per-plan cap + compute-budget vs per-block metering) — see fragility #3 |
| `KMSVerifier` (on-chain decrypt-sig threshold verify) | verify KMS sigs on-chain | on-chain secp256k1: `eip712::verify_kms_public_decrypt` recovers EVM KMS signers and threshold-checks them against the witness-pinned `KmsContext` (rejects high-s) | **MET** (DD-021: mirrors EVM `KMSVerifier`) |
| `ProtocolConfig` / `KMSGeneration` / `PauserSet` (role set) | KMS node/threshold registry, keygen, pauser role set | none (subset in `HostConfig`: authorities/chain_id/flags) | **PRODUCT-OPEN** |
| `FheType` (86 variants) | type enum | supported set Bool/Uint8..Uint256 (covers token + shipped ops) | **MET (partial)** / **SCOPE** (signed/large/string types) |

---

## 3. Gateway / KMS (`gateway-contracts/`) → `zama-host` + `kms-connector` + token disclose

| EVM gateway capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `Decryption.publicDecryptionRequest/Response` | request + threshold-consensus public decrypt | token `request_disclose_*` (sets `public_decrypt`, creates a `DisclosureRequest` witness PDA); on-chain `disclose_*_secp` consumes a secp256k1 KMS cert verified against the witness-pinned `KmsContext` | **MET** (DD-012/DD-021/DD-022: same EVM KMS trust model, verified on-chain) |
| `Decryption.userDecryptionRequest` + EIP-712 | user-signed, contract-scoped, validity window | routed through the unified Gateway V2 path via the typed `userDecryptionRequestSolana(HandleEntry[], UserDecryptionRequestSolanaPayload)` entrypoint; chain-aware validator branches on `contracts_chain_id` | **MET** (DD-012/DD-026/DD-027: reuses the Gateway/EVM stack, typed Solana fields) |
| `Decryption.delegatedUserDecryptionRequest` + RFC-017 wildcard | delegate-signed; wildcard contract scope | `UserDecryptionDelegation` PDA per `(delegator,delegate,app)`, slot expiry, wildcard = `[0xff;32]` app-context sentinel; carried in the same Gateway V2 Solana user-decrypt payload | **MET** (semantics) / **DIVERGENCE** (PDA mechanism) |
| `Decryption.userDecryptionResponse` (per-share sigs → threshold) | threshold response | Gateway V2 response path; connector verifies the KMS threshold response | **MET** (DD-012) |
| `checkDecryptionReady` (material added) | all handles have ciphertext material | the host-owned `HandleMaterialCommitment` subsystem was deleted; materiality is checked against the gateway's `CiphertextCommits`, where the coprocessor already registers Solana handles (DD-031) | **DIVERGENCE** (moved off host-chain state entirely, not re-modeled on Solana) |
| `CiphertextCommits.addCiphertextMaterial` | multi-coprocessor consensus adds (keyId, ctDigest, snsDigest) | same `CiphertextCommits` contract as EVM — no Solana-side host equivalent; `commit_handle_material` and `HandleMaterialCommitment` were deleted (DD-031) | **MET** (shared with EVM, not re-implemented on Solana) |
| `checkCiphertextMaterial` | material-present check | same `CiphertextCommits` check as EVM (DD-031) | **MET** (shared with EVM) |
| `InputVerification.verifyProofRequest/Response` (ZKPoK consensus) + `FHEVMExecutor.verifyInput` (tx-scoped transient allow, no persistent ACL) | coprocessor ZK-proof verify + consensus + EIP-712; verifyInput grants only a transient allow | `FheEvalOperand::VerifiedInput` (the `FHE.fromExternal` analog) — consuming it in `fhe_eval` does on-chain secp256k1 recover + threshold-check of the coprocessor EIP-712 `CiphertextVerification` attestation and asserts `contract_chain_id == host chain id`; transient-allows the input for that eval, creates **no persistent ACL** (DD-007) | **MET** (parity with `verifyInput`: verify ≠ allow; Solana has no transient store so durable perms are a separate explicit grant) + partial **SCOPE/PRODUCT-OPEN** (external proof/transciphering service, no `rejectProofResponse`) |
| `HandleOps`/`FHETypeBitSizes` | parse chainId/fheType; bit-size table | mirrored exactly in connector (`solana_native_handle_chain_id`, `*_fhe_type_encrypted_bits`) | **MET** |
| `MAX_DECRYPTION_REQUEST_BITS=2048` | per-request cleartext cap | `SOLANA_NATIVE_MAX_ENCRYPTED_BITS_PER_REQUEST=2048` enforced | **MET** |
| `Structs` (Sns material, delegation, pairs) | cross-contract DTOs | witness structs + on-chain `EncryptedValue`/`UserDecryptionDelegation` (material DTOs are gateway-side `CiphertextCommits` structs, unchanged by Solana, DD-031) | **MET** (re-modeled) |
| unified decrypt routing | (Solana as a Gateway host chain) | Solana is registered as a host chain (bytes32 ACL = the `zama_host` program id, high-bit chain id); decrypt reuses the Gateway V2 path rather than a parallel native stack. Residual `native-v0` connector library/store code exists but is not the chosen path (DD-012) | **MET** (unified) / superseded native-v0 subsystem |
| `GatewayConfig` (KMS/coprocessor/host-chain/threshold registry) | on-chain registry | connector reads registry off-chain (`Config`); `HostConfig` holds authorities/flags, the single coprocessor input signer, and the current `KmsContext` pointer | **PRODUCT-OPEN** |
| `KMSGeneration` (keygen/crsgen ceremony) | key/CRS lifecycle | none (referenced only by `key_id`) | **PRODUCT-OPEN** |
| `ProtocolPayment` ($ZAMA fees) | per-request fee | none (rent/tx fees only) | **PRODUCT-OPEN** |

---

## 4. Coprocessor / KMS adapter integration

- **Coprocessor host-listener** (`coprocessor/.../host-listener/src/solana_adapter.rs` +
  `solana_reconstruct.rs`): reconstructs compute rows and handle-only ciphertext-material requests
  from confirmed Yellowstone transaction instructions plus streamed Clock/SlotHashes state. Create,
  update, make-public, and durable outputs carry the concrete handle(s) to prepare. Subject grants and
  removals emit no material request because material was prepared at handle birth; KMS checks the live
  ACL before plaintext release (DD-025/DD-033/DD-034). `cargo check -p host-listener` → exit 0.
  The adapter maps the merged `fhe_eval` operator surface: binary catalog, ternary select, trivial,
  rand/rand-bounded, unary, sum, isIn, and mulDiv.
- **Relayer MMR proof service** (`relayer/src/solana_proof`): untrusted, relayer-colocated
  instruction-replay ingestion + MMR proof builder for historical/public decrypt (DD-035). Interim
  internal HTTP endpoint (`GET /internal/solana/mmr-proof`); the relayer's own Solana user-decrypt flow
  does not yet call it in-process.
- **KMS connector** (`kms-connector/crates/kms-worker/src/core/solana_*.rs` and
  `kms-connector/crates/tx-sender/src/core/solana_native.rs`): witness decoders +
  `SolanaAclVerifier`, now dual-path against `EncryptedValue` (live confirmed-account read, or an MMR
  inclusion proof re-verified against live confirmed peaks via the shared `zama_solana_acl` crate,
  DD-032). Decrypt reuses the unified Gateway V2 path with a typed Solana user-decrypt entrypoint and
  on-chain secp256k1 cert verification (DD-012/DD-021/DD-026); the older `native-v0` request/response
  subsystem remains as library/store code but is not the chosen path and is not wired into the
  worker/tx-sender binaries. `HandleMaterialCommitmentWitness` is deleted (DD-031). Account layouts are
  hand-mirrored 1:1 against `zama-host/src/state/` (verified) but **version-pinned / no compile-time
  link** — a layout/seed/hash-domain/EVENT_VERSION change in zama-host requires editing the connector
  decoder, bumping listener event constants, and regenerating the coprocessor IDLs/ABI golden manifest.

**Adapters are present and integrated at the PoC boundary.** Live transport (production Geyser
provider, full KMS-connector wiring beyond the harness, optional reorg resource recovery, and calling
the MMR proof service in-process from the relayer's user-decrypt flow) is PRODUCT-OPEN by design
(DD-024/DD-025/DD-028/DD-035).

---

## 5. Critical assessment — solid vs fragile

**Solid (faithful, often stricter than EVM):** the ACL core (`allow`/`isAllowed`/grant authority/
append-only/no-generic-bind, now on stable `EncryptedValue` lineages with an MMR history, DD-032),
public-decrypt release gated on a dedicated role and never at birth (an exact-handle proof, not a live
flag), delegation lifecycle (slot expiry, same-slot guard, wildcard-delegate rejection), one-shot
transient capabilities with same-tx creation proof, preserved handle byte-layout,
operand-ACL discipline + scalar-RHS rule, ABI/account-meta exactness (DD-004, extensive negative
tests), the confidential-token flows (owner-authorized transfer/transfer-and-call split/wrap/burn/redeem/
disclose) with separate payer semantics and label-scoped disclosure (DD-010), and the
connector's canonical-PDA + MMR-proof verification (DD-032; materiality now lives in the gateway's
`CiphertextCommits`, DD-031).

**Fragile / attention for a security pass (PoC shortcuts, not correctness bugs):**

1. **Input verification trusts a single coprocessor signer at threshold 1** (`HostConfig::coprocessor_signer`),
   not yet a registered n-of-m set — the threshold machinery (`eip712::verify_threshold`) exists but
   input verification uses the single-signer path. Remaining work is the registered signer set
   (FUTURE_DESIGN §1) and the real proof/transciphering service behind the attestation.
2. **Test/mock bypass controls** (`set_test_shims_enabled`, `set_mock_input_enabled`) are `#[cfg(feature = "poc")]`
   — compiled out of default/production builds. The surviving state relaxation, the zero birth-entropy
   fallback, is additionally confined to `SOLANA_POC_CHAIN_ID` via `HostConfig::zero_birth_entropy_allowed`,
   so it cannot weaken birth entropy on a deployed chain regardless of flags (DD-014). The former
   `mock_input_verified_and_bind` input short-circuit and the event-only `test_emit_*` instructions
   were removed entirely.
3. **No per-block HCU / complexity metering.** The host caps total and critical-path HCU per
   `fhe_eval` plan (`HostConfig::max_hcu_per_tx` / `max_hcu_depth_per_tx`, `0` = off) plus the Solana
   compute budget, but there is no EVM-style per-block `HCULimit` plane. Relevant to DoS/cost-bounding.
4. **On-chain disclosure/redemption uses secp256k1 KMS-cert verification + request accounts.**
   The deleted coprocessor request-witness store had no consumer. The residual risk is off-chain
   integration of KMS certificate publication before the flow is end-to-end production ready.
5. **The confirmed Yellowstone listener is not wired into the EVM reorg substrate**
   (DD-025/DD-028): it reconstructs instruction effects and inserts directly, bypassing the
   block-status machine. This is accepted for authorization because KMS revalidates confirmed live
   state before release; optional reorg unwind remains resource-recovery work. The residual
   `native-v0` connector code is dead scaffolding to retire.
6. **Hand-mirrored, version-pinned ABI across repos with no compile-time link** (connector decoders +
   vendored coprocessor IDL). Lengths are checked, but a same-length field reorder in `EncryptedValue`
   would not be caught at build time. Event versions are now pinned by
   `solana/scripts/check_solana_abi.py` and `check-zama-host-idl.sh`; `MAX_ENCRYPTED_VALUE_SUBJECTS=8`
   remains a PoC capacity limit mirrored by off-chain decoders (subject-list overflow beyond 8 is
   deferred, DD-032).
7. **`previous_bank_hash` is fail-closed on real chains.** When the prior bank hash is unavailable,
   handle birth returns `PreviousBankHashUnavailable` rather than substituting a zero hash. The
   zero-hash fallback (intended only for local Mollusk bootstrap) is confined to `SOLANA_POC_CHAIN_ID` via
   `HostConfig::zero_birth_entropy_allowed`, so it cannot weaken birth entropy on a deployed chain
   (DD-014). Bank-hash + timestamp entropy is the resolved policy (keep per-block entropy, DD-015).
8. **Materiality is entirely off-chain-Solana now.** The host-owned `HandleMaterialCommitment`
   subsystem was deleted (DD-031); Solana handles rely on the same gateway `CiphertextCommits` the
   coprocessor already registers them into, so there is no Solana-side one-shot/irreversible seal to
   assess here anymore.

None of the fragilities are correctness defects in the implemented paths (the negative-test suite is
extensive); they are the expected boundary between a PoC and a production deployment, and every one
maps to an item tracked as PRODUCT-OPEN in `DESIGN_DECISIONS.md` / `FUTURE_DESIGN.md`.

---

## 6. Net parity verdict

- **Confidential token (ERC7984):** intentionally **not fully ported**. The owner-authorized
  transfer, wrap/burn/redeem, disclosure, and encrypted-balance flows cover the Solana PoC surface;
  ERC7984 operator/delegated-transfer APIs are deliberate gaps, transfer-and-call is replaced by
  app-driven CPI composition (DD-011), and Solidity view-style calls are account-read equivalents.
- **FHEVM host authorization model** (`EncryptedValue`+MMR ACL, executor authorization, input
  verification, transient, decrypt release, delegation): faithfully ported, frequently stricter.
  Materiality is no longer host-chain state on Solana — it is the gateway's `CiphertextCommits`,
  shared with EVM (DD-031).
- **Constraint-driven divergences** are all intentional and documented (no tstore, no `msg.sender`,
  shallow CPI, per-plan HCU cap vs per-block metering, account-witnesses vs storage reads). Input and
  KMS-cert verification use on-chain secp256k1 recovery — the same EIP-712 trust roots as EVM.
- **SCOPE** items (executor opcode breadth, full FheType set) are not Solana-limited and not needed
  by the token; mechanically extensible.
- **PRODUCT-OPEN** items (`ProtocolConfig`/`KMSGeneration`/`GatewayConfig`/payment, registered
  coprocessor signer set, production Geyser provider + reorg wiring, full KMS-connector wiring,
  external input proof service) are off-chain services or future work by design (DD-012, DD-003,
  DD-007, DD-025).
