import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { HandleAccountPair } from '../types/other-p.js';
import { AclUserDecryptionError } from '../errors/AclError.js';
import { persistAllowed } from './persistAllowed.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly userAddress: ChecksummedAddress;
  readonly handleContractPairs: ReadonlyArray<{
    readonly handle: Handle;
    readonly contractAddress: ChecksummedAddress;
  }>;
};

////////////////////////////////////////////////////////////////////////////////

export async function checkPersistAllowed(context: Context, parameters: Parameters): Promise<void> {
  const { address, userAddress, handleContractPairs } = parameters;

  if (handleContractPairs.length === 0) {
    throw new Error('checkPersistAllowed requires at least one (handle, contractAddress) pair');
  }

  // runtime defensive check, use lowercase
  function getKey(addr: string, handleBytes32Hex: string): string {
    return `${addr}:${handleBytes32Hex}`.toLowerCase();
  }

  // 1. Verify rule: userAddress !== contractAddress
  for (const pair of handleContractPairs) {
    if (parameters.userAddress.toLowerCase() === pair.contractAddress.toLowerCase()) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `userAddress ${userAddress} should not be equal to contractAddress when requesting user decryption!`,
      });
    }
  }

  // 2. Collect all unique (address, handle) pairs to avoid duplicate RPC calls
  const allChecks: HandleAccountPair[] = [];
  const seenKeys = new Set<string>();

  for (const pair of handleContractPairs) {
    // User check (runtime defensive check, use lowercase)
    const userKey = getKey(parameters.userAddress, pair.handle.bytes32Hex);
    if (!seenKeys.has(userKey)) {
      seenKeys.add(userKey);
      allChecks.push({
        handle: pair.handle,
        account: parameters.userAddress,
      });
    }
    // Contract check (runtime defensive check, use lowercase)
    const contractKey = getKey(pair.contractAddress, pair.handle.bytes32Hex);
    if (!seenKeys.has(contractKey)) {
      seenKeys.add(contractKey);
      allChecks.push({
        handle: pair.handle,
        account: pair.contractAddress,
      });
    }
  }

  if (allChecks.length === 0) {
    throw new Error('checkPersistAllowed: no ACL checks to perform (unexpected empty dedup result)');
  }

  // 3. Single batched RPC call for all unique checks
  const allResults = await persistAllowed(context, {
    address,
    pairs: allChecks,
  });

  // 4. Build result map for lookup
  const resultMap = new Map<string, boolean>();
  for (const [i, check] of allChecks.entries()) {
    const result = allResults[i];
    // tsc: noUncheckedIndexedAccess
    if (result === undefined) {
      throw new Error(`Missing result at index ${i}`);
    }
    const key = getKey(check.account, check.handle.bytes32Hex);
    resultMap.set(key, result);
  }

  for (const pair of handleContractPairs) {
    // 5. Verify user permissions
    const userKey = getKey(parameters.userAddress, pair.handle.bytes32Hex);
    if (resultMap.get(userKey) !== true) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `User ${parameters.userAddress} is not authorized to decrypt handle ${pair.handle.bytes32Hex}!`,
      });
    }

    // 6. Verify contract permissions
    const contractKey = getKey(pair.contractAddress, pair.handle.bytes32Hex);
    if (resultMap.get(contractKey) !== true) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `Dapp contract ${pair.contractAddress} is not authorized to user decrypt handle ${pair.handle.bytes32Hex}!`,
      });
    }
  }
}
