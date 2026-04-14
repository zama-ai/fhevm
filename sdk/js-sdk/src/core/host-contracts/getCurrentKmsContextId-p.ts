import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { getCurrentKmsContextIdAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { getVersion, isVersionStrictlyBefore } from './HostContractVersion-p.js';
import { assertIsUint256 } from '../base/uint.js';
import { CACHE_TTL_24H, createCachedFetch } from '../base/cachedFetch.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = Uint256BigInt;

////////////////////////////////////////////////////////////////////////////////

const cachedGetCurrentKmsContextId = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getCurrentKmsContextId,
  cacheKeyFn: (context, params) => `${context.runtime.uid.toLowerCase()}:${params.address.toLowerCase()}`,
  // Host contract versions are immutable per deployment, so a long TTL is safe.
  ttlMs: CACHE_TTL_24H,
});

/**
 * Reads the current KMS context ID for a KMSVerifier contract.
 *
 * Results are cached per (runtime, address) with a 24-hour TTL.
 * Concurrent callers share a single in-flight RPC request (deduplication).
 *
 * @param parameters.address - The checksummed address of the KMSVerifier contract.
 * @param parameters.forceRefresh - If `true`, invalidates the cached entry and
 *   makes a fresh RPC call. The new result is stored back in the cache.
 */
export function getCurrentKmsContextId(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean | undefined },
): Promise<ReturnType> {
  return cachedGetCurrentKmsContextId.execute(context, parameters);
}

async function _getCurrentKmsContextId(context: Context, parameters: Parameters): Promise<ReturnType> {
  const version = await getVersion(context, parameters);
  // getCurrentKmsContextId has been introduced in KMSVerifier.sol v0.2.0
  if (isVersionStrictlyBefore(version, { major: 0, minor: 2 })) {
    return 0n as Uint256BigInt;
  }

  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getCurrentKmsContextIdAbi,
    args: [],
    functionName: getCurrentKmsContextIdAbi[0].name,
  });

  try {
    assertIsUint256(res, {});
  } catch (e) {
    throw new Error(`Invalid KMS Context Id.`, {
      cause: e,
    });
  }

  return BigInt(res) as Uint256BigInt;
}
