import { ExitCode, FhevmCliError } from "../errors";
import { deriveAllKeys } from "./keys";
import {
  DEFAULT_GATEWAY_CHAIN_ID,
  DEFAULT_HOST_CHAIN_ID,
  DEFAULT_MNEMONIC,
  MAX_COPROCESSORS,
  createDefaultConfig,
  type FhevmConfig,
} from "./model";

export interface ConfigOptions {
  numCoprocessors?: number;
  threshold?: number;
  local?: string[];
}

export interface ConfigVariants {
  docker: FhevmConfig;
  local?: FhevmConfig;
}

function parseIntegerEnv(name: string): number | undefined {
  const raw = Bun.env[name];
  if (!raw) {
    return undefined;
  }
  const value = Number.parseInt(raw, 10);
  if (!Number.isFinite(value)) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "config-build",
      message: `invalid integer in ${name}: ${raw}`,
    });
  }
  return value;
}

function parseHost(url: string): string {
  return new URL(url).host;
}

export function computeThresholds(
  numKmsNodes: number,
  numCoprocessors: number,
): FhevmConfig["thresholds"] {
  const quorum = Math.floor(numKmsNodes / 2) + 1;
  const coprocessor = Math.floor(numCoprocessors / 2) + 1;

  return {
    publicDecryption: quorum,
    userDecryption: quorum,
    kmsGeneration: quorum,
    coprocessor,
    // Contract requires mpcThreshold < numKmsNodes (strict less-than).
    mpc: numKmsNodes > 1 ? 1 : 0,
  };
}

export function applyLocalMode(config: FhevmConfig, localComponents: string[]): FhevmConfig {
  if (localComponents.length === 0) {
    return config;
  }

  const local = structuredClone(config);
  local.db.host = "localhost";
  local.db.relayerHost = "localhost";
  local.rpc.hostHttp = `http://localhost:${local.ports.hostRpc}`;
  local.rpc.hostWs = `ws://localhost:${local.ports.hostRpc}`;
  local.rpc.gatewayHttp = `http://localhost:${local.ports.gatewayRpc}`;
  local.rpc.gatewayWs = `ws://localhost:${local.ports.gatewayRpc}`;
  local.rpc.kmsCore = `http://localhost:${local.ports.kmsCore}`;
  local.rpc.relayerHttp = `http://localhost:${local.ports.relayerHttp}`;
  local.minio.endpoint = `http://localhost:${local.ports.minioApi}`;

  for (const key of Object.keys(local.contracts)) {
    const value = local.contracts[key as keyof typeof local.contracts];
    if (typeof value === "string" && value.startsWith("http://")) {
      try {
        const parsed = new URL(value);
        parsed.hostname = "localhost";
        local.contracts[key as keyof typeof local.contracts] = parsed.toString();
      } catch {
        // Ignore malformed values; contract addresses may not be URLs.
      }
    }
  }

  for (const component of localComponents) {
    if (!component.trim()) {
      throw new FhevmCliError({
        exitCode: ExitCode.CONFIG,
        step: "config-build",
        message: "empty component name in --local option",
      });
    }
  }

  return local;
}

function buildBaseConfig(options: ConfigOptions = {}): FhevmConfig {
  const numCoprocessors = options.numCoprocessors ?? parseIntegerEnv("FHEVM_COPROCESSORS") ?? 1;
  if (numCoprocessors < 1 || numCoprocessors > MAX_COPROCESSORS) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "config-build",
      message: `numCoprocessors must be between 1 and ${MAX_COPROCESSORS}`,
    });
  }

  const numKmsNodes = parseIntegerEnv("FHEVM_KMS_NODES") ?? 1;
  if (numKmsNodes < 1) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "config-build",
      message: "numKmsNodes must be at least 1",
    });
  }

  const mnemonic = Bun.env.FHEVM_MNEMONIC ?? DEFAULT_MNEMONIC;
  const keys = deriveAllKeys(mnemonic, numCoprocessors, numKmsNodes);
  const base = createDefaultConfig(keys, {
    mnemonic,
    chainIds: {
      host: parseIntegerEnv("FHEVM_HOST_CHAIN_ID") ?? DEFAULT_HOST_CHAIN_ID,
      gateway: parseIntegerEnv("FHEVM_GATEWAY_CHAIN_ID") ?? DEFAULT_GATEWAY_CHAIN_ID,
    },
  });

  base.db.user = Bun.env.FHEVM_DB_USER ?? base.db.user;
  base.db.password = Bun.env.FHEVM_DB_PASSWORD ?? base.db.password;
  base.topology.numCoprocessors = numCoprocessors;
  base.topology.numKmsNodes = numKmsNodes;
  base.thresholds = computeThresholds(numKmsNodes, numCoprocessors);

  if (typeof options.threshold === "number") {
    base.thresholds.coprocessor = options.threshold;
    base.thresholds.mpc = options.threshold;
  }

  const dbHostPort = parseHost(base.minio.endpoint);
  if (!dbHostPort.includes(":")) {
    throw new FhevmCliError({
      exitCode: ExitCode.CONFIG,
      step: "config-build",
      message: "minio endpoint must include host and port",
    });
  }

  return base;
}

export function buildConfigVariants(options: ConfigOptions = {}): ConfigVariants {
  const docker = buildBaseConfig(options);
  const local = applyLocalMode(docker, options.local ?? []);
  return {
    docker,
    local: local === docker ? undefined : local,
  };
}

export function buildConfig(options: ConfigOptions = {}): FhevmConfig {
  return buildConfigVariants(options).docker;
}
