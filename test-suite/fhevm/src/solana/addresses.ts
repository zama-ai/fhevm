// addresses — the live gateway inputs the Solana host bootstrap reads before it can configure
// zama-host: the deployed EVM contract addresses from the fhevm-cli address artifact plus the
// coprocessor/KMS signer sets registered on the gateway. This replaces the retired
// `setup-solana-side.sh` "[1/5] gathering live gateway addresses" phase (dotenv `source` + `cast
// call`) with the same reads done in-process, so the bootstrap tracks whatever signer the running
// stack actually generated — no hardcoded values.

import { gatewayAddressesPath } from "../layout";
import { readEnvFile } from "../utils/fs";

// RFC-021 Solana host chain id: the chain-type high bit ORed over 12345. The coprocessor DB
// stores chain ids as PostgreSQL BIGINT, so the same bit pattern reads back as the negative i64.
export const SOLANA_HOST_CHAIN_ID = 9223372036854788153n;
export const SOLANA_HOST_CHAIN_ID_I64 = SOLANA_HOST_CHAIN_ID - (1n << 64n);

// 4-byte EVM function selectors, pinned from the GatewayConfig ABI the retired bash resolved with
// `cast call` — keccak-256("<signature>")[0..4]. Both getters take no arguments and return
// `address[]`.
const GET_COPROCESSOR_SIGNERS_CALLDATA = "0x9164d0ae"; // getCoprocessorSigners()
const GET_KMS_SIGNERS_CALLDATA = "0x7eaac8f2"; // getKmsSigners()

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

/** Decodes a 0x-prefixed 20-byte EVM address into its raw bytes. */
export const evmAddressBytes = (address: string): Uint8Array => {
  const hex = address.replace(/^0x/, "");
  if (hex.length !== 40 || !/^[0-9a-f]{40}$/i.test(hex)) {
    throw new Error(`expected a 20-byte EVM address, got "${address}"`);
  }
  return Uint8Array.from(Buffer.from(hex, "hex"));
};

/**
 * Decodes an ABI-encoded `address[]` return value (head offset word, length word, then one
 * left-padded 32-byte word per address) into raw 20-byte entries.
 */
export const decodeEvmAddressArray = (returnData: string): readonly Uint8Array[] => {
  const hex = returnData.replace(/^0x/, "");
  const word = (index: number): string => {
    const start = index * 64;
    const value = hex.slice(start, start + 64);
    if (value.length !== 64) throw new Error(`address[] return data truncated at word ${index}`);
    return value;
  };
  const offsetWords = Number(BigInt(`0x${word(0)}`)) / 32;
  const length = Number(BigInt(`0x${word(offsetWords)}`));
  return Array.from({ length }, (_, index) => {
    const entry = word(offsetWords + 1 + index);
    if (!/^0{24}/.test(entry)) throw new Error(`address[] entry ${index} is not a left-padded address`);
    return evmAddressBytes(entry.slice(24));
  });
};

const gatewayRpcRequest = async (rpcUrl: string, method: string, params: readonly unknown[]): Promise<string> => {
  const response = await fetch(rpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params }),
  });
  const body = (await response.json()) as { result?: string; error?: { message?: string } };
  if (typeof body.result !== "string") {
    throw new Error(`${method} against ${rpcUrl} failed: ${body.error?.message ?? "no result"}`);
  }
  return body.result;
};

const gatewayAddressArrayCall = async (rpcUrl: string, contract: string, calldata: string) =>
  decodeEvmAddressArray(await gatewayRpcRequest(rpcUrl, "eth_call", [{ to: contract, data: calldata }, "latest"]));

/**
 * Reads the gateway inputs the zama-host bootstrap needs: contract addresses from the fhevm-cli
 * address artifact (`.fhevm/runtime/addresses/gateway/.env.gateway`), signer sets and chain id
 * live from the gateway RPC.
 */
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
  const gatewayConfig = required("GATEWAY_CONFIG_ADDRESS");
  const [gatewayChainIdHex, coprocessorSigners, kmsSigners] = await Promise.all([
    gatewayRpcRequest(parameters.gatewayRpcUrl, "eth_chainId", []),
    gatewayAddressArrayCall(parameters.gatewayRpcUrl, gatewayConfig, GET_COPROCESSOR_SIGNERS_CALLDATA),
    gatewayAddressArrayCall(parameters.gatewayRpcUrl, gatewayConfig, GET_KMS_SIGNERS_CALLDATA),
  ]);
  return {
    gatewayChainId: BigInt(gatewayChainIdHex),
    inputVerificationContract: evmAddressBytes(required("INPUT_VERIFICATION_ADDRESS")),
    decryptionContract: evmAddressBytes(required("DECRYPTION_ADDRESS")),
    coprocessorSigners,
    kmsSigners,
  };
};
