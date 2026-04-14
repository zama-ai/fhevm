import type { HandleLike } from '../../types/encryptedTypes.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { Bytes32Hex, ChecksummedAddress } from '../../types/primitives.js';
import { assertIsChecksummedAddress } from '../../base/address.js';
import { AclUserDecryptionError } from '../../errors/AclError.js';
import { assertIsHandleLike, toHandle } from '../../handle/FhevmHandle.js';
import { persistAllowed } from './persistAllowed.js';

export type CheckUserAllowedForDecryptionParameters = {
  readonly userAddress: ChecksummedAddress;
  readonly handleContractPairs:
    | {
        readonly contractAddress: ChecksummedAddress;
        readonly handle: HandleLike;
      }
    | ReadonlyArray<{
        readonly contractAddress: ChecksummedAddress;
        readonly handle: HandleLike;
      }>;
  readonly options?: { readonly checkArguments?: boolean };
};

/**
 * Verifies that a user is allowed to decrypt handles through specific contracts.
 *
 * For each (handle, contractAddress) pair, checks that:
 * 1. The userAddress has permission to decrypt the handle
 * 2. The contractAddress has permission to decrypt the handle
 * 3. The userAddress is not equal to any contractAddress
 *
 * @throws A {@link FhevmHandleError} If checkArguments is true and any handle is not a valid Bytes32Hex
 * @throws A {@link ChecksummedAddressError} If checkArguments is true and any address is not a valid checksummed address
 * @throws A {@link AclUserDecryptionError} If userAddress equals any contractAddress
 * @throws A {@link AclUserDecryptionError} If user is not authorized to decrypt any handle
 * @throws A {@link AclUserDecryptionError} If any contract is not authorized to decrypt its handle
 */
export async function checkUserAllowedForDecryption(
  fhevm: Fhevm<FhevmChain>,
  parameters: CheckUserAllowedForDecryptionParameters,
): Promise<void> {
  // runtime defensive check, use lowercase
  function getKey(address: string, handleBytes32Hex: string): string {
    return `${address}:${handleBytes32Hex}`.toLowerCase();
  }

  // 1. convert to array
  const pairsArray: ReadonlyArray<{
    readonly contractAddress: ChecksummedAddress;
    readonly handle: HandleLike;
  }> = Array.isArray(parameters.handleContractPairs)
    ? parameters.handleContractPairs
    : [parameters.handleContractPairs];

  // 2. check arguments if needed
  if (parameters.options?.checkArguments !== false) {
    assertIsChecksummedAddress(parameters.userAddress, {});
    for (const pair of pairsArray) {
      assertIsHandleLike(pair.handle);
      assertIsChecksummedAddress(pair.contractAddress, {});
    }
  }

  // 3. Verify rule: userAddress !== contractAddress
  for (const pair of pairsArray) {
    if (parameters.userAddress.toLowerCase() === pair.contractAddress.toLowerCase()) {
      throw new AclUserDecryptionError({
        contractAddress: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        message: `userAddress ${parameters.userAddress} should not be equal to contractAddress when requesting user decryption!`,
      });
    }
  }

  const pairsArrayHex = pairsArray.map((pair) => ({
    contractAddress: pair.contractAddress,
    handleBytes32Hex: toHandle(pair.handle).bytes32Hex,
  }));

  // 4. Collect all unique (address, handle) pairs to avoid duplicate RPC calls
  const allChecks: Array<{
    address: ChecksummedAddress;
    handle: Bytes32Hex;
  }> = [];
  const seenKeys = new Set<string>();

  for (const pair of pairsArrayHex) {
    // User check (runtime defensive check, use lowercase)
    const userKey = getKey(parameters.userAddress, pair.handleBytes32Hex);
    if (!seenKeys.has(userKey)) {
      seenKeys.add(userKey);
      allChecks.push({
        address: parameters.userAddress,
        handle: pair.handleBytes32Hex,
      });
    }
    // Contract check (runtime defensive check, use lowercase)
    const contractKey = getKey(pair.contractAddress, pair.handleBytes32Hex);
    if (!seenKeys.has(contractKey)) {
      seenKeys.add(contractKey);
      allChecks.push({
        address: pair.contractAddress,
        handle: pair.handleBytes32Hex,
      });
    }
  }

  // 5. Single batched RPC call for all unique checks
  const allResults = await persistAllowed(fhevm, {
    handleAddressPairs: allChecks,
    options: {
      checkArguments: false,
    },
  });

  // 6. Build result map for lookup
  const resultMap = new Map<string, boolean>();
  for (const [i, check] of allChecks.entries()) {
    const result = allResults[i];
    // tsc: noUncheckedIndexedAccess
    if (result === undefined) {
      throw new Error(`Missing result at index ${i}`);
    }
    const key = getKey(check.address, check.handle);
    resultMap.set(key, result);
  }

  for (const pair of pairsArrayHex) {
    // 7. Verify user permissions
    const userKey = getKey(parameters.userAddress, pair.handleBytes32Hex);
    if (resultMap.get(userKey) !== true) {
      throw new AclUserDecryptionError({
        contractAddress: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        message: `User ${parameters.userAddress} is not authorized to decrypt handle ${pair.handleBytes32Hex}!`,
      });
    }

    // 8. Verify contract permissions
    const contractKey = getKey(pair.contractAddress, pair.handleBytes32Hex);
    if (resultMap.get(contractKey) !== true) {
      throw new AclUserDecryptionError({
        contractAddress: fhevm.chain.fhevm.contracts.acl.address as ChecksummedAddress,
        message: `Dapp contract ${pair.contractAddress} is not authorized to user decrypt handle ${pair.handleBytes32Hex}!`,
      });
    }
  }
}
