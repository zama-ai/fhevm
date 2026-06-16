// One-off: true Decryption proxy upgrade for the Solana user-decrypt path, using fhevm-cli's own
// mechanism (NOT a docker service-version `upgrade`). Rebuilds the gateway-sc image from local
// source (so it has `userDecryptionRequestSolana` + `task:upgradeDecryptionSolana`), then runs the
// proxy upgrade INSIDE the gateway-sc-deploy container via `runContractTask` — which injects the
// fhevm-cli-configured deployer (the GatewayConfig owner) env. The proxy address is preserved.
//
//   cd test-suite/fhevm && bun run solana-gateway-upgrade.ts
import { loadState } from "./src/state/state";
import { ensureRuntimeArtifacts } from "./src/flow/artifacts";
import { maybeBuild } from "./src/flow/runtime-compose";
import { runContractTask } from "./src/flow/contracts";

const DECRYPTION_PROXY = "0xF0bFB159C7381F7CB332586004d8247252C5b816";

const state = await loadState();
if (!state) throw new Error("no fhevm state; the stack is not running");

await ensureRuntimeArtifacts(state, "solana-gateway-upgrade");

console.log("[solana] rebuilding gateway-sc image from local source...");
await maybeBuild("gateway-sc", state, { force: true });

console.log("[solana] running task:upgradeDecryptionSolana inside gateway-sc-deploy (owner env)...");
await runContractTask(
  "gateway-sc",
  "gateway-sc-deploy",
  `npx hardhat compile && npx hardhat task:upgradeDecryptionSolana --proxy-address ${DECRYPTION_PROXY}`,
);

console.log("[solana] gateway Decryption proxy upgraded.");
