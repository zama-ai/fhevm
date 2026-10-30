import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { createKmsSignersContext } from './KmsSignersContext-p.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';
import { assertIsKmsExtraData, createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';
import { getKmsContextSignersAndThresholdFromExtraData } from './getKmsContextSignersAndThresholdFromExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly kmsVerifierAddress: ChecksummedAddress;
  readonly protocolConfigAddress: ChecksummedAddress | undefined;
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly forceRefresh?: boolean | undefined;
};

type ParametersWithExtraData = Parameters & {
  readonly extraData: KmsExtraData;
};

type ReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////
//
// Invariant — a KmsSignersContext carries the key it was indexed by.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the **current** on-chain KMS context into a {@link KmsSignersContext},
 * indexed on the **best `extraData` this SDK can produce** for that context.
 *
 * This is the read used when *creating* a permit: the returned context's `extraData`
 * is what the permit embeds and the user signs.
 *
 * The returned encoding is always v2 (`contextId` + `epochId`), read from the
 * current on-chain context and epoch. `protocolConfigAddress` is required: throws
 * if it is missing.
 *
 * NOTE — this differs from {@link readKmsSignersContextFromPermitExtraData}, which
 * resolves an *already-chosen* permit `extraData` to the most precise context
 * available — here we instead choose the most precise encoding we are *allowed to
 * produce*.
 */
export async function readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  return _readCurrentKmsSignersContext(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the {@link KmsSignersContext} for a caller-provided `extraData` — the exact
 * `extraData` a user signed into a permit — resolving it against the on-chain
 * KMSVerifier **as given**. It never re-encodes, upgrades, or substitutes the
 * `extraData`: the read is deliberately faithful to the signed permit, so the signer
 * set it returns is the one that permit committed to.
 *
 * 1. Reject a malformed `extraData` ({@link assertIsKmsExtraData}), throwing.
 * 2. Resolve through the shared reader
 *    ({@link getKmsContextSignersAndThresholdFromExtraData}), which keys the signer
 *    set on the `extraData`'s own `contextId`. A v0 sentinel is accepted here and
 *    passed through verbatim — the on-chain view understands it as "current context" —
 *    so the result is indexed on the `extraData` as provided (`contextId` 0 for v0),
 *    not on a re-derived concrete context.
 *
 * The result is self-describing (see the invariant above): its `kmsContextId` (plus
 * `kmsEpochId` on v2) identify the `extraData` it was indexed by — the value the caller
 * passed in, never a "more precise" one chosen by this function.
 */
export async function readKmsSignersContextFromPermitExtraData(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  assertIsKmsExtraData(parameters.extraData, {});

  return _readKmsSignersContextFromExtraData(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }
  const protocolConfigAddress = parameters.protocolConfigAddress;

  // TTL-Cached
  const { contextId, epochId } = await getCurrentKmsContextAndEpoch(context, {
    ...parameters,
    protocolConfigAddress,
  });

  const extraDataV2 = createKmsExtraDataV2({
    kmsContextId: contextId,
    kmsEpochId: epochId,
  });

  return _readKmsSignersContextFromExtraData(context, { ...parameters, extraData: extraDataV2 });
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContextFromExtraData(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  // TTL-Cached
  const { signers: kmsSigners, threshold: kmsSignerThreshold } = await getKmsContextSignersAndThresholdFromExtraData(
    context,
    parameters,
  );

  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: parameters.extraData.kmsContextId,
    kmsEpochId: parameters.extraData.kmsEpochId,
    kmsSigners,
    kmsSignerThreshold,
    kmsMpcThreshold: undefined,
  });
}

////////////////////////////////////////////////////////////////////////////////
