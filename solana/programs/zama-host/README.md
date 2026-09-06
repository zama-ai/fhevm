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
  PDA("encrypted-value", program, encrypted_value_account_authority, scope, label)
  one stable PDA per logical encrypted value, reused across every handle update; stores its
  four identity seeds, current_handle, and an on-account keccak Merkle Mountain Range
  (peaks + leaf_count) sealing one HistoricalAccessLeaf per key allowed on every handle a
  write installs, and a PublicDecryptLeaf when a handle is made public. It stores nothing about
  who may decrypt: every allow is a leaf. `program` is verified on every write (the authority
  must be a PDA of it); `scope` is what that program declares. 181 + 32·peaks bytes, at most
  2229. See `solana/crates/zama-solana-acl/src/lib.rs` for the shared MMR/leaf-commitment math
  and `docs/DESIGN_DECISIONS.md` DD-032/DD-047/DD-048 for the rationale.

DenyScopeRecord
  PDA("deny-scope", program, scope)
  optional deny-list witness for one application when HostConfig enables deny-list checks;
  gates every allow the host would seal for it

HcuBlockMeter / HcuTrustedAppRecord
  PDA("hcu-block-meter", program, scope) / PDA("hcu-trusted", program, scope)
  the per-slot HCU budget and admin trust record of one application

RandNonce
  PDA("rand-nonce")
  the host's global rand counter; every execution with a rand step passes and advances it

UserDecryptionDelegation
  PDA("user-decryption-delegation", delegator, delegate, encrypted_value_account_authority)
  read by the KMS connector on a delegated decrypt (INVARIANTS #27); counter-changing updates are
  slot-guarded to reject same-slot regrant/revoke races
```

Handles are stored inside `EncryptedValue` accounts; they are not PDA seeds.

## Materiality

Whether ciphertext material is available and bound to the right key is no longer host-chain state on
Solana. The earlier `HandleMaterialCommitment` subsystem (`commit_handle_material`) was deleted;
materiality is now the gateway's `CiphertextCommits`, where the coprocessor already registers Solana
handles (`docs/DESIGN_DECISIONS.md` DD-031). `EncryptedValue` only answers "who may use or decrypt
this handle" — an MMR-proven allow or public-decrypt leaf.

## Instruction-Local Transients

`fhe_execute` composes mixed FHE steps in one host instruction: **Binary / Ternary / Unary /
TrivialEncrypt / Rand / RandBounded / Sum / IsIn / MulDiv** (no `Input` step — DD-007/DD-023). Binary scratch results can feed ternary
`if_then_else`, and trivial-encrypt / random creations can participate in the same execution. Outputs
produced earlier in the execution can be referenced as transient operands by later operations.
A stored operand is admitted by its value authority's signature. Transient outputs create no
`EncryptedValue` state at all. Only outputs marked persistent create (first bind) or update
(subsequent binds) an `EncryptedValue`, carrying `previous_handle` on update and the keys allowed on
the new handle, so every transaction stays independently interpretable
(`docs/DESIGN_DECISIONS.md` DD-032/DD-033/DD-048).

This is the supported replacement for the older `execute_frame` prototype, not a port of that ABI.
Keeping persistent output authority on a signer witness (the `encrypted_value_account_authority` signer, or an
explicit per-output authority account in `remaining_accounts`), and proving that signer is a PDA of
the declared `program`, is what makes the application identity unforgeable (DD-047). The
prototype's unsigned list of authorized encrypted value account authorities is deliberately not
revived.

Ordinary compute facts, MMR leaves, and persistent-output binds are reconstructed from instruction data;
the host emits no per-operation replay stream. An execution with created-public persistent outputs emits exactly
one versioned Anchor CPI lifecycle execution containing their ordered step index, host-owned
`EncryptedValue` account, and host-derived output handle. An execution without created-public outputs emits
no lifecycle execution. The bounded 16-output maximum fits one CPI; other `EncryptedValue` lifecycle
paths remain event-free (`docs/DESIGN_DECISIONS.md` DD-033/DD-038).

Admission invariants for `fhe_execute`:

- The execution must contain 1 to 32 steps (`MAX_FHE_EXECUTION_STEPS`), and an execution with a rand
  step must pass the `RandNonce` account (the rand seed is anchored to the host's counter, which
  the execution advances).
- Every persistent output's authority must be a PDA of the output's declared `program`, proven by
  the declared seeds (`EncryptedValueAuthorityNotProgramPda`), and every stored operand and output
  must carry the same `(program, scope)` (`FheExecuteMixedScopes`).
- Every dynamic account passed through `remaining_accounts` must be unique and referenced by an
  operand or output, and every referenced account index must be present.
- The optional instructions sysvar account must be present only for steps that need instruction
  witness checks, and when present its key must be the canonical instructions sysvar id.
- Transient operands may only reference outputs produced by earlier steps in the same execution.
- Only the RHS of a binary operation may be scalar; encrypted operands must match the operator's FHE
  type rules.
- External encrypted inputs enter compute through the `FheExecuteOperand::VerifiedInput` operand: the
  coprocessor attestation is re-verified in-execution and the input is transient-allowed for that execution only
  (the EVM `fromExternal` / `allowTransient(input, msg.sender)` analog). The caller-is-contract gate is
  checked at input consumption (`attestation.contract_address == program`, the execution's verified
  application program); derived outputs are unconstrained. The redundant standalone
  `verify_coprocessor_input` instruction was removed (DD-007).
- Persistent outputs declare the keys allowed on the handle they install; the host seals one leaf
  per key. Public decrypt is never a live flag; it is granted by `make_handle_public`, or at
  persistent-output creation when `make_public=true`, which appends an exact-handle
  `PublicDecryptLeaf` to the encrypted value account MMR after the allows.

## External Inputs

The `FheExecuteOperand::VerifiedInput` operand is the production encrypted-input path (the Solana
`FHE.fromExternal` analog). When an `fhe_execute` step consumes it, the host re-verifies the
**coprocessor's EIP-712 `CiphertextVerification` attestation on-chain via secp256k1** (recovering the
EVM coprocessor signers and threshold-checking them against the configured coprocessor signer set),
asserts the attested `contract_chain_id` equals the host chain id (EVM's `contractChainId ==
block.chainid`), and transient-allows the input for that execution only — no persistent ACL, matching
`FHEVMExecutor.verifyInput` + `allowTransient(result, msg.sender)`. The "caller is the attested
contract" check is enforced at consumption: `attestation.contract_address` must equal the
execution's application `program`, which the host has verified from the output authority's seeds
(DD-047) — the msg.sender analog, and exactly the EVM binding (a program id, not a signer). Derived
persistent outputs are **not tainted** by the input — any persistent output's allows are the app's
separate explicit choice, exactly like EVM.

`user_address` is **not** EVM `msg.sender`; checking the attested `user_address` is **app policy**
(confidential-token checks the attested user equals the token account owner). An app that skips that
check lets an observer replay another user's verified input.

This mirrors the EVM `InputVerification` coprocessor-threshold model; the gateway counterpart is the
RFC-021 bytes32 path `InputVerification.verifyProofRequestSolana`. The host-listener reconstruct path
resolves the operand from `attestation.input_handle`. The shared verifier is
`eip712::verify_coprocessor_input` (via `instructions::input_verification::verify_input_attestation`);
the earlier standalone `verify_coprocessor_input`/`verify_input_and_bind`/`mock_input_verified_and_bind`
instructions and the `InputVerifiedEvent` receipt were removed. The former Ed25519 verifier-set path
is retained only as the replaced-design stub in DD-007.

## ACL Model

The account keeps no list of who may decrypt. **Use** is the authority's: a stored value is read
into a computation, written, or made public only under the signature of its
`encrypted_value_account_authority`, a PDA of the value's `program`. **Decrypt** is a leaf: a write
declares the keys allowed on the handle it installs and the host seals one `HistoricalAccessLeaf`
per key, in list order, then a `PublicDecryptLeaf` when the output is `make_public`;
`make_handle_public` seals the same leaf for the current handle later. A key so allowed is a
viewer — it decrypts that handle, current or replaced, by proving its leaf — and nothing more: it
cannot grant peers, seal the handle public, or write. There is no instruction to add or remove an
allow after the write; changing viewers is the authority's next write (confidential-token's
`allow_balance_viewers` / `allow_total_supply_viewers` are exactly that). Every allow the host
seals passes the deny list for the value's application when the list is enabled. Public
decryptability is represented only by `PublicDecryptLeaf`; it never rolls forward to later handles.
(DD-047/DD-048; INVARIANTS #10, #11.)

## Test setup

The host has no test-only verification or handle-creation path. Tests that create handles seed the
`Clock` and `SlotHashes` sysvars; missing previous-bank entropy fails closed exactly as it does in a
deployed program (DD-014). Registered-signer threshold policy and real proof/transciphering
validation are still external/open design items.
Trivial and random handle creation paths (now `fhe_execute` `TrivialEncrypt`/`Rand`/`RandBounded` steps —
the standalone `trivial_encrypt_and_bind`/`fhe_rand*_and_bind` instructions were removed) include
output entropy in handle derivation before binding the result into an `EncryptedValue`.
