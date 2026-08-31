//! Heap budget for building and invoking one `fhe_execute` instruction on-chain.
//!
//! An app program pays for its execution on the SBF entrypoint's default allocator
//! (`solana-program-entrypoint`, not Anchor): a bump allocator over a fixed 32 KB region that
//! never frees, so what decides whether an instruction fits is the *total* number of bytes it
//! requests, not its peak live set. Two phases share that one region and neither gives
//! anything back:
//!
//! 1. **Building** the execution — lowering interns into the builder's own tables.
//! 2. **Invoking** it — `invoke_execution_signed_resolved` stamps the final account count into the
//!    args in place, serializes the whole packet once into a right-sized buffer, resolves the
//!    dynamic accounts, and assembles the CPI account tables.
//!
//! Measuring only the build is how the first version of this test reported a budget the runtime
//! does not have: the build alone fits comfortably where the whole instruction does not. Every
//! phase is counted here, with a global allocator that models a never-freeing bump region —
//! every request tallied, every deallocation ignored — and `finish` charges all of it against
//! the budget: the build tally, the exact packet, and the invoke-side table model
//! (`invoke_table_heap_bytes`), each proven against the counting allocator by its own test
//! below. The only cost left to the [`crate::cost::APP_HEAP_RESERVE_BYTES`] reserve is what the
//! builder genuinely cannot see: Anchor's deserialization of the instruction's accounts before
//! any of this runs, and the app's own allocations. The at-cap dep-chain specimen
//! (`runtime-tests/tests/dep_chain_mollusk.rs`) exercises those for real under SBF at full
//! depth.
//!
//! What is measured is a *matrix of buildable shapes*, not one worst case, because the builder's
//! typed ceilings shape what can exist at all: the instruction-trace check caps persistent
//! creates at twenty, and the CPI packet check caps attestation-heavy executions well below the
//! step cap. Every shape the builder admits must fit — that is the claim the single step ceiling
//! rests on — and the fit test below asserts it for each row of the matrix.
//!
//! Counted on the host rather than under SBF because the quantity that regresses — bytes
//! requested per step — is the same in both places, and here it can be attributed to a phase.

mod frontier;
mod harness;
mod proofs;
mod shapes;
