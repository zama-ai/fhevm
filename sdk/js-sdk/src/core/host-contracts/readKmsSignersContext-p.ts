import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { BytesHex, ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { assertIsKmsSignersContext, createKmsSignersContext } from './KmsSignersContext-p.js';
import { getCurrentKmsContextId } from './getCurrentKmsContextId-p.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';
import { getHostContractVersion, isVersionStrictlyBefore } from './HostContractVersion-p.js';
import {
  assertIsKmsExtraData,
  createKmsExtraDataV1,
  createKmsExtraDataV2,
  EXTRA_DATA_V0,
  fromKmsExtraDataBytesHex,
  isKmsExtraDataCompatibleWithKmsVerifier,
} from '../kms/kmsExtraData-p.js';
import { getKmsContextSignersAndThresholdFromExtraData } from './getKmsContextSignersAndThresholdFromExtraData-p.js';
import { getKmsSignersAndThreshold } from './getKmsContextSignersAndThreshold-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly kmsVerifierAddress: ChecksummedAddress;
  readonly protocolConfigAddress: ChecksummedAddress | undefined;
  readonly forceRefresh?: boolean | undefined;
};

type ParametersWithExtraData = Parameters & {
  readonly extraData: KmsExtraData;
};

type ReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  return _readCurrentKmsSignersContext(context, parameters);
}

export async function readCurrentKmsSignersContextV1(context: Context, parameters: Parameters): Promise<ReturnType> {
  return _readCurrentKmsSignersContextV1(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the {@link KmsSignersContext} for a caller-provided `extraData`.
 *
 * A v0 (`0x`) `extraData` is the reserved sentinel for "the current KMS context"
 * and resolves to the current on-chain context, exactly like
 * {@link readCurrentKmsSignersContext}.
 *
 * For any concrete version (v1, v2, …), the underlying resolver selects the right
 * protocol path and verifies the version is compatible with the on-chain
 * KMSVerifier, throwing if it is not.
 */
export async function readKmsSignersContextFromExtraData(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  assertIsKmsExtraData(parameters.extraData, {});

  if (parameters.extraData.version === EXTRA_DATA_V0) {
    return _readCurrentKmsSignersContext(context, {
      kmsVerifierAddress: parameters.kmsVerifierAddress,
      protocolConfigAddress: parameters.protocolConfigAddress,
      forceRefresh: parameters.forceRefresh,
    });
  }

  return _readKmsSignersContextFromExtraData(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  // TTL-cached
  const kmsVerifierVersion = await getHostContractVersion(context, {
    address: parameters.kmsVerifierAddress,
  });

  // KMSVerifier.version < 0.2.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    return _readCurrentKmsSignersContext_Protocol_11(context, parameters);
  }

  // KMSVerifier.version < 0.4.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 })) {
    return _readCurrentKmsSignersContext_Protocol_12_13(context, parameters);
  }

  // KMSVerifier.version >= 0.4.0
  return _readCurrentKmsSignersContext_Protocol_14_or_higher(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _readKmsSignersContextFromExtraData(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  // TTL-cached
  const kmsVerifierVersion = await getHostContractVersion(context, {
    address: parameters.kmsVerifierAddress,
  });

  if (!isKmsExtraDataCompatibleWithKmsVerifier(parameters.extraData, kmsVerifierVersion)) {
    throw new Error(
      `KmsExtraData ${parameters.extraData.toBytesHex()} is not compatible with ${kmsVerifierVersion.contractName} ${kmsVerifierVersion.version}`,
    );
  }

  // KMSVerifier.version < 0.2.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    // extraData.version = 0 only
    return _readKmsSignersContextFromExtraData_Protocol_11(context, parameters);
  }

  // KMSVerifier.version < 0.4.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 })) {
    // extraData.version = 0 or 1 only
    return _readKmsSignersContextFromExtraData_Protocol_12_13(context, parameters);
  }

  // KMSVerifier.version >= 0.4.0
  // any extraData.version
  return _readKmsSignersContextFromExtraData_Protocol_14_or_higher(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _readCurrentKmsSignersContextV1(context: Context, parameters: Parameters): Promise<ReturnType> {
  // TTL-cached
  const kmsVerifierVersion = await getHostContractVersion(context, {
    address: parameters.kmsVerifierAddress,
  });

  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    return _readCurrentKmsSignersContext_Protocol_11(context, parameters);
  }

  return _readCurrentKmsSignersContext_Protocol_12_13(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.11.0 (KMSVerifier < v0.2.0) had no context ID support, so the only valid
// context ID is 0. Any other value is invalid and should throw.
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_Protocol_11(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  // TTL-Cached
  const c = await getKmsSignersAndThreshold(context, parameters);

  const data = createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: 0n as Uint256BigInt,
    kmsEpochId: 0n as Uint256BigInt,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
  });

  return data;
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.11.0 has no context concept; `extraData` is v0 (enforced by the
// compatibility check in `_readKmsSignersContextFromExtraData`), so this resolves
// identically to the current context.
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContextFromExtraData_Protocol_11(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  return _readCurrentKmsSignersContext_Protocol_11(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.12.0 / v0.13.0 (KMSVerifier = v0.2.0 / v0.3.0)
// On protocol v0.12/v0.13, the KMS signers and threshold are looked up by
// `extraData` (which encodes the KMS context id). Here we derive it from the
// chain's current KMS context id using the v1 extraData encoding (version byte +
// context id, no epoch).
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_Protocol_12_13(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  // TTL-Cached
  const kmsContextId = await getCurrentKmsContextId(context, parameters);
  const extraDataV1 = createKmsExtraDataV1({
    kmsContextId,
  });

  return _readKmsSignersContextFromExtraData_Protocol_12_13(context, { ...parameters, extraData: extraDataV1 });
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.12.0 / v0.13.0 (KMSVerifier = v0.2.0 / v0.3.0)
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContextFromExtraData_Protocol_12_13(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  // v0 or v1
  const extraData = parameters.extraData;

  // Permanent-Cached
  // On protocol v0.12/v0.13, getKmsContextSignersAndThresholdFromExtraData
  // supports extraData v0 and v1 only.
  const { signers: kmsSigners, threshold: kmsSignerThreshold } = await getKmsContextSignersAndThresholdFromExtraData(
    context,
    {
      ...parameters,
      extraData,
    },
  );

  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: extraData.kmsContextId,
    kmsEpochId: 0n as Uint256BigInt,
    kmsSigners,
    kmsSignerThreshold,
  });
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.14.0+ (KMSVerifier = v0.4.0+)
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readCurrentKmsSignersContext_Protocol_14_or_higher(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }
  const protocolConfigAddress = parameters.protocolConfigAddress;

  // TTL-Cached
  const { contextId, epochId } = await getCurrentKmsContextAndEpoch(context, { protocolConfigAddress });
  const extraData = createKmsExtraDataV2({
    kmsContextId: contextId,
    kmsEpochId: epochId,
  });

  return _readKmsSignersContextFromExtraData_Protocol_14_or_higher(context, { ...parameters, extraData });
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.14.0+ (KMSVerifier = v0.4.0+)
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContextFromExtraData_Protocol_14_or_higher(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }

  // any version: v0, v1, v2, ... vn
  const extraData = parameters.extraData;

  // Permanent-Cached
  // May revert if extraData.version is incompatible (see `getKmsContextSignersAndThresholdFromExtraData` comments)
  const { signers: kmsSigners, threshold: kmsSignerThreshold } = await getKmsContextSignersAndThresholdFromExtraData(
    context,
    {
      ...parameters,
      extraData,
    },
  );

  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: extraData.kmsContextId,
    kmsEpochId: extraData.kmsEpochId,
    kmsSigners,
    kmsSignerThreshold,
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reconciles the KMS signers context the SDK anchored on the **signed permit**
 * with the context the relayer/KMS actually used, and returns the signer set the
 * signcrypted shares must be verified against.
 *
 * This replaced the earlier mode-based (`'exact' | 'strict' | 'loose'`)
 * reconciliation: it has
 * no `mode`, and it mirrors the on-chain gateway `Decryption.sol` rule
 * (`_extractContextId(request) == _extractContextId(response)`, then context-scoped
 * signer verification). The SDK re-checks it here because the relayer is untrusted
 * and returns the shares off-chain.
 *
 * ### Assumptions
 *
 * - `requestedKmsSignersContext` is already **anchored on the permit's `extraData`**
 *   and fully **concrete**: a v0 (`0x`) permit was already resolved to the current
 *   context upstream (see {@link readKmsSignersContextFromExtraData}), so its
 *   `id`/`epochId` are real values, never the `0` sentinel.
 * - The relayer **cannot be trusted**; on-chain data is the source of truth.
 *
 * ### Rule
 *
 * 1. If the relayer's returned `extraData` already equals the permit's concrete
 *    `extraData`, accept — this is the common path (the KMS echoes the permit's
 *    `extraData` verbatim).
 * 2. Otherwise, resolve the relayer's `extraData` to a concrete context, applying
 *    the same `v0 → current` normalization used for the permit side. This covers a
 *    lagging KMS that echoes `v0` (`0x00`) while the permit is concrete.
 * 3. Require the resolved relayer context to be the context the user committed to,
 *    compared on `kmsContextId` **only**. Any other context — even a still-valid,
 *    non-destroyed one — is a substitution and is **rejected** (mirrors the gateway
 *    `DecryptionContextMismatch`).
 *
 * `kmsEpochId` is deliberately **not** compared: the gateway's `_extractContextId`
 * ignores the epoch bytes even for v2, the KMS signer set is keyed by `contextId`
 * (not epoch), and RFC 005 epoch rotations reshare the *same* key to the *same*
 * party set — so an epoch difference is neither a signer-set change nor a
 * result-integrity risk. Comparing epoch here would also produce false mismatches
 * across a `v1`↔`v2` encoding difference or a KMSVerifier upgrade between the
 * anchor read and this call (v0/v1 carry `epochId = 0`; v2 carries a real epoch).
 *
 * On success the **permit-anchored** context is returned (its signer set is the
 * trusted one); a destroyed / out-of-range relayer context resolves to an empty
 * signer set and is rejected either by step 3 or by later share verification.
 *
 * Note (TOCTOU): for a v0 permit the "current" is read once upstream (anchor time)
 * and again here for a v0 relayer response; a rotation in between makes this
 * stricter (it may reject a response the chain accepted) — the safe direction.
 *
 * ### Host-Contract Compatibility
 *
 * `extraData` is versioned independently from the host-contract package. The
 * currently supported combinations are:
 *
 * - host-contracts v11 support `extraData` v0.
 * - host-contracts v12 and v13 support `extraData` v0 and v1.
 * - host-contracts v14 support `extraData` v0, v1, and v2.
 */
export async function reconcileKmsSignersContext(
  context: Context,
  parameters: {
    readonly kmsVerifierAddress: ChecksummedAddress;
    readonly protocolConfigAddress: ChecksummedAddress | undefined;
    readonly requestedKmsSignersContext: KmsSignersContext;
    readonly relayerKmsExtraDataBytesHex: BytesHex;
  },
): Promise<KmsSignersContext> {
  const { kmsVerifierAddress, protocolConfigAddress, requestedKmsSignersContext, relayerKmsExtraDataBytesHex } =
    parameters;

  assertIsKmsSignersContext(requestedKmsSignersContext, {});

  // if protocol is v11 then requestedKmsSignersContext.id === 0
  // as well as relayerKmsExtraData

  const relayerKmsExtraData = fromKmsExtraDataBytesHex(relayerKmsExtraDataBytesHex);

  // 1. Fast path — the relayer's extraData already names the permit's (concrete)
  //    context. Compared on `contextId` only, consistent with the step-3 check
  //    below (epoch and encoding version are irrelevant). A `v0` relayer response
  //    carries `contextId === 0` here, so it correctly misses this fast path and
  //    falls through to the `v0 → current` resolution in step 2.
  if (relayerKmsExtraData.kmsContextId === requestedKmsSignersContext.id) {
    return requestedKmsSignersContext;
  }

  // 2. Resolve the relayer's extraData to a concrete context (v0 -> current), the
  //    same normalization applied to the permit side. This is the only place a
  //    `v0` relayer response is legitimately reconciled against a concrete permit.
  const relayerKmsSignersContext = await readKmsSignersContextFromExtraData(context, {
    kmsVerifierAddress,
    protocolConfigAddress,
    forceRefresh: true,
    extraData: relayerKmsExtraData,
  });

  // 3. Anti-substitution — the context that produced the shares must be the context
  //    the user signed. Compared on `contextId` only (see doc): the gateway's
  //    `_extractContextId` ignores epoch, signers are keyed by contextId, and RFC 005
  //    epochs reshare the same key to the same party set. A different valid context
  //    is rejected (gateway `DecryptionContextMismatch` equivalent).
  if (relayerKmsSignersContext.id !== requestedKmsSignersContext.id) {
    throw new Error(
      `KMS context mismatch: the signed permit commits to context ${requestedKmsSignersContext.id}, ` +
        `but the relayer returned shares for context ${relayerKmsSignersContext.id}. ` +
        `The KMS response must come from the context the user authorized.`,
    );
  }

  // Contexts match: verify against the trusted permit-anchored signer set.
  return requestedKmsSignersContext;
}
