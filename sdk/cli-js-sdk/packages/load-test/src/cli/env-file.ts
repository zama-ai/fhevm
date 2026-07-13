import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Load the workspace-root .env (packages/load-test/src/cli -> workspace root)
// before any module reads env. Existing environment variables win.
const envFile = fileURLToPath(new URL("../../../../.env", import.meta.url));

if (existsSync(envFile)) {
  process.loadEnvFile(envFile);
}
