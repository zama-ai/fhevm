# Solana PoC Permanent Docs

This directory holds stable design rationale for the Solana FHEVM PoC.

Use these files for decisions that future implementers, reviewers, SDK authors, or product owners
need to understand after the development issue ledger has been pruned.

```text
DESIGN_DECISIONS.md
  Stable decision index. Read this before changing ACL, KMS, event transport, decrypt, or token
  transfer behavior.

TRANSIENT_ALLOW.md
  Detailed rationale for the Solana equivalent of EVM transient allowance.
```

The temporal files are still useful, but they are not the source of durable product rationale:

```text
../DEVELOPMENT_LOGBOOK.md
  Resume protocol, current audit state, and short-lived development notes.

../DEVELOPMENT_ISSUES.md
  Rolling issue ledger, recent failures, fixes, verification commands, and open follow-up items.
```

When a development issue changes the intended architecture, update `DESIGN_DECISIONS.md` or a
focused document in this directory before treating the work as ready for handoff.
