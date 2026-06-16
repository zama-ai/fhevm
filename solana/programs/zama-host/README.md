# Zama Host Solana PoC

`zama-host` is the protocol-side Anchor program for the Solana FHEVM PoC. It owns host ACL state,
checks FHE operation authorization, emits generic host events, and provides the CPI surface used by
application programs such as `confidential-token`.

## State

```text
HostConfig
  PDA("host-config")
  stores chain id, admin authority, input verifier authority, material authority,
  test authority, pause state, and feature gates for mock/test/deny-list behavior

AclRecord
  PDA("acl-record", nonce_key, nonce_sequence)
  stores the opaque FHE handle, nonce metadata, inline subjects, subject roles,
  overflow count, and public_decrypt flag

AclPermission
  PDA("acl-permission", acl_record, subject)
  overflow witness for subjects beyond the inline capacity

HandleMaterialCommitment
  PDA("handle-material", host_config, acl_record)
  committed material witness for decryptability; stores key id, ciphertext digest,
  SNS digest, coprocessor-set digest, commitment hash, created slot, and state;
  the matching commitment account, hash, and key id are sealed onto the ACL record

DenySubjectRecord
  PDA("deny-subject", subject)
  optional grant deny-list witness when HostConfig enables deny-list checks

UserDecryptionDelegation
  PDA("delegation", delegator, delegate, app_context)
  PoC delegation state for future Gateway/KMS witness verification; counter-changing updates are
  slot-guarded to reject same-slot regrant/revoke races
```

Handles are stored in ACL records; they are not PDA seeds.

## Material Commitments

ACL records prove who may use or decrypt a handle. They do not prove that ciphertext material is
ready. `commit_handle_material` lets the configured `HostConfig::material_authority` create a
one-shot `HandleMaterialCommitment` PDA for an existing canonical ACL record whose handle targets
this host chain and uses a supported FHE type/version. The instruction also seals the material
commitment PDA, hash, and key id onto the ACL record. KMS-style native Solana decryption must verify
both records and require the sealed ACL fields to agree with the material commitment account.

## Instruction-Local Transients

`fhe_eval` composes mixed FHE steps in one host instruction: **Binary / Ternary / TrivialEncrypt /
Rand** (no `Input` step — DD-007/DD-023). Binary scratch results can feed ternary `if_then_else`, and
trivial-encrypt / rand births can participate in the same frame. Outputs produced earlier in the eval
can be referenced as transient operands by later operations.
Durable operands must still be authorized by canonical ACL records. Transient outputs do not create
`AclRecord`, `AclPermission`, or `AclAllowedEvent` state. Only outputs marked durable are bound into an
ACL record with the existing nonce-key model.

This is the supported replacement for the older `execute_frame` prototype, not a port of that ABI.
Keeping durable output authority on a signer witness (`app_account_authority` or an explicit
per-output authority account in `remaining_accounts`) preserves the role-aware ACL,
overflow-permission, material-commitment, and public-decrypt rules enforced by the current host without
reviving unsigned `authorized_app_accounts`.

For small frames, worker-replay events are emitted through Anchor event CPI. Larger frames emit the
same replay payloads through Anchor `Program data` logs to avoid self-CPI heap pressure. Durable ACL
metadata events (`AclRecordBoundEvent` and `AclSubjectAllowedEvent`) are always emitted as logs; the
listener accepts one replay transport per transaction and rejects mixed host CPI/log replay events.

Admission invariants for `fhe_eval`:

- `context_id` must be nonzero and the frame must contain 1 to 16 steps.
- Every dynamic account passed through `remaining_accounts` must be unique and referenced by an
  operand or output, and every referenced account index must be present.
- The optional instructions sysvar account must be present only for steps that need instruction
  witness checks, and when present its key must be the canonical instructions sysvar id.
- Transient operands may only reference outputs produced by earlier steps in the same frame.
- Only the RHS of a binary operation may be scalar; encrypted operands must match the operator's FHE
  type rules.
- There is no verified-input eval step; input verification is the separate `verify_coprocessor_input`
  instruction, which verifies the coprocessor attestation and emits a receipt only — it creates no ACL
  (DD-007).
- Durable outputs must be born with `public_decrypt = false`; public decrypt is granted later through
  the dedicated role-aware instruction path. `ACL_ROLE_PUBLIC_DECRYPT` in output subjects authorizes
  that later explicit path; it does not set the durable record's `public_decrypt` flag at birth.
- When a derived durable output grants `ACL_ROLE_PUBLIC_DECRYPT` without any input carrying that role,
  the output authority must be an initialized non-system app account. This blocks direct/system-owned
  callers from laundering compute/use permission into future disclosure authority while preserving
  app-owned token output policies.

## External Inputs

> Superseded (reconciliation, June 2026): the Ed25519 verifier-set path described in older revisions
> was REMOVED. See `solana/docs/DESIGN_DECISIONS.md` DD-007.

`verify_coprocessor_input` is the production-shaped encrypted-input *verification* path. It verifies
the **coprocessor's EIP-712 `CiphertextVerification` attestation on-chain via secp256k1** (recovering
the EVM coprocessor signers and threshold-checking them against the configured coprocessor signer set),
then emits an `InputVerifiedEvent` receipt. It does **not** create a persistent ACL record and takes no
`output_*` / ACL-binding parameters — matching the EVM `FHEVMExecutor.verifyInput`, which grants only a
tx-scoped transient allow. Solana has no transient-storage analog, so the verified input is surfaced
solely as the signed receipt; durable permission on an input handle is a separate explicit app grant.
This mirrors the EVM `InputVerification` coprocessor-threshold model; the gateway counterpart is the
RFC-021 bytes32 path `InputVerification.verifyProofRequestSolana`. There is no `FheEvalStep::Input` —
input verification is its own instruction, not an eval step. (Fully using a verified input as a compute
operand still needs the host-listener to consume `InputVerifiedEvent`, a follow-up.)

`mock_input_verified_and_bind` remains local-PoC test-only glue, chain-id confined (DD-014). The
removed `verify_input_and_bind` (native Ed25519 over `SolanaInputProof` + `SolanaInputBindIntent`,
anchored to a Solana input verifier set) is retained only as the superseded design in DD-007.

## Roles

Inline and overflow subjects carry role flags:

```text
ACL_ROLE_USE
ACL_ROLE_COMPUTE
ACL_ROLE_GRANT
ACL_ROLE_PUBLIC_DECRYPT
ACL_ROLE_USER
```

Compute signers should be granted `ACL_ROLE_USE | ACL_ROLE_COMPUTE`, while owner/user subjects
remain full ACL subjects. Persistent grants require `ACL_ROLE_GRANT`. Public decrypt requires
`ACL_ROLE_PUBLIC_DECRYPT`; compute-only subjects cannot extend ACL membership or flip
`public_decrypt`.

## Test-Only Entrypoints

`mock_input_verified_and_bind` and `test_emit_*` are not protocol APIs. They require `HostConfig`
feature gates and the configured authority signer:

```text
mock_input_verified_and_bind -> mock_input_enabled + local PoC chain id (DD-014)
test_emit_*                  -> test_shims_enabled + test_authority
```

Threshold policy and real proof/transciphering validation are still external/open design items.
Trivial and random handle birth paths include output nonce metadata in handle derivation before
binding the result into a canonical ACL record. Random handle birth is covered by
`fhe_rand_and_bind` and `fhe_rand_bounded_and_bind`, which derive the event seed on-chain.
