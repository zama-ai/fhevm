import type { BytesHex, ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { getVersion, isVersionStrictlyBefore } from './HostContractVersion-p.js';
import { createCachedFetch } from '../base/cachedFetch.js';
import { assertIsKmsExtraData, fromKmsExtraData } from '../kms/kmsExtraData.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { isUint8 } from '../base/uint.js';
import { getContextSignersAndThresholdFromExtraDataAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly extraData: BytesHex;
};

type ReturnType = {
  readonly threshold: Uint8Number;
  readonly signers: readonly ChecksummedAddress[];
};

////////////////////////////////////////////////////////////////////////////////

const cachedGetKmsContextSignersAndThresholdFromExtraData = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getKmsContextSignersAndThresholdFromExtraData,
  cacheKeyFn: (context, params) => {
    const { kmsContextId } = fromKmsExtraData(params.extraData);
    const contextIdHex = kmsContextId.toString(16).padStart(64, '0');
    return `${context.runtime.uid.toLowerCase()}:${params.address.toLowerCase()}:${contextIdHex}`;
  },
  // Permanent cache: signers and threshold are immutable per context ID.
});

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the KMS signers and threshold for a given extraData context.
 *
 * Only available on KMSVerifier contracts >= v0.2.0. Throws on older versions.
 *
 * **Caching:** Results are cached permanently per (runtime, address, contextId)
 * with no TTL. On-chain, new KMS context IDs can be added over time, but once
 * a context is created its signers and threshold are immutable — they can never
 * be modified or removed. This makes indefinite caching safe: a cached entry
 * will always match the on-chain state. Concurrent callers to the same context
 * share a single in-flight RPC request (deduplication).
 *
 * @param parameters.address - The checksummed address of the KMSVerifier contract.
 * @param parameters.extraData - The encoded extraData (v1 format: version byte + 32-byte context ID).
 */
export function getKmsContextSignersAndThresholdFromExtraData(
  context: Context,
  parameters: Parameters & { readonly forceRefresh?: boolean | undefined },
): Promise<ReturnType> {
  assertIsKmsExtraData(parameters.extraData, {});

  return cachedGetKmsContextSignersAndThresholdFromExtraData.execute(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _getKmsContextSignersAndThresholdFromExtraData(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const version = await getVersion(context, { address: parameters.address });
  if (isVersionStrictlyBefore(version, { major: 0, minor: 2 })) {
    throw new Error('getContextSignersAndThresholdFromExtraData requires KMSVerifier >= v0.2.0');
  }

  const trustedClient = getTrustedClient(context);
  const address = parameters.address;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: address,
    abi: getContextSignersAndThresholdFromExtraDataAbi,
    args: [parameters.extraData],
    functionName: getContextSignersAndThresholdFromExtraDataAbi[0].name,
  });

  if (!Array.isArray(res) || res.length < 2) {
    throw new Error(`Invalid getContextSignersAndThresholdFromExtraData result.`);
  }

  const unknownSigners = res[0] as unknown;
  const unknownThreshold = res[1] as unknown;

  try {
    assertIsChecksummedAddressArray(unknownSigners, {});
  } catch (e) {
    throw new Error(`Invalid kms signers addresses.`, {
      cause: e,
    });
  }

  if (!isUint8(unknownThreshold)) {
    throw new Error('Invalid threshold.');
  }

  return Object.freeze({
    threshold: Number(unknownThreshold) as Uint8Number,
    signers: unknownSigners,
  });
}
