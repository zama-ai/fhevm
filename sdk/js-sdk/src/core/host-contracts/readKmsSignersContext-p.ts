import {
  assertIsKmsSignersContext,
  createKmsSignersContext,
  kmsSignersContextToExtraData,
} from './KmsSignersContext-p.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type {
  BytesHex,
  ChecksummedAddress,
  Uint256BigInt,
  Uint8Number,
} from '../types/primitives.js';
import { getCurrentKmsContextId } from './getCurrentKmsContextId-p.js';
import {
  getVersion,
  isVersionStrictlyBefore,
} from './HostContractVersion-p.js';
import {
  assertIsKmsExtraData,
  fromKmsExtraData,
  toKmsExtraData,
} from '../kms/kmsExtraData.js';
import { getKmsContextSignersAndThresholdFromExtraData } from './getKmsContextSignersAndThresholdFromExtraData-p.js';
import { getKmsSignersAndThreshold } from './getKmsContextSignersAndThreshold-p.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly address: ChecksummedAddress;
  readonly forceRefresh?: boolean | undefined;
  readonly kmsContextId?: Uint256BigInt | undefined;
};

type ReturnType = KmsSignersContext;

////////////////////////////////////////////////////////////////////////////////

export async function readKmsSignersContext(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  const kmsVerifierContractAddress = parameters.address;

  // TTL-cached
  const version = await getVersion(context, {
    address: kmsVerifierContractAddress,
  });

  if (isVersionStrictlyBefore(version, { major: 0, minor: 2 })) {
    return _readKmsSignersContextV1(context, parameters);
  } else {
    return _readKmsSignersContext(context, parameters);
  }
}

////////////////////////////////////////////////////////////////////////////////

async function _readKmsSignersContextV1(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  if (parameters.kmsContextId !== undefined && parameters.kmsContextId !== 0n) {
    throw new Error('Impossible on v1');
  }

  // TTL-Cached
  const c = await getKmsSignersAndThreshold(context, parameters);

  const data = createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId: 0n as Uint256BigInt,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
  });

  return data;
}

////////////////////////////////////////////////////////////////////////////////

async function _readKmsSignersContext(
  context: Context,
  parameters: Parameters,
): Promise<ReturnType> {
  let kmsContextId: Uint256BigInt;
  if (parameters.kmsContextId !== undefined) {
    if (parameters.kmsContextId === 0n) {
      throw new Error('Impossible on v2');
    }
    kmsContextId = parameters.kmsContextId;
  } else {
    // TTL-Cached
    kmsContextId = await getCurrentKmsContextId(context, parameters);
  }

  return _readKmsSignersContextForContextId(context, {
    ...parameters,
    kmsContextId,
  });
}

async function _readKmsSignersContextForContextId(
  context: Context,
  parameters: Parameters & { readonly kmsContextId: Uint256BigInt },
): Promise<ReturnType> {
  const { kmsContextId } = parameters;
  const extraData = toKmsExtraData({
    version: 1 as Uint8Number,
    kmsContextId,
  });

  // Permanent-Cached
  const c = await getKmsContextSignersAndThresholdFromExtraData(context, {
    ...parameters,
    extraData,
  });

  const data = createKmsSignersContext(new WeakRef(context.runtime), {
    ...parameters,
    kmsContextId,
    kmsSigners: c.signers,
    kmsSignerThreshold: c.threshold,
  });

  return data;
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
 * ### Resolution (checked in order)
 *
 * 1. `relayerExtraData === requestedExtraData` → exact byte match, return
 *    `requestedKmsSignersContext`. This is the **only** path that trusts cached data.
 * 2. Serialization version mismatch → **always throw**. The SDK and relayer must
 *    agree on the `extraData` encoding format, even if the context ID is the same.
 * 3. Context IDs differ → **full on-chain refetch**. The cached context is never
 *    reused, because any mismatch means on-chain state may have diverged
 *    (rotation, destruction, signer changes, etc.).
 *    - Context is **valid** on-chain (current or non-destroyed) → return it.
 *    - Context is **destroyed or out of range** → throw.
 *
 * ### Modes
 *
 * - `'strict'` — Accept step 1 (exact match) or `relayerKmsContextId === currentKmsContextId`.
 *   Rejects valid-but-not-current contexts.
 * - `'loose'` — Accept any on-chain valid context (non-destroyed, in range),
 *   regardless of whether it is current. Covers context rotations in either direction.
 */

////////////////////////////////////////////////////////////////////////////////

export type ReconcileMode = 'strict' | 'loose';

export async function reconcileKmsSignersContext(
  context: Context,
  parameters: {
    readonly address: ChecksummedAddress;
    readonly requestedKmsSignersContext: KmsSignersContext;
    readonly relayerExtraData: BytesHex;
    readonly mode: ReconcileMode;
  },
): Promise<KmsSignersContext> {
  const { address, requestedKmsSignersContext, relayerExtraData, mode } =
    parameters;

  assertIsKmsExtraData(relayerExtraData, {});
  assertIsKmsSignersContext(requestedKmsSignersContext, {});

  const requestedExtraData = kmsSignersContextToExtraData(
    requestedKmsSignersContext,
  );

  // 1. Exact match — the relayer used the same context as the SDK.
  if (relayerExtraData === requestedExtraData) {
    return requestedKmsSignersContext;
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

  // 2. In strict mode, only accept if the relayer used the current on-chain context.
  if (mode === 'strict') {
    // `readKmsSignersContext` with `forceRefresh` fetches the current context.
    // If `relayerKmsContextId` matches current, we're good.
    const currentContext = await readKmsSignersContext(context, {
      address,
      forceRefresh: true,
    });

    if (currentContext.id !== relayerKmsContextId) {
      throw new Error(
        `Strict reconciliation failed: relayer used context ${relayerKmsContextId}, ` +
          `but the current on-chain context is ${currentContext.id}.`,
      );
    }

    return currentContext;
  }

  // 3. Loose mode — accept any valid (non-destroyed, in-range) context.
  //    `readKmsSignersContext` with `forceRefresh` + specific `kmsContextId` will
  //    throw if the context is destroyed or out of range.
  return readKmsSignersContext(context, {
    address,
    kmsContextId: relayerKmsContextId,
    forceRefresh: true,
  });
}
