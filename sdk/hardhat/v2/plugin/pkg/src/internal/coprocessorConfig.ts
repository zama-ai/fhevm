import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';

/**
 * Reads the `CoprocessorConfig` a dApp contract was initialized with.
 *
 * This has no `@fhevm/sdk` equivalent and is not the same thing as the SDK's `resolveFhevmConfig`:
 * that resolves a *chain*'s configuration from the deployed protocol contracts, whereas this reads
 * the struct a *user's own contract* stored when it inherited `ZamaConfig` (or called
 * `FHE.setCoprocessor()`). It is what lets the plugin say "your contract points at ACL X, but the
 * deployed ACL is Y".
 *
 * Ported from `@fhevm/mock-utils`, which the plugin no longer depends on.
 */

/**
 * Maps the Solidity `CoprocessorConfig` struct in `@fhevm/solidity/lib/Impl.sol`.
 */
export type CoprocessorConfig = {
  ACLAddress: `0x${string}`;
  CoprocessorAddress: `0x${string}`;
  KMSVerifierAddress: `0x${string}`;
};

/**
 * The ERC-7201 storage location for a namespace:
 * `keccak256(abi.encode(uint256(keccak256(namespace)) - 1)) & ~bytes32(uint256(0xff))`.
 */
function computeStorageLocation(storageName: string): string {
  const enc = EthersT.AbiCoder.defaultAbiCoder().encode(
    ['uint256'],
    [BigInt(EthersT.keccak256(EthersT.toUtf8Bytes(storageName))) - 1n],
  );
  return EthersT.toBeHex(
    BigInt(EthersT.keccak256(enc)) & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00n,
    32,
  );
}

/**
 * `@fhevm/solidity`'s namespace for the struct, and the slot it hashes to. The constant is asserted
 * rather than trusted: if `@fhevm/solidity` ever renames the namespace, this fails loudly here
 * instead of silently reading three unrelated words of storage.
 */
const COPROCESSOR_CONFIG_STORAGE_NAME = 'confidential.storage.config';
const COPROCESSOR_CONFIG_STORAGE_LOCATION = '0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700';

export async function getCoprocessorConfig(
  provider: EthersT.Provider,
  contractAddress: string,
): Promise<CoprocessorConfig> {
  const storageLocation = computeStorageLocation(COPROCESSOR_CONFIG_STORAGE_NAME);
  if (storageLocation !== COPROCESSOR_CONFIG_STORAGE_LOCATION) {
    throw new HardhatFhevmError(
      `Unexpected CoprocessorConfig storage location. Computed ${storageLocation}, expected ${COPROCESSOR_CONFIG_STORAGE_LOCATION}.`,
    );
  }

  /*
    struct CoprocessorConfig {
      address ACLAddress;
      address CoprocessorAddress;
      address KMSVerifierAddress;
    }
    Three addresses, one per slot, laid out consecutively from the namespace location.
  */
  const addresses = await __readAddressesFromStorage(provider, contractAddress, storageLocation, 3);
  const [ACLAddress, CoprocessorAddress, KMSVerifierAddress] = addresses;

  if (ACLAddress === undefined || CoprocessorAddress === undefined || KMSVerifierAddress === undefined) {
    throw new HardhatFhevmError(
      `Unexpected CoprocessorConfig storage read at ${contractAddress}: expected 3 addresses, got ${addresses.length}.`,
    );
  }

  return { ACLAddress, CoprocessorAddress, KMSVerifierAddress };
}

// eslint-disable-next-line @typescript-eslint/naming-convention
async function __readAddressesFromStorage(
  provider: EthersT.Provider,
  contractAddress: string,
  storageLocationBytes32: string,
  numAddresses: number,
): Promise<Array<`0x${string}`>> {
  const errorMsg = `The contract at address ${contractAddress} has not been initialized properly.`;
  const addresses: Array<`0x${string}`> = [];

  for (let i = 0; i < numAddresses; ++i) {
    const raw = await provider.getStorage(contractAddress, BigInt(storageLocationBytes32) + BigInt(i));
    if (typeof raw !== 'string' || !EthersT.isBytesLike(raw) || raw.length !== 66) {
      throw new HardhatFhevmError(errorMsg);
    }
    try {
      addresses.push(EthersT.getAddress(EthersT.toBeHex(BigInt(raw), 20)) as `0x${string}`);
    } catch {
      throw new HardhatFhevmError(errorMsg);
    }
  }

  return addresses;
}
