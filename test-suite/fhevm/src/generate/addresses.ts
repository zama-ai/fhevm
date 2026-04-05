/**
 * Renders address artifacts consumed by contracts, operators, and local tooling after deployment discovery.
 */
import type { State } from "../types";

const SOLIDITY_HEADER = `// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

`;

/** Renders address entries into dotenv file contents. */
const renderEnvFile = (entries: Array<[string, string | undefined]>) =>
  entries
    .filter(([, value]) => value)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n")
    .concat("\n");

/** Renders address entries into a Solidity constants file. */
const renderSolidityFile = (entries: Array<[string, string | undefined]>) =>
  SOLIDITY_HEADER +
  entries
    .filter(([, value]) => value)
    .map(([name, value]) => `address constant ${name} = ${value};`)
    .join("\n")
    .concat("\n");

/** Renders discovered gateway addresses into a dotenv artifact. */
export const renderGatewayAddressesEnv = (state: Pick<State, "discovery">) =>
  renderEnvFile([
    ["GATEWAY_CONFIG_ADDRESS", state.discovery?.gateway.GATEWAY_CONFIG_ADDRESS],
    ["INPUT_VERIFICATION_ADDRESS", state.discovery?.gateway.INPUT_VERIFICATION_ADDRESS],
    ["KMS_GENERATION_ADDRESS", state.discovery?.gateway.KMS_GENERATION_ADDRESS],
    ["CIPHERTEXT_COMMITS_ADDRESS", state.discovery?.gateway.CIPHERTEXT_COMMITS_ADDRESS],
    ["DECRYPTION_ADDRESS", state.discovery?.gateway.DECRYPTION_ADDRESS],
    ["PROTOCOL_PAYMENT_ADDRESS", state.discovery?.gateway.PROTOCOL_PAYMENT_ADDRESS],
    ["PAUSER_SET_ADDRESS", state.discovery?.gateway.PAUSER_SET_ADDRESS],
    ["MULTICHAIN_ACL_ADDRESS", state.discovery?.gateway.MULTICHAIN_ACL_ADDRESS],
  ]);

/** Renders discovered gateway addresses into Solidity constants. */
export const renderGatewayAddressesSolidity = (state: Pick<State, "discovery">) =>
  renderSolidityFile([
    ["gatewayConfigAddress", state.discovery?.gateway.GATEWAY_CONFIG_ADDRESS],
    ["inputVerificationAddress", state.discovery?.gateway.INPUT_VERIFICATION_ADDRESS],
    ["kmsGenerationAddress", state.discovery?.gateway.KMS_GENERATION_ADDRESS],
    ["ciphertextCommitsAddress", state.discovery?.gateway.CIPHERTEXT_COMMITS_ADDRESS],
    ["decryptionAddress", state.discovery?.gateway.DECRYPTION_ADDRESS],
    ["multichainACLAddress", state.discovery?.gateway.MULTICHAIN_ACL_ADDRESS],
    ["protocolPaymentAddress", state.discovery?.gateway.PROTOCOL_PAYMENT_ADDRESS],
    ["pauserSetAddress", state.discovery?.gateway.PAUSER_SET_ADDRESS],
  ]);

/** Renders payment-bridging gateway addresses into Solidity constants. */
export const renderPaymentBridgingAddressesSolidity = (gatewayEnv: Record<string, string>) =>
  renderSolidityFile([
    ["zamaOFTAddress", gatewayEnv.ZAMA_OFT_ADDRESS],
    ["feesSenderToBurnerAddress", gatewayEnv.FEES_SENDER_TO_BURNER_ADDRESS],
  ]);

const HOST_ADDRESS_KEYS = [
  "ACL_CONTRACT_ADDRESS",
  "FHEVM_EXECUTOR_CONTRACT_ADDRESS",
  "KMS_VERIFIER_CONTRACT_ADDRESS",
  "INPUT_VERIFIER_CONTRACT_ADDRESS",
  "HCU_LIMIT_CONTRACT_ADDRESS",
  "PAUSER_SET_CONTRACT_ADDRESS",
] as const;

const SOLANA_HOST_ADDRESS_KEY_ORDER = [
  "HOST_CHAIN_KIND",
  "SOLANA_HOST_RPC_URL",
  "SOLANA_HOST_WS_URL",
  "SOLANA_HOST_KIND",
  "SOLANA_HOST_PROGRAM_ID",
  "SOLANA_HOST_STATE_PDA",
  "SOLANA_HOST_SESSION_PDA",
  "SOLANA_HOST_ACL_PROGRAM_ID",
  "SOLANA_TEST_INPUT_PROGRAM_ID",
  "SOLANA_TEST_INPUT_STATE_PDA",
  "SOLANA_CONFIDENTIAL_TOKEN_PROGRAM_ID",
  "SOLANA_CONFIDENTIAL_TOKEN_STATE_PDA",
  "SOLANA_HOST_AUTHORITY",
  "SOLANA_TOKEN_RECIPIENT",
  "SOLANA_HOST_CHAIN_ID",
  "CHAIN_ID_GATEWAY",
  "INPUT_VERIFICATION_ADDRESS",
  "DECRYPTION_ADDRESS",
  "NUM_COPROCESSORS",
  "COPROCESSOR_THRESHOLD",
  "NUM_KMS_NODES",
  "PUBLIC_DECRYPTION_THRESHOLD",
  ...HOST_ADDRESS_KEYS,
] as const;

const isSolanaHostAddresses = (addresses?: Record<string, string>) =>
  addresses?.HOST_CHAIN_KIND === "solana" || Boolean(addresses?.SOLANA_HOST_PROGRAM_ID);

const renderSolanaHostChainAddressesEnv = (addresses: Record<string, string>) => {
  const explicitOrder = new Map(SOLANA_HOST_ADDRESS_KEY_ORDER.map((key, index) => [key, index]));
  const entries = Object.entries(addresses)
    .filter(([, value]) => Boolean(value))
    .sort(([left], [right]) => {
      const leftOrder = explicitOrder.get(left);
      const rightOrder = explicitOrder.get(right);
      if (leftOrder != null || rightOrder != null) {
        return (leftOrder ?? Number.MAX_SAFE_INTEGER) - (rightOrder ?? Number.MAX_SAFE_INTEGER);
      }
      return left.localeCompare(right);
    });
  return renderEnvFile(entries);
};

const renderHostChainAddressesEnv = (addresses?: Record<string, string>) => {
  if (!addresses) {
    return renderEnvFile([]);
  }
  if (isSolanaHostAddresses(addresses)) {
    return renderSolanaHostChainAddressesEnv(addresses);
  }
  return renderEnvFile(HOST_ADDRESS_KEYS.map((key) => [key, addresses[key]]));
};

/** Renders discovered host addresses for a given chain key into a dotenv artifact. */
export const renderHostChainAddresses = (state: Pick<State, "discovery">, chainKey: string) =>
  renderHostChainAddressesEnv(state.discovery?.hosts[chainKey]);

/** Renders discovered host addresses for a given chain key into Solidity constants. */
export const renderHostChainAddressesSolidity = (state: Pick<State, "discovery">, chainKey: string) => {
  const host = state.discovery?.hosts[chainKey];
  return renderSolidityFile([
    ["aclAdd", host?.ACL_CONTRACT_ADDRESS],
    ["fhevmExecutorAdd", host?.FHEVM_EXECUTOR_CONTRACT_ADDRESS],
    ["kmsVerifierAdd", host?.KMS_VERIFIER_CONTRACT_ADDRESS],
    ["inputVerifierAdd", host?.INPUT_VERIFIER_CONTRACT_ADDRESS],
    ["hcuLimitAdd", host?.HCU_LIMIT_CONTRACT_ADDRESS],
    ["pauserSetAdd", host?.PAUSER_SET_CONTRACT_ADDRESS],
  ]);
};
