import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Load the project-level .env relative to this file so the globally linked
// `fhevm-sdk` binary picks it up from any working directory. Variables
// already present in the environment take precedence over `.env` values.
const envFile = fileURLToPath(new URL("../.env", import.meta.url));

if (existsSync(envFile)) {
  process.loadEnvFile(envFile);
}
