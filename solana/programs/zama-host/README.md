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

`fhe_eval` composes mixed FHE steps in one host instruction. Binary scratch results can feed ternary
`if_then_else`, while trivial-encrypt, rand, and verified-input births can participate in the same
frame. Outputs produced earlier in the eval can be referenced as transient operands by later
operations.
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
- Verified input steps must bind a durable output immediately and require the instructions sysvar for
  the Ed25519 verifier witness.
- Durable outputs must be born with `public_decrypt = false`; public decrypt is granted later through
  the dedicated role-aware instruction path. `ACL_ROLE_PUBLIC_DECRYPT` in output subjects authorizes
  that later explicit path; it does not set the durable record's `public_decrypt` flag at birth.
- When a derived durable output grants `ACL_ROLE_PUBLIC_DECRYPT` without any input carrying that role,
  the output authority must be an initialized non-system app account. This blocks direct/system-owned
  callers from laundering compute/use permission into future disclosure authority while preserving
  app-owned token output policies.

## External Inputs

`verify_input_and_bind` is the production-shaped encrypted-input birth path. It accepts a
`SolanaInputProof`, requires the immediately preceding transaction instruction to be Solana's native
the canonical verifier-set-bound `input_proof_message` bytes for the proof and
`SolanaInputBindIntent`. The selected proof handle must match the requested input handle, and every
proof handle must carry this host chain id, a supported FHE type, its proof index in byte 21, and the
current handle version before the instruction writes the canonical ACL record named by the signed
bind intent.

`mock_input_verified_and_bind` remains local-PoC test-only glue for cases that do not need verifier
transaction witnesses. It is still constrained to the active verifier set by requiring a signer from
that set.

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
mock_input_verified_and_bind -> mock_input_enabled + local PoC chain id + active verifier-set signer
test_emit_*                  -> test_shims_enabled + test_authority
```

Threshold policy and real proof/transciphering validation are still external/open design items.
Trivial and random handle birth paths include output nonce metadata in handle derivation before
binding the result into a canonical ACL record. Random handle birth is covered by
`fhe_rand_and_bind` and `fhe_rand_bounded_and_bind`, which derive the event seed on-chain.
