// addresses — the live gateway inputs the Solana host bootstrap reads before it can configure
// zama-host: the deployed EVM contract addresses from the fhevm-cli address artifact plus the
// coprocessor/KMS signer sets registered on the gateway. This replaces the retired
// `setup-solana-side.sh` "[1/5] gathering live gateway addresses" phase (dotenv `source` + `cast
// call`) with the same reads done in-process, so the bootstrap tracks whatever signer the running
// stack actually generated — no hardcoded values.

import { createPublicClient, http, parseAbi } from "viem";

import {
  evmAddressBytes,
  readGatewayBootstrapInputs as readGatewayBootstrapInputsFromAddresses,
  type GatewayBootstrapInputs,
} from "./host-deploy/gateway";
import { DEFAULT_HOST_CHAIN_KEY, gatewayAddressesPath, hostChainAddressesPath } from "../layout";
import { readEnvFile } from "../utils/fs";

export {
  BRINGUP_KMS_CONTEXT_ID,
  SOLANA_HOST_CHAIN_ID,
  SOLANA_HOST_CHAIN_ID_I64,
} from "./host-deploy/constants";
export { evmAddressBytes, type GatewayBootstrapInputs };

/**
 * Reads the gateway inputs the zama-host bootstrap needs: contract addresses from the fhevm-cli
 * address artifact (`.fhevm/runtime/addresses/gateway/.env.gateway`), signer sets and chain id
 * live from the gateway RPC.
 */
// The one ProtocolConfig getter the user-decrypt trust inputs need: the currently active KMS
// context/epoch pair, the pair the KMS Connector validates every signed permit route against.
const PROTOCOL_CONFIG_ABI = parseAbi([
  "function getCurrentKmsContextAndEpoch() view returns (uint256 contextId, uint256 epochId)",
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
 * Reads the active KMS context/epoch pair from the deployed `ProtocolConfig` — the contract on the
 * primary EVM host chain the KMS Connector itself validates each permit's signed pair against, so a
 * permit built from this read names a pair the Connector will serve. Nothing here may be assumed:
 * even a fresh stack activates a type-tagged, non-zero epoch id, so seeding zero (or any other
 * guess) is rejected before the request reaches KMS.
 */
export const readActiveKmsPair = async (parameters: {
  readonly hostRpcUrl: string;
  /** Override for tests; defaults to the fhevm-cli primary host chain address artifact. */
  readonly addressesPath?: string;
}): Promise<ActiveKmsPair> => {
  const addresses = await readEnvFile(parameters.addressesPath ?? hostChainAddressesPath(DEFAULT_HOST_CHAIN_KEY));
  const protocolConfig = addresses["PROTOCOL_CONFIG_CONTRACT_ADDRESS"];
  if (!protocolConfig) {
    throw new Error("missing PROTOCOL_CONFIG_CONTRACT_ADDRESS in the host chain address artifact");
  }
  const client = createPublicClient({ transport: http(parameters.hostRpcUrl) });
  const [contextId, epochId] = await client.readContract({
    address: protocolConfig as `0x${string}`,
    abi: PROTOCOL_CONFIG_ABI,
    functionName: "getCurrentKmsContextAndEpoch",
  });
  return { kmsContextId: contextId, kmsEpochId: epochId };
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
