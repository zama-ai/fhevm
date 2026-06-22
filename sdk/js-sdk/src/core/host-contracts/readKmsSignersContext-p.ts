import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { BytesHex, ChecksummedAddress, Uint256BigInt } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import {
  assertIsKmsSignersContext,
  createKmsSignersContext,
  kmsSignersContextToExtraData,
} from './KmsSignersContext-p.js';
import { getCurrentKmsContextId } from './getCurrentKmsContextId-p.js';
import { getCurrentKmsContextAndEpoch } from './getCurrentKmsContextAndEpoch-p.js';
import { getHostContractVersion, isVersionStrictlyBefore } from './HostContractVersion-p.js';
import {
  assertIsKmsExtraData,
  EXTRA_DATA_V1,
  EXTRA_DATA_V2,
  fromKmsExtraData,
  toKmsExtraData,
} from '../kms/kmsExtraData.js';
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
  readonly kmsContextId?: Uint256BigInt | undefined;
  readonly kmsEpochId?: Uint256BigInt | undefined;
};

type ReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readKmsSignersContext(context: Context, parameters: Parameters): Promise<ReturnType> {
  // TTL-cached
  const kmsVerifierVersion = await getHostContractVersion(context, {
    address: parameters.kmsVerifierAddress,
  });

  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    return _readKmsSignersContext_Protocol_11(context, parameters);
  }

  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 })) {
    return _readKmsSignersContext_Protocol_12_13(context, parameters);
  }

  return _readKmsSignersContext_Protocol_14(context, parameters);
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.11.0 (KMSVerifier < v0.2.0) had no context ID support, so the only valid
// context ID is 0. Any other value is invalid and should throw.
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContext_Protocol_11(context: Context, parameters: Parameters): Promise<ReturnType> {
  if (parameters.kmsContextId !== undefined && parameters.kmsContextId !== 0n) {
    throw new Error('Impossible on protocol v0.11.0');
  }

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
async function _readKmsSignersContext_Protocol_12_13(context: Context, parameters: Parameters): Promise<ReturnType> {
  if (parameters.kmsEpochId !== undefined && parameters.kmsEpochId !== 0n) {
    throw new Error('kmsEpochId should be 0 on protocol v0.12.0 / v0.13.0');
  }

  let kmsContextId: Uint256BigInt;
  if (parameters.kmsContextId === undefined) {
    // TTL-Cached
    kmsContextId = await getCurrentKmsContextId(context, parameters);
  } else {
    if (parameters.kmsContextId === 0n) {
      throw new Error('kmsContextId cannot be 0 on protocol v0.12.0 / v0.13.0');
    }
    kmsContextId = parameters.kmsContextId;
  }

  const extraData = toKmsExtraData({
    version: EXTRA_DATA_V1,
    kmsContextId,
    kmsEpochId: 0n as Uint256BigInt,
  });

  // Permanent-Cached
  const c = await getKmsContextSignersAndThresholdFromExtraData(context, {
    ...parameters,
    extraData,
  });

  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId,
    kmsEpochId: 0n as Uint256BigInt,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
  });
}

////////////////////////////////////////////////////////////////////////////////

// Protocol v0.14.0+ (KMSVerifier = v0.4.0+)
// eslint-disable-next-line @typescript-eslint/naming-convention
async function _readKmsSignersContext_Protocol_14(context: Context, parameters: Parameters): Promise<ReturnType> {
  if (parameters.protocolConfigAddress === undefined) {
    throw new Error('protocolConfigAddress is required on protocol v0.14.0+');
  }
  const protocolConfigAddress = parameters.protocolConfigAddress;

  let kmsContextId: Uint256BigInt;
  if (parameters.kmsContextId === undefined) {
    // TTL-Cached
    ({ contextId: kmsContextId } = await getCurrentKmsContextAndEpoch(context, { protocolConfigAddress }));
  } else {
    if (parameters.kmsContextId === 0n) {
      throw new Error('kmsContextId cannot be 0 on protocol v0.14.0+');
    }
    kmsContextId = parameters.kmsContextId;
  }

  let kmsEpochId: Uint256BigInt;
  if (parameters.kmsEpochId === undefined) {
    // TTL-Cached
    ({ epochId: kmsEpochId } = await getCurrentKmsContextAndEpoch(context, { protocolConfigAddress }));
  } else {
    if (parameters.kmsEpochId === 0n) {
      throw new Error('kmsEpochId cannot be 0 on protocol v0.14.0+');
    }
    kmsEpochId = parameters.kmsEpochId;
  }

  const extraData = toKmsExtraData({
    version: EXTRA_DATA_V2,
    kmsContextId,
    kmsEpochId,
  });

  // Permanent-Cached
  const c = await getKmsContextSignersAndThresholdFromExtraData(context, {
    ...parameters,
    extraData,
  });

  return createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId,
    kmsEpochId,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
  });
}

////////////////////////////////////////////////////////////////////////////////

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
 * - `requestedExtraData` — `kmsSignersContextToExtraData(requestedKmsSignersContext)`.
 * - `relayerKmsContextId` — `fromKmsExtraData(relayerExtraData).kmsContextId`.
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
 * ### Resolution (checked in order)
 *
 * 1. `relayerExtraData === requestedExtraData` → exact byte match, return
 *    `requestedKmsSignersContext`. This is the **only** path that trusts cached data.
 * 2. In `'exact'` mode, any byte mismatch → **always throw**. This mode does
 *    not parse, refresh, or reconcile different `extraData`.
 * 3. Serialization version mismatch → **always throw**. The SDK and relayer must
 *    agree on the `extraData` encoding format, even if the context ID is the same.
 * 4. Context IDs differ → **full on-chain refetch**. The cached context is never
 *    reused, because any mismatch means on-chain state may have diverged
 *    (rotation, destruction, signer changes, etc.).
 *    - Context is **valid** on-chain (current or non-destroyed) → return it.
 *    - Context is **destroyed or out of range** → throw.
 *
 * ### Modes
 *
 * - `'exact'` — Accept only step 1 (byte-for-byte `extraData` equality).
 *   Rejects any relayer/context drift without parsing or on-chain recovery.
 * - `'strict'` — Accept step 1 (exact match) or both `relayerKmsContextId === currentKmsContextId`
 *   and `relayerKmsEpochId === currentKmsEpochId`. Rejects valid-but-not-current contexts.
 *   The epoch check is required because RFC 005 introduces same-context epoch rotations
 *   (new shares, same party set) — a stale epoch must be rejected even if the context matches.
 * - `'loose'` — Accept any on-chain valid context (non-destroyed, in range),
 *   regardless of whether it is current. Covers context rotations in either direction.
 */

////////////////////////////////////////////////////////////////////////////////

export type ReconcileMode = 'exact' | 'strict' | 'loose';

export async function reconcileKmsSignersContext(
  context: Context,
  parameters: {
    readonly kmsVerifierAddress: ChecksummedAddress;
    readonly protocolConfigAddress: ChecksummedAddress | undefined;
    readonly requestedKmsSignersContext: KmsSignersContext;
    readonly relayerExtraData: BytesHex;
    readonly mode: ReconcileMode;
  },
): Promise<KmsSignersContext> {
  const { kmsVerifierAddress, protocolConfigAddress, requestedKmsSignersContext, relayerExtraData, mode } = parameters;

  assertIsKmsExtraData(relayerExtraData, {});
  assertIsKmsSignersContext(requestedKmsSignersContext, {});

  const requestedExtraData = kmsSignersContextToExtraData(requestedKmsSignersContext);

  // 1. Exact match — the relayer used the same context as the SDK.
  if (relayerExtraData === requestedExtraData) {
    return requestedKmsSignersContext;
  }

  if (mode === 'exact') {
    throw new Error(
      `Exact reconciliation failed: relayer extraData ${relayerExtraData} ` +
        `does not match requested extraData ${requestedExtraData}.`,
    );
  }

  // Bytes differ — extract and compare serialization versions.
  const relayerKmsExtraData = fromKmsExtraData(relayerExtraData);
  const requestedKmsExtraData = fromKmsExtraData(requestedExtraData);

  // Reject if extraData serialization version differs.
  if (relayerKmsExtraData.version !== requestedKmsExtraData.version) {
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
    const currentContext = await readKmsSignersContext(context, {
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
  return readKmsSignersContext(context, {
    kmsVerifierAddress,
    protocolConfigAddress,
    kmsContextId: relayerKmsContextId,
    kmsEpochId: relayerKmsEpochId,
    forceRefresh: true,
  });
}
