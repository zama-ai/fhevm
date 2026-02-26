import type { ContractAddresses, FhevmConfig } from "../config/model";
import { ExitCode, FhevmCliError } from "../errors";

import { getContainerLogs } from "../docker/containers";
import { discoverMinioIp } from "../docker/services";
import { toLogLines } from "../docker/logs";
import { exec } from "../utils/shell";

interface DiscoveryOps {
  discoverMinioIp: typeof discoverMinioIp;
  getContainerLogs: typeof getContainerLogs;
  fetch: typeof fetch;
  readFileFromContainer: (containerName: string, containerPath: string) => Promise<string | undefined>;
}

/**
 * Read a file from inside a Docker container via `docker cp`.
 * Returns the file content as a string, or undefined if the file doesn't exist.
 */
async function defaultReadFileFromContainer(
  containerName: string,
  containerPath: string,
): Promise<string | undefined> {
  const tmpPath = `/tmp/fhevm-cli-${containerName}-${Date.now()}.env`;
  const cpResult = await exec(["docker", "cp", `${containerName}:${containerPath}`, tmpPath]);
  if (cpResult.exitCode !== 0) {
    return undefined;
  }

  try {
    const file = Bun.file(tmpPath);
    if (!(await file.exists())) {
      return undefined;
    }
    return await file.text();
  } finally {
    try {
      const { unlink } = await import("fs/promises");
      await unlink(tmpPath);
    } catch {
      // best-effort cleanup
    }
  }
}

const DEFAULT_DISCOVERY_OPS: DiscoveryOps = {
  discoverMinioIp,
  getContainerLogs,
  fetch,
  readFileFromContainer: defaultReadFileFromContainer,
};

let discoveryOps: DiscoveryOps = DEFAULT_DISCOVERY_OPS;

const ETH_ADDRESS_REGEX = /\b0x[a-fA-F0-9]{40}\b/;
const HEX_64_REGEX = /\b(?:0x)?([a-fA-F0-9]{64})\b/;
const SIGNER_PATH_PREFIXES = ["VerfAddress", "VerifierAddress", "SignerAddress", "Address"] as const;

export interface DiscoveryResult {
  minioIp: string;
  kmsSigner?: string;
  fheKeyId?: string;
  crsKeyId?: string;
}

interface KmsSignerDiscoveryOptions {
  timeoutMs?: number;
  pollIntervalMs?: number;
}

function normalizeHexHandle(value: string): string {
  return value.replace(/^0x/i, "").toLowerCase();
}

function normalizeEndpoint(endpoint: string): string {
  return endpoint.replace(/\/$/, "");
}

function normalizePrefix(prefix: string): string {
  return prefix.replace(/^\/+/, "").replace(/\/+$/, "");
}

function toHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

function parseAddressFromPayload(payload: Uint8Array): string | undefined {
  if (payload.length === 20) {
    return `0x${toHex(payload)}`;
  }

  const text = new TextDecoder().decode(payload);
  const match = text.match(ETH_ADDRESS_REGEX);
  if (match) {
    return match[0];
  }

  const hexOnly = text.replace(/[^a-fA-F0-9]/g, "");
  if (hexOnly.length === 40) {
    return `0x${hexOnly}`;
  }

  return undefined;
}

async function fetchAddressAtUrl(url: string): Promise<string | undefined> {
  const response = await discoveryOps.fetch(url);
  if (response.status === 404) {
    return undefined;
  }
  if (!response.ok) {
    throw new Error(`request failed (${response.status})`);
  }

  const bytes = new Uint8Array(await response.arrayBuffer());
  return parseAddressFromPayload(bytes);
}

async function listSignerObjectKeys(
  minioEndpoint: string,
  bucket: string,
  prefix: string,
): Promise<string[]> {
  const listUrl = `${normalizeEndpoint(minioEndpoint)}/${bucket}?prefix=${encodeURIComponent(prefix)}`;
  const response = await discoveryOps.fetch(listUrl);
  if (!response.ok) {
    return [];
  }

  const xml = await response.text();
  return [...xml.matchAll(/<Key>([^<]+)<\/Key>/g)].map((match) => match[1] ?? "").filter(Boolean);
}

export function parseSigningKeyHandle(logs: string): string | undefined {
  const lines = logs
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);

  for (let index = lines.length - 1; index >= 0; index -= 1) {
    const line = lines[index] ?? "";
    const hasKeyword = /sign|key|handle|pub|verf|address/i.test(line);
    if (!hasKeyword) {
      continue;
    }

    const match = line.match(HEX_64_REGEX);
    if (match?.[1]) {
      return normalizeHexHandle(match[1]);
    }
  }

  for (let index = lines.length - 1; index >= 0; index -= 1) {
    // Fallback for log format drift: this may capture unrelated 64-char hex values (e.g. tx hashes).
    // We keep it as a best-effort path when keyword-based extraction finds nothing.
    const match = (lines[index] ?? "").match(HEX_64_REGEX);
    if (match?.[1]) {
      return normalizeHexHandle(match[1]);
    }
  }

  return undefined;
}

export async function fetchSignerAddress(
  minioEndpoint: string,
  bucket: string,
  prefix: string,
  keyHandle: string,
): Promise<string> {
  const handle = normalizeHexHandle(keyHandle);
  const normalizedPrefix = normalizePrefix(prefix);
  const root = `${normalizeEndpoint(minioEndpoint)}/${bucket}`;

  for (const pathPrefix of SIGNER_PATH_PREFIXES) {
    const keyPath = `${normalizedPrefix}/${pathPrefix}/${handle}`;
    const address = await fetchAddressAtUrl(`${root}/${keyPath}`);
    if (address) {
      return address;
    }
  }

  const knownPrefix = `${normalizedPrefix}/VerfAddress/`;
  const keys = await listSignerObjectKeys(minioEndpoint, bucket, knownPrefix);
  for (const key of keys) {
    const address = await fetchAddressAtUrl(`${root}/${key}`);
    if (address) {
      return address;
    }
  }

  throw new Error(`could not fetch signer address for key handle ${handle}`);
}

export async function discoverAndApplyMinioIp(
  config: FhevmConfig,
): Promise<string> {
  const minioIp = await discoveryOps.discoverMinioIp("fhevm-minio");
  config.runtime.minioIp = minioIp;
  return minioIp;
}

export async function discoverKmsSigner(
  config: FhevmConfig,
  options: KmsSignerDiscoveryOptions = {},
): Promise<string> {
  const timeoutMs = options.timeoutMs ?? 120_000;
  const pollIntervalMs = options.pollIntervalMs ?? 1_000;
  const startedAt = Date.now();
  // Always use localhost for host-side MinIO access (container IPs are not
  // reachable from macOS). The container IP in config.runtime.minioIp is only
  // for container-to-container env vars.
  const minioEndpoint = `http://localhost:${config.ports.minioApi}`;

  let lastLogs = "";
  let lastError: unknown;

  while (Date.now() - startedAt < timeoutMs) {
    lastLogs = await discoveryOps.getContainerLogs("kms-core", { tail: 200 });
    const keyHandle = parseSigningKeyHandle(lastLogs);

    if (keyHandle) {
      try {
        const signer = await fetchSignerAddress(minioEndpoint, config.minio.buckets.public, "PUB", keyHandle);
        config.runtime.kmsSigner = signer;
        return signer;
      } catch (error) {
        lastError = error;
      }
    }

    await Bun.sleep(pollIntervalMs);
  }

  const suffix = lastError instanceof Error ? ` Last error: ${lastError.message}` : "";
  throw new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "kms-signer-discovery",
    service: "kms-core",
    message: `KMS signer address discovery failed after ${Math.floor(timeoutMs / 1000)}s.${suffix}`,
    logLines: toLogLines(lastLogs, 20),
    logHint: "fhevm-cli logs kms-core",
  });
}

/**
 * After kms-connector catches up on keygen events and kms-core generates FHE
 * keys, a ServerKey object appears in MinIO. Poll until a key is found and
 * return its key ID.
 */
export async function discoverFheKeyId(
  config: FhevmConfig,
  options: { timeoutMs?: number; pollIntervalMs?: number } = {},
): Promise<string> {
  const timeoutMs = options.timeoutMs ?? 300_000;
  const pollIntervalMs = options.pollIntervalMs ?? 5_000;
  const startedAt = Date.now();
  const minioEndpoint = `http://localhost:${config.ports.minioApi}`;
  const bucket = config.minio.buckets.public;
  const prefix = "PUB/ServerKey/";

  while (Date.now() - startedAt < timeoutMs) {
    const keys = await listSignerObjectKeys(minioEndpoint, bucket, prefix);
    if (keys.length > 0) {
      const firstKey = keys[0] ?? "";
      const keyId = firstKey.slice(prefix.length);
      if (keyId) {
        config.runtime.fheKeyId = keyId;
        return keyId;
      }
    }

    await Bun.sleep(pollIntervalMs);
  }

  throw new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "fhe-key-discovery",
    message: `FHE key ID discovery timed out after ${Math.floor(timeoutMs / 1000)}s. No ServerKey objects found in MinIO bucket "${bucket}".`,
    logHint: "fhevm-cli logs kms-core",
  });
}

/**
 * After crsgen is triggered and kms-core generates the CRS, a CRS object
 * appears in MinIO. Poll until one is found and return its key ID.
 */
export async function discoverCrsKeyId(
  config: FhevmConfig,
  options: { timeoutMs?: number; pollIntervalMs?: number } = {},
): Promise<string> {
  const timeoutMs = options.timeoutMs ?? 300_000;
  const pollIntervalMs = options.pollIntervalMs ?? 5_000;
  const startedAt = Date.now();
  const minioEndpoint = `http://localhost:${config.ports.minioApi}`;
  const bucket = config.minio.buckets.public;
  const prefix = "PUB/CRS/";

  while (Date.now() - startedAt < timeoutMs) {
    const keys = await listSignerObjectKeys(minioEndpoint, bucket, prefix);
    if (keys.length > 0) {
      const firstKey = keys[0] ?? "";
      const keyId = firstKey.slice(prefix.length);
      if (keyId) {
        config.runtime.crsKeyId = keyId;
        return keyId;
      }
    }

    await Bun.sleep(pollIntervalMs);
  }

  throw new FhevmCliError({
    exitCode: ExitCode.DOCKER,
    step: "crs-key-discovery",
    message: `CRS key ID discovery timed out after ${Math.floor(timeoutMs / 1000)}s. No CRS objects found in MinIO bucket "${bucket}".`,
    logHint: "fhevm-cli logs kms-core",
  });
}

/**
 * Parse a simple KEY=VALUE env file into a Record.
 */
function parseEnvFile(content: string): Record<string, string> {
  const vars: Record<string, string> = {};
  for (const line of content.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const eqIndex = trimmed.indexOf("=");
    if (eqIndex === -1) continue;
    const key = trimmed.slice(0, eqIndex).trim();
    const value = trimmed.slice(eqIndex + 1).trim();
    if (key) vars[key] = value;
  }
  return vars;
}

/**
 * Map env vars from .env.gateway and .env.host to ContractAddresses.
 *
 * Gateway env vars (from pascalCaseToAddressEnvVar):
 *   GATEWAY_CONFIG_ADDRESS, KMS_GENERATION_ADDRESS, INPUT_VERIFICATION_ADDRESS,
 *   DECRYPTION_ADDRESS, MULTICHAIN_ACL_ADDRESS, CIPHERTEXT_COMMITS_ADDRESS, etc.
 *
 * Host env vars (manually set in taskDeploy.ts):
 *   ACL_CONTRACT_ADDRESS, FHEVM_EXECUTOR_CONTRACT_ADDRESS,
 *   INPUT_VERIFIER_CONTRACT_ADDRESS, KMS_VERIFIER_CONTRACT_ADDRESS, etc.
 */
function mapEnvToContractAddresses(
  gatewayVars: Record<string, string>,
  hostVars: Record<string, string>,
): ContractAddresses {
  return {
    gatewayConfig: gatewayVars.GATEWAY_CONFIG_ADDRESS || undefined,
    kmsGeneration: gatewayVars.KMS_GENERATION_ADDRESS || undefined,
    inputVerification: gatewayVars.INPUT_VERIFICATION_ADDRESS || undefined,
    decryption: gatewayVars.DECRYPTION_ADDRESS || undefined,
    multichainAcl: gatewayVars.MULTICHAIN_ACL_ADDRESS || undefined,
    ciphertextCommits: gatewayVars.CIPHERTEXT_COMMITS_ADDRESS || undefined,
    pauserSet: gatewayVars.PAUSER_SET_ADDRESS || undefined,
    hostPauserSet: hostVars.PAUSER_SET_CONTRACT_ADDRESS || undefined,
    protocolPayment: gatewayVars.PROTOCOL_PAYMENT_ADDRESS || undefined,
    acl: hostVars.ACL_CONTRACT_ADDRESS || undefined,
    fhevmExecutor: hostVars.FHEVM_EXECUTOR_CONTRACT_ADDRESS || undefined,
    kmsVerifier: hostVars.KMS_VERIFIER_CONTRACT_ADDRESS || undefined,
    inputVerifier: hostVars.INPUT_VERIFIER_CONTRACT_ADDRESS || undefined,
  };
}

/**
 * After gateway-sc-deploy and host-sc-deploy have run, read the deployed
 * contract addresses from the containers' addresses-volume mount and
 * populate config.contracts.
 */
export async function discoverContractAddresses(config: FhevmConfig): Promise<ContractAddresses> {
  // Both containers mount addresses-volume at /app/addresses
  const gatewayContent = await discoveryOps.readFileFromContainer("gateway-sc-deploy", "/app/addresses/.env.gateway");
  const hostContent = await discoveryOps.readFileFromContainer("host-sc-deploy", "/app/addresses/.env.host");

  if (!gatewayContent && !hostContent) {
    throw new FhevmCliError({
      exitCode: ExitCode.DOCKER,
      step: "contract-address-discovery",
      message: "could not read contract addresses from deployment containers (both .env.gateway and .env.host are missing)",
    });
  }

  const gatewayVars = gatewayContent ? parseEnvFile(gatewayContent) : {};
  const hostVars = hostContent ? parseEnvFile(hostContent) : {};
  const addresses = mapEnvToContractAddresses(gatewayVars, hostVars);

  // Apply to config
  Object.assign(config.contracts, addresses);

  return addresses;
}

/**
 * After gateway-deploy-mocked-zama-oft has run, read the deployed
 * ZamaOFT address from the container and populate config.contracts.zamaOft.
 *
 * The deploy task writes MOCKED_ZAMA_OFT_ADDRESS to /app/addresses/.env.mocked_payment_bridging.
 * Consumers read it as ZAMA_OFT_ADDRESS so we map accordingly.
 */
export async function discoverMockedPaymentAddress(config: FhevmConfig): Promise<string | undefined> {
  const content = await discoveryOps.readFileFromContainer(
    "gateway-deploy-mocked-zama-oft",
    "/app/addresses/.env.mocked_payment_bridging",
  );

  if (!content) {
    return undefined;
  }

  const vars = parseEnvFile(content);
  const address = vars.MOCKED_ZAMA_OFT_ADDRESS || undefined;

  if (address) {
    config.contracts.zamaOft = address;
  }

  return address;
}

export const __internal = {
  resetDiscoveryOpsForTests(): void {
    discoveryOps = DEFAULT_DISCOVERY_OPS;
  },
  setDiscoveryOpsForTests(overrides: Partial<DiscoveryOps>): void {
    discoveryOps = { ...DEFAULT_DISCOVERY_OPS, ...overrides };
  },
};
