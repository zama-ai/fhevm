@.claude/naming.md
@.claude/coding-conventions.md
@.claude/architecture.md
@.claude/api.md

## Solana (`src/solana`)

Solana code follows `solana/AGENTS.md`: nothing is deployed, so there is no backward compatibility and no defensive fallback, and everything that can change has one source (the IDL via Codama, or a shared Rust crate). DD-072 records it, and check 8 of `solana/scripts/dead-surface-check.sh` enforces it.
