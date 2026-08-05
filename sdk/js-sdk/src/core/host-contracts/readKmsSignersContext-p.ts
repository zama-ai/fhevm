import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { createKmsSignersContext } from './KmsSignersContext-p.js';
import { getCurrentKmsContextId } from './getCurrentKmsContextId-p.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';
import { isVersionStrictlyBefore } from './HostContractVersion-p.js';
import {
  assertIsKmsExtraData,
  createKmsExtraDataV1,
  createKmsExtraDataV2,
  isKmsExtraDataCompatibleWithKmsVerifier,
} from '../kms/kmsExtraData-p.js';
import { getKmsContextSignersAndThresholdFromExtraData } from './getKmsContextSignersAndThresholdFromExtraData-p.js';
import { getKmsSignersAndThreshold } from './getKmsContextSignersAndThreshold-p.js';
import { SDK_PROTOCOL_API_MAJOR_VERSION, SDK_PROTOCOL_API_MINOR_VERSION } from '../runtime/sdkProtocolApiVersion.js';

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
// CRITICAL RULE — a v13-capped SDK must NEVER produce extraData v2.
// The extraData version the SDK emits is gated by the RELAYER, not by the host
// contracts: a v13 relayer rejects an unknown v2 `extraData` in its request
// validation (HTTP 400 `validation_failed`), so the request never reaches the KMS.
// This matters most during a v13 -> v14 rollout: the host contracts can already
// support v2 while the relayer is still v13, so "the chain accepts v2" is NOT
// sufficient — the SDK stays at v1 until the relayer is known to accept v2.
// Hence the per-version cap enforced below.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves the **current** on-chain KMS context into a {@link KmsSignersContext},
 * indexed on the **best `extraData` this SDK can produce** for that context.
 *
 * This is the read used when *creating* a permit: the returned context's `extraData`
 * is what the permit embeds and the user signs. So "best" is bounded by **two**
 * limits, not one:
 *   - the on-chain KMSVerifier version (which encodings the contract supports), and
 *   - this SDK's protocol-API cap ({@link SDK_PROTOCOL_API_MINOR_VERSION}) — the most
 *     recent `extraData` version the SDK knows how to build and sign.
 *
 * The returned encoding is therefore `min(chain capability, SDK capability)`:
 *   - v11 (KMSVerifier < 0.2.0)                         → v0 (no context concept)
 *   - >= 0.2.0 but a v13-capped SDK, or KMSVerifier < 0.4.0,
 *     or no `protocolConfigAddress`                     → v1 (`contextId` only)
 *   - v14 chain (KMSVerifier >= 0.4.0, `protocolConfigAddress`
 *     set) **and** a v14-capable SDK                    → v2 (`contextId` + `epochId`)
 *
 * NOTE — the meaning is subtly different from a plain "read the current context at
 * full chain precision": a v13-capped SDK deliberately returns a **v1** context on a
 * **v14** chain (dropping the epoch), because it must produce a v1 permit. It also
 * differs from {@link readKmsSignersContextFromPermitExtraData}, which resolves an
 * *already-chosen* permit `extraData` to the most precise context available — here we
 * instead choose the most precise encoding we are *allowed to produce*.
 */
export async function readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  // This version comes from the frozen context — a snapshot, NOT necessarily the
  // live on-chain version, which may have been bumped by an upgrade since the
  // context was resolved. Because on-chain versions only ever increase, treat it
  // as a lower bound: the deployed KMSVerifier is *at least* this version. That
  // minimum is enough to select the matching protocol-API read path (view
  // function) below.
  const kmsVerifierVersion = parameters.fhevmContext.hostContractVersion('KMSVerifier');

  // KMSVerifier.version < 0.2.0, use only Protocol API v11
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    // -> KmsSignersContext.extraData.version == 0
    return _readCurrentKmsSignersContext_ProtocolApi_11(context, parameters);
  }

  // If SDK is restricted to protocol API v13 then the best extraData
  // we can get it must be v1
  // KMSVerifier.version >= 0.2.0, it supports at least API Protocol 13
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    // -> KmsSignersContext.extraData.version == 1
    return _readCurrentKmsSignersContext_ProtocolApi_12_13(context, parameters);
  }

  // API Protocol 14+

  // KMSVerifier.version < 0.4.0, use only Protocol API v13
  if (
    isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 }) ||
    parameters.protocolConfigAddress === undefined
  ) {
    // -> KmsSignersContext.extraData.version == 1
    return _readCurrentKmsSignersContext_ProtocolApi_12_13(context, parameters);
  }

  // -> KmsSignersContext.extraData.version == 2
  return _readCurrentKmsSignersContext_ProtocolApi_14_or_higher(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the {@link KmsSignersContext} for a caller-provided `extraData` — the exact
 * `extraData` a user signed into a permit — resolving it against the on-chain
 * KMSVerifier **as given**. It never re-encodes, upgrades, or substitutes the
 * `extraData`: the read is deliberately faithful to the signed permit, so the signer
 * set it returns is the one that permit committed to.
 *
 * 1. Reject an `extraData` whose version the on-chain KMSVerifier cannot accept
 *    ({@link isKmsExtraDataCompatibleWithKmsVerifier}), throwing.
 * 2. KMSVerifier `< 0.2.0` has no context concept: return the single global signer
 *    set (`contextId`/`epochId` = 0) via the v11 read path.
 * 3. KMSVerifier `>= 0.2.0`: resolve through the shared, version-agnostic reader
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

  // This version comes from the frozen context — a snapshot, NOT necessarily the
  // live on-chain version, which may have been bumped by an upgrade since the
  // context was resolved. Because on-chain versions only ever increase, treat it
  // as a lower bound: the deployed KMSVerifier is *at least* this version. That
  // minimum is enough to select the matching protocol-API read path (view
  // function) below.
  const kmsVerifierVersion = parameters.fhevmContext.hostContractVersion('KMSVerifier');

  if (!isKmsExtraDataCompatibleWithKmsVerifier(parameters.extraData, kmsVerifierVersion)) {
    throw new Error(
      `KmsExtraData ${parameters.extraData.bytesHex} is not compatible with ${kmsVerifierVersion.contractName} ${kmsVerifierVersion.version}`,
    );
  }

  // KMSVerifier.version < 0.2.0, use only Protocol API v11
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    return _readKmsSignersContext_ProtocolApi_11(context, parameters);
  }

  // KMSVerifier.version >= 0.2.0, use only Protocol API v11
  // use the general purpose `getKmsContextSignersAndThresholdFromExtraData`
  return _readKmsSignersContextFromExtraData_ProtocolApi_12_13_14(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_ProtocolApi_11(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  return _readKmsSignersContext_ProtocolApi_11(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_ProtocolApi_12_13(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  // TTL-Cached (available in KMSVerifier.sol >= v0.2.0)
  const kmsContextId = await getCurrentKmsContextId(context, parameters);
  const extraDataV1 = createKmsExtraDataV1({
    kmsContextId,
  });

  return _readKmsSignersContextFromExtraData_ProtocolApi_12_13_14(context, { ...parameters, extraData: extraDataV1 });
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_ProtocolApi_14_or_higher(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  // Defense-in-depth for the CRITICAL RULE above: this is the only function that
  // MINTS an extraData v2. It must never run under a v13-capped SDK. The caller
  // (readCurrentKmsSignersContext) already gates on SDK_PROTOCOL_API_MINOR_VERSION,
  // so reaching here with a capped SDK means that gate was bypassed/refactored away
  // — fail loudly rather than silently emit a v2 the relayer would reject.
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    throw new Error(
      `Refusing to produce extraData v2: this SDK is capped at protocol API v0.${SDK_PROTOCOL_API_MINOR_VERSION} and must only emit extraData v1 (a v13 relayer rejects v2).`,
    );
  }

  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }
  const protocolConfigAddress = parameters.protocolConfigAddress;

  // TTL-Cached (available in KMSVerifier.sol >= v0.4.0)
  const { contextId, epochId } = await getCurrentKmsContextAndEpoch(context, {
    ...parameters,
    protocolConfigAddress,
  });

  const extraDataV2 = createKmsExtraDataV2({
    kmsContextId: contextId,
    kmsEpochId: epochId,
  });

  return _readKmsSignersContextFromExtraData_ProtocolApi_12_13_14(context, { ...parameters, extraData: extraDataV2 });
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContext_ProtocolApi_11(context: Context, parameters: Parameters): Promise<ReturnType> {
  // TTL-Cached (available in KMSVerifier.sol >= v0.1.0)
  const c = await getKmsSignersAndThreshold(context, parameters);

  const data = createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: 0n as Uint256BigInt,
    kmsEpochId: 0n as Uint256BigInt,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
    kmsMpcThreshold: undefined,
  });

  return data;
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContextFromExtraData_ProtocolApi_12_13_14(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  // TTL-Cached (available in KMSVerifier.sol >= v0.2.0)
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
