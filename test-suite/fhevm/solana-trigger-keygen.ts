// Re-trigger the network FHE keygen so the (recreated, key-less) kms-core mints a fresh FHE key.
// Only the FHE key changes; the KMS signing key + the Solana host_config (disclose/redeem) are
// unaffected. Uses fhevm-cli's compose invocation for the host-sc-trigger-keygen one-shot.
import { loadState } from "./src/state/state";
import { dockerArgs } from "./src/layout";
import { resolvedComposeEnv } from "./src/generate/compose";
import { runStreaming } from "./src/utils/process";

const state = await loadState();
if (!state) throw new Error("no fhevm state");
const env = { ...process.env, ...resolvedComposeEnv(state), CORE_VERSION: "solana-ud-c57f52f" };

console.log("[solana] triggering network keygen (host-sc-trigger-keygen)...");
await runStreaming(
  [...dockerArgs("host-sc"), "run", "--rm", "--no-deps", "host-sc-trigger-keygen"],
  { env },
);
console.log("[solana] keygen trigger submitted.");
