# Transient Allow On Solana

> **Status (Group A, zama-ai/fhevm#2834 / fhevm-internal#1543):** the persisted one-shot
> `TransientSession`/capability tier described below has been **removed**. The lifetime model is
> now `{AllowedLocal, AllowedDurable}`: an `fhe_eval`-local value (`AllowedLocal`) or a durable ACL
> record (`AllowedDurable`). EVM `allowTransient` (tx-scoped) maps to `AllowedLocal` + CPI signer
> propagation within one instruction's CPI tree; there is no analog for passing a not-yet-durable
> value across separate top-level instructions. The text below is retained as design rationale.

**Scope:** how to carry the EVM `FHE.allowTransient(handle, account)` idea into the Solana ACL
design without creating durable ACL or decrypt authority by accident.

## Decision

Support three patterns, but guide SDKs, app libraries, and host APIs toward the cheapest safe
primitive.

```text
Can the whole expression run inside one host instruction?
  yes -> use instruction-local intermediates
  no  -> continue

Can the app authorize existing inputs through a propagated compute signer?
  yes -> use signer propagation
  no  -> continue

Does a handle need temporary access across an instruction or program boundary?
  yes -> use a one-shot transient capability account
  no  -> create durable ACL only if persistence is intended
```

- **Option A: batch execution** is the default for internal expression graphs.
  Use `fhe_eval` when scratch values need to feed later binary/ternary steps before a durable
  output is bound. Trivial-encrypt and rand births can coexist in the same frame and produce
  transient or durable outputs. (Input verification is no longer an eval step; it is the separate
  `verify_coprocessor_input` instruction, which verifies the coprocessor attestation and emits an
  `InputVerifiedEvent` receipt — it creates NO persistent ACL, matching the EVM transient model where
  `verifyInput` grants only a tx-scoped allow. Since Solana has no transient-storage analog, any durable
  permission on an input handle is a separate explicit app grant — DD-007/DD-023.)
  App-side code should use `zama-fhe::EvalBuilder` so transient producer indices,
  host account indices, output types for common operations, durable output metadata, and
  signer/writable roles are generated consistently; with the `cpi` feature, the app can execute an
  opaque `EvalPlan` through a pubkey-keyed account resolver instead of maintaining host account
  order itself.
- **Option B: signer propagation** is the preferred Solana-native CPI-chain authorization model
  for existing allowed inputs.
- **Option C: one-shot capability account** is the escape hatch for real cross-instruction or
  cross-program handle handoff.

## Rationale

EVM transient allowance is backed by transaction-local storage. Solana has no equivalent hidden
transaction-local map. A later instruction can only verify explicit inputs such as accounts,
instruction data, signer privileges, sysvars, or return data.

Therefore, intermediate FHE values should stay instruction-local whenever possible. For a flow
like:

```text
tmp = encrypted_balance + encrypted_delta
out = tmp - fee
persist out only
```

`tmp` should not become account state, require rent, or be reusable later.

Signer propagation is useful when the same compute subject can be carried through the CPI chain,
but it is not a general `(handle, receiver)` grant and does not by itself authorize a freshly
produced handle in a later instruction.

## One-Shot Capability Account

Use a host-owned capability account only when signer propagation is not expressive enough. It
should be narrow:

```text
TransientCapabilityAccount
  authority
  refund_recipient
  subject
  receiver_program_context
  handle
  session_nonce
  state: Open | Consumed | Closed
  output_policy
```

Required defaults:

- exactly one capability per account;
- exactly one successful consume;
- consume before authorizing the FHE operation;
- close during consume when practical;
- if close is separate, `Consumed` or expiry must already make it unusable;
- never itself valid as a KMS or user-decrypt witness.

Do not call this "transient storage." It is real Solana state with strict one-shot semantics.

## Transaction Boundary

Same-slot is not same-transaction. Two transactions in the same slot can observe the same slot
value.

A capability account may claim same-transaction behavior only when the host can prove the matching
creation context, for example through a verified earlier top-level host init instruction. Slot
expiry is defense in depth, not the transaction boundary.

The PoC enforces this by requiring consume to see an earlier top-level `create_transient_session`
for the same session account in the current transaction's instructions sysvar. The current receiver
program is still checked at consume time.

## Durable Output Policy

A temporary compute grant must not become a broad durable grant. Any durable output created from
transient inputs must pass an explicit output policy binding:

- output ACL domain;
- app account;
- allowed subject roles;
- public decrypt flag permission;
- source-origin restrictions from every transient input.

If the policy does not explicitly allow durable output, the host must reject.
The current PoC does not let a transient capability carry public-decrypt authority or set the
durable output's `public_decrypt` flag. Ordinary app-authorized eval outputs may still grant
`ACL_ROLE_PUBLIC_DECRYPT` as ACL role metadata so the owner can later request disclosure, but every
durable record starts with `public_decrypt = false`; a later role-aware instruction must set the flag
explicitly.
If no input propagated the public-decrypt role, the host only accepts that future-disclosure role on
derived durable outputs when the output authority is an initialized non-system app account. Direct
system-owned callers cannot manufacture public-decrypt role metadata from compute/use-only inputs.

## Final Position

Use instruction-local transient values for internal expression graphs. Use signer propagation for
CPI authority over existing allowed inputs. Use one-shot capability accounts only when a handle must
cross an instruction or program boundary without becoming durable.
