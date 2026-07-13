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
  createKmsExtraData,
  createKmsExtraDataV1,
  createKmsExtraDataV2,
  equalsKmsExtraData,
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

type ParametersWithOptionalExtraData = Parameters & {
  readonly extraData?: KmsExtraData | undefined;
};

type ReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readCurrentKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  return _readKmsSignersContext(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the {@link KmsSignersContext} for a caller-provided `extraData`.
 *
 * Accepts an `extraData` of any version (v0, v1, v2, …); the underlying resolver
 * selects the right protocol path and verifies the version is compatible with
 * the on-chain KMSVerifier, throwing if it is not.
 */
export async function readKmsSignersContextFromExtraData(
  context: Context,
  parameters: ParametersWithExtraData,
): Promise<ReturnType> {
  assertIsKmsExtraData(parameters.extraData, {});
  return _readKmsSignersContext(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

async function _readKmsSignersContext(
  context: Context,
  parameters: ParametersWithOptionalExtraData,
): Promise<ReturnType> {
  // TTL-cached
  const kmsVerifierVersion = await getHostContractVersion(context, {
    address: parameters.kmsVerifierAddress,
  });

  if (parameters.extraData !== undefined) {
    if (!isKmsExtraDataCompatibleWithKmsVerifier(parameters.extraData, kmsVerifierVersion)) {
      throw new Error(
        `KmsExtraData ${parameters.extraData.toBytesHex()} is not compatible with ${kmsVerifierVersion.contractName} ${kmsVerifierVersion.version}`,
      );
    }
  }

  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    // extraData.version = 0 only
    return _readKmsSignersContext_Protocol_11(context, parameters);
  }

  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 })) {
    // extraData.version = 0 or 1 only
    return _readKmsSignersContext_Protocol_12_13(context, parameters);
  }

  // any extraData.version
  return _readKmsSignersContext_Protocol_14_or_higher(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.11.0 (KMSVerifier < v0.2.0) had no context ID support, so the only valid
// context ID is 0. Any other value is invalid and should throw.
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContext_Protocol_11(context: Context, parameters: Parameters): Promise<ReturnType> {
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

// Protocol v0.12.0 / v0.13.0 (KMSVerifier = v0.2.0 / v0.3.0)
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContext_Protocol_12_13(
  context: Context,
  parameters: ParametersWithOptionalExtraData,
): Promise<ReturnType> {
  // On protocol v0.12/v0.13, the KMS signers and threshold are looked up by
  // `extraData` (which encodes the KMS context id). The caller may pass one in;
  // if not, we derive it from the chain's current KMS context id using the v1
  // extraData encoding (version byte + context id, no epoch).
  let extraData: KmsExtraData;

  if (parameters.extraData === undefined) {
    // TTL-Cached
    const kmsContextId = await getCurrentKmsContextId(context, parameters);
    extraData = createKmsExtraDataV1({
      kmsContextId,
    });
  } else {
    // v0 or v1
    extraData = parameters.extraData;
  }

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
async function _readKmsSignersContext_Protocol_14_or_higher(
  context: Context,
  parameters: ParametersWithOptionalExtraData,
): Promise<ReturnType> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }
  const protocolConfigAddress = parameters.protocolConfigAddress;

  let extraData: KmsExtraData;

  if (parameters.extraData === undefined) {
    // TTL-Cached
    const { contextId, epochId } = await getCurrentKmsContextAndEpoch(context, { protocolConfigAddress });
    extraData = createKmsExtraDataV2({
      kmsContextId: contextId,
      kmsEpochId: epochId,
    });
  } else {
    // any version: v0, v1, v2, ... vn
    extraData = parameters.extraData;
  }

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

////////////////////////////////////////////////////////////////////////////////

export type ReconcileMode = 'exact' | 'strict' | 'loose';

/**
 * Reconciles the KMS signers context used by the SDK with the one the relayer
 * actually used when forwarding the decryption request to KMS nodes.
 *
 * The returned {@link KmsSignersContext} provides the signers and threshold
 * needed to verify the KMS signcrypted shares and reconstruct the decrypted values.
 *
 * ### Assumptions
 *
 * - The relayer **cannot be trusted**.
 * - On-chain data is the **source of truth**.
 *
 * ### Nomenclature
 *
 * - `requestedKmsExtraData` — `createKmsExtraData({ kmsContextId, kmsEpochId })`
 *   derived from `requestedKmsSignersContext`.
 * - `relayerKmsExtraData` — `fromKmsExtraDataBytesHex(relayerKmsExtraDataBytesHex)`
 *   (its `.kmsContextId` / `.kmsEpochId`).
 *
 * ### Host-Contract Compatibility
 *
 * `extraData` is versioned independently from the host-contract package. The
 * currently supported combinations are:
 *
 * - host-contracts v11 support `extraData` v0.
 * - host-contracts v12 and v13 support `extraData` v0 and v1.
 * - host-contracts v14 support `extraData` v0, v1, and v2.
 *
 * ### Resolution (checked in order, then by `mode`)
 *
 * 1. `relayerKmsExtraData` equals `requestedKmsExtraData` → return
 *    `requestedKmsSignersContext`. This is the **only** path that trusts cached
 *    data, and it applies in every mode.
 * 2. Otherwise, in `'exact'` mode → **throw**. This mode never parses, refreshes,
 *    or reconciles differing `extraData`.
 * 3. `extraData` serialization version mismatch → **throw** (`'strict'`/`'loose'`).
 *    The SDK and relayer must agree on the encoding format, even for the same context ID.
 * 4. Versions match but context/epoch differ → force a **full on-chain refetch**
 *    (cached data is never reused, since a mismatch means on-chain state may have
 *    diverged: rotation, destruction, signer/epoch changes). The accept criterion
 *    then depends on `mode`:
 *    - `'strict'` → accept **only** if the relayer's context equals the *current*
 *      on-chain context (both `kmsContextId` and `kmsEpochId`); otherwise throw.
 *    - `'loose'` → accept **any** on-chain-valid context (non-destroyed, in range),
 *      current or not; throw only if destroyed / out of range.
 *
 * ### Modes
 *
 * - `'exact'` — Accept only step 1 (`extraData` equality). Rejects any
 *   relayer/context drift without parsing or on-chain recovery.
 * - `'strict'` — Accept step 1, or a relayer context whose `kmsContextId` **and**
 *   `kmsEpochId` equal the current on-chain values. Rejects valid-but-not-current
 *   contexts. The epoch check is required because RFC 005 introduces same-context
 *   epoch rotations (new shares, same party set) — a stale epoch must be rejected
 *   even when the context ID matches.
 * - `'loose'` — Accept any on-chain valid context (non-destroyed, in range),
 *   regardless of whether it is current. Covers context rotations in either direction.
 */
export async function reconcileKmsSignersContext(
  context: Context,
  parameters: {
    readonly kmsVerifierAddress: ChecksummedAddress;
    readonly protocolConfigAddress: ChecksummedAddress | undefined;
    readonly requestedKmsSignersContext: KmsSignersContext;
    readonly relayerKmsExtraDataBytesHex: BytesHex;
    readonly mode: ReconcileMode;
  },
): Promise<KmsSignersContext> {
  const { kmsVerifierAddress, protocolConfigAddress, requestedKmsSignersContext, relayerKmsExtraDataBytesHex, mode } =
    parameters;

  const relayerKmsExtraData = fromKmsExtraDataBytesHex(relayerKmsExtraDataBytesHex);

  assertIsKmsSignersContext(requestedKmsSignersContext, {});

  const requestedKmsExtraData = createKmsExtraData({
    kmsContextId: requestedKmsSignersContext.id,
    kmsEpochId: requestedKmsSignersContext.epochId,
  });

  // 1. Exact match — the relayer used the same context as the SDK.
  if (equalsKmsExtraData(relayerKmsExtraData, requestedKmsExtraData)) {
    return requestedKmsSignersContext;
  }

  if (mode === 'exact') {
    throw new Error(
      `Exact reconciliation failed: relayer extraData ${relayerKmsExtraData.toBytesHex()} ` +
        `does not match requested extraData ${requestedKmsExtraData.toBytesHex()}.`,
    );
  }

  // Reject if extraData serialization version differs.
  if (relayerKmsExtraData.version !== 0 && relayerKmsExtraData.version !== requestedKmsExtraData.version) {
    throw new Error(
      `ExtraData serialization version mismatch: SDK uses v${requestedKmsExtraData.version}, ` +
        `relayer returned v${relayerKmsExtraData.version}. ` +
        `The SDK and relayer must agree on the extraData encoding format.`,
    );
  }

  // Versions match but context IDs differ — verify the relayer's context on-chain.
  const relayerKmsContextId = relayerKmsExtraData.kmsContextId;
  const relayerKmsEpochId = relayerKmsExtraData.kmsEpochId;

  // 2. In strict mode, only accept if the relayer used the current on-chain context.
  if (mode === 'strict') {
    // `readKmsSignersContext` with `forceRefresh` fetches the current context.
    // If `relayerKmsContextId` matches current, we're good.
    const currentContext = await readCurrentKmsSignersContext(context, {
      kmsVerifierAddress,
      protocolConfigAddress,
      forceRefresh: true,
    });

    if (currentContext.id !== relayerKmsContextId || currentContext.epochId !== relayerKmsEpochId) {
      throw new Error(
        `Strict reconciliation failed: relayer used context ${relayerKmsContextId}, ` +
          `but the current on-chain context is ${currentContext.id}.`,
      );
    }

    return currentContext;
  }

  // 3. Loose mode — accept any valid (non-destroyed, in-range) context.
  //    `readKmsSignersContext` with `forceRefresh` + specific `kmsContextId`/`kmsEpochId` will
  //    throw if the context is destroyed or out of range.
  return readKmsSignersContextFromExtraData(context, {
    kmsVerifierAddress,
    protocolConfigAddress,
    forceRefresh: true,
    extraData: relayerKmsExtraData,
  });
}
