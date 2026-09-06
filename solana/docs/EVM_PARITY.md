# EVM → Solana Parity Note

This note maps each EVM-side capability of the Zama confidential-token + FHEVM host design to
its Solana equivalent in this PoC, or records a justified divergence. It is grounded in direct
code review of `ERC7984.sol`, `host-contracts/`, `gateway-contracts/`, and the Solana programs +
adapters (`fhevm/solana/programs/{zama-host,confidential-token,confidential-batcher}`,
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
[`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) (DD-001..DD-048), RFC 024 and RFC 035.

---

## 1. ERC7984 confidential token → `confidential-token`

| ERC7984 capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `confidentialTransfer(to, euint64)` | holder transfers an already-allowed handle; `require isAllowed(amount, sender)` | `confidential_transfer(amount_handle)` — owner-signed; the amount value's authority signs for it; updates sender/recipient balance handles and seals the allows of each new handle on the write | **MET** |
| `confidentialTransfer(to, externalEuint64, inputProof)` | transfer a freshly verified external input | `confidential_transfer(amount_attestation)` — the attestation is consumed inline as the `fhe_execute` `FheExecuteOperand::VerifiedInput` operand (the Solana `FHE.fromExternal` analog): verified in-execution, transient-allowed, no persistent input ACL | **MET** (DD-007: verify ≠ allow; caller-is-contract enforced via `attestation.contract_address == program`, the verified program of the execution's application) |
| `confidentialTransferFrom(from,to,euint64)` | operator transfer; `require isOperator` + `isAllowed`; `allowTransient(transferred, sender)` | no Solana production equivalent | **INTENTIONAL GAP** — operator/delegated transfer APIs were removed to simplify authority and reduce attack surface (tracked: zama-ai/fhevm-internal#1692) |
| `confidentialTransferFrom(from,to,externalEuint64,proof)` | operator transfer of external input | no Solana production equivalent | **INTENTIONAL GAP** — the owner-authorized `confidential_transfer(amount_attestation)` fromExternal path is the supported route (tracked: zama-ai/fhevm-internal#1692) |
| `confidentialTransferAndCall(...)` ×4 | transfer then call `onConfidentialTransferReceived`; refund `select(success,0,sent)`; `transferred = sent - refund`; transient-allow to sender | not ported; a receiving app exposes its own instruction that CPIs `confidential_transfer` with the user as sole signer (see `confidential-batcher::join`) | **INTENTIONAL GAP** (DD-011) — the callback is an EVM workaround for contracts not observing incoming transfers; Solana propagates signer authority through CPI. Token-2022 transfer hooks are rejected as a substitute (veto-only, not receiver callbacks) |
| `setOperator(operator, until)` | time-bounded operator approval (`uint48` deadline); `OperatorSet` event | no Solana production equivalent | **INTENTIONAL GAP** — no operator rows or operator events (tracked: zama-ai/fhevm-internal#1692) |
| `isOperator(holder, spender)` | `holder==spender \|\| now <= until` | no Solana production equivalent | **INTENTIONAL GAP** — holder self-authority is handled by owner-signed paths (tracked: zama-ai/fhevm-internal#1692) |
| `requestDiscloseEncryptedAmount(euint64)` | `makePubliclyDecryptable` + event; `require isAllowed(amount, sender)` | there is no request account (DD-040): the token owner or mint authority calls a token wrapper, which validates the exact state field and signs the Host `make_handle_public` CPI as encrypted value account authority (exact-handle `PublicDecryptLeaf`; no per-request PDA, context pin, or expiry) | **INTENTIONAL DIVERGENCE** — viewers are not ACL administrators (DD-045, DD-048) |
| `discloseEncryptedAmount(euint64, cleartext, proof)` | `FHE.checkSignatures` + `AmountDisclosed` event | token `disclose_secp(kind, handle, cleartext, signatures, extra_data, proof)` (DD-040/DD-045) CPIs the stateless host `verify_public_decrypt`, which authorizes the caller-pinned exact handle by its encrypted value account public-decrypt MMR proof (the sealed leaf is permanent, so a disclosure survives replacement during the KMS round-trip) + on-chain secp256k1 KMS-cert verification against the live `KmsContext` named by the certificate; binds the mint scope, canonical encrypted value account, authority, label, and token state kind before emitting `HandleDisclosedEvent`; idempotent — no on-chain replay marker | **MET** (DD-021/DD-040: mirrors EVM `KMSVerifier`; DD-045 strengthens app-state binding) |
| `confidentialTotalSupply()` view | encrypted total supply handle | one stable total-supply `EncryptedValue` encrypted value account (created in `initialize_mint`; replaced by `confidential_burn`, DD-032) — read off-chain | **MET** (DIVERGENCE: account read, not a view call) |
| `confidentialBalanceOf(account)` view | encrypted balance handle | `ConfidentialTokenAccount.balance_encrypted_value` points to the stable `EncryptedValue` encrypted value account whose `current_handle` is read off-chain | **MET** (DIVERGENCE: account read) |
| `name/symbol/decimals(=6)/contractURI` views | metadata | mint/app config; `wrap_usdc` ties decimals to the underlying SPL mint | **DIVERGENCE** (app config / off-chain reads) |
| `_mint(to, amount)` | increase total supply + credit | `initialize_mint` (total-supply encrypted value account creation) + `wrap_usdc` (escrow SPL USDC → trivial-encrypt → add to balance) + `initialize_token_account` (zero-balance encrypted value account creation) | **DIVERGENCE** — minting modeled as SPL wrapping (RFC 024 wrap flow; real SPL-token boundary) |
| `_burn(from, amount)` | decrease balance + total supply | `confidential_burn` updates balance + encrypted total supply and opens the token account's single `PendingBurn`; `redeem_burned_amount` consumes the stateless verifier and releases underlying, while `cancel_pending_burn` restores balance and supply. No next burn is accepted until redeem or cancel closes the pending account. | **MET** + Solana-only sequential settlement phase (DD-045; wrapper needs underlying release) |
| `_update` safe-math (`tryIncrease`/`tryDecrease`, `select`, allow/allowThis) | overflow/underflow-safe FHE balance update, conditional transfer of `select(success, amount, 0)` | `confidential-token/src/fhe/` + `fhe_execute` Binary/Ternary steps (add/sub/ge/select — the standalone `fhe_binary_op*`/`fhe_ternary_op*` instructions were removed, DD-032) + output binding into `EncryptedValue` | **PARTIAL** — `tryDecrease` reproduced (transfer/burn debit: `ge` → `sub` → `select(success, candidate, balance)`); wrap `tryIncrease` reproduced (`ge` on remaining room → `select` the credit). Recipient transfer credit is still a plain `add`. Total-supply overflow on wrap is clamped together with the balance credit. Output ACL via authorized producer paths |
| `FHE.allow / allowThis` | persistent ACL grant to user / contract | the allows declared on the write that installs a handle (`PersistentOutput::allow`, sealed as leaves, DD-048); a later change of viewers is a re-write of the value (`allow_balance_viewers` / `allow_total_supply_viewers`) | **MET** (DIVERGENCE: allows are per handle and sealed once, not a mutable per-value list) |
| `FHE.allowTransient(transferred, sender)` | transaction-local grant | instruction-local `Transient` result within one `fhe_execute`, referenced by later steps as `EarlierStep`, plus CPI signer propagation within one instruction | **DIVERGENCE** (no tstore; DD-008) |
| `ConfidentialTransfer` / `OperatorSet` / `AmountDisclose(d/Requested)` events | indexing | token-local events; the `EncryptedValue` ACL lifecycle itself emits no events by design (DD-033), reconstructed instead by instruction-replay indexing; no operator events | **PARTIAL** (DD-003: events are indexing hints, not authorization) |
| self-transfer | (EVM updates regardless) | `confidential_transfer` no-op when from==to (no handle update, no output ACL) | **MET** (RFC 024 explicit; avoids useless historical handles) |

ERC7984's FHE op footprint is exactly **add / sub / compare(ge) / select / trivial-encrypt / rand /
fromExternal** — all implemented. The confidential token is therefore **op-complete**.

---

## 2. FHEVM host (`host-contracts/`) → `zama-host`

| EVM unit | Capability | Solana equivalent | Status |
|---|---|---|---|
| `ACL.allow(handle, account)` | persistent grant; caller must be allowed | no standalone instruction: the value authority declares the keys allowed on the handle it writes, and the host seals one leaf per key on that write (+ deny list by application + pause, DD-048). Viewers are not co-admins; changing viewers is a re-write by the authority (confidential-token `allow_balance_viewers`, CPI + `invoke_signed` as the token-account PDA) | **MET** (DIVERGENCE: authority-gated and sealed on the write, not caller-is-allowed) |
| `ACL.allowForDecryption(handles[])` | mark publicly decryptable | `make_handle_public` — only the encrypted value account authority can seal an exact-handle `PublicDecryptLeaf` into the MMR; that handle's publicness is then **permanent** and survives a later persistent-output update, while remaining exact-handle scoped (DD-032/DD-045) | **INTENTIONAL DIVERGENCE** — encrypted value account authority, not any viewer |
| `ACL.allowTransient(handle, account)` | tx-local grant | no persistent analog; instruction-local `Transient` results within one `fhe_execute`, plus CPI signer propagation within one instruction | **DIVERGENCE** (no tstore; DD-008) |
| `ACL.isAllowed(handle, account)` | transient OR persistent | one path: an MMR `HistoricalAccessLeaf(handle, key)` inclusion proof against the live confirmed peaks of the canonical `EncryptedValue` PDA, for the current handle and a replaced one alike; the proof is fetched by the KMS connector from the coprocessors' leaf record, never supplied by the client (DD-048). On-chain, a stored value is read by its authority's signature — there is no compute-side `isAllowed` | **MET** (DIVERGENCE: proof, not a stored list) |
| `ACL.isAllowedForDecryption(handle)` | public-decrypt flag | exact-handle `PublicDecryptLeaf` MMR inclusion proof (no live flag — DD-032); exposed to any app (not token-only) as the stateless host `verify_public_decrypt`, which checks that proof + a KMS cert against the live context the cert names (any non-destroyed context, fhevm-internal#1765) and returns `(handle, cleartext, context_id)` via `return_data` (DD-040) | **MET** (DIVERGENCE: proof, not a stored flag) |
| `ACL.persistAllowed` / `allowedTransient` | read pair | no stored list to read: allows are leaves, proven off-chain (row above); transient = instruction-local `Transient` result | **DIVERGENCE** |
| `ACL.cleanTransientStorage()` | wipe tx transient (AA bundling) | nothing to reclaim — `Transient` results are instruction-scoped and never persisted (the one-shot `TransientSession` account tier was removed, DD-008) | **DIVERGENCE** (no persistent transient state) |
| `ACL.delegate/revokeForUserDecryption` | user-decrypt delegation lifecycle | `delegate_for_user_decryption` / `revoke_...` — PDA per `(delegator,delegate,app)`, slot-based expiry, same-slot double-update guard, wildcard-delegate rejected | **MET** |
| `ACL` deny list (`blockAccount`/`isAccountDenied`) | owner deny list of accounts | `set_deny_scope` + `DenyScopeRecord` at `["deny-scope", program, scope]`: names an application, not a key, and gates every allow it would seal (persistent writes, `make_handle_public`; DD-048) | **DIVERGENCE** (an application is denied, not a viewer) |
| `ACL` pause/owner/UUPS | pauser role, 2-step ownership, upgrade | single `admin` (`set_host_pause`; `set_admin` one-step with keypair co-sign / existing-PDA skip; `set_eip712_domain`) | **DIVERGENCE** (admin signer, not Ownable2Step) + **PRODUCT-OPEN** (BPF upgrade authority handles program upgrade; init admin must be that authority) |
| `FHEVMExecutor.fheAdd/fheSub` | binary add/sub | `fhe_execute` Binary step, op=Add/Sub (the standalone `fhe_binary_op*` instructions were removed; `fhe_execute` is the only compute path, DD-032) | **MET** |
| `FHEVMExecutor.fheGe` | ≥ comparison → ebool | `fhe_execute` Binary step, op=Ge | **MET** |
| `FHEVMExecutor.fheIfThenElse(select)` | ternary; ebool control, branch type-checked | `fhe_execute` Ternary step, op=IfThenElse (the standalone `fhe_ternary_op*` instruction was removed, DD-032) | **MET** |
| `FHEVMExecutor.fheRand/fheRandBounded` | random / bounded random | `fhe_execute` Rand and RandBounded steps (pow2+≤max bound check); the seed binds the application, the host's `RandNonce` counter and the step index (DD-043) | **MET** (DIVERGENCE: bank hash + host nonce vs `counterRand` + blockhash) |
| `FHEVMExecutor.trivialEncrypt` | plaintext → ct handle | `fhe_execute` TrivialEncrypt step (the standalone `trivial_encrypt_and_bind` instruction was removed, DD-032) | **MET** |
| `FHEVMExecutor.verifyCiphertext` (input) | verify signed/proved input, allowTransient | `fhe_execute` `FheExecuteOperand::VerifiedInput` operand — in-execution secp256k1 recover + threshold-check of the coprocessor EIP-712 `CiphertextVerification` attestation, asserts `contract_chain_id == host chain id` and `contract_address == program` (the execution's verified application program); transient-allowed, no persistent ACL | **MET** (DD-007: verify ≠ allow; same coprocessor EIP-712 trust root as EVM) |
| `FHEVMExecutor` batched/expression compute | — | `fhe_execute` (bounded mixed-step execution; `MAX_FHE_EXECUTION_STEPS=32`) | **MET** (Solana-native batching to limit CPI depth; DD-008; mixed binary/ternary/unary/trivial-encrypt/rand/rand-bounded/sum/isIn/mulDiv steps with `Transient` results, and `VerifiedInput` operands) |
| operand ACL + scalar rule | encrypted operand needs ACL; scalar exempt | a stored operand is admitted by its value authority's signature (RFC 035); scalar RHS rejects a permission witness | **MET** (DIVERGENCE: the owning program signs, no per-caller list) |
| handle byte-layout (ver/type/chainid/computed-marker) + entropy-seeded derivation | symbolic-exec handle; no per-output nonce (only `fheRand`'s global `counterRand`, folded into the rand seed) | identical layout; deterministic ops content-addressed `keccak(domain, op, operands, programID, chainid, prev_bank_hash, ts)`; rand seeds anchored to the application + the host's rand nonce + the step index (DD-043). Persistent outputs use the same base handle as local outputs; the account's seeds are never mixed into the handle (DD-015). | **MET** (DIVERGENCE: bankhash vs blockhash → handles not cross-chain-interoperable) |
| compute identity = `msg.sender` | implicit caller | the application `(program, scope)`: `program` is proven from the output authority's seeds on every write (`assert_authority_is_program_pda`), `scope` is what that program declares (DD-047). There is no separate compute identity: reading a value is admitted by the value authority's signature | **DIVERGENCE** (no `msg.sender`; a verified program identity) |
| `FHEVMExecutor` op breadth: Mul/Div/Rem/BitAnd/Or/Xor/Shl/Shr/Rotl/Rotr/Eq/Ne/Gt/Lt/Le/Min/Max, Neg/Not, cast, Sum/IsIn, MulDiv | full opcode catalog | implemented in `fhe_execute` as the binary catalog, unary ops, `Sum`, `IsIn`, and `MulDiv`; constrained by the supported `FheType` set below | **MET** for the modeled operator surface; remaining breadth is type support, not dispatch shape |
| `HCULimit` (per-op/tx/block/depth homomorphic-compute caps) | gas-like metering | `HostConfig::max_hcu_per_tx` / `max_hcu_depth_per_tx` summed over one `fhe_execute` execution (`u64::MAX` = off), plus per-application per-slot block cap (`hcu_block_cap_per_app`, keyed on `(program, scope)`, DD-039/DD-047), plus Solana compute-budget + op-count/collection caps | **DIVERGENCE** (per-execution + per-application-per-slot cap vs global per-block metering) — see fragility #3 |
| `KMSVerifier` (on-chain decrypt-sig threshold verify) | verify KMS sigs on-chain | on-chain secp256k1: `eip712::verify_kms_public_decrypt` recovers EVM KMS signers and threshold-checks them against the witness-pinned `KmsContext` (rejects high-s) | **MET** (DD-021: mirrors EVM `KMSVerifier`) |
| `ProtocolConfig` / `KMSGeneration` / `PauserSet` (role set) | KMS node/threshold registry, keygen, pauser role set | none (subset in `HostConfig`: authorities/chain_id/flags) | **PRODUCT-OPEN** |
| `FheType` (86 variants) | type enum | supported set Bool/Uint8..Uint128 (covers token + shipped ops; types 7/8 rejected) | **MET (partial)** / **SCOPE** (signed/large/string types) |

---

## 3. Gateway / KMS (`gateway-contracts/`) → `zama-host` + `kms-connector` + token disclose

| EVM gateway capability | Semantics | Solana equivalent | Status |
|---|---|---|---|
| `Decryption.publicDecryptionRequest/Response` | request + threshold-consensus public decrypt | request = host `make_handle_public` (encrypted value account authority seals the public-decrypt leaf); consume = token `disclose_secp` CPIing the stateless host `verify_public_decrypt`, which checks a secp256k1 KMS cert against the live `KmsContext` named by the certificate; idempotent, no request witness (DD-040/DD-045) | **MET** for KMS trust; intentional authority divergence |
| `Decryption.userDecryptionRequest` + EIP-712 | user-signed, contract-scoped, validity window | routed through the unified Gateway V2 path via the host-generic `userDecryptionRequest(...)` overload carrying the `solana-srfc38-user-decrypt-v1` payload; chain-aware validator branches on `contracts_chain_id`. The permit scopes to `allowedScopes` — at most seven `(program, scope)` pairs, empty = permissive, tested per entry against the account's own pair — and the request names only the encrypted value account and an allowed key: no proof, no scope (DD-048) | **MET** (DD-012/DD-026/DD-027: reuses the Gateway/EVM stack, typed Solana fields) |
| `Decryption.delegatedUserDecryptionRequest` + RFC-017 wildcard | delegate-signed; wildcard contract scope | `UserDecryptionDelegation` PDA per `(delegator,delegate,encrypted value account authority)`, slot expiry, wildcard = `[0xff;32]` authority sentinel; a delegated entry names the delegator as its allowed key, and the connector reads the record in its deciding snapshot and authorizes against the delegator's allow leaf (INVARIANTS #27) | **MET** (semantics) / **DIVERGENCE** (PDA mechanism) |
| `Decryption.userDecryptionResponse` (per-share sigs → threshold) | threshold response | Gateway V2 response path; connector verifies the KMS threshold response | **MET** (DD-012) |
| `checkDecryptionReady` (material added) | all handles have ciphertext material | the host-owned `HandleMaterialCommitment` subsystem was deleted; materiality is checked against the gateway's `CiphertextCommits`, where the coprocessor already registers Solana handles (DD-031) | **DIVERGENCE** (moved off host-chain state entirely, not re-modeled on Solana) |
| `CiphertextCommits.addCiphertextMaterial` | multi-coprocessor consensus adds (keyId, ctDigest, snsDigest) | same `CiphertextCommits` contract as EVM — no Solana-side host equivalent; `commit_handle_material` and `HandleMaterialCommitment` were deleted (DD-031) | **MET** (shared with EVM, not re-implemented on Solana) |
| `checkCiphertextMaterial` | material-present check | same `CiphertextCommits` check as EVM (DD-031) | **MET** (shared with EVM) |
| `InputVerification.verifyProofRequest/Response` (ZKPoK consensus) + `FHEVMExecutor.verifyInput` (tx-scoped transient allow, no persistent ACL) | coprocessor ZK-proof verify + consensus + EIP-712; verifyInput grants only a transient allow | `FheExecuteOperand::VerifiedInput` (the `FHE.fromExternal` analog) — consuming it in `fhe_execute` does on-chain secp256k1 recover + threshold-check of the coprocessor EIP-712 `CiphertextVerification` attestation and asserts `contract_chain_id == host chain id`; transient-allows the input for that execution, creates **no persistent ACL** (DD-007) | **MET** (parity with `verifyInput`: verify ≠ allow; Solana has no transient store so persistent perms are a separate explicit grant) + partial **SCOPE/PRODUCT-OPEN** (external proof/transciphering service, no `rejectProofResponse`) |
| `HandleOps`/`FHETypeBitSizes` | parse chainId/fheType; bit-size table | the same two libraries, called on the same code path: the Solana entrypoint's `_checkCtHandlesConformanceHostChain` in `Decryption.sol` extracts each handle's chain id and FHE type there. Nothing is re-implemented connector-side | **MET** (shared with EVM) |
| `MAX_DECRYPTION_REQUEST_BITS=2048` | per-request cleartext cap | enforced by the same gateway check the EVM path uses, in `_checkCtHandlesBitBudget`; the Solana entrypoint additionally caps a request at `MAX_SOLANA_USER_DECRYPT_HANDLES = 33` handles | **MET** (shared with EVM) |
| `Structs` (Sns material, delegation, pairs) | cross-contract DTOs | on-chain `EncryptedValue`/`UserDecryptionDelegation`, read and decoded by the connector through `zama-solana-acl` (material DTOs are gateway-side `CiphertextCommits` structs, unchanged by Solana, DD-031) | **MET** (re-modeled) |
| `GatewayConfig` (KMS/coprocessor/host-chain/threshold registry) | on-chain registry | connector reads registry off-chain (`Config`); `HostConfig` holds authorities/flags, the coprocessor signer set + threshold, and the current `KmsContext` pointer | **PRODUCT-OPEN** |
| `KMSGeneration` (keygen/crsgen ceremony) | key/CRS lifecycle | none (referenced only by `key_id`) | **PRODUCT-OPEN** |
| `ProtocolPayment` ($ZAMA fees) | per-request fee | none (rent/tx fees only) | **PRODUCT-OPEN** |

Two cross-cutting notes on the gateway routing above:

- **Batched entries.** A Solana user-decrypt request carries a list of entries, each naming one
  encrypted value account, one handle and (for a delegated entry) the delegator as allowed key;
  the connector builds one leaf-proof read for the whole request and authorizes every entry
  against its own account's peaks. The permit's scope test runs per entry, so a narrowly scoped
  permit cannot decrypt a foreign handle mixed into the batch.
- **Solana-over-EIP-712 is a bridge, not a committed end state.** Routing Solana decrypts
  through the EVM-shaped EIP-712 gateway (DD-012/DD-026) was chosen to reuse the audited
  gateway/KMS stack rather than build a parallel native one. Whether it remains the end state or
  is later replaced by a Solana-native request path is an open product decision; the Solana-side
  signed preimages (`zama-solana-permit`) are already chain-native, so the EIP-712 shape is
  confined to the gateway leg.

---

## 4. Coprocessor / KMS adapter integration

- **Coprocessor host-listener** (`coprocessor/.../host-listener/src/solana_adapter.rs` +
  `solana_reconstruct.rs` + `database/solana_leaves.rs`): reconstructs compute rows, handle-only
  ciphertext-material requests and the MMR leaves from confirmed Yellowstone transaction
  instructions plus streamed Clock/SlotHashes state, in one database transaction. Create, update,
  make-public, and persistent outputs carry the concrete handle(s) to prepare; material is prepared
  at handle creation and the KMS checks the leaf record before plaintext release
  (DD-025/DD-033/DD-034). The leaf record is served over `POST /v1/solana/leaf-proofs` behind an
  API key (`http_server.rs`, committed OpenAPI). The adapter maps the merged `fhe_execute`
  operator surface: binary catalog, ternary select, trivial, rand/rand-bounded, unary, sum, isIn,
  and mulDiv.
- **KMS connector** (`kms-connector/crates/kms-worker/src/core/solana/`): one authorization
  pipeline — strict decode, envelope signature, window, deployment identity, KMS pair, the
  confirmed account snapshot (two reads when an entry is delegated), pause, watermark, scope,
  the leaf-proof read from every configured coprocessor, handle binding against the account's own
  peaks, delegation freshness. Every proof comes from the coprocessors' leaf record and is verified
  with the shared `zama_solana_acl` crate; a client-supplied proof is rejected (DD-048). Decrypt
  reuses the unified Gateway V2 path with the host-generic user-decrypt overload and on-chain
  secp256k1 cert verification (DD-012/DD-021/DD-026). Account layouts, seeds and leaf
  commitments come from `zama-solana-acl`, the same crate the program compiles; the request and
  permit canons are the `zama-solana-request` / `zama-solana-permit` crates the relayer and SDK
  share, pinned by the committed byte vectors under `solana/test-fixtures/`.
- **Host-RPC read parity** — why the connector reading Solana state directly is not new trust: the
  upstream connector already reads every EVM host chain over RPC for decrypt authorization
  (per-chain `ACL` instances issuing `isAllowedForDecryption` / `isAllowed` eth_calls against the
  host `ACL.sol`). The Solana branch generalized that per-chain map to
  `HostChainAclBackend::{Evm, Solana}` — same component, same trust model (each KMS party
  trusts its own RPC endpoint on both chains), different read mechanics (eth_call vs
  `getAccountInfo` at confirmed commitment + local shared-crate decode with PDA/owner
  re-derivation). Nothing new was granted to the KMS; each party's connector always needed read
  access to every host chain it authorizes for.

**Adapters are present and integrated at the PoC boundary.** Live transport (production Geyser
provider, full KMS-connector wiring beyond the harness, optional reorg resource recovery) is
PRODUCT-OPEN by design (DD-024/DD-025/DD-028).

---

## 5. Critical assessment — solid vs fragile

**Solid (faithful, often stricter than EVM):** the ACL core (allows sealed on the write, one
leaf-proof path, authority = verified program PDA, append-only/no-generic-bind, on stable
`EncryptedValue` accounts with an MMR history, DD-032/DD-047/DD-048),
public-decrypt release gated on a dedicated role and never at creation (an exact-handle proof, not a live
flag), delegation lifecycle (slot expiry, same-slot guard, wildcard-delegate rejection), one-shot
transient capabilities with same-tx creation proof, preserved handle byte-layout,
operand-ACL discipline + scalar-RHS rule, ABI/account-meta exactness (DD-004, extensive negative
tests), the confidential-token flows (owner-authorized transfer/transfer-and-call split/wrap/burn/redeem/
disclose) with separate payer semantics and disclosure now a thin `disclose_secp` consumer of the
stateless host `verify_public_decrypt` verifier (DD-040, binds the disclosed encrypted value account to the mint
scope rather than label-scoping per-instruction), and the
connector's canonical-PDA + coprocessor-served MMR-proof verification (DD-048; materiality now lives in
the gateway's `CiphertextCommits`, DD-031).

**Fragile / attention for a security pass (PoC shortcuts, not correctness bugs):**

1. **Input verification against a registered n-of-m coprocessor signer set — CLOSED (DD-041).**
   `HostConfig` now stores a registered coprocessor signer set + configurable threshold
   (`coprocessor_signers` / `coprocessor_signer_count` / `coprocessor_threshold`), verified via
   `eip712::verify_threshold` — EVM `InputVerifier` parity, replacing the former single-signer /
   threshold-1 path. Admin-gated rotation via `set_coprocessor_signers`. Remaining forward work is the
   gateway-sync authority + the real proof/transciphering service behind the attestation
   (FUTURE_DESIGN §1), not the trust model. One intentional divergence: `verify_threshold`
   ignores signatures that recover to an address outside the signer set, where the EVM
   `InputVerifier`/`KMSVerifier` revert on any unknown signer. An outsider signature cannot
   raise the distinct-in-set count, so the threshold is enforced identically; skipping keeps a
   packet live across signer rotation instead of failing it (fhevm-internal#1888, won't fix).
2. **No host-side test/mock bypass remains.** The former `mock_input_verified_and_bind` input
   short-circuit, admin toggles, zero creation-entropy fallback, and event-only `test_emit_*`
   instructions were removed entirely (DD-014).
3. **No *global* per-block HCU plane.** The host enforces a per-application, per-slot HCU block cap
   (`HostConfig::hcu_block_cap_per_app`, keyed on the verified `(program, scope)`, DD-039/DD-047)
   plus per-execution total and critical-path caps
   (`max_hcu_per_tx` / `max_hcu_depth_per_tx`, `u64::MAX` = off) and the Solana compute budget.
   There is no EVM-style *global* per-block `HCULimit` aggregating across all apps; the per-app
   cap is the Solana analog. Per-op costs are the EVM `HCULimit.sol` tables through euint128
   (types 7/8 have no rows); a unit test parses that Solidity file and checks every shared cell in
   both directions, so neither side can drift silently. Limits ship disabled (`u64::MAX` =
   unrestricted). Relevant to DoS/cost-bounding.
4. **On-chain disclosure/redemption uses secp256k1 KMS-cert verification.** Both call the stateless
   host `verify_public_decrypt` verifier and have no request-witness accounts. Disclosure is
   idempotent; redemption additionally consumes the token account's single `PendingBurn`, so payout
   remains act-once (`disclose_secp`, `redeem_burned_amount`, DD-040/DD-045). The residual risk is
   off-chain integration of KMS certificate publication before the flow is production ready.
5. **The confirmed Yellowstone listener is not wired into the EVM reorg substrate**
   (DD-025/DD-028): it reconstructs instruction effects and inserts directly, bypassing the
   block-status machine. This is accepted for authorization because KMS revalidates confirmed live
   state before release; optional reorg unwind remains resource-recovery work. The residual
6. **Version-pinned ABI across repos.** The account layout, seeds and leaf commitments are one
   crate (`zama-solana-acl`) compiled into the program, the listener and the connector, so a layout
   change breaks the build rather than decoding at runtime; the vendored coprocessor IDL and the
   listener event constants are still mirrored and pinned by `solana/scripts/check_solana_abi.py`,
   `check-zama-host-idl.sh` and `check-pda-seeds.py` (the TypeScript seed mirrors). The account has
   no per-value capacity limit any more: allows are leaves, and the only bound is
   `MAX_MMR_PEAKS = 64` (2229 bytes for all time).
7. **`previous_bank_hash` is fail-closed.** When the prior bank hash is unavailable, handle creation
   returns `PreviousBankHashUnavailable`; tests must seed the real `Clock` and `SlotHashes` sysvars.
   Bank-hash + timestamp entropy is the resolved policy (keep per-block entropy, DD-015).
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
- **FHEVM host authorization model** (`EncryptedValue`+MMR ACL with allows sealed on the write,
  the verified application identity, input verification, transient, decrypt release through
  coprocessor-served leaf proofs, delegation): faithfully ported, frequently stricter.
  Materiality is no longer host-chain state on Solana — it is the gateway's `CiphertextCommits`,
  shared with EVM (DD-031).
- **Constraint-driven divergences** are all intentional and documented (no tstore, no `msg.sender`,
  shallow CPI, per-execution HCU cap vs per-block metering, account-witnesses vs storage reads). Input and
  KMS-cert verification use on-chain secp256k1 recovery — the same EIP-712 trust roots as EVM.
- **SCOPE** items (executor opcode breadth, full FheType set) are not Solana-limited and not needed
  by the token; mechanically extensible.
- **PRODUCT-OPEN** items (`ProtocolConfig`/`KMSGeneration`/`GatewayConfig`/payment, registered
  coprocessor signer set, production Geyser provider + reorg wiring, full KMS-connector wiring,
  external input proof service) are off-chain services or future work by design (DD-012, DD-003,
  DD-007, DD-025).
