# Nonce manager

Shared allocation and per-nonce bookkeeping for concurrent Alloy transaction
senders. Extracted from the relayer's `NonceManagerNonOptimistic`; its public API
and `NonceManager` implementation support both Ethereum and generic networks.

An account is seeded once from its **confirmed** transaction count. Allocation
reserves the smallest released nonce, or advances the next nonce. The caller
must retire a used nonce with `confirm_nonce`, or make an unsuccessful attempt's
nonce reusable with `release_nonce`. Updating one reservation preserves all the
other in-flight reservations. Clones share account state, including accounts
first initialized after cloning.

## Caller responsibilities

- One active writer per signing address. State is process-local and does not
  coordinate replicas or persist across restarts.
- Complete the reservation lifecycle, including timeouts and task cancellation.
  The allocator has no cancellation guard.
- Decide what the send outcome establishes. A lost response does not establish
  whether the transaction was accepted, and `already known` does not establish
  that it was mined. The allocator tracks reservations, not transaction hashes,
  receipts, replacements, or fees.
- Reconcile uncertain submissions, existing pending transactions, and incorrect
  initial RPC nonce responses when those cases require recovery. The allocator
  offers explicit `restore_nonces` and `reconcile_nonce` APIs for callers with
  durable submission tracking. Restore before allocation; reconcile only after
  a definite nonce rejection, preserving all other active reservations.

Error classification, retry budgets, and backoff belong to each sender.
