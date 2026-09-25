import { isIP } from "node:net";
import {
  requiresGatewayKmsGenerationAddress,
  requiresMultichainAclAddress,
  requiresModernHostAddressArtifacts,
} from "../compat/compat";
import { PreflightError } from "../errors";
import {
  DEFAULT_GATEWAY_RPC_PORT,
  OBJECT_STORE_EXTERNAL_URL,
  OBJECT_STORE_INTERNAL_URL,
  OBJECT_STORE_PORT,
  gatewayAddressesPath,
  hostChainAddressesPath,
} from "../layout";
import type { Discovery, State } from "../types";
import { predictedCrsId, predictedKeyId, readEnvFile } from "../utils/fs";
import { run } from "../utils/process";
import { hostChainsForState } from "./topology";

/**
 * Discover the published port on the bridge gateway rather than the object store's leased IP.
 * A stopped object store releases its address: a recovering worker can acquire it
 * before the object store restarts. The gateway remains stable while the stack exists.
 * A numeric endpoint also preserves path-style S3 requests in released workers.
 */
export const objectStorePublishedEndpoint = async () => {
  const result = await run(["docker", "inspect", "fhevm-object-store"], { allowFailure: true });
  if (result.code !== 0) throw new PreflightError("Could not inspect the published object-store endpoint");
  let inspected: Array<{
    NetworkSettings: {
      Networks: Record<string, { Gateway?: string }>;
      Ports?: Record<string, Array<{ HostIp: string; HostPort: string }> | null>;
    };
  }>;
  try {
    inspected = JSON.parse(result.stdout);
  } catch (error) {
    throw new PreflightError(
      `docker inspect fhevm-object-store returned invalid JSON: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  const network = inspected[0]?.NetworkSettings;
  const gateways = [...new Set(Object.values(network?.Networks ?? {}).map(value => value.Gateway)
    .filter((value): value is string => typeof value === "string" && isIP(value) === 4))];
  if (gateways.length !== 1) throw new PreflightError("The object store requires one unambiguous IPv4 bridge gateway");
  const ports = [...new Set((network?.Ports?.[`${OBJECT_STORE_PORT}/tcp`] ?? [])
    .filter(binding => binding.HostIp === "0.0.0.0" || binding.HostIp === "")
    .map(binding => binding.HostPort))];
  if (ports.length !== 1 || !/^[1-9][0-9]*$/.test(ports[0]) || Number(ports[0]) > 65535) {
    throw new PreflightError("The object store requires a published IPv4 port reachable through its bridge gateway");
  }
  return `http://${gateways[0]}:${ports[0]}`;
};

/** Builds the initial endpoint discovery structure before addresses are known. */
export const defaultEndpoints = async () => {
  const objectStoreExternal = await objectStorePublishedEndpoint();
  const hosts: Discovery["endpoints"]["hosts"] = {};
  return {
    gateway: {
      http: `http://gateway-node:${DEFAULT_GATEWAY_RPC_PORT}`,
      ws: `ws://gateway-node:${DEFAULT_GATEWAY_RPC_PORT}`,
    },
    hosts,
    objectStoreInternal: OBJECT_STORE_INTERNAL_URL,
    objectStoreExternal,
  };
};

/** Creates an empty discovery object seeded with predicted key ids and known endpoints. */
export const createDiscovery = (endpoints: Discovery["endpoints"]): Discovery => ({
  gateway: {},
  hosts: {},
  kmsSigners: [],
  kmsCaCerts: [],
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
  const readAddressEnv = async (file: string) => {
    try {
      return await readEnvFile(file);
    } catch (error) {
      if (error && typeof error === "object" && "code" in error && error.code === "ENOENT") {
        throw new PreflightError("Missing generated address files under .fhevm/runtime/addresses");
      }
      throw error;
    }
  };
  return {
    gateway: await readAddressEnv(gatewayAddressesPath),
    hosts: Object.fromEntries(
      await Promise.all(
        hostChains.map(async (chain) => [chain.key, await readAddressEnv(hostChainAddressesPath(chain.key))] as const),
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
    ...(requiresModernHostAddressArtifacts(state) ? ["PROTOCOL_CONFIG_CONTRACT_ADDRESS"] : []),
  ];
  for (const key of requiredGateway) {
    if (!discovery.gateway[key]) {
      throw new PreflightError(`Missing gateway discovery value ${key}`);
    }
  }
  for (const chain of hostChainsForState(state)) {
    const host = discovery.hosts[chain.key];
    if (!host) {
      throw new PreflightError(`Missing discovery for host chain "${chain.key}"`);
    }
    const requiredHostForChain = [
      ...requiredHost,
      ...(chain.isDefault && requiresModernHostAddressArtifacts(state) ? ["KMS_GENERATION_CONTRACT_ADDRESS"] : []),
    ];
    for (const key of requiredHostForChain) {
      if (!host[key]) {
        throw new PreflightError(`Missing host discovery value ${key} for chain "${chain.key}"`);
      }
    }
    if (!chain.isDefault && host.KMS_GENERATION_CONTRACT_ADDRESS) {
      throw new PreflightError(
        `Host discovery for non-canonical chain "${chain.key}" contains KMS_GENERATION_CONTRACT_ADDRESS; this belongs on the canonical host only`,
      );
    }
  }
};
