import {
  requiresGatewayKmsGenerationAddress,
  requiresMultichainAclAddress,
  usesHostKmsGeneration,
} from "../compat/compat";
import { PreflightError } from "../errors";
import {
  DEFAULT_GATEWAY_RPC_PORT,
  MINIO_EXTERNAL_URL,
  MINIO_INTERNAL_URL,
  MINIO_PORT,
  gatewayAddressesPath,
  hostChainAddressesPath,
} from "../layout";
import type { Discovery, State } from "../types";
import { predictedCrsId, predictedKeyId, readEnvFile } from "../utils/fs";
import { exists } from "../utils/fs";
import { run } from "../utils/process";
import { hostChainsForState } from "./topology";

/** Resolves the MinIO container IP used for host-reachable material URLs. */
export const minioIp = async () => {
  const result = await run(["docker", "inspect", "fhevm-minio"], { allowFailure: true });
  if (result.code !== 0) {
    throw new PreflightError("Could not determine MinIO IP");
  }
  let inspected: Array<{
    NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
  }>;
  try {
    inspected = JSON.parse(result.stdout) as Array<{
      NetworkSettings: { Networks: Record<string, { IPAddress: string }> };
    }>;
  } catch (error) {
    throw new PreflightError(
      `docker inspect fhevm-minio returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  const ip = inspected[0] ? Object.values(inspected[0].NetworkSettings.Networks)[0]?.IPAddress : "";
  if (!ip) {
    throw new PreflightError("Could not determine MinIO IP");
  }
  return ip;
};

/** Builds the initial endpoint discovery structure before addresses are known. */
export const defaultEndpoints = async () => {
  const ip = await minioIp();
  const hosts: Discovery["endpoints"]["hosts"] = {};
  return {
    gateway: {
      http: `http://gateway-node:${DEFAULT_GATEWAY_RPC_PORT}`,
      ws: `ws://gateway-node:${DEFAULT_GATEWAY_RPC_PORT}`,
    },
    hosts,
    minioInternal: MINIO_INTERNAL_URL,
    minioExternal: `http://${ip}:${MINIO_PORT}`,
  };
};

/** Creates an empty discovery object seeded with predicted key ids and known endpoints. */
export const createDiscovery = (endpoints: Discovery["endpoints"]): Discovery => ({
  gateway: {},
  hosts: {},
  kmsSigner: "",
  fheKeyId: predictedKeyId(),
  crsKeyId: predictedCrsId(),
  endpoints,
});

/** Ensures discovery state exists before later steps mutate it. */
export const ensureDiscovery = async (state: State) => {
  if (!state.discovery) {
    const endpoints = await defaultEndpoints();
    for (const chain of hostChainsForState(state)) {
      endpoints.hosts[chain.key] = {
        http: `http://${chain.node}:${chain.rpcPort}`,
        ws: `ws://${chain.node}:${chain.rpcPort}`,
      };
    }
    state.discovery = createDiscovery(endpoints);
  }
  return state.discovery;
};

/** Loads generated gateway and host address artifacts from disk. */
export const discoverContracts = async (state: Pick<State, "scenario">) => {
  const hostChains = hostChainsForState(state);
  const gwExists = await exists(gatewayAddressesPath);
  const hostExistence = await Promise.all(hostChains.map((chain) => exists(hostChainAddressesPath(chain.key))));
  if (!gwExists || hostExistence.some((value) => !value)) {
    throw new PreflightError("Missing generated address files under .fhevm/runtime/addresses");
  }
  return {
    gateway: await readEnvFile(gatewayAddressesPath),
    hosts: Object.fromEntries(
      await Promise.all(
        hostChains.map(async (chain) => [chain.key, await readEnvFile(hostChainAddressesPath(chain.key))] as const),
      ),
    ),
  };
};

/** Verifies that required discovery fields are present before rendering runtime artifacts. */
export const validateDiscovery = (
  state: Pick<State, "target" | "versions" | "discovery" | "overrides" | "scenario">,
) => {
  const discovery = state.discovery;
  if (!discovery) {
    throw new PreflightError("Missing discovery state");
  }
  const requiredGateway = [
    "GATEWAY_CONFIG_ADDRESS",
    ...(requiresGatewayKmsGenerationAddress(state) ? ["KMS_GENERATION_ADDRESS"] : []),
    "DECRYPTION_ADDRESS",
    "INPUT_VERIFICATION_ADDRESS",
    "CIPHERTEXT_COMMITS_ADDRESS",
    ...(requiresMultichainAclAddress(state) ? ["MULTICHAIN_ACL_ADDRESS"] : []),
  ];
  const requiredHost = [
    "ACL_CONTRACT_ADDRESS",
    "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
    "KMS_VERIFIER_CONTRACT_ADDRESS",
    "INPUT_VERIFIER_CONTRACT_ADDRESS",
    "PAUSER_SET_CONTRACT_ADDRESS",
    ...(usesHostKmsGeneration(state) ? ["PROTOCOL_CONFIG_CONTRACT_ADDRESS"] : []),
  ];
  for (const key of requiredGateway) {
    if (!discovery.gateway[key]) {
      throw new PreflightError(`Missing gateway discovery value ${key}`);
    }
  }
  for (const [index, chain] of hostChainsForState(state).entries()) {
    const host = discovery.hosts[chain.key];
    if (!host) {
      throw new PreflightError(`Missing discovery for host chain "${chain.key}"`);
    }
    const requiredHostForChain = [
      ...requiredHost,
      ...(index === 0 && usesHostKmsGeneration(state) ? ["KMS_GENERATION_CONTRACT_ADDRESS"] : []),
    ];
    for (const key of requiredHostForChain) {
      if (!host[key]) {
        throw new PreflightError(`Missing host discovery value ${key} for chain "${chain.key}"`);
      }
    }
    if (index > 0 && host.KMS_GENERATION_CONTRACT_ADDRESS) {
      throw new PreflightError(
        `Host discovery for non-canonical chain "${chain.key}" contains KMS_GENERATION_CONTRACT_ADDRESS; this belongs on the canonical host only`,
      );
    }
  }
};
