import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { asUint8Number, isUint8 } from '../base/uint.js';
import { getVersion } from './HostContractVersion-p.js';
import { isVersionStrictlyBefore } from '../host-contracts/HostContractVersion-p.js';
import { executeWithBatching } from '../base/promise.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { getKmsSignersAbi, getThresholdAbi } from './abi-fragments/fragments.js';
import { CACHE_TTL_24H, createCachedFetch } from '../base/cachedFetch.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
};

type ReturnType = {
  readonly threshold: Uint8Number;
  readonly signers: readonly ChecksummedAddress[];
};

////////////////////////////////////////////////////////////////////////////////

const cachedGetKmsContextSignersAndThreshold = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getKmsContextSignersAndThreshold,
  cacheKeyFn: (context, params) => `${context.runtime.uid.toLowerCase()}:${params.address.toLowerCase()}`,
  // Host contract versions are immutable per deployment, so a long TTL is safe.
  ttlMs: CACHE_TTL_24H,
});

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the global KMS signers and threshold.
 *
 * Only available on KMSVerifier contracts < v0.2.0 (which have a single,
 * global set of signers with no context ID support). Throws on newer versions
 * — use {@link getContextSignersAndThresholdFromExtraData} instead.
 *
 * **Caching:** Results are cached per (runtime, address) with a 24-hour TTL.
 * In KMSVerifier v0.1.x, signers and threshold are considered immutable for
 * the duration of the session, so the TTL is a safety net for long-running
 * environments rather than a correctness requirement.
 * Concurrent callers share a single in-flight RPC request (deduplication).
 *
 * @param parameters.address - The checksummed address of the KMSVerifier contract.
 * @param parameters.forceRefresh - If `true`, invalidates the cached entry and
 *   makes a fresh RPC call. The new result is stored back in the cache.
 */
export function getKmsSignersAndThreshold(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean | undefined },
): Promise<ReturnType> {
  return cachedGetKmsContextSignersAndThreshold.execute(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _getKmsContextSignersAndThreshold(context: Context, parameters: Parameters): Promise<ReturnType> {
  const version = await getVersion(context, { address: parameters.address });
  if (!isVersionStrictlyBefore(version, { major: 0, minor: 2 })) {
    throw new Error('getContextSignersAndThreshold requires KMSVerifier < v0.2.0');
  }
  ////////////////////////////////////////////////////////////////////////////
  //
  // Important remark:
  // =================
  // Do NOTE USE `Promise.all` here!
  // You may get a server response 500 Internal Server Error
  // "Batch of more than 3 requests are not allowed on free tier, to use this
  // feature register paid account at drpc.org"
  //
  ////////////////////////////////////////////////////////////////////////////

  const rpcCalls = [() => _getThreshold(context, parameters), () => _getKmsSigners(context, parameters)];

  const res = await executeWithBatching<unknown>(rpcCalls, context.options.batchRpcCalls);

  const threshold = res[0];
  const kmsSigners = res[1];

  if (!isUint8(threshold)) {
    throw new Error(`Invalid KMSVerifier kms signers threshold.`);
  }

  try {
    assertIsChecksummedAddressArray(kmsSigners, {});
  } catch (e) {
    throw new Error(`Invalid KMSVerifier kms signers addresses.`, {
      cause: e,
    });
  }

  return Object.freeze({
    threshold: asUint8Number(Number(threshold)),
    signers: kmsSigners,
  });
}

////////////////////////////////////////////////////////////////////////////////

async function _getThreshold(context: Context, parameters: Parameters): Promise<Uint8Number> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getThresholdAbi,
    args: [],
    functionName: getThresholdAbi[0].name,
  });

  if (!isUint8(res)) {
    throw new Error(`Invalid threshold.`);
  }

  return Number(res) as Uint8Number;
}

////////////////////////////////////////////////////////////////////////////////

async function _getKmsSigners(context: Context, parameters: Parameters): Promise<ChecksummedAddress[]> {
  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getKmsSignersAbi,
    args: [],
    functionName: getKmsSignersAbi[0].name,
  });

  try {
    assertIsChecksummedAddressArray(res, {});
  } catch (e) {
    throw new Error(`Invalid kms signers addresses.`, {
      cause: e,
    });
  }

  return res;
}
