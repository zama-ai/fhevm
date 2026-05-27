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

`fhe_eval` composes multiple FHE binary operations in one host instruction. Durable operands must
still be authorized by canonical ACL records, but outputs produced earlier in the same eval can be
referenced as transient operands by later operations. Transient outputs do not create `AclRecord`,
`AclPermission`, or `AclAllowedEvent` state. Only outputs marked durable are bound into an ACL record
with the existing nonce-key model.

## External Inputs

`verify_input_and_bind` is the production-shaped encrypted-input birth path. It accepts a
`SolanaInputProof`, requires the immediately preceding transaction instruction to be Solana's native
Ed25519 verifier, and checks that `HostConfig::input_verifier_authority` signed the canonical
`input_proof_message` bytes for the proof and `SolanaInputBindIntent`. The selected proof handle
must match the requested input handle, and every proof handle must carry this host chain id, a
supported FHE type, its proof index in byte 21, and the current handle version before the instruction
writes the canonical ACL record named by the signed bind intent.

`mock_input_verified_and_bind` remains test-only glue for cases that do not need verifier
transaction witnesses.

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
mock_input_verified_and_bind -> mock_input_enabled + input_verifier_authority signer
test_emit_*                  -> test_shims_enabled + test_authority
```

Threshold policy and real proof/transciphering validation are still external/open design items.
Trivial and random handle birth paths include output nonce metadata in handle derivation before
binding the result into a canonical ACL record. Random handle birth is covered by
`fhe_rand_and_bind` and `fhe_rand_bounded_and_bind`, which derive the event seed on-chain.
