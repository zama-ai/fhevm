// Slot ids for the browser cells.
//
// The single source of truth is test/infra/config.ts. The browser bundle can't
// import it directly (Turbopack's root is pinned to browser-next, so files outside
// it don't resolve), so playwright.config.ts reads config.ts and injects the ids as
// NEXT_PUBLIC_FHEVM_SLOT_* — inlined into the client bundle at `next dev` startup,
// exactly like the LIB / THREADS / WASM_LOAD / MODULE knobs.
//
// The literal fallbacks below are only for a bare `next dev` run outside the test
// runner. On a version roll, edit config.ts (the runner carries it through); update
// these fallbacks too only if you exercise the app standalone.
export const LEGACY_SLOT = process.env.NEXT_PUBLIC_FHEVM_SLOT_LEGACY ?? 'v12';
export const CURRENT_SLOT = process.env.NEXT_PUBLIC_FHEVM_SLOT_CURRENT ?? 'v13';
export const OLD_MODULE_NEW_KEY_SLOT = process.env.NEXT_PUBLIC_FHEVM_SLOT_OLDMOD ?? 'oldmod-newkey';
