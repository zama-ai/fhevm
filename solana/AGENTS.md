# Agent Guidance

Two rules apply to every Solana code path: the programs, `solana/`, the SDK's `src/solana`, the
Solana parts of the test suite, and the Solana adapters in the off-chain services.

## Pre-production: no backward compatibility

Nothing on Solana is deployed, the dapps included. Edit in place, go straight to the target design,
delete the old path and simplify. Do not program defensively: no fallbacks, compatibility branches
or "just in case" handling. When in doubt, ask rather than hedge in code.

EVM paths keep their compatibility, because they have consumers and a release policy. Once Solana is
deployed, its default flips to no breaking changes, forward and backward compatible.

## One source for everything that can change

PDA seeds and derivations, instruction building, account and event decoders, types, structs and
constants each have exactly one source. Every consumer imports from it or generates from it. A
handwritten client, or an adapter with its own PDA or instruction code, is not acceptable.

- The norm is the program's IDL rendered by Codama, or a shared Rust crate.
- PDA recipes are Anchor `seeds = [...]` constraints in the programs. Off-chain Rust takes them from
  the program crate or from `zama-solana-acl`.
- A golden test catches an accidental change; it is not a second source.
- Pending Elias's decision: behaviour mirrors, such as the SDK's cleartext client, stay pinned by
  the shared fixtures in `solana/test-fixtures`.

The decision and its compatibility rule are DD-072 in `docs/DESIGN_DECISIONS.md`. Check 8 of
`scripts/dead-surface-check.sh` enforces the PDA part. Its `HAND_DERIVATIONS_ALLOWED` list holds
today's copies and checks their counts. Review rejects new entries.
