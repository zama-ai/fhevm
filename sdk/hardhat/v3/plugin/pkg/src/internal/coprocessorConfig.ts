// Reads the `CoprocessorConfig` a consumer contract stored when it inherited `ZamaConfig` (or called
// `FHE.setCoprocessor()`): three addresses in consecutive slots at the ERC-7201 location of
// `confidential.storage.config`. Not the SDK's chain resolution — this is what a USER's contract
// points at, so the plugin can say "your contract names ACL X, the deployed ACL is Y".

import { HardhatPluginError } from 'hardhat/plugins';
import {
  type Address,
  type Hex,
  type PublicClient,
  encodeAbiParameters,
  getAddress,
  isAddress,
  keccak256,
  stringToBytes,
  toHex,
} from 'viem';

import type { CoprocessorConfig, FhevmAddressLike } from '../types.js';
import {
  FHEVM_SOLIDITY_CONFIG_CONTRACT_NAME,
  FHEVM_SOLIDITY_CONFIG_FILE,
  FHEVM_SOLIDITY_PACKAGE_NAME,
  PLUGIN_ID,
} from './constants.js';
import type { FhevmContractsRepository } from './contracts.js';

const STORAGE_NAME = 'confidential.storage.config';
// Asserted, not trusted: a renamed namespace fails here instead of reading three unrelated words.
const STORAGE_LOCATION = 0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700n;
const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

/** `keccak256(abi.encode(uint256(keccak256(name)) - 1)) & ~bytes32(uint256(0xff))` (ERC-7201). */
export function computeStorageLocation(storageName: string): bigint {
  const inner = BigInt(keccak256(stringToBytes(storageName))) - 1n;
  const outer = BigInt(keccak256(encodeAbiParameters([{ type: 'uint256' }], [inner])));
  return outer & ~0xffn;
}

export async function resolveAddress(contract: FhevmAddressLike): Promise<Address> {
  const raw =
    typeof contract === 'string' ? contract : 'getAddress' in contract ? await contract.getAddress() : contract.address;
  if (!isAddress(raw)) throw new HardhatPluginError(PLUGIN_ID, `'${raw}' is not a valid contract address.`);
  return getAddress(raw);
}

export async function readCoprocessorConfig(
  client: PublicClient,
  contractAddress: Address,
): Promise<CoprocessorConfig> {
  const location = computeStorageLocation(STORAGE_NAME);
  if (location !== STORAGE_LOCATION) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Unexpected CoprocessorConfig storage location. Computed ${toHex(location)}, expected ${toHex(STORAGE_LOCATION)}.`,
    );
  }
  const [ACLAddress, CoprocessorAddress, KMSVerifierAddress] = await Promise.all(
    [0n, 1n, 2n].map((offset) => readAddressSlot(client, contractAddress, toHex(location + offset, { size: 32 }))),
  );
  if (ACLAddress === undefined || CoprocessorAddress === undefined || KMSVerifierAddress === undefined) {
    throw notInitialized(contractAddress);
  }
  return { ACLAddress, CoprocessorAddress, KMSVerifierAddress };
}

async function readAddressSlot(client: PublicClient, address: Address, slot: Hex): Promise<Address> {
  const word = await client.getStorageAt({ address, slot });
  if (word === undefined) throw notInitialized(address);
  const value = BigInt(word);
  if (value >= 1n << 160n) throw notInitialized(address);
  return getAddress(toHex(value, { size: 20 }));
}

function notInitialized(address: Address): HardhatPluginError {
  return new HardhatPluginError(PLUGIN_ID, `The contract at address ${address} has not been initialized properly.`);
}

/** The consumer contract points at the stack this connection runs — the classic Sepolia-config-on-localhost mistake. */
export async function assertCoprocessorInitialized(
  repository: FhevmContractsRepository,
  contract: FhevmAddressLike,
  contractName: string | undefined,
): Promise<void> {
  const contractAddress = await resolveAddress(contract);
  const expected: CoprocessorConfig = {
    ACLAddress: repository.acl.address,
    CoprocessorAddress: repository.fhevmExecutor.address,
    KMSVerifierAddress: repository.kmsVerifier.address,
  };
  const prefix =
    contractName === undefined ? `Contract at ${contractAddress}` : `Contract ${contractName} at ${contractAddress}`;
  const configFile = `${FHEVM_SOLIDITY_PACKAGE_NAME}/${FHEVM_SOLIDITY_CONFIG_FILE}`;

  const actual = await readCoprocessorConfig(repository.client, contractAddress);
  if (Object.values(actual).includes(ZERO_ADDRESS)) {
    throw new HardhatPluginError(
      PLUGIN_ID,
      `${prefix} is not initialized for FHE operations. Make sure it either inherits from ${configFile}:${FHEVM_SOLIDITY_CONFIG_CONTRACT_NAME} or explicitly calls FHE.setCoprocessor() in its constructor.`,
    );
  }
  for (const key of ['ACLAddress', 'CoprocessorAddress', 'KMSVerifierAddress'] as const) {
    if (actual[key].toLowerCase() === expected[key].toLowerCase()) continue;
    throw new HardhatPluginError(
      PLUGIN_ID,
      `Coprocessor ${key} mismatch. ${prefix} was initialized with FHEVM contract addresses that do not match the currently deployed FHEVM contracts. ` +
        `This is likely due to incorrect addresses in the file ${configFile}. ${key}: ${actual[key]}, expected ${key}: ${expected[key]}`,
    );
  }
}
