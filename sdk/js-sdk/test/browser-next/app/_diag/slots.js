// Slot id for the browser cells.
//
// The single source of truth is test/infra/config.ts. The browser bundle can't
// import it directly (Turbopack's root is pinned to browser-next, so files outside
// it don't resolve), so playwright.config.ts reads config.ts and injects the id as
// NEXT_PUBLIC_FHEVM_SLOT_CURRENT — inlined into the client bundle at `next dev` startup,
// exactly like the LIB / THREADS / WASM_LOAD / MODULE knobs.
//
// The literal fallback below is only for a bare `next dev` run outside the test
// runner. On a version roll, edit config.ts (the runner carries it through); update
// this fallback too only if you exercise the app standalone.
export const CURRENT_SLOT = process.env.NEXT_PUBLIC_FHEVM_SLOT_CURRENT ?? 'v1.0';
