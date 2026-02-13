import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/** Root of the fhevm test-suite directory (test-suite/fhevm/) */
export const FHEVM_ROOT = resolve(__dirname, "..");

/** Docker compose files directory */
export const COMPOSE_DIR = resolve(FHEVM_ROOT, "docker-compose");

/** Staging environment files directory */
export const ENV_DIR = resolve(FHEVM_ROOT, "env", "staging");

/** Config directory */
export const CONFIG_DIR = resolve(FHEVM_ROOT, "config");

/** Scripts directory */
export const SCRIPTS_DIR = resolve(FHEVM_ROOT, "scripts");

/** Docker compose project name */
export const PROJECT_NAME = "fhevm";

/** Returns the docker-compose file path for a component */
export function composeFile(component: string): string {
  return resolve(COMPOSE_DIR, `${component}-docker-compose.yml`);
}

/** Returns the base env file path for a component */
export function envFile(component: string): string {
  return resolve(ENV_DIR, `.env.${component}`);
}

/** Returns the local (generated) env file path for a component */
export function localEnvFile(component: string): string {
  return resolve(ENV_DIR, `.env.${component}.local`);
}
