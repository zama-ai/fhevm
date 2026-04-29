import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { HandleContractPair } from '../types/other-p.js';
import { AclUserDecryptionError } from '../errors/AclError.js';
import { isHandleDelegatedForUserDecryption } from './isHandleDelegatedForUserDecryption-p.js';
import { WILDCARD_DELEGATION_ADDRESS } from './constants.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly delegator: ChecksummedAddress;
  readonly delegate: ChecksummedAddress;
  readonly handleContractPairs: ReadonlyArray<{
    readonly handle: Handle;
    readonly contractAddress: ChecksummedAddress;
  }>;
};

////////////////////////////////////////////////////////////////////////////////

export async function checkDelegation(context: Context, parameters: Parameters): Promise<void> {
  const { address, delegator, delegate, handleContractPairs } = parameters;

  if (handleContractPairs.length === 0) {
    throw new Error('checkDelegation requires at least one (handle, contractAddress) pair');
  }

  // runtime defensive check, use lowercase
  function getKey(addr: string, handleBytes32Hex: string): string {
    return `${addr}:${handleBytes32Hex}`.toLowerCase();
  }

  // 1. Verify rule: delegator !== delegate
  if (delegator.toLowerCase() === delegate.toLowerCase()) {
    throw new AclUserDecryptionError({
      contractAddress: address,
      message: `delegator ${delegator} cannot be equal to delegate`,
    });
  }

  // 2. Verify rule: delegate !== WILDCARD_CONTRACT
  if (delegate.toLowerCase() === WILDCARD_DELEGATION_ADDRESS.toLowerCase()) {
    throw new AclUserDecryptionError({
      contractAddress: address,
      message: `delegate cannot be equal to wildcard contract address`,
    });
  }
  if (delegator.toLowerCase() === WILDCARD_DELEGATION_ADDRESS.toLowerCase()) {
    throw new AclUserDecryptionError({
      contractAddress: address,
      message: `delegator cannot be equal to wildcard contract address`,
    });
  }

  // 3. Verify rules: delegator !== contractAddress & delegate !== contractAddress
  for (const pair of handleContractPairs) {
    if (delegator.toLowerCase() === pair.contractAddress.toLowerCase()) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `delegator ${delegator} cannot be equal to contractAddress`,
      });
    }
    if (delegate.toLowerCase() === pair.contractAddress.toLowerCase()) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `delegate ${delegate} cannot be equal to contractAddress`,
      });
    }
  }

  // 4. Collect all unique (contractAddress, handle) pairs to avoid duplicate RPC calls
  const allChecks: HandleContractPair[] = [];
  const seenKeys = new Set<string>();

  for (const pair of handleContractPairs) {
    // runtime defensive check, use lowercase
    const key = getKey(pair.contractAddress, pair.handle.bytes32Hex);
    if (!seenKeys.has(key)) {
      seenKeys.add(key);
      allChecks.push({
        handle: pair.handle,
        contractAddress: pair.contractAddress,
      });
    }
  }

  if (allChecks.length === 0) {
    throw new Error('checkDelegation: no delegation checks to perform (unexpected empty dedup result)');
  }

  // 5. Single batched RPC call for all unique checks
  const allResults = await isHandleDelegatedForUserDecryption(context, {
    address,
    delegator,
    delegate,
    pairs: allChecks,
  });

  // 6. Build result map for lookup
  const resultMap = new Map<string, boolean>();
  for (const [i, check] of allChecks.entries()) {
    const result = allResults[i];
    // tsc: noUncheckedIndexedAccess
    if (result === undefined) {
      throw new Error(`Missing result at index ${i}`);
    }
    const key = getKey(check.contractAddress, check.handle.bytes32Hex);
    resultMap.set(key, result);
  }

  // 7. Verify delegation for each requested pair
  for (const pair of handleContractPairs) {
    const key = getKey(pair.contractAddress, pair.handle.bytes32Hex);
    if (resultMap.get(key) !== true) {
      throw new AclUserDecryptionError({
        contractAddress: address,
        message: `Delegate ${delegate} is not delegated by ${delegator} to user decrypt handle ${pair.handle.bytes32Hex} on contract ${pair.contractAddress}!`,
      });
    }
  }
}
