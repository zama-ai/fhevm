# Solana PoC Docs

Durable documentation for the Solana FHEVM PoC — the design, how it maps to the EVM side, how it
was built, and how to test it. These are the files that stay useful after the day-to-day scratch
notes are gone.

```text
DESIGN_DECISIONS.md
  The stable *why* (DD-001..DD-030) + open product decisions. Read this before changing ACL, KMS,
  event transport, decrypt, or token-transfer behavior. DD-020..DD-030, plus the "Open questions for
  the e2e flow debate" and "Next steps after merge" sections, are the June 2026 reconciliation; DD-007,
  DD-012, and DD-015 were updated there. This is the current authoritative view of the e2e PoC.

EVM_PARITY.md
  Capability-by-capability EVM->Solana parity map (ERC7984 + host-contracts + gateway-contracts)
  and a critical solid-vs-fragile assessment.

TRANSIENT_ALLOW.md
  The Solana equivalent of EVM transient allowance, in depth.

DEVELOPMENT_HISTORY.md
  How the PoC was built: the hardening story, the ~176-issue catalog, and an honest status
  snapshot (solid / PoC-scope / product-open).

TESTING.md
  Test layout, Mollusk runtime coverage, how to run the suites, and the traps.
```

When a change alters the intended architecture, update `DESIGN_DECISIONS.md` (or a focused doc
here) before treating the work as ready for handoff.
