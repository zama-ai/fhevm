# Design note: minted-in-transaction as a handle discriminant

**Problem.** Handles are content-derived, so a replacement block sharing a
parent and timestamp (L1 equivocation; routine unsafe-head reorgs on
fixed-block-time chains like Base) can mint the *same* handle from
transactions that source its operands differently — persisted boundaries on
one side, raw local intermediates on the other. The two sourcings
legitimately produce different bytes (`decompress(compress(x))` is
bit-inexact; divergence surfaces when the delta crosses a PBS mod-switch
rounding boundary), leaving a rare, probabilistic cross-node divergence that
today only drift-revert repairs.

**Proposal.** The executor records every handle it derives in its own
transient storage (ops, trivial encrypts, randoms — not verified inputs) and
folds one *boundary bit* per operand into the minted handle's preimage:
1 iff the operand was NOT minted in this transaction, i.e. its only
consumable representation is the canonical persisted form.

**Why it closes the residual.** A bit-0 collision means both blocks produced
the operand inside the consuming transaction — same operand handle, so by
induction (bottoming out at ZK-pinned inputs and tx-scoped rand seeds) the
same raw bytes. A bit-1 collision means both consumed the canonical form of
the same persisted bytes. Collision ⟺ identical sourcing, deterministically.

**Why transient storage is exactly right.** The executor is a singleton, so
the record spans every call frame of the transaction (multicalls, deep
composition) — and EIP-1153 journaling rolls a reverted subcall's marks back
together with its events, keeping the on-chain record and the coprocessor's
log-derived view identical by construction. No ACL involvement, no persistent
state, no reorg surface.

Measured cost (forge gas-report, versus the same contracts with the
discriminant neutralised): +135 gas per mint and +108 per encrypted operand
consumed, i.e. +351 on a binary op, +472 on a ternary, +1207 on a five-element
`fheSum`, and nothing at all on `allow()`. The slot is the handle itself; see
`_markMinted` for why namespacing is unnecessary in transient storage.

**Coprocessor alignment.** "Minted in this transaction" is precisely
per-transaction graph membership, which the scheduler already knows
structurally: in-graph edges forward raw working values, transaction
boundaries are consumed canonically, and compression happens iff the output
is allowed (any cross-transaction consumer implies a prior persistent allow).
No new tables, columns, or ingestion marking. Handles change fleet-wide;
covered by the already-planned cutover.
