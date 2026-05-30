# EVM → Solana Parity Note

This note maps each EVM-side capability of the Zama confidential-token + FHEVM host design to
its Solana equivalent in this PoC, or records a justified divergence. It is grounded in direct
code review of `ERC7984.sol`, `host-contracts/`, `gateway-contracts/`, and the Solana programs +
adapters (`fhevm/solana/programs/{zama-host,confidential-token,confidential-token-receiver}`,
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
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (DD-001..DD-013), RFC 024, and
[`TRANSIENT_ALLOW.md`](./TRANSIENT_ALLOW.md).

---

## 1. ERC7984 confidential token → `confidential-token`

| ERC7984 capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `confidentialTransfer(to, euint64)` | holder transfers an already-allowed handle; `require isAllowed(amount, sender)` | `confidential_transfer(amount_handle)` — owner-signed; owner-scoped amount ACL; rotates sender/recipient balance handles + births output ACL records | **MET** |
| `confidentialTransfer(to, externalEuint64, inputProof)` | transfer a freshly verified external input | `verify_input_and_bind` (host) → `confidential_transfer` | **DIVERGENCE** — input verification is a separate host instruction (no inline `FHE.fromExternal`; DD-007) |
| `confidentialTransferFrom(from,to,euint64)` | operator transfer; `require isOperator` + `isAllowed`; `allowTransient(transferred, sender)` | `confidential_transfer_from(amount_handle)` — active-operator check; **operator-scoped** amount ACL | **MET** (DD-009: amount input authority scoped to the operator, mirroring `msg.sender` binding) |
| `confidentialTransferFrom(from,to,externalEuint64,proof)` | operator transfer of external input | `verify_input_and_bind` (operator-scoped) → `confidential_transfer_from` | **DIVERGENCE** (DD-007/DD-009) |
| `confidentialTransferAndCall(...)` ×4 | transfer then call `onConfidentialTransferReceived`; refund `select(success,0,sent)`; `transferred = sent - refund`; transient-allow to sender | `confidential_call_transfer_receiver{,_from}` (hook, returns encrypted success handle) → `confidential_prepare_transfer_callback` (compute refund) → `confidential_finalize_transfer_callback` (credit refund, record transfer) | **DIVERGENCE** — split receiver-hook + settlement (DD-011: single-instruction callback exceeds SBF heap/CPI depth); hook causality + replay markers enforced explicitly |
| `setOperator(operator, until)` | time-bounded operator approval (`uint48` deadline); `OperatorSet` event | `set_operator(operator, expiration_slot)` + `close_operator` (rent refund) | **MET** (DIVERGENCE: slot deadline vs `block.timestamp`; explicit PDA row + close) |
| `isOperator(holder, spender)` | `holder==spender \|\| now <= until` | `Operator` PDA `expiration_slot >= slot` (holder==spender implicit by owner-signed paths) | **MET** |
| `requestDiscloseEncryptedAmount(euint64)` | `makePubliclyDecryptable` + event; `require isAllowed(amount, sender)` | `request_disclose_balance` / `request_disclose_amount` → host `allow_for_decryption`; emits `*DisclosureRequestedEvent` | **MET** (DD-010: label-scoped — balance vs amount paths distinct) |
| `discloseEncryptedAmount(euint64, cleartext, proof)` | `FHE.checkSignatures` + `AmountDisclosed` event | `disclose_balance` / `disclose_amount(cleartext)` — verify ACL + material commitment + `public_decrypt` + a preceding Ed25519 KMS-cert instruction | **MET** (DIVERGENCE: Ed25519 instructions-sysvar cert vs EIP-712 `checkSignatures`) |
| `confidentialTotalSupply()` view | encrypted total supply handle | `Mint` state total-supply handle (born in `initialize_mint`; rotated by `confidential_burn`) — read off-chain | **MET** (DIVERGENCE: account read, not a view call) |
| `confidentialBalanceOf(account)` view | encrypted balance handle | `ConfidentialTokenAccount.balance_handle` (+ `balance_acl_record`, `next_balance_nonce_sequence`; RFC 024 token shape) — read off-chain | **MET** (DIVERGENCE: account read) |
| `name/symbol/decimals(=6)/contractURI` views | metadata | mint/app config; `wrap_usdc` ties decimals to the underlying SPL mint | **DIVERGENCE** (app config / off-chain reads) |
| `_mint(to, amount)` | increase total supply + credit | `initialize_mint` (total-supply ACL birth) + `wrap_usdc` (escrow SPL USDC → trivial-encrypt → add to balance) + `initialize_token_account` (zero-balance birth) | **DIVERGENCE** — minting modeled as SPL wrapping (RFC 024 wrap flow; real SPL-token boundary) |
| `_burn(from, amount)` | decrease balance + total supply | `confidential_burn` (rotate balance + total supply) + `redeem_burned_amount` (release underlying from vault — KMS-cert gated, public-decrypt gated) | **MET** + Solana-only redeem leg (no ERC7984 analogue; wrapper needs underlying release) |
| `_update` safe-math (`tryIncrease`/`tryDecrease`, `select`, allow/allowThis) | overflow/underflow-safe FHE balance update, conditional transfer of `select(success, amount, 0)` | `confidential-token/src/fhe.rs` + host `fhe_binary_op*`/`fhe_ternary_op_*` (add/sub/ge/select) + producer ACL birth | **PARTIAL** — `tryDecrease` reproduced (transfer/burn debit: `ge` → `sub` → `select(success, candidate, balance)`); `tryIncrease` NOT reproduced — the wrap/mint total-supply increase and the recipient credit use a plain `add`. Total-supply overflow is instead bounded by the 1:1 SPL backing (a real `u64` mint), per the `_mint` DIVERGENCE above. Output ACL via authorized producer paths |
| `FHE.allow / allowThis` | durable ACL grant to user / contract | output `*_and_bind` producer paths (births canonical `AclRecord` with subjects) + `allow_acl_subjects` (append) | **MET** |
| `FHE.allowTransient(transferred, sender)` | transaction-local grant | one-shot `TransientSession`/capability (host) or signer propagation | **DIVERGENCE** (no tstore; DD-008 / TRANSIENT_ALLOW.md) |
| `ConfidentialTransfer` / `OperatorSet` / `AmountDisclose(d/Requested)` events | indexing | token-local + host events (`AclAllowedEvent`, disclosure/operator events) | **MET** (DD-003: events are indexing hints, not authorization) |
| self-transfer | (EVM updates regardless) | `confidential_transfer` no-op when from==to (no handle rotation, no output ACL) | **MET** (RFC 024 explicit; avoids useless historical handles) |

ERC7984's FHE op footprint is exactly **add / sub / compare(ge) / select / trivial-encrypt / rand /
fromExternal** — all implemented. The confidential token is therefore **op-complete**.

---

## 2. FHEVM host (`host-contracts/`) → `zama-host`

| EVM unit | Capability | Solana equivalent | Status |
|---|---|---|---|
| `ACL.allow(handle, account)` | persistent grant; caller must be allowed | `allow_acl_subjects` — authority `Signer` + `ACL_ROLE_GRANT` on record + canonical PDA + deny-list + pause; append-only | **MET** (stricter: explicit grant role) |
| `ACL.allowForDecryption(handles[])` | mark publicly decryptable | `allow_for_decryption` — dedicated `ACL_ROLE_PUBLIC_DECRYPT`, never set at birth, idempotent | **MET** (DIVERGENCE: per-handle; DD-005) |
| `ACL.allowTransient(handle, account)` | tx-local grant | `allow_transient_handle` + one-shot `TransientSession` PDA (same-tx creation proof via instructions sysvar) | **DIVERGENCE** (no tstore; DD-008) |
| `ACL.isAllowed(handle, account)` | transient OR persistent | `assert_acl_record` / `assert_record_subject_role(ACL_ROLE_USE)` | **MET** |
| `ACL.isAllowedForDecryption(handle)` | public-decrypt flag | `AclRecord.public_decrypt` | **MET** |
| `ACL.persistAllowed` / `allowedTransient` | read pair | inline subject lookup + `AclPermission` overflow PDA / sealed session | **MET** / **DIVERGENCE** |
| `ACL.cleanTransientStorage()` | wipe tx transient (AA bundling) | `close_transient_session` (per-session rent reclaim) | **MET** (per-session) |
| `ACL.delegate/revokeForUserDecryption` | user-decrypt delegation lifecycle | `delegate_for_user_decryption` / `revoke_...` — PDA per `(delegator,delegate,app)`, slot-based expiry, same-slot double-update guard, wildcard-delegate rejected | **MET** |
| `ACL` deny list (`blockAccount`/`isAccountDenied`) | owner deny list | `set_deny_subject` + `DenySubjectRecord`, gated into grant paths | **MET** |
| `ACL` pause/owner/UUPS | pauser role, 2-step ownership, upgrade | single `admin` (`set_host_pause` etc.) | **DIVERGENCE** (admin signer) + **PRODUCT-OPEN** (BPF upgrade authority handles program upgrade) |
| `FHEVMExecutor.fheAdd/fheSub` | binary add/sub | `fhe_binary_op{,_and_bind_output}` op=Add/Sub | **MET** |
| `FHEVMExecutor.fheGe` | ≥ comparison → ebool | op=Ge | **MET** |
| `FHEVMExecutor.fheIfThenElse(select)` | ternary; ebool control, branch type-checked | `fhe_ternary_op_and_bind_output` op=IfThenElse | **MET** |
| `FHEVMExecutor.fheRand/fheRandBounded` | random / bounded random | `fhe_rand_and_bind` / `fhe_rand_bounded_and_bind` (pow2+≤max bound check) | **MET** (DIVERGENCE: slot/bankhash+nonce seed vs counterRand+blockhash) |
| `FHEVMExecutor.trivialEncrypt` | plaintext → ct handle | `trivial_encrypt_and_bind` | **MET** |
| `FHEVMExecutor.verifyCiphertext` (input) | verify signed/proved input, allowTransient | `verify_input_and_bind` (Ed25519 pre-ix over canonical `SolanaInputProof`+intent) + test-only `mock_input_verified_and_bind` | **MET** (DIVERGENCE: single Ed25519 authority vs EIP-712 coprocessor threshold; DD-007) |
| `FHEVMExecutor` batched/expression compute | — | `fhe_eval` (bounded multi-op; `MAX_FHE_EVAL_OPS=16`) | **MET** (Solana-native batching to limit CPI depth; DD-008 batch option) |
| operand ACL + scalar rule | encrypted operand needs ACL; scalar exempt | `assert_record_subject_role(ACL_ROLE_USE)` per operand; scalar RHS rejects a permission witness | **MET** (RFC 024 scalar rule; DD-006) |
| handle byte-layout (ver/type/chainid/computed-marker) + deterministic derivation | symbolic-exec handle | identical layout; `sha256(domain, op, operands, programID, chainid, prev_bank_hash, ts)` + nonce-bound variant for durable outputs | **MET** (DIVERGENCE: sha256 vs keccak, bankhash vs blockhash → handles not cross-chain-interoperable) |
| compute identity = `msg.sender` | implicit caller | explicit `compute_subject: Signer` checked `is_signer && is_allowed` | **DIVERGENCE** (no `msg.sender`; RFC 024 compute-signer) |
| `FHEVMExecutor` op breadth: Mul/Div/Rem/BitAnd/Or/Xor/Shl/Shr/Rotl/Rotr/Eq/Ne/Gt/Lt/Le/Min/Max, Neg/Not, cast, Sum/IsIn | full opcode catalog | only Add/Sub/Ge/IfThenElse shipped | **SCOPE** — not Solana-constrained; mechanically extensible (enum + per-op type-gate table + handle hash already op-parametrized + coprocessor map arm + tests); **not used by ERC7984** |
| `HCULimit` (per-op/tx/block/depth homomorphic-compute caps) | gas-like metering | none on-chain; relies on Solana compute-budget + op-count/collection caps | **DIVERGENCE** (compute-budget) — see fragility #3 |
| `KMSVerifier` (on-chain decrypt-sig threshold verify) | verify KMS sigs on-chain | none on-chain; host exposes witnesses (ACL record + material commitment + `public_decrypt`) for an external verifier | **PRODUCT-OPEN** (DD-012) |
| `ProtocolConfig` / `KMSGeneration` / `PauserSet` (role set) | KMS node/threshold registry, keygen, pauser role set | none (subset in `HostConfig`: authorities/chain_id/flags) | **PRODUCT-OPEN** |
| `FheType` (86 variants) | type enum | supported set Bool/Uint8..Uint256 (covers token + shipped ops) | **MET (partial)** / **SCOPE** (signed/large/string types) |

---

## 3. Gateway / KMS (`gateway-contracts/`) → `zama-host` + `kms-connector` + token disclose

| EVM gateway capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `Decryption.publicDecryptionRequest/Response` | request + threshold-consensus public decrypt | token `request_disclose_*` (sets `public_decrypt`); connector native-v0 PUBLIC mode + `verify_solana_kms_response_v0`; on-chain `disclose_*` consumes Ed25519 cert | **DIVERGENCE** (DD-012) |
| `Decryption.userDecryptionRequest` + EIP-712 | user-signed, contract-scoped, validity window | connector native-v0 DIRECT_SCOPED (`verify_native_direct_request`) — owner-signed, ACL-domain-scoped | **DIVERGENCE** (Ed25519 over keccak domain-sep vs EIP-712) |
| `Decryption.delegatedUserDecryptionRequest` + RFC-017 wildcard | delegate-signed; wildcard contract scope | native-v0 DELEGATED_SCOPED / DELEGATED_WILDCARD_SCOPED; `UserDecryptionDelegation` PDA; wildcard = `[0xff;32]` app-context sentinel | **MET** (semantics) / **DIVERGENCE** (mechanism) |
| `Decryption.userDecryptionResponse` (per-share sigs → threshold) | threshold response | native-v0 `verify_response_certificate` (sorted distinct Ed25519 signer set, signer-set hash, request binding) | **DIVERGENCE** |
| `checkDecryptionReady` (material added) | all handles have ciphertext material | `verify_material_commitment` (state==COMMITTED + canonical PDA) / on-chain `assert_material_commitment` | **MET** |
| `CiphertextCommits.addCiphertextMaterial` | multi-coprocessor consensus adds (keyId, ctDigest, snsDigest) | `commit_handle_material` — single `material_authority` writes one-shot `HandleMaterialCommitment`, seals ACL record | **DIVERGENCE** (consensus → single authority; consensus is off-chain) |
| `checkCiphertextMaterial` | material-present check | `commitment.state==COMMITTED` + hash binding | **MET** |
| `InputVerification.verifyProofRequest/Response` (ZKPoK consensus) | coprocessor ZK-proof verify + consensus + EIP-712 | `verify_input_and_bind` (signed input proof, single authority) | **DIVERGENCE** + partial **SCOPE/PRODUCT-OPEN** (no consensus / `rejectProofResponse`) |
| `HandleOps`/`FHETypeBitSizes` | parse chainId/fheType; bit-size table | mirrored exactly in connector (`solana_native_handle_chain_id`, `*_fhe_type_encrypted_bits`) | **MET** |
| `MAX_DECRYPTION_REQUEST_BITS=2048` | per-request cleartext cap | `SOLANA_NATIVE_MAX_ENCRYPTED_BITS_PER_REQUEST=2048` enforced | **MET** |
| `Structs` (Sns material, delegation, pairs) | cross-contract DTOs | witness structs + on-chain `AclRecord`/`UserDecryptionDelegation`/`HandleMaterialCommitment` | **MET** (re-modeled) |
| native-v0 request/response transport loop + publisher | (the eventual EVM-bypass) | implemented + unit-tested (`solana_{request,live,native,response,flow,store,replay}.rs`) but **not wired into worker/tx-sender binaries**; `SolanaNativeResponsePublisher` target unimplemented | **PRODUCT-OPEN** (DD-012) |
| `GatewayConfig` (KMS/coprocessor/host-chain/threshold registry) | on-chain registry | connector reads registry off-chain (`Config`); `HostConfig` holds only authorities/flags | **PRODUCT-OPEN** |
| `KMSGeneration` (keygen/crsgen ceremony) | key/CRS lifecycle | none (referenced only by `key_id`) | **PRODUCT-OPEN** |
| `ProtocolPayment` ($ZAMA fees) | per-request fee | none (rent/tx fees only) | **PRODUCT-OPEN** |

---

## 4. Coprocessor / KMS adapter integration

- **Coprocessor host-listener** (`coprocessor/.../host-listener/src/solana_adapter.rs`): decodes
  zama-host Anchor CPI events (codegen from vendored IDL `idl/zama_host.json`) into TFHE/ACL DB
  rows. `cargo check -p host-listener` → exit 0. **Library-only**: no live Geyser/Yellowstone
  subscriber is wired (PRODUCT-OPEN, DD-003). Only Add/Sub/Ge / IfThenElse mapped (tracks §2 SCOPE).
- **KMS connector** (`kms-connector/crates/kms-worker/src/core/solana_acl.rs`, 3097 ln): witness
  decoders + `SolanaAclVerifier` (canonical-PDA, owner, hash-domain, role checks). `cargo check
  -p kms-worker` → exit 0; **114 Solana unit tests**. The **Gateway-PoC `extraData` path is wired
  live** into public/user decryption (`event_processor/decryption.rs`) with canonical-PDA + SNS
  material binding. Account layouts are hand-mirrored 1:1 against `zama-host/src/state.rs` (verified)
  but **version-pinned / no compile-time link** — a layout/seed/hash-domain/EVENT_VERSION change in
  zama-host requires editing the connector decoder + regenerating the coprocessor IDL.

**Adapters are present and integrated at the PoC boundary.** Live transport (Geyser provider,
native-v0 binary wiring, native response publisher, live KMS Core dispatch) is PRODUCT-OPEN by
design.

---

## 5. Critical assessment — solid vs fragile

**Solid (faithful, often stricter than EVM):** the ACL core (`allow`/`isAllowed`/grant authority/
append-only/no-generic-bind per RFC 024), public-decrypt release gated on a dedicated role and never
at birth, delegation lifecycle (slot expiry, same-slot guard, wildcard-delegate rejection), one-shot
transient capabilities with same-tx creation proof, deterministic handle derivation + byte-layout,
operand-ACL discipline + scalar-RHS rule, ABI/account-meta exactness (DD-004, extensive negative
tests), the confidential-token flows (transfer/operator/transfer-and-call split/wrap/burn/redeem/
disclose) with caller-scoped operator amounts (DD-009) and label-scoped disclosure (DD-010), and the
connector's canonical-PDA + material-binding verification.

**Fragile / attention for a security pass (PoC shortcuts, not correctness bugs):**

1. **Input verification trust = one Ed25519 key** (`input_verifier_authority`), not an N-of-M
   coprocessor EIP-712 threshold. Compromise forges any input. The multisig/threshold path is unbuilt
   (PRODUCT-OPEN, DD-007).
2. **Test/mock bypass gates** (`mock_input_verified_and_bind`, `set_test_shims_enabled`,
   `set_mock_input_enabled`, `test_emit_*`). The two that can mint/relax authorization are now
   **chain-id confined**: `mock_input_verified_and_bind` and the zero birth-entropy fallback only run
   on `SOLANA_POC_CHAIN_ID` (`HostConfig::mock_input_allowed` / `zero_birth_entropy_allowed`), so an
   admin cannot enable them on a deployed chain regardless of the flags (DD-014). The residual is
   `test_emit_*`, which only emits events and mutates no state; it should still be forced-off (ideally
   compiled-out) for mainnet.
3. **No on-chain HCU / complexity metering** (the EVM `HCULimit` plane has no analogue). Off-chain
   workers get no on-chain cost signal beyond the CU budget + op-count/collection caps. Largest
   semantic gap by surface area; relevant to DoS/cost-bounding.
4. **On-chain disclosure trusts a single preceding Ed25519 instruction**, not a threshold cert
   (threshold lives off-chain in `solana_response.rs`, which is not on the disclose path);
   `kms_verifier_authority` is an `UncheckedAccount`. Single-key on-chain trust for `disclose_*`.
5. **Two connector verifier entrypoints** (Gateway-PoC `extraData` vs native-v0) have subtly
   different app-context handling — a maintenance hazard until one is retired.
6. **Hand-mirrored, version-pinned ABI across repos with no compile-time link** (connector decoders +
   vendored coprocessor IDL). Lengths are checked, but a same-length field reorder in `AclRecord`
   would not be caught at build time. `EVENT_VERSION=0` / `MAX_ACL_SUBJECTS=8` hardcoded in two places.
7. **`previous_bank_hash` is fail-closed on real chains.** When the prior bank hash is unavailable,
   handle birth returns `PreviousBankHashUnavailable` rather than substituting a zero hash. The
   zero-hash fallback (intended only for LiteSVM bootstrap) is confined to `SOLANA_POC_CHAIN_ID` via
   `HostConfig::zero_birth_entropy_allowed`, so it cannot weaken birth entropy on a deployed chain
   (DD-014). Note bank-hash + timestamp entropy is itself a PoC choice; native-v0's deterministic
   `BirthContextV0` removes this surface entirely (DD-015).
8. **Material commitment is one-shot/irreversible** with single-authority blast radius — a wrong seal
   permanently bricks a handle's decryptability (matches EVM "no delete" but without consensus).
9. **Transient capability `max_uses`/`max_entries` are silently pinned to 1** though the ABI advertises
   them as configurable — a client foot-gun (conservative, safe default).

None of the fragilities are correctness defects in the implemented paths (the negative-test suite is
extensive); they are the expected boundary between a PoC and a production deployment, and every one
maps to an item already tracked as PRODUCT-OPEN in `DESIGN_DECISIONS.md` / the issue ledger.

---

## 6. Net parity verdict

- **Confidential token (ERC7984):** fully ported — every external function maps to a Solana
  instruction (or constraint-driven split), and the FHE op footprint it requires is complete.
- **FHEVM host authorization model** (ACL, executor authorization, input binding, transient,
  decrypt release, delegation, material commitment): faithfully ported, frequently stricter.
- **Constraint-driven divergences** are all intentional and documented (no tstore, no `msg.sender`,
  no `ecrecover`/EIP-712, shallow CPI, compute-budget vs HCU, account-witnesses vs storage reads).
- **SCOPE** items (executor opcode breadth, full FheType set) are not Solana-limited and not needed
  by the token; mechanically extensible.
- **PRODUCT-OPEN** items (on-chain KMSVerifier/ProtocolConfig/KMSGeneration/GatewayConfig/payment,
  live native-v0 transport + publisher, Geyser provider, threshold input verification) are off-chain
  services or future programs by design (DD-012, DD-003, DD-007).
