# Solana PoC Docs

Durable documentation for the Solana FHEVM PoC. Every statement is kept true against the code on this
branch.

```text
DESIGN_DECISIONS.md
  The stable *why* (DD-001..DD-030) + open product decisions. Read this before changing ACL, input
  verification, decrypt, KMS context, event transport, or token-transfer behavior.

EVM_PARITY.md
  Capability-by-capability EVM -> Solana parity map (ERC7984 + host-contracts + gateway-contracts)
  and a critical solid-vs-fragile assessment.

FUTURE_DESIGN.md
  Forward design requirements and open decisions the port defers (coprocessor signer set, PDA-binding
  convention, operator model, receiver payments, reorg/finality wiring, dead-variant cleanup).

TESTING.md
  Test layout, Mollusk runtime coverage, how to run the suites, and the build traps.
```

When a change alters the intended architecture, update `DESIGN_DECISIONS.md` (or the relevant doc
here) before treating the work as ready for handoff.
