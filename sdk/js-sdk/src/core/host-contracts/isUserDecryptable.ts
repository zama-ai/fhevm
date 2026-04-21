import type { ChecksummedAddress } from '../types/primitives.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { HandleAccountPair } from '../types/other-p.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
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

type ReturnType = ReadonlyArray<{
  readonly contractAllowed: boolean;
  readonly userAllowed: boolean;
}>;

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 * See FHE.sol : isUserDecryptable()
 */
export async function isUserDecryptable(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { address, userAddress, handleContractPairs } = parameters;

  function getKey(addr: string, handleBytes32Hex: string): string {
    return `${addr}:${handleBytes32Hex}`.toLowerCase();
  }

  const dedupedChecks: HandleAccountPair[] = [];
  const seenKeys = new Set<string>();

  for (const pair of handleContractPairs) {
    const contractKey = getKey(pair.contractAddress, pair.handle.bytes32Hex);
    if (!seenKeys.has(contractKey)) {
      seenKeys.add(contractKey);
      dedupedChecks.push({
        handle: pair.handle,
        account: pair.contractAddress,
      });
    }

    const userKey = getKey(userAddress, pair.handle.bytes32Hex);
    if (!seenKeys.has(userKey)) {
      seenKeys.add(userKey);
      dedupedChecks.push({
        handle: pair.handle,
        account: userAddress,
      });
    }
  }

  const dedupedResults = await persistAllowed(context, {
    address,
    pairs: dedupedChecks,
  });

  const resultMap = new Map<string, boolean>();
  for (const [i, check] of dedupedChecks.entries()) {
    const result = dedupedResults[i];
    if (result === undefined) {
      throw new Error(`Missing result at index ${i}`);
    }
    resultMap.set(getKey(check.account, check.handle.bytes32Hex), result);
  }

  return handleContractPairs.map((pair) => {
    const contractAllowed = resultMap.get(getKey(pair.contractAddress, pair.handle.bytes32Hex));
    const userAllowed = resultMap.get(getKey(userAddress, pair.handle.bytes32Hex));

    if (contractAllowed === undefined || userAllowed === undefined) {
      throw new Error('Missing deduped persistAllowed result');
    }

    return {
      contractAllowed,
      userAllowed,
    };
  });
}
