import type { ChecksummedAddress, Uint8Number } from '../types/primitives.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { isVersionStrictlyBefore } from './HostContractVersion-p.js';
import { CACHE_TTL_15MIN, createCachedFetch } from '../base/cachedFetch.js';
import { assertIsChecksummedAddressArray } from '../base/address.js';
import { isUint8 } from '../base/uint.js';
import { getContextSignersAndThresholdFromExtraDataAbi } from './abi-fragments/fragments.js';
import { getTrustedClient } from '../runtime/CoreFhevm-p.js';
import { assertIsKmsExtraData } from '../kms/kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
};

type Parameters = {
  readonly kmsVerifierAddress: ChecksummedAddress;
  readonly extraData: KmsExtraData;
  readonly fhevmContext: FhevmClientFrozenContext;
};

type ReturnType = {
  readonly threshold: Uint8Number;
  readonly signers: readonly ChecksummedAddress[];
};

////////////////////////////////////////////////////////////////////////////////

const cachedGetKmsContextSignersAndThresholdFromExtraData = createCachedFetch<Context, Parameters, ReturnType>({
  executeFn: _getKmsContextSignersAndThresholdFromExtraData,
  cacheKeyFn: (context, params) => {
    const kmsContextId = params.extraData.kmsContextId;
    const kmsContextIdHex = kmsContextId.toString(16).padStart(64, '0');
    return `${context.runtime.uid.toLowerCase()}:${params.kmsVerifierAddress.toLowerCase()}:${kmsContextIdHex}`;
  },
  // Signers + threshold are immutable per context id, so in principle this could be
  // cached forever. But this on-chain read also serves as a context validity check —
  // it reverts (InvalidKmsContext) for an unknown/revoked context — and validity is
  // NOT immutable: a context can be revoked. Because the relayer is untrusted (it could
  // keep serving shares for a revoked context and won't signal the revocation), we must
  // re-verify on-chain within a bounded window rather than trust a stale "valid" entry.
  // Hence a short TTL, not a permanent cache.
  // NOTE: this only catches revocation if the on-chain read actually reflects it (reverts
  // for a *revoked*, not just non-existent, context). If revocation lives only on the
  // gateway (GatewayConfig.isValidKmsContext), a separate validity read is still needed.
  ttlMs: CACHE_TTL_15MIN,
});

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads the KMS signers and threshold for a given extraData context.
 *
 * Only available on KMSVerifier contracts >= v0.2.0. Throws on older versions.
 *
 * **Caching:** Cached per (runtime, address, contextId) with a short TTL
 * ({@link CACHE_TTL_15MIN}). The signers and threshold themselves are immutable per
 * context id, so they alone could be cached indefinitely — but this read also acts as
 * an on-chain context validity check (it reverts for an unknown/revoked context), and
 * validity is mutable. Since the relayer is untrusted, a TTL bounds how long a revoked
 * context could still be served from a stale entry before it is re-verified on-chain.
 * Concurrent callers to the same context share a single in-flight RPC request
 * (deduplication).
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
  const kmsVerifierVersion = parameters.fhevmContext.hostContractVersion('KMSVerifier');
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    throw new Error('getContextSignersAndThresholdFromExtraData requires KMSVerifier >= v0.2.0');
  }

  const trustedClient = getTrustedClient(context);

  const extraDataBytesHex = parameters.extraData.bytesHex;

  const res = await context.runtime.ethereum.readContract(trustedClient, {
    address: parameters.kmsVerifierAddress,
    abi: getContextSignersAndThresholdFromExtraDataAbi,
    args: [extraDataBytesHex],
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
