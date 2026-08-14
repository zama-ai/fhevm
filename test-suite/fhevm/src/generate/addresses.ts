/**
 * Renders address artifacts consumed by contracts, operators, and local tooling after deployment discovery.
 */
import type { HostChainScenario, State } from "../types";

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
  "PROTOCOL_CONFIG_CONTRACT_ADDRESS",
  "KMS_GENERATION_CONTRACT_ADDRESS",
  "CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS",
  "LZ_ENDPOINT_ADDRESS",
] as const;

const renderHostChainAddressesEnv = (addresses?: Record<string, string>) =>
  renderEnvFile(HOST_ADDRESS_KEYS.map((key) => [key, addresses?.[key]]));

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
    ["protocolConfigAdd", host?.PROTOCOL_CONFIG_CONTRACT_ADDRESS],
    ["kmsGenerationAdd", host?.KMS_GENERATION_CONTRACT_ADDRESS],
    ["confidentialBridgeAdd", host?.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS],
  ]);
};

/**
 * Cleartext-mode (chainid 31337) coprocessor config, preserved for CLEARTEXT builds. These
 * are fixed local addresses, not discovered ones, so they stay static across regenerations.
 */
const CLEARTEXT_COPROCESSOR_CONFIG = {
  aclAddress: "0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D",
  coprocessorAddress: "0xe3a9105a3a932253A70F126eb1E3b589C643dD24",
  kmsVerifierAddress: "0x901F8942346f7AB3a01F6D7613119Bca447Bb030",
} as const;

/** Renders a `CoprocessorConfig({...})` struct literal, indented to sit inside a `return`. */
const renderCoprocessorConfigLiteral = (acl: string, coprocessor: string, kmsVerifier: string) =>
  [
    "CoprocessorConfig({",
    `                ACLAddress: ${acl},`,
    `                CoprocessorAddress: ${coprocessor},`,
    `                KMSVerifierAddress: ${kmsVerifier}`,
    "            })",
  ].join("\n");

/**
 * Renders `E2ECoprocessorConfigLocal.sol` with one `block.chainid` branch per host chain.
 *
 * The e2e `BridgeApp` (and the generated operations contracts) are a single bytecode deployed
 * to every host chain, and their `E2ECoprocessorConfig` base calls `FHE.setCoprocessor` in the
 * constructor. A single hardcoded address set therefore points every chain's instance at one
 * chain's FHEVM system contracts — which silently breaks cross-chain flows (e.g. the confidential
 * bridge's destination compose leg, where the dst app's `makePubliclyDecryptable` runs against the
 * wrong ACL). Selecting the config by `block.chainid` from discovered per-chain addresses fixes it.
 *
 * Note the struct's `CoprocessorAddress` is the FHEVMExecutor, hence `FHEVM_EXECUTOR_CONTRACT_ADDRESS`.
 */
export const renderE2ECoprocessorConfigSolidity = (
  state: Pick<State, "discovery">,
  hostChains: Array<Pick<HostChainScenario, "key" | "chainId">>,
): string => {
  const branches = hostChains.map((chain) => {
    const host = state.discovery?.hosts[chain.key];
    const acl = host?.ACL_CONTRACT_ADDRESS;
    const coprocessor = host?.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
    const kmsVerifier = host?.KMS_VERIFIER_CONTRACT_ADDRESS;
    if (!acl || !coprocessor || !kmsVerifier) {
      throw new Error(
        `renderE2ECoprocessorConfigSolidity: missing discovered ACL/executor/KMS-verifier address for host chain "${chain.key}"`,
      );
    }
    return `        if (block.chainid == ${chain.chainId}) {
            return ${renderCoprocessorConfigLiteral(acl, coprocessor, kmsVerifier)};
        }`;
  });

  return `${SOLIDITY_HEADER}// AUTO-GENERATED by the fhevm CLI (generate step) from discovered host-chain addresses.
// Do not edit by hand: the per-chain branches are injected so the shared BridgeApp / operations
// bytecode selects the correct FHEVM system contracts by block.chainid.

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    function getConfig() internal view returns (CoprocessorConfig memory) {
        if (block.chainid == 31337) {
            return _getCleartextConfig();
        }
${branches.join("\n")}
        revert("E2ECoprocessorConfig: no coprocessor config for this chainid");
    }

    function _getCleartextConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: ${CLEARTEXT_COPROCESSOR_CONFIG.aclAddress},
                CoprocessorAddress: ${CLEARTEXT_COPROCESSOR_CONFIG.coprocessorAddress},
                KMSVerifierAddress: ${CLEARTEXT_COPROCESSOR_CONFIG.kmsVerifierAddress}
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}
`;
};
