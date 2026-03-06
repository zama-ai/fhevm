import { detectCI } from "./detect";

export type CacheBackend = "gha" | "local" | "none";

export interface CacheConfig {
  backend: CacheBackend;
  envVars: Record<string, string>;
}

type EnvReader = (name: string) => string | undefined;

const DEFAULT_ENV_READER: EnvReader = (name) => process.env[name];
const LOCAL_FROM = "type=local,src=.buildx-cache/";
const LOCAL_TO = "type=local,dest=.buildx-cache/,mode=max";

let readEnv: EnvReader = DEFAULT_ENV_READER;

const CACHE_ENV_VARS = [
  "FHEVM_CACHE_FROM_COPROCESSOR",
  "FHEVM_CACHE_TO_COPROCESSOR",
  "FHEVM_CACHE_FROM_KMS_CONNECTOR_DB_MIGRATION",
  "FHEVM_CACHE_TO_KMS_CONNECTOR_DB_MIGRATION",
  "FHEVM_CACHE_FROM_KMS_CONNECTOR",
  "FHEVM_CACHE_TO_KMS_CONNECTOR",
  "FHEVM_CACHE_FROM_GATEWAY_SC_DEPLOY",
  "FHEVM_CACHE_TO_GATEWAY_SC_DEPLOY",
  "FHEVM_CACHE_FROM_GATEWAY_SC_ADD_NETWORK",
  "FHEVM_CACHE_TO_GATEWAY_SC_ADD_NETWORK",
  "FHEVM_CACHE_FROM_GATEWAY_SC_ADD_PAUSERS",
  "FHEVM_CACHE_TO_GATEWAY_SC_ADD_PAUSERS",
  "FHEVM_CACHE_FROM_GATEWAY_SC_TRIGGER_KEYGEN",
  "FHEVM_CACHE_TO_GATEWAY_SC_TRIGGER_KEYGEN",
  "FHEVM_CACHE_FROM_GATEWAY_SC_TRIGGER_CRSGEN",
  "FHEVM_CACHE_TO_GATEWAY_SC_TRIGGER_CRSGEN",
  "FHEVM_CACHE_FROM_GATEWAY_SC_PAUSE",
  "FHEVM_CACHE_TO_GATEWAY_SC_PAUSE",
  "FHEVM_CACHE_FROM_GATEWAY_SC_UNPAUSE",
  "FHEVM_CACHE_TO_GATEWAY_SC_UNPAUSE",
  "FHEVM_CACHE_FROM_HOST_SC_DEPLOY",
  "FHEVM_CACHE_TO_HOST_SC_DEPLOY",
  "FHEVM_CACHE_FROM_HOST_SC_ADD_PAUSERS",
  "FHEVM_CACHE_TO_HOST_SC_ADD_PAUSERS",
  "FHEVM_CACHE_FROM_HOST_SC_PAUSE",
  "FHEVM_CACHE_TO_HOST_SC_PAUSE",
  "FHEVM_CACHE_FROM_HOST_SC_UNPAUSE",
  "FHEVM_CACHE_TO_HOST_SC_UNPAUSE",
  "FHEVM_CACHE_FROM_GATEWAY_DEPLOY_MOCKED_ZAMA_OFT",
  "FHEVM_CACHE_TO_GATEWAY_DEPLOY_MOCKED_ZAMA_OFT",
  "FHEVM_CACHE_FROM_GATEWAY_SET_RELAYER_MOCKED_PAYMENT",
  "FHEVM_CACHE_TO_GATEWAY_SET_RELAYER_MOCKED_PAYMENT",
  "FHEVM_CACHE_FROM_TEST_SUITE_E2E_DEBUG",
  "FHEVM_CACHE_TO_TEST_SUITE_E2E_DEBUG",
] as const;

function isCacheToVar(name: string): boolean {
  return name.includes("_TO_");
}

function isCacheWriterVar(name: string): boolean {
  return name === "FHEVM_CACHE_TO_COPROCESSOR";
}

function defaultValue(name: string, backend: CacheBackend): string | undefined {
  if (backend === "gha") {
    return undefined;
  }
  if (backend === "none") {
    return "";
  }
  if (isCacheToVar(name)) {
    return isCacheWriterVar(name) ? LOCAL_TO : "";
  }
  return LOCAL_FROM;
}

export function buildCacheEnvVars(backend: CacheBackend): Record<string, string> {
  const envVars: Record<string, string> = {};

  for (const name of CACHE_ENV_VARS) {
    const existing = readEnv(name);
    if (existing !== undefined) {
      envVars[name] = existing;
      continue;
    }

    const value = defaultValue(name, backend);
    if (value !== undefined) {
      envVars[name] = value;
    }
  }

  return envVars;
}

export function buildCacheConfig(options: { noCache?: boolean } = {}): CacheConfig {
  const ci = detectCI(options);
  return {
    backend: ci.cacheType,
    envVars: buildCacheEnvVars(ci.cacheType),
  };
}

export const __internal = {
  cacheEnvVars: CACHE_ENV_VARS,
  resetEnvReaderForTests(): void {
    readEnv = DEFAULT_ENV_READER;
  },
  setEnvReaderForTests(next: EnvReader): void {
    readEnv = next;
  },
};
