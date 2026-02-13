import { resolve } from "path";
import { mkdirSync } from "fs";
import { FHEVM_ROOT } from "./paths.js";

/** Services that use individual BuildKit caches */
const INDIVIDUAL_CACHE_SERVICES = [
  "gateway-deploy-mocked-zama-oft",
  "gateway-sc-add-network",
  "gateway-sc-add-pausers",
  "gateway-sc-deploy",
  "gateway-sc-pause",
  "gateway-sc-trigger-crsgen",
  "gateway-sc-trigger-keygen",
  "gateway-sc-unpause",
  "gateway-set-relayer-mocked-payment",
  "host-sc-add-pausers",
  "host-sc-deploy",
  "host-sc-pause",
  "host-sc-unpause",
  "kms-connector-db-migration",
  "test-suite-e2e-debug",
];

/** Convert a service name to an env var key (e.g. "host-sc-deploy" → "HOST_SC_DEPLOY") */
function toEnvKey(serviceName: string): string {
  return serviceName.replace(/-/g, "_").toUpperCase();
}

/**
 * Compute all BuildKit local cache environment variables for --local builds.
 * Returns a Record to spread into process.env or Bun.spawn env.
 */
export function computeCacheEnvVars(cacheDir?: string): Record<string, string> {
  const baseCacheDir = resolve(FHEVM_ROOT, cacheDir ?? ".buildx-cache");
  mkdirSync(baseCacheDir, { recursive: true });

  const env: Record<string, string> = {
    DOCKER_BUILDKIT: "1",
    COMPOSE_DOCKER_CLI_BUILD: "1",
    BUILDX_NO_DEFAULT_ATTESTATIONS: "1",
    DOCKER_BUILD_PROVENANCE: "false",
    FHEVM_CARGO_PROFILE: "local",
  };

  // Unified coprocessor workspace cache — all coprocessor services share one cache
  // since they are built from a single Dockerfile.workspace with multi-stage targets
  const coprocessorCacheDir = resolve(baseCacheDir, "coprocessor");
  mkdirSync(coprocessorCacheDir, { recursive: true });
  env.FHEVM_CACHE_FROM_COPROCESSOR = `type=local,src=${coprocessorCacheDir}`;
  env.FHEVM_CACHE_TO_COPROCESSOR = `type=local,dest=${coprocessorCacheDir},mode=max`;

  // Unified kms-connector workspace cache — gw-listener, kms-worker, tx-sender
  // share Dockerfile.workspace; db-migration uses a separate Dockerfile
  const kmsConnectorCacheDir = resolve(baseCacheDir, "kms-connector");
  mkdirSync(kmsConnectorCacheDir, { recursive: true });
  env.FHEVM_CACHE_FROM_KMS_CONNECTOR = `type=local,src=${kmsConnectorCacheDir}`;
  env.FHEVM_CACHE_TO_KMS_CONNECTOR = `type=local,dest=${kmsConnectorCacheDir},mode=max`;

  // Individual caches for all other services
  for (const serviceName of INDIVIDUAL_CACHE_SERVICES) {
    const key = toEnvKey(serviceName);
    const serviceDir = resolve(baseCacheDir, serviceName);
    mkdirSync(serviceDir, { recursive: true });
    env[`FHEVM_CACHE_FROM_${key}`] = `type=local,src=${serviceDir}`;
    env[`FHEVM_CACHE_TO_${key}`] = `type=local,dest=${serviceDir},mode=max`;
  }

  return env;
}
