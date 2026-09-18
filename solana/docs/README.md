# Solana docs

Documentation for the Solana port of the Zama fhevm host. Every statement is kept true against the
code on this branch; when code and a document disagree, one of them is wrong and the fix names
which.

| File                                               | Read it for                                                                                           |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| [`GLOSSARY.md`](GLOSSARY.md)                       | The normative vocabulary. Code, docs, IDL and tests use these names and no synonyms.                  |
| [`EVM_PARITY.md`](EVM_PARITY.md)                   | Where each EVM fhevm capability lands on Solana and how it differs.                                   |
| [`DESIGN_DECISIONS.md`](DESIGN_DECISIONS.md)       | The numbered decisions the code follows today, with an index by status. Read DD-049 and DD-050 first. |
| [`DESIGN_HISTORY.md`](DESIGN_HISTORY.md)           | Decisions a later one replaced, in their original wording.                                            |
| [`INVARIANTS.md`](INVARIANTS.md)                   | What the system guarantees and deliberately does not, then sizes, limits and operations.              |
| [`FUTURE_DESIGN.md`](FUTURE_DESIGN.md)             | Requirements the port defers to production and the open product decisions.                            |
| [`MMR_ACL_MVP.md`](MMR_ACL_MVP.md)                 | A reviewer's map of the Store and MMR implementation.                                                 |
| [`CONFIDENTIAL_VAULTS.md`](CONFIDENTIAL_VAULTS.md) | What the batcher and vault demo build and why, in plain language.                                     |
| [`TESTING.md`](TESTING.md)                         | Test layers, exact commands, what each proves, and the build traps.                                   |

Program-level docs live next to the code: [`../programs/zama-host/README.md`](../programs/zama-host/README.md)
(accounts, roles, external-input flow) and
[`../scripts/e2e/test-keypairs/README.md`](../scripts/e2e/test-keypairs/README.md) (committed test
keypairs and rotation).

When a change alters the intended architecture, add or amend the decision in `DESIGN_DECISIONS.md`
before treating the work as ready for handoff. A replaced decision moves to `DESIGN_HISTORY.md`.
