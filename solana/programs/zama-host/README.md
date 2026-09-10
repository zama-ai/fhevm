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

## Composed FHE execution

A client calls `open_transient_store`, submits application instructions, then calls `close_transient_store` as the
final top-level instruction. The SDK transaction wrapper inserts both lifecycle instructions. Applications forward the same transient store and Instructions sysvar through every host CPI.
`TransientStore` records all produced handles, their producing State, cumulative HCU depth, and explicit cross-State grants.
Its payer funds rent and receives the refund; the payer acquires no handle permission.

`fhe_execute` takes an explicit canonical producing `EncryptedState` and that State authority's signature. It runs
1–32 ordered operations: Binary, Ternary, Unary, TrivialEncrypt, Rand, RandBounded, Sum, IsIn and MulDiv. The Rust SDK
builds typed expressions and attaches separate output effects:

```rust
let execution = FheExecution::build(state.id(), |fhe| {
    let next = fhe.add(balance, amount)?;
    fhe.output(next, state.set(balance_key).allow(owner))?;
    Ok(())
})?;
```

A current-call `EarlierStep` names one produced occurrence. A later call by the same State can use the handle from
transient store without another grant. A different State must receive an explicit `StateOutput::allow_transient` grant and
its authority must sign consumption. A slot operand requires its authority's signature and the expected current
handle. Other participating signatures do not implicitly share every intermediate result.

All arithmetic reads the execution's initial State snapshots. Up to 32 ordered effects then optionally write slots,
append private/public MMR leaves and grant consumers. A slot may be written once per execution; its previous handle
and the ordered shared leaf count are checked. Effects can authorize a fresh result without storing it in a slot.
Historical re-sharing remains outside this interface.

`returned_results` selects up to 32 `(step_index, output_index)` entries; output index is currently zero. Requested
order and duplicates are preserved, and empty selection returns no handles. Callers read return data immediately
after CPI; a subsequent CPI or the final close can overwrite the return channel. Return bytes grant no permission.

Every operand-bearing handle preimage includes a mask derived from earlier production in the transaction. Slot,
grant and earlier-step witnesses all use the same origin rule. The listener reconstructs operations and effects in
order from instruction data; random operations also use their execution's `FheExecuteRandomSeedsEvent`.

The host checks pause, canonical accounts, signers, all touched application deny records, operand types and current
slot handles. Transaction HCU total and depth span calls and applications; block meters charge each execution to
its producing State's `(program, scope)`. Random steps require the host nonce. All failures roll back atomically.
Resource bounds and their SBF measurements are documented in `docs/INVARIANTS.md` and runtime cost snapshots.

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
