// Force-rebuild the relayer override image from local source and recreate the container in place,
// mirroring fhevm-cli's `upgrade` body (maybeBuild force + composeUp) for the relayer group, which
// the `upgrade` command itself rejects (only coprocessor/kms-connector/test-suite are upgradeable).
// The relayer holds no keys, so recreating it is safe and preserves the rest of the stack (kms-core
// keys, committed handles). relayer.yaml (incl. the Solana host chain) is untouched (no regenerate).
//
//   cd test-suite/fhevm && bun run solana-rebuild-relayer.ts
import { loadState } from "./src/state/state";
import { maybeBuild, composeUp } from "./src/flow/runtime-compose";
import { waitForContainer, waitForLog } from "./src/flow/readiness";

const state = await loadState();
if (!state) throw new Error("no fhevm state; the stack is not running");

console.log("[relayer-rebuild] force-building relayer override from source (the fix)...");
await maybeBuild("relayer", state, { force: true });
console.log("[relayer-rebuild] recreating relayer container...");
await composeUp("relayer", ["relayer"], { noDeps: true, forceRecreate: true });
await waitForContainer("fhevm-relayer", "running");
await waitForLog("fhevm-relayer", /All servers are ready and responding/);
console.log("[relayer-rebuild] relayer rebuilt with the fix and ready.");
