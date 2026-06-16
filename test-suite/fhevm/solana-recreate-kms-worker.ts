// Recreate the kms-connector kms-worker via fhevm-cli's composeUp so it re-reads kms-connector.env
// (the env_file is read at container create, not on restart). Used after registering the Solana
// host chain in KMS_CONNECTOR_HOST_CHAINS (chain_kind=solana + zama-host program id) so the worker's
// public-decrypt ACL check routes through the Solana branch (verify_solana_public_decrypt_allowed).
// The connector holds no FHE/signing keys, so recreate is safe. Uses the existing image (no rebuild).
//
//   cd test-suite/fhevm && bun run solana-recreate-kms-worker.ts
import { loadState } from "./src/state/state";
import { composeUp } from "./src/flow/runtime-compose";
import { waitForContainer } from "./src/flow/readiness";

const state = await loadState();
if (!state) throw new Error("no fhevm state; the stack is not running");

console.log("[kms-worker] recreating to pick up KMS_CONNECTOR_HOST_CHAINS (Solana host chain)...");
await composeUp("kms-connector", ["kms-connector-kms-worker"], { noDeps: true, forceRecreate: true });
await waitForContainer("kms-connector-kms-worker", "healthy");
console.log("[kms-worker] recreated and healthy with the Solana host chain registered.");
