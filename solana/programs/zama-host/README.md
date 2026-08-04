# Zama Host Solana PoC

`zama-host` is the protocol-side Anchor program for the Solana FHEVM PoC. It owns host ACL state,
checks FHE operation authorization, emits generic host events, and provides the CPI surface used by
application programs such as `confidential-token`.

## State

```text
HostConfig
  PDA("host-config")
  stores chain id, gateway chain id, protocol authorities (admin, coprocessor signer set +
  threshold, decryption contract, input verification contract), current KMS context pointer,
  pause state, HCU limits (per-tx, per-depth, per-app-per-slot block cap), and the
  persistent-grant deny-list policy

EncryptedValue
  PDA("encrypted-value", encrypted_value_id)
  encrypted_value_id = derive_encrypted_value_id(domain, encrypted_value_account_authority, label)
  one stable PDA per logical encrypted value, reused across every handle update; stores
  current_handle, inline allowed subjects (up to MAX_ENCRYPTED_VALUE_SUBJECTS=8),
  and an on-account SHA-256 Merkle Mountain Range (peaks + leaf_count) sealing one
  HistoricalAccessLeaf per allowed subject on every handle update, and a
  PublicDecryptLeaf on every make_handle_public call. See
  `solana/crates/zama-solana-acl/src/lib.rs` for the shared MMR/leaf-commitment math and
  `docs/DESIGN_DECISIONS.md` DD-032 for the rationale (replaces the earlier keyed-nonce
  AclRecord/AclPermission model).

DenySubjectRecord
  PDA("deny-subject", subject)
  optional grant deny-list witness when HostConfig enables deny-list checks

UserDecryptionDelegation
  PDA("delegation", delegator, delegate, app_context)
  PoC delegation state for future Gateway/KMS witness verification; counter-changing updates are
  slot-guarded to reject same-slot regrant/revoke races
```

Handles are stored inside `EncryptedValue` accounts; they are not PDA seeds.

## Materiality

Whether ciphertext material is available and bound to the right key is no longer host-chain state on
Solana. The earlier `HandleMaterialCommitment` subsystem (`commit_handle_material`) was deleted;
materiality is now the gateway's `CiphertextCommits`, where the coprocessor already registers Solana
handles (`docs/DESIGN_DECISIONS.md` DD-031). `EncryptedValue` only answers "who may use or decrypt
this handle" — current membership, or an MMR-proven historical/public-decrypt claim.

## Instruction-Local Transients

`fhe_execute` composes mixed FHE steps in one host instruction: **Binary / Ternary / Unary /
TrivialEncrypt / Rand / RandBounded / Sum / IsIn / MulDiv** (no `Input` step — DD-007/DD-023). Binary scratch results can feed ternary
`if_then_else`, and trivial-encrypt / random creations can participate in the same execution. Outputs
produced earlier in the eval can be referenced as transient operands by later operations.
Persistent operands must still be authorized by a live or historically-proven `EncryptedValue`. Transient
outputs create no `EncryptedValue` state at all. Only outputs marked persistent create (first bind) or
update (subsequent binds) an `EncryptedValue`, carrying `previous_handle`/`previous_subjects`
attestation on update so every transaction stays independently interpretable
(`docs/DESIGN_DECISIONS.md` DD-032/DD-033).

This is the supported replacement for the older `execute_frame` prototype, not a port of that ABI.
Keeping persistent output authority on a signer witness (the `encrypted_value_account_authority` signer, or an
explicit per-output authority account in `remaining_accounts`) preserves the membership-based ACL
and public-decrypt rules enforced by the current host. The prototype's unsigned list of authorized
app accounts is deliberately not revived.

Ordinary compute facts, MMR leaves, and persistent-output binds are reconstructed from instruction data;
the host emits no per-operation replay stream. An execution with created-public persistent outputs emits exactly
one versioned Anchor CPI lifecycle execution containing their ordered step index, host-owned
`EncryptedValue` account, and host-derived output handle. An execution without created-public outputs emits
no lifecycle execution. The bounded 16-output maximum fits one CPI; other `EncryptedValue` lifecycle
paths remain event-free (`docs/DESIGN_DECISIONS.md` DD-033/DD-038).

Admission invariants for `fhe_execute`:

- The execution must contain 1 to 16 steps, and an execution with a rand step must declare at least one
  persistent output (the rand seed is anchored to the execution's persistent writes).
- Every dynamic account passed through `remaining_accounts` must be unique and referenced by an
  operand or output, and every referenced account index must be present.
- The optional instructions sysvar account must be present only for steps that need instruction
  witness checks, and when present its key must be the canonical instructions sysvar id.
- Transient operands may only reference outputs produced by earlier steps in the same execution.
- Only the RHS of a binary operation may be scalar; encrypted operands must match the operator's FHE
  type rules.
- External encrypted inputs enter compute through the `FheExecuteOperand::VerifiedInput` operand: the
  coprocessor attestation is re-verified in-execution and the input is transient-allowed for that eval only
  (the EVM `fromExternal` / `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is
  checked at input consumption (`attestation.contract_address == compute_subject`); derived outputs are
  unconstrained. The redundant standalone `verify_coprocessor_input` instruction was removed (DD-007).
- Persistent outputs are created with an allowed-subject set. Public decrypt is never a live flag or subject
  attribute; it is granted by `make_handle_public`, or at persistent-output creation when `make_public=true`,
  which appends an exact-handle `PublicDecryptLeaf` to the encrypted value account MMR.

## External Inputs

The `FheExecuteOperand::VerifiedInput` operand is the production encrypted-input path (the Solana
`FHE.fromExternal` analog). When an `fhe_execute` step consumes it, the host re-verifies the
**coprocessor's EIP-712 `CiphertextVerification` attestation on-chain via secp256k1** (recovering the
EVM coprocessor signers and threshold-checking them against the configured coprocessor signer set),
asserts the attested `contract_chain_id` equals the host chain id (EVM's `contractChainId ==
block.chainid`), and transient-allows the input for that eval only — no persistent ACL, matching
`FHEVMExecutor.verifyInput` + `allowTransient(result, msg.sender)`. The "caller is the attested
contract" check is enforced at consumption (`attestation.contract_address` must equal the eval's
`compute_subject`, the msg.sender analog); derived persistent outputs are **not tainted** by the input —
any persistent output ACL is the app's separate explicit choice, exactly like EVM.

The EVM `contractAddress` analog is the consuming program's **compute-authority PDA** — a PDA the
program signs with via `invoke_signed` (in confidential-token, the `[b"fhe-compute", mint]` compute
signer), never a user key and never the bare program id (program ids cannot sign). The host only
enforces `contract_address == compute_subject` (any signer); binding the attestation to that PDA and
checking the attested `user_address` are **app policy** (confidential-token checks the attested user
equals the token account owner), mirroring EVM where `userAddress` is attested but the contract
decides its meaning. Per-state-account (per-mint) scoping is deliberate and finer-grained than EVM's
per-contract binding.

This mirrors the EVM `InputVerification` coprocessor-threshold model; the gateway counterpart is the
RFC-021 bytes32 path `InputVerification.verifyProofRequestSolana`. The host-listener reconstruct path
resolves the operand from `attestation.input_handle`. The shared verifier is
`eip712::verify_coprocessor_input` (via `instructions::input_verification::verify_input_attestation`);
the earlier standalone `verify_coprocessor_input`/`verify_input_and_bind`/`mock_input_verified_and_bind`
instructions and the `InputVerifiedEvent` receipt were removed. The former Ed25519 verifier-set path
is retained only as the replaced-design stub in DD-007.

## ACL Model

`EncryptedValue.subjects` is the complete MVP ACL for **use**: if a subject is in the set, it can use the
current handle in `fhe_execute`, request user decrypt, and call `make_handle_public` for the exact
current handle. Subject-list mutation (`allow_subjects` / `remove_subject`) is **not** subject-admin:
the signer must equal `EncryptedValue.encrypted_value_account_authority` (same gate as persistent create/update;
fhevm-internal#1862 #13). Apps that store a PDA in `account` rotate auditors by CPI + `invoke_signed`
as that PDA (confidential-token: `allow_token_account_subjects` /
`remove_token_account_subject` for token-account-scoped values; total-supply follow-up).
If a subject is not in the set, it cannot use the handle or make it public.

`allow_subjects` is append-only and idempotent for existing subjects. Its authority must equal
`EncryptedValue.encrypted_value_account_authority`, and deny-list/pause checks still apply. Updating a handle seals one
`HistoricalAccessLeaf` per allowed subject in current order. Public decryptability is represented
only by `PublicDecryptLeaf`; it never rolls forward to later handles.

## Test setup

The host has no test-only verification or handle-creation path. Tests that create handles seed the
`Clock` and `SlotHashes` sysvars; missing previous-bank entropy fails closed exactly as it does in a
deployed program (DD-014). Registered-signer threshold policy and real proof/transciphering
validation are still external/open design items.
Trivial and random handle creation paths (now `fhe_execute` `TrivialEncrypt`/`Rand`/`RandBounded` steps —
the standalone `trivial_encrypt_and_bind`/`fhe_rand*_and_bind` instructions were removed) include
output entropy in handle derivation before binding the result into an `EncryptedValue`.
