// addresses — the live gateway inputs the Solana host bootstrap reads before it can configure
// zama-host: the deployed EVM contract addresses from the fhevm-cli address artifact plus the
// coprocessor/KMS signer sets registered on the gateway. This replaces the retired
// `setup-solana-side.sh` "[1/5] gathering live gateway addresses" phase (dotenv `source` + `cast
// call`) with the same reads done in-process, so the bootstrap tracks whatever signer the running
// stack actually generated — no hardcoded values.

import type { KmsThresholds } from "@fhevm/solana-zama-host";
import { type ContractFunctionName, createPublicClient, http, parseAbi } from "viem";

import {
  evmAddressBytes,
  readGatewayBootstrapInputs as readGatewayBootstrapInputsFromAddresses,
  type GatewayBootstrapInputs,
} from "../../../../solana/deploy/src/gateway";
import { DEFAULT_HOST_CHAIN_KEY, gatewayAddressesPath, hostChainAddressesPath } from "../layout";
import { readEnvFile } from "../utils/fs";

export { BRINGUP_KMS_CONTEXT_ID, BRINGUP_KMS_EPOCH_ID } from "../../../../solana/deploy/src/constants";
export { evmAddressBytes, type GatewayBootstrapInputs };

/**
 * Reads the gateway inputs the zama-host bootstrap needs: contract addresses from the fhevm-cli
 * address artifact (`.fhevm/runtime/addresses/gateway/.env.gateway`), signer sets and chain id
 * live from the gateway RPC.
 */
// The ProtocolConfig getters the Solana side reads: the active KMS context/epoch pair the KMS
// Connector validates every signed permit route against, that context's thresholds, and a
// context's KMS signers.
const PROTOCOL_CONFIG_ABI = parseAbi([
  "function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)",
  "function getKmsSignersForContext(uint256 kmsContextId) view returns (address[])",
  "function getPublicDecryptionThreshold() view returns (uint256)",
  "function getUserDecryptionThreshold() view returns (uint256)",
  "function getKmsGenThreshold() view returns (uint256)",
  "function getMpcThreshold() view returns (uint256)",
]);

/** The active KMS context/epoch pair declared by the deployed protocol configuration. */
export type ActiveKmsPair = {
  /** Active KMS context id (32-byte unsigned; type-tagged `0x07` in the high byte). */
  readonly kmsContextId: bigint;
  /** Active KMS epoch id (32-byte unsigned; type-tagged `0x08` in the high byte — never zero). */
  readonly kmsEpochId: bigint;
};

/** Formats a 32-byte unsigned id (KMS context/epoch) as 0x-prefixed bytes32 hex. */
export const bytes32HexFromId = (id: bigint): `0x${string}` =>
  `0x${id.toString(16).padStart(64, "0")}` as `0x${string}`;

/**
 * Reads the active KMS context/epoch pair from the deployed `ProtocolConfig`, the contract on the
 * primary EVM host chain the KMS Connector validates each request's pair against.
 */
export const readActiveKmsPair = async (parameters: {
  readonly hostRpcUrl: string;
  /** Override for tests; defaults to the fhevm-cli primary host chain address artifact. */
  readonly addressesPath?: string;
}): Promise<ActiveKmsPair> => {
  const protocolConfig = await readProtocolConfigAddress(parameters.addressesPath);
  const client = createPublicClient({ transport: http(parameters.hostRpcUrl) });
  const [contextId, epochId] = await client.readContract({
    address: protocolConfig as `0x${string}`,
    abi: PROTOCOL_CONFIG_ABI,
    functionName: "getCurrentKmsContextAndEpoch",
  });
  return { kmsContextId: contextId, kmsEpochId: epochId };
};

/** The current KMS context's thresholds in the primary host chain's `ProtocolConfig`. */
export const readEvmKmsThresholds = async (parameters: {
  readonly hostRpcUrl: string;
  readonly addressesPath?: string;
}): Promise<KmsThresholds> => {
  const address = (await readProtocolConfigAddress(parameters.addressesPath)) as `0x${string}`;
  const client = createPublicClient({ transport: http(parameters.hostRpcUrl) });
  const read = async (
    functionName: Exclude<
      ContractFunctionName<typeof PROTOCOL_CONFIG_ABI>,
      "getCurrentKmsContextAndEpoch" | "getKmsSignersForContext"
    >,
  ) => Number(await client.readContract({ address, abi: PROTOCOL_CONFIG_ABI, functionName }));
  const [publicDecryption, userDecryption, kmsGen, mpc] = await Promise.all([
    read("getPublicDecryptionThreshold"),
    read("getUserDecryptionThreshold"),
    read("getKmsGenThreshold"),
    read("getMpcThreshold"),
  ]);
  return { publicDecryption, userDecryption, kmsGen, mpc };
};

/**
 * The KMS signers of `contextId` in the primary host chain's `ProtocolConfig`, in party order: the
 * signer at index i is KMS party i+1.
 */
export const readEvmKmsSignersForContext = async (parameters: {
  readonly hostRpcUrl: string;
  readonly contextId: bigint;
  readonly addressesPath?: string;
}): Promise<Uint8Array[]> => {
  const address = (await readProtocolConfigAddress(parameters.addressesPath)) as `0x${string}`;
  const client = createPublicClient({ transport: http(parameters.hostRpcUrl) });
  const signers = await client.readContract({
    address,
    abi: PROTOCOL_CONFIG_ABI,
    functionName: "getKmsSignersForContext",
    args: [parameters.contextId],
  });
  return signers.map(evmAddressBytes);
};

/** The primary host chain's `ProtocolConfig`, from the fhevm-cli address artifact. */
export const readProtocolConfigAddress = async (addressesPath?: string): Promise<string> => {
  const addresses = await readEnvFile(addressesPath ?? hostChainAddressesPath(DEFAULT_HOST_CHAIN_KEY));
  const protocolConfig = addresses["PROTOCOL_CONFIG_CONTRACT_ADDRESS"];
  if (!protocolConfig) {
    throw new Error("missing PROTOCOL_CONFIG_CONTRACT_ADDRESS in the host chain address artifact");
  }
  return protocolConfig;
};

export const readGatewayBootstrapInputs = async (parameters: {
  readonly gatewayRpcUrl: string;
  /** Override for tests; defaults to the fhevm-cli state layout. */
  readonly addressesPath?: string;
}): Promise<GatewayBootstrapInputs> => {
  const addresses = await readEnvFile(parameters.addressesPath ?? gatewayAddressesPath);
  const required = (name: string): string => {
    const value = addresses[name];
    if (!value) throw new Error(`missing ${name} in the gateway address artifact`);
    return value;
  };
  return readGatewayBootstrapInputsFromAddresses({
    gatewayRpcUrl: parameters.gatewayRpcUrl,
    gatewayConfigAddress: required("GATEWAY_CONFIG_ADDRESS"),
    inputVerificationAddress: required("INPUT_VERIFICATION_ADDRESS"),
    decryptionAddress: required("DECRYPTION_ADDRESS"),
  });
};
