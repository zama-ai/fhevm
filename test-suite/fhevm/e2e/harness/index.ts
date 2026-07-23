// The Solana e2e scenario harness — three primitives, no framework.
//
// Two rules (from the #1656 / #1658 plan):
//   1. Each behavior is tested at exactly one layer. Mollusk owns instruction admission, guards,
//      arithmetic and cost; scenarios own only composition seams (proofs vs live state, KMS
//      round-trips, relayer seams, wire size, timing). No behavior is authored twice.
//   2. The harness carries zero protocol knowledge. Scenarios reach the protocol only through
//      `@fhevm/sdk` Solana actions; assertion reads go through SDK read paths. A missing SDK
//      read/action is an SDK gap to report, never a thing to hand-roll here.

export { loadEnv, resolveEnv } from "./loadEnv";
export type { TestEnv, Capabilities } from "./loadEnv";
export { loadPersonas } from "./personas";
export type { Persona, Personas } from "./personas";
export { until } from "./until";
export type { UntilOptions } from "./until";
