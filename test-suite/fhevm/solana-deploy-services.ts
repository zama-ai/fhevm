// Recreate the backend services onto the Solana user-decrypt images via fhevm-cli's compose
// invocation (dockerArgs) with version-tag env overrides (shell env wins over the env_file). The
// kms-core image (separate kms repo) + the locally-built relayer/connector images carry the
// committed Solana user-decrypt changes. Non-invasive: no state.json edit, --no-deps so only the
// named services are recreated.
//
//   cd test-suite/fhevm && bun run solana-deploy-services.ts
import { loadState } from "./src/state/state";
import { dockerArgs } from "./src/layout";
import { resolvedComposeEnv } from "./src/generate/compose";
import { runStreaming } from "./src/utils/process";

const state = await loadState();
if (!state) throw new Error("no fhevm state; the stack is not running");

const env = {
  ...process.env,
  ...resolvedComposeEnv(state),
  CORE_VERSION: "solana-ud-c57f52f",
  CONNECTOR_GW_LISTENER_VERSION: "solana-ud",
  CONNECTOR_KMS_WORKER_VERSION: "solana-ud-v2",
  RELAYER_IMAGE_REPOSITORY: "solana-e2e/relayer",
  RELAYER_VERSION: "solana-ud",
};

const up = (component: string, services: string[]) =>
  runStreaming([...dockerArgs(component), "up", "-d", "--no-deps", "--force-recreate", ...services], {
    env,
  });

console.log("[solana] recreating kms-core (solana-ud-c57f52f)...");
await up("core", ["kms-core"]);
console.log("[solana] recreating kms-connector gw-listener + kms-worker...");
await up("kms-connector", ["kms-connector-gw-listener", "kms-connector-kms-worker"]);
console.log("[solana] recreating relayer...");
await up("relayer", ["relayer"]);
console.log("[solana] services recreated onto Solana user-decrypt images.");
