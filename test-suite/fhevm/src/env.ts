import { existsSync } from "fs";
import { log } from "./log.js";
import { envFile, localEnvFile, CONFIG_DIR } from "./paths.js";
import { resolve } from "path";

/** Default versions for all FHEVM stack services */
export const VERSION_DEFAULTS: Record<string, string> = {
  // KMS connector services
  CONNECTOR_DB_MIGRATION_VERSION: "v0.11.0-1",
  CONNECTOR_GW_LISTENER_VERSION: "v0.11.0-1",
  CONNECTOR_KMS_WORKER_VERSION: "v0.11.0-1",
  CONNECTOR_TX_SENDER_VERSION: "v0.11.0-1",

  // Coprocessor services
  COPROCESSOR_DB_MIGRATION_VERSION: "v0.11.0-1",
  COPROCESSOR_GW_LISTENER_VERSION: "v0.11.0-1",
  COPROCESSOR_HOST_LISTENER_VERSION: "v0.11.0-1",
  COPROCESSOR_TX_SENDER_VERSION: "v0.11.0-1",
  COPROCESSOR_TFHE_WORKER_VERSION: "v0.11.0-1",
  COPROCESSOR_SNS_WORKER_VERSION: "v0.11.0-1",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "v0.11.0-1",

  // Gateway and Host contracts
  GATEWAY_VERSION: "v0.11.0-1",
  HOST_VERSION: "v0.11.0-1",

  // Other services
  CORE_VERSION: "v0.13.0-rc.2",
  RELAYER_VERSION: "v0.9.0-rc.1",
  RELAYER_MIGRATE_VERSION: "v0.9.0-rc.1",
  TEST_SUITE_VERSION: "v0.11.0-1",
};

/** Resolve versions from environment variables, falling back to defaults */
export function resolveVersions(): Record<string, string> {
  const resolved: Record<string, string> = {};
  for (const [key, defaultValue] of Object.entries(VERSION_DEFAULTS)) {
    resolved[key] = process.env[key] ?? defaultValue;
  }
  return resolved;
}

/** Export all resolved versions into process.env */
export function exportVersions(): Record<string, string> {
  const versions = resolveVersions();
  for (const [key, value] of Object.entries(versions)) {
    process.env[key] = value;
  }
  return versions;
}

/** Log all resolved versions */
export function logVersions(buildTag: string): void {
  const v = resolveVersions();
  log.info("FHEVM Stack Versions:");
  log.info("FHEVM Contracts:");
  log.info(`  gateway-contracts:${v.GATEWAY_VERSION}${buildTag}`);
  log.info(`  host-contracts:${v.HOST_VERSION}${buildTag}`);
  log.info("FHEVM Coprocessor Services:");
  log.info(`  coprocessor/db-migration:${v.COPROCESSOR_DB_MIGRATION_VERSION}${buildTag}`);
  log.info(`  coprocessor/gw-listener:${v.COPROCESSOR_GW_LISTENER_VERSION}${buildTag}`);
  log.info(`  coprocessor/host-listener:${v.COPROCESSOR_HOST_LISTENER_VERSION}${buildTag}`);
  log.info(`  coprocessor/poller:${v.COPROCESSOR_HOST_LISTENER_VERSION}${buildTag}`);
  log.info(`  coprocessor/tx-sender:${v.COPROCESSOR_TX_SENDER_VERSION}${buildTag}`);
  log.info(`  coprocessor/tfhe-worker:${v.COPROCESSOR_TFHE_WORKER_VERSION}${buildTag}`);
  log.info(`  coprocessor/sns-worker:${v.COPROCESSOR_SNS_WORKER_VERSION}${buildTag}`);
  log.info(`  coprocessor/zkproof-worker:${v.COPROCESSOR_ZKPROOF_WORKER_VERSION}${buildTag}`);
  log.info("FHEVM KMS Connector Services:");
  log.info(`  kms-connector/db-migration:${v.CONNECTOR_DB_MIGRATION_VERSION}${buildTag}`);
  log.info(`  kms-connector/gw-listener:${v.CONNECTOR_GW_LISTENER_VERSION}${buildTag}`);
  log.info(`  kms-connector/kms-worker:${v.CONNECTOR_KMS_WORKER_VERSION}${buildTag}`);
  log.info(`  kms-connector/tx-sender:${v.CONNECTOR_TX_SENDER_VERSION}${buildTag}`);
  log.info("FHEVM Test Suite:");
  log.info(`  test-suite/e2e:${v.TEST_SUITE_VERSION}${buildTag}`);
  log.info("External Dependencies:");
  log.info(`  kms-core-service:${v.CORE_VERSION}`);
  log.info(`  fhevm-relayer:${v.RELAYER_VERSION}`);
}

/** All env components that need local copies */
const ENV_COMPONENTS = [
  "minio", "database", "core", "gateway-node", "host-node",
  "gateway-sc", "gateway-mocked-payment", "host-sc",
  "kms-connector", "coprocessor", "relayer", "test-suite",
];

/** Copy base env file to local for a single component */
async function prepareLocalEnvFile(component: string): Promise<void> {
  const base = envFile(component);
  const local = localEnvFile(component);

  if (!existsSync(base)) {
    throw new Error(`Base environment file for ${component} not found: ${base}`);
  }

  log.info(`Creating/updating local environment file for ${component}...`);
  const content = await Bun.file(base).text();
  await Bun.write(local, content);
}

/** Prepare the local relayer config file */
async function prepareLocalConfigRelayer(): Promise<void> {
  const base = resolve(CONFIG_DIR, "relayer", "local.yaml");
  const local = resolve(CONFIG_DIR, "relayer", "local.yaml.local");

  if (!existsSync(base)) {
    throw new Error(`Base configuration file for relayer not found: ${base}`);
  }

  log.info("Creating/updating local configuration file for relayer...");
  const content = await Bun.file(base).text();
  await Bun.write(local, content);
}

/** Prepare all local env files and relayer config */
export async function prepareAllEnvFiles(): Promise<void> {
  log.info("Preparing all local environment files...");

  for (const component of ENV_COMPONENTS) {
    await prepareLocalEnvFile(component);
  }

  await prepareLocalConfigRelayer();
  log.info("All local environment files prepared successfully");
}

/**
 * Patch a single env var in a file.
 * Replaces the line matching `^KEY=.*` with `KEY=newValue`.
 * If the key is not found, appends it.
 */
export async function patchEnvVar(
  filePath: string,
  key: string,
  newValue: string,
): Promise<void> {
  const content = await Bun.file(filePath).text();
  const lines = content.split("\n");
  const pattern = new RegExp(`^${key}=`);
  let found = false;

  for (let i = 0; i < lines.length; i++) {
    if (pattern.test(lines[i])) {
      lines[i] = `${key}=${newValue}`;
      found = true;
      break;
    }
  }

  if (!found) {
    lines.push(`${key}=${newValue}`);
  }

  await Bun.write(filePath, lines.join("\n"));
}
