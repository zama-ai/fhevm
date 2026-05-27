# Transient Allow On Solana

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
- never valid for KMS, public decrypt, or user decrypt witness paths.

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
- public decrypt permission;
- source-origin restrictions from every transient input.

If the policy does not explicitly allow durable output, the host must reject.

## Final Position

Use instruction-local transient values for internal expression graphs. Use signer propagation for
CPI authority over existing allowed inputs. Use one-shot capability accounts only when a handle must
cross an instruction or program boundary without becoming durable.
