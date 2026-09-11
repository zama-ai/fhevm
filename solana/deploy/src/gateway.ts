// Live gateway reads the zama-host bootstrap needs: chain id + KMS/coprocessor signer sets from
// GatewayConfig, plus the two EIP-712 verifying-contract addresses. Signers are always fetched
// live so a preview-env KMS that minted keys this run is what gets defined on-chain — never a
// hardcoded committee.
import { createPublicClient, http, parseAbi } from 'viem';

export type GatewayBootstrapInputs = {
  readonly gatewayChainId: bigint;
  /** EVM `InputVerification` contract (EIP-712 verifying contract for input attestations). */
  readonly inputVerificationContract: Uint8Array;
  /** EVM `Decryption` contract (EIP-712 verifying contract for KMS certificates). */
  readonly decryptionContract: Uint8Array;
  /** Coprocessor attestation signer set registered on the gateway (EVM `InputVerifier` parity). */
  readonly coprocessorSigners: readonly Uint8Array[];
  /** KMS certificate signer set registered on the gateway. */
  readonly kmsSigners: readonly Uint8Array[];
};

const GATEWAY_CONFIG_ABI = parseAbi([
  'function getCoprocessorSigners() view returns (address[])',
  'function getKmsSigners() view returns (address[])',
]);

/** Decodes a 0x-prefixed 20-byte EVM address into its raw bytes. */
export const evmAddressBytes = (address: string): Uint8Array => {
  const hex = address.replace(/^0x/, '');
  if (hex.length !== 40 || !/^[0-9a-f]{40}$/i.test(hex)) {
    throw new Error(`expected a 20-byte EVM address, got "${address}"`);
  }
  return Uint8Array.from(Buffer.from(hex, 'hex'));
};

export type GatewayAddressInputs = {
  readonly gatewayRpcUrl: string;
  readonly gatewayConfigAddress: string;
  readonly inputVerificationAddress: string;
  readonly decryptionAddress: string;
};

/**
 * Reads signer sets and chain id from the gateway RPC, using caller-supplied contract addresses
 * (Helm ConfigMap `valueFrom`, or the fhevm-cli address artifact after a file load).
 */
export const readGatewayBootstrapInputs = async (parameters: GatewayAddressInputs): Promise<GatewayBootstrapInputs> => {
  const gatewayConfig = parameters.gatewayConfigAddress as `0x${string}`;
  const client = createPublicClient({ transport: http(parameters.gatewayRpcUrl) });
  const [gatewayChainId, coprocessorSigners, kmsSigners] = await Promise.all([
    client.getChainId(),
    client.readContract({ address: gatewayConfig, abi: GATEWAY_CONFIG_ABI, functionName: 'getCoprocessorSigners' }),
    client.readContract({ address: gatewayConfig, abi: GATEWAY_CONFIG_ABI, functionName: 'getKmsSigners' }),
  ]);
  return {
    gatewayChainId: BigInt(gatewayChainId),
    inputVerificationContract: evmAddressBytes(parameters.inputVerificationAddress),
    decryptionContract: evmAddressBytes(parameters.decryptionAddress),
    coprocessorSigners: coprocessorSigners.map(evmAddressBytes),
    kmsSigners: kmsSigners.map(evmAddressBytes),
  };
};

const requiredEnv = (name: string): string => {
  const value = process.env[name];
  if (!value) throw new Error(`missing required env ${name}`);
  return value;
};

/** Helm/Job path: contract addresses and RPC come from env (ConfigMap `valueFrom` + anvil service). */
export const readGatewayBootstrapInputsFromEnv = async (): Promise<GatewayBootstrapInputs> =>
  readGatewayBootstrapInputs({
    gatewayRpcUrl: requiredEnv('GATEWAY_RPC_URL'),
    gatewayConfigAddress: requiredEnv('GATEWAY_CONFIG_ADDRESS'),
    inputVerificationAddress: requiredEnv('INPUT_VERIFICATION_ADDRESS'),
    decryptionAddress: requiredEnv('DECRYPTION_ADDRESS'),
  });
