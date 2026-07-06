# MMR ACL MVP

This is the canonical reviewer map for the Solana `EncryptedValue` + MMR ACL MVP. The detailed
rationale lives in DD-031 through DD-035 in [`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md);
this note records the operational model in one place.

## Identity And Authority

- `EncryptedValue` identity is `derive_value_key(acl_domain_key, app_account, encrypted_value_label)`.
  The derived key prevents collisions between app domains, accounts, and labels. It is not an
  authority check.
- `compute_signer` is separate from identity. In confidential-token it is a mint-scoped PDA and must
  be present in the value's allowed-subject set when token compute needs to use that value.
- Only the app-account authority can update `current_handle`. Being an allowed subject is enough to
  compute/use, grant, request user decrypt, or make the exact current handle public, but it is not
  enough to supersede the lineage. `update_encrypted_value` checks `previous_handle` and
  `previous_subjects` against current account state so stale off-chain state cannot rotate a handle.

## Allowed Subjects

- The ACL is one allowed-subject set. There are no role bits in the MVP account layout. Allowed means
  compute/use, grant another subject, request user decrypt, and make the exact current handle public.
- Creation requires at least one subject and at most `MAX_ENCRYPTED_VALUE_SUBJECTS = 8`. Subject-list
  overflow and MMR peak overflow fail explicitly instead of relying on implicit vector or arithmetic
  limits.
- Subject removal changes only current and future authorization. No new historical leaf is written for
  the removed subject after removal; access sealed before removal remains valid.

## History And Decrypt

- Historical authorization is handle-scoped and permanent. When a handle is superseded, the program
  seals one `HistoricalAccessLeaf` per then-allowed subject into the value's MMR. Historical reads roll
  forward by proving inclusion against finalized on-chain peaks.
- Public decrypt is exact-handle. `make_handle_public` seals a `PublicDecryptLeaf` for the current
  handle only; a later handle update does not inherit public decryptability.
- Delegated user decrypt is isolated from the core ACL path. Delegation uses standalone
  `UserDecryptionDelegation` PDAs and does not add subjects or mutate `EncryptedValue`.

## Gates And Trust Boundary

- Pause gates ACL mutations plus update/eval output paths. The deny-list gates the acting
  caller/authority for grant/update/eval flows; it blocks new action and is not an erasure mechanism
  for already sealed history.
- Solana programs enforce authorization. The relayer, proof builder, host-listener ingestion, and
  coprocessor scheduling are untrusted for authorization. KMS release must verify finalized on-chain
  facts, including live `EncryptedValue` state or MMR proof validity, before releasing plaintext.
- Materiality is not Solana host state. DD-031 moved ciphertext material commitments to the gateway
  `CiphertextCommits`; Solana ACL state answers only who may use or decrypt a handle.
- The relayer-colocated MMR proof service is an untrusted helper (DD-035). Current limitation: the
  relayer's Solana user-decrypt path does not yet call the proof builder in-process; proofs are
  attached out of band through the interim proof-service path.

## Decision Links

- DD-031: deleted host-owned `HandleMaterialCommitment`; materiality lives in `CiphertextCommits`.
- DD-032: introduced stable `EncryptedValue` lineages, single allowed-subject ACL, and MMR leaves.
- DD-033: lifecycle instructions emit no events; indexers replay instruction data.
- DD-035: proof building is relayer-colocated and untrusted; KMS re-verifies proofs against finalized
  chain state.
