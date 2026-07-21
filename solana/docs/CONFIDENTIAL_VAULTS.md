# Confidential Vaults on Solana — What We Are Building and Why

This document explains the confidential vault design in plain language. It is
the intent behind the `demo-vault` and `confidential-batcher` programs and the
vault demo. Read this before reading their code.

## The goal

Let people earn yield on tokens **without revealing how much they hold or
move**. Balances and amounts stay encrypted end to end; the yield source is an
ordinary, public Solana vault.

## The core idea: keep the vault public, make the doorway confidential

A vault needs to know how much money it manages — that is how it computes the
price of a share. If every position were encrypted, the vault could not price
anything without heavy encrypted math (in particular, dividing one encrypted
number by another, which FHE cannot do efficiently today).

So we split the problem:

- The **vault stays public and ordinary**: tokens in, shares out, share price
  = assets / shares. Anyone can read its totals. This is exactly how the big
  Solana vaults (Kamino, Jupiter Earn, Meteora, liquid staking) already work:
  a program-owned account holds the assets, and a normal SPL token represents
  shares.
- The **doorway is confidential**: a *batcher* program collects many encrypted
  deposits, adds them up while still encrypted, and reveals **only the total**.
  That single public number goes into the vault. Individual amounts are never
  revealed — only the sum of everyone in the batch.

This is the same architecture Zama shipped on Ethereum (the confidential
batcher in front of a Morpho vault). We port the *idea*, not the mechanics:
everything below uses Solana-native building blocks.

```
   users (encrypted amounts)          one public number           public vault
   ┌────┐ ┌────┐ ┌────┐
   │ e(a)│ │e(b)│ │e(c)│ ──join──►  BATCHER  ──"total = a+b+c"──►  deposit
   └────┘ └────┘ └────┘            (adds while                     mint shares
                                    encrypted)                        │
   ┌────────────────────────◄──────────────────────────────────────┘
   │  each user's encrypted cut: e(amount) × batch shares ÷ batch total
```

## How a deposit works, step by step

1. **Shield.** The user wraps public tokens into confidential tokens (their
   balance becomes an encrypted number only they can decrypt).
2. **Join.** The user sends an encrypted amount to the current batch. In the
   same transaction the batcher records a private receipt of the deposit —
   readable by the user and the batcher, nobody else.
3. **Dispatch.** After a waiting period, anyone can dispatch the batch. The
   batcher burns the batch's encrypted total and asks the key-management
   network (KMS) to certify the burned amount. That certificate is how the
   one public number is produced — it reveals the batch total and nothing
   else.
4. **Settle.** Anyone can submit the certificate. The batcher verifies it
   on-chain, deposits the now-public total into the vault, receives ordinary
   share tokens, and wraps them into **confidential shares**. It fixes one
   public exchange rate for the whole batch: shares received ÷ total
   deposited.
5. **Claim.** Each user collects their shares: their encrypted deposit's
   exact proportional part of the batch's shares — an encrypted number times
   a public number, divided by a public number, which FHE does cheaply. No
   one ever divides encrypted by encrypted, and the floor rounding means the
   claims can never add up to more than the batch received.

Withdrawing works the same way in mirror, in the same program: a *redeem*
batcher's batches are joined with encrypted shares, the batch total of shares
is revealed and withdrawn from the vault, and users claim their exact
proportional part of the returned tokens, encrypted. Deposit and redeem
batchers run side by side — one pending batch each — so exits never wait on
entries.

## What is public and what is private

| Always private (encrypted)              | Public by design                    |
| --------------------------------------- | ----------------------------------- |
| each user's deposit / withdrawal amount | each batch's total                  |
| each user's confidential balance        | the vault share price               |
| each user's share position              | who participated in a batch (addresses) |

Honest limits, learned on the EVM side: a batch with one participant reveals
that participant's amount (the total *is* their amount), and an attacker who
joins a batch with known amounts can subtract them from the total. Privacy
grows with the number of genuine participants per batch. We do not enforce a
minimum participant count — such gates are trivially defeated by one person
joining many times with zero — and we say so instead of pretending otherwise.

## Safety rules carried over from the EVM design

- **Deposit and wait, never react.** No user action ever branches on an
  encrypted value (no "instant exit if the pool is big enough"). Anything
  that reacts to encrypted state can be probed until the secret leaks.
- **Self-serve everything.** Dispatch, settle, and claim are permissionless;
  `quit` returns your exact deposit from a pending batch. No operator can
  hold user funds hostage.
- **One batch, one account.** Each batch has its own token account, so the
  revealed total is exactly that batch's sum — leftover dust from an earlier
  batch can never leak into it.

## Why the vault itself is new code (and deliberately boring)

The demo vault is ~200 lines and copies the shape every major Solana vault
uses: a program-owned state account, a program-derived authority that owns the
assets and mints shares, a share price computed on the fly, and yield delivered
as share-price appreciation (never rebasing). Its instruction interface mirrors
Jupiter Earn's, and the batcher talks to it through one small file — so
pointing the batcher at a real venue later is a one-file change. We did not
import an existing vault program: real ones drag in oracles, admin roles and
strategy machinery that a demo does not need, and none of them is a standard
(Solana has no adopted equivalent of Ethereum's ERC-4626 vault interface).

## What this is not (yet)

- Not a natively confidential vault (one whose *own accounting* is encrypted).
  That is the long-term direction; the batcher is the pragmatic first step,
  same as on Ethereum.
- Not a privacy mixer. Participation is visible; amounts are what is hidden.
- No incentive/reward machinery. Yield in the demo is simulated by donating
  tokens to the vault so the share price rises.

Design record: DD-042 in `DESIGN_DECISIONS.md`. Demo tracking: the
confidential-vault epic, fhevm-internal#1754.
