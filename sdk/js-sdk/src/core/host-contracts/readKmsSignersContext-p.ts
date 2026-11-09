import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import { createKmsSignersContext } from './KmsSignersContext-p.js';
import { InvalidSignedPermitError } from '../errors/InvalidSignedPermitError.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';
import { assertIsKmsExtraData, createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';
import { getKmsContextSignersAndThresholdFromExtraData } from './getKmsContextSignersAndThresholdFromExtraData-p.js';
import { VersionedValueCache, type CachedValue } from '../caching/versionedValueCache.js';
import { uint256ToBytes32Hex } from '../base/uint.js';
import { getCacheTtl } from '../caching/cacheOptions-p.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly chain?: { readonly id: number } | undefined;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly kmsVerifierAddress: ChecksummedAddress;
  readonly protocolConfigAddress: ChecksummedAddress | undefined;
};

type ParametersWithExtraData = Parameters & {
  readonly extraData: KmsExtraData;
};

type ReturnType = KmsSignersContext;

/** A KMS context as cached: its `extraData` (the ids), signers and threshold. */
type KmsContextValue = {
  readonly extraData: KmsExtraData;
  readonly signers: readonly ChecksummedAddress[];
  readonly threshold: Uint8Number;
};

////////////////////////////////////////////////////////////////////////////////
//
// KMS context cache: the current context (with its epoch), one per chain and
// contracts. A read reads the ids first and keeps the cached signers when
// the ids are unchanged.
//
////////////////////////////////////////////////////////////////////////////////

function _kmsContextSlot(context: Context, params: Parameters): string {
  return [context.runtime.uid, context.chain?.id ?? '', params.kmsVerifierAddress, params.protocolConfigAddress ?? '']
    .join(':')
    .toLowerCase();
}

class KmsContextCache extends VersionedValueCache<Context, Parameters, KmsContextValue> {
  protected override getSlotKey(context: Context, params: Parameters): string {
    return _kmsContextSlot(context, params);
  }

  protected override async fetch(
    context: Context,
    params: Parameters,
    cached: CachedValue<KmsContextValue> | undefined,
  ): Promise<{ readonly version: string; readonly value: KmsContextValue }> {
    const extraData = await _readCurrentKmsExtraData(context, params);
    const version = kmsContextVersion(extraData.kmsContextId, extraData.kmsEpochId);
    if (cached?.version === version) {
      return cached;
    }
    const { signers, threshold } = await getKmsContextSignersAndThresholdFromExtraData(context, {
      ...params,
      extraData,
    });
    return { version, value: Object.freeze({ extraData, signers, threshold }) };
  }
}

const kmsContextCache = new KmsContextCache();

/** The cache version of a KMS context: `contextId:epochId`, as 32-byte hex. */
export function kmsContextVersion(kmsContextId: bigint, kmsEpochId: bigint): string {
  return `${uint256ToBytes32Hex(kmsContextId)}:${uint256ToBytes32Hex(kmsEpochId)}`;
}

function _currentKmsContextParameters(context: Context & { readonly chain: FhevmChain }): Parameters {
  return {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
  };
}

/**
 * Reads the current KMS context again (see `VersionedValueCache.refresh`).
 *
 * @returns the cache version of the current context.
 */
export async function refreshCurrentKmsContext(
  context: Context & { readonly chain: FhevmChain },
  invalidVersion: string | undefined,
): Promise<string> {
  const { version } = await kmsContextCache.refresh(context, _currentKmsContextParameters(context), invalidVersion);
  return version;
}

/** Reads the version of the current KMS context, without caching it. */
export async function readCurrentKmsContextVersion(context: Context & { readonly chain: FhevmChain }): Promise<string> {
  const extraData = await _readCurrentKmsExtraData(context, _currentKmsContextParameters(context));
  return kmsContextVersion(extraData.kmsContextId, extraData.kmsEpochId);
}

////////////////////////////////////////////////////////////////////////////////
//
// Invariant — a KmsSignersContext carries the key it was indexed by.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the **current** on-chain KMS context (with its epoch) into a
 * {@link KmsSignersContext}, from the KMS context cache. This is the read used
 * when *creating* a permit: the returned context's v2 `extraData` is what the
 * permit embeds and the user signs.
 */
export async function readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { value } = await kmsContextCache.get(context, parameters, getCacheTtl(context, 'kmsContext'));
  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: value.extraData.kmsContextId,
    kmsEpochId: value.extraData.kmsEpochId,
    kmsSigners: value.signers,
    kmsSignerThreshold: value.threshold,
    kmsMpcThreshold: undefined,
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the {@link KmsSignersContext} for the `extraData` a user signed into a
 * permit. Only the current KMS context is accepted; the result is indexed on
 * the `extraData` as given (its `kmsContextId`, plus `kmsEpochId` on v2), with
 * the current context's signers.
 *
 * @throws {InvalidSignedPermitError} when the permit's `extraData` names a
 * context that is not the current one.
 */
export async function readKmsSignersContextFromPermitExtraData(
  context: Context,
  parameters: ParametersWithExtraData & { readonly usedVersions?: UsedCacheVersions | undefined },
): Promise<ReturnType> {
  assertIsKmsExtraData(parameters.extraData, {});

  // Only the current context: the KMS may refuse any other at any time. The
  // cache may be behind (or ahead of) the chain, e.g. after a reorg or on a
  // lagging RPC node, so a mismatch reads the chain again once.
  let current = await kmsContextCache.get(context, parameters, getCacheTtl(context, 'kmsContext'));
  if (!_namesKmsContext(parameters.extraData, current.value.extraData)) {
    current = await kmsContextCache.refresh(context, parameters, current.version);
    if (!_namesKmsContext(parameters.extraData, current.value.extraData)) {
      throw new InvalidSignedPermitError({
        message:
          `The permit was signed for KMS context ${_formatKmsContext(parameters.extraData)}, ` +
          `but the current KMS context is ${_formatKmsContext(current.value.extraData)}: sign a new permit`,
      });
    }
  }
  // For the check after a failure (see retryOnStaleCache).
  recordUsedCacheVersion(parameters.usedVersions, 'kmsContext', current.version);
  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: parameters.extraData.kmsContextId,
    kmsEpochId: parameters.extraData.kmsEpochId,
    kmsSigners: current.value.signers,
    kmsSignerThreshold: current.value.threshold,
    kmsMpcThreshold: undefined,
  });
}

/**
 * The error to throw when a permit request failed on a stale KMS context (the
 * current one is now cached): {@link InvalidSignedPermitError}, with the
 * request's error as `cause`, when the permit no longer names the current
 * context; else the request's own error (e.g. a v1 permit after an
 * epoch-only switch).
 */
export function permitRequestStaleContextError(
  context: Context & { readonly chain: FhevmChain },
  permitExtraData: KmsExtraData,
  error: unknown,
): unknown {
  const current = kmsContextCache.peek(_kmsContextSlot(context, _currentKmsContextParameters(context)));
  if (current === undefined || _namesKmsContext(permitExtraData, current.value.extraData)) {
    return error;
  }
  return new InvalidSignedPermitError({
    message:
      `The KMS context switched since the permit was signed for ${_formatKmsContext(permitExtraData)} ` +
      `(now ${_formatKmsContext(current.value.extraData)}): sign a new permit`,
    cause: error instanceof Error ? error : undefined,
  });
}

/**
 * Whether a permit's `extraData` names the current context: its context id,
 * and its epoch id for a v2 one (v1 has none).
 */
function _namesKmsContext(permit: KmsExtraData, current: KmsExtraData): boolean {
  return permit.kmsContextId === current.kmsContextId && (permit.lt(2) || permit.kmsEpochId === current.kmsEpochId);
}

function _formatKmsContext(extraData: KmsExtraData): string {
  return extraData.lt(2) ? `${extraData.kmsContextId}` : `${extraData.kmsContextId} (epoch ${extraData.kmsEpochId})`;
}

////////////////////////////////////////////////////////////////////////////////

/** Reads the ids of the current KMS context on ProtocolConfig, as v2 `extraData`. */
async function _readCurrentKmsExtraData(context: Context, parameters: Parameters): Promise<KmsExtraData> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new FhevmConfigError({ message: 'A ProtocolConfig address is required to read the current KMS context.' });
  }
  const { contextId, epochId } = await getCurrentKmsContextAndEpoch(context, {
    ...parameters,
    protocolConfigAddress: parameters.protocolConfigAddress,
  });
  return createKmsExtraDataV2({ kmsContextId: contextId, kmsEpochId: epochId });
}
