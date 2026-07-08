import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Load the workspace-root .env relative to this file so the globally linked
// `fhevm-sdk` binary picks it up from any working directory. The path climbs
// packages/cli/{src,dist} -> workspace root, so it resolves identically from
// source (src/env.ts) and from the bundled entry (dist/index.mjs). Variables
// already present in the environment take precedence over `.env` values.
const envFile = fileURLToPath(new URL("../../../.env", import.meta.url));

if (existsSync(envFile)) {
  process.loadEnvFile(envFile);
}
