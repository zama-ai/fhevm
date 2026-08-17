import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { getCurrentKmsContextAndEpochAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { isVersionStrictlyBefore } from './HostContractVersion-p.js';
import { assertIsUint256 } from '../base/uint.js';
import { CACHE_TTL_15MIN, createCachedFetch } from '../base/cachedFetch.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly protocolConfigAddress: ChecksummedAddress;
  readonly fhevmContext: FhevmClientFrozenContext;
};

type ReturnType = { readonly contextId: Uint256BigInt; readonly epochId: Uint256BigInt };

////////////////////////////////////////////////////////////////////////////////

const cachedGetCurrentKmsContextAndEpoch = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getCurrentKmsContextAndEpoch,
  cacheKeyFn: (context, params) => `${context.runtime.uid.toLowerCase()}:${params.protocolConfigAddress.toLowerCase()}`,
  // Host contract values are immutable per deployment, so a long TTL is safe.
  ttlMs: CACHE_TTL_15MIN,
});

/**
 * Reads the current KMS context ID and epoch ID from a ProtocolConfig contract.
 *
 * Requires ProtocolConfig >= v0.2.0 (protocol v0.14.0).
 *
 * Results are cached per (runtime, address) with a 24-hour TTL.
 * Concurrent callers share a single in-flight RPC request (deduplication).
 *
 * @param parameters.address - The checksummed address of the ProtocolConfig contract.
 * @param parameters.forceRefresh - If `true`, invalidates the cached entry and
 *   makes a fresh RPC call. The new result is stored back in the cache.
 */
export function getCurrentKmsContextAndEpoch(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean | undefined },
): Promise<ReturnType> {
  return cachedGetCurrentKmsContextAndEpoch.execute(context, parameters);
}

async function _getCurrentKmsContextAndEpoch(context: Context, parameters: Parameters): Promise<ReturnType> {
  const protocolConfigVersion = parameters.fhevmContext.hostContractVersion('ProtocolConfig');

  // getCurrentKmsContextAndEpoch requires ProtocolConfig >= v0.2.0 (protocol v0.14.0)
  if (isVersionStrictlyBefore(protocolConfigVersion, { major: 0, minor: 2 })) {
    throw new Error(
      'ProtocolConfig.getCurrentKmsContextAndEpoch() requires ProtocolConfig >= v0.2.0 (protocol v0.14.0)',
    );
  }

  const trustedClient = getTrustedClient(context);

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: parameters.protocolConfigAddress,
    abi: getCurrentKmsContextAndEpochAbi,
    args: [],
    functionName: getCurrentKmsContextAndEpochAbi[0].name,
  });

  if (!Array.isArray(res) || res.length < 2) {
    throw new Error(`Invalid getCurrentKmsContextAndEpoch result.`);
  }

  const unknownContextId = res[0] as unknown;
  const unknownEpochId = res[1] as unknown;

  try {
    assertIsUint256(unknownContextId, {});
  } catch (e) {
    throw new Error(`Invalid KMS Context Id.`, { cause: e });
  }

  try {
    assertIsUint256(unknownEpochId, {});
  } catch (e) {
    throw new Error(`Invalid KMS Epoch Id.`, { cause: e });
  }

  return Object.freeze({
    contextId: BigInt(unknownContextId) as Uint256BigInt,
    epochId: BigInt(unknownEpochId) as Uint256BigInt,
  });
}
