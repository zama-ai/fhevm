import type { RelayerPublicDecryptOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import type { PublicDecryptionProof } from '../types/publicDecryptionProof-p.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { KmsExtraData } from '../types/kms-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { assertHandlesBelongToSameChainId } from '../handle/FhevmHandle.js';
import { assertKmsDecryptionBitLimit } from './utils.js';
import { readCurrentKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { createPublicDecryptionProof } from '../kms/PublicDecryptionProof-p.js';
import { checkAllowedForDecryption } from '../host-contracts/checkAllowedForDecryption.js';
import { kmsContextVersion } from '../host-contracts/readKmsSignersContext-p.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';
import { retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain>;

type Parameters = {
  readonly originToken: symbol;
  readonly handles: readonly Handle[];
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerPublicDecryptOptions | undefined;
};

type ReturnType = PublicDecryptionProof;

////////////////////////////////////////////////////////////////////////////////

export async function publicDecryptWithGateway(context: Context, parameters: Parameters): Promise<ReturnType> {
  // A failure after the request was sent (error, timeout, abort) checks the KMS
  // context: if it changed, the request runs once more (unless aborted).
  return retryOnStaleCache(context, parameters.fhevmContext, (usedVersions) =>
    _publicDecryptWithGateway(context, parameters, usedVersions),
  );
}

async function _publicDecryptWithGateway(
  context: Context,
  parameters: Parameters,
  usedVersions: UsedCacheVersions,
): Promise<ReturnType> {
  const { handles, options, originToken, fhevmContext } = parameters;

  // Request side: build dynamic extraData from current context ID
  // Contrary to the userDecrypt flow, the publicDecrypt doesn't require for an
  // EIP-712 signature from the user, so the SDK can safely fetch the current
  // context ID and build the extraData transparently to the user.
  const requestedKmsSignersContext = await readCurrentKmsSignersContext(context, {
    kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
    protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
  });

  const requestedExtraData: KmsExtraData = kmsSignersContextToExtraData(requestedKmsSignersContext);

  const orderedHandles = handles;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerPublicDecryptOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  // 1. Check: At least one handle is required
  if (orderedHandles.length === 0) {
    throw Error(`handles must not be empty, at least one handle is required`);
  }

  // 2. Check: 2048 bits limit
  assertKmsDecryptionBitLimit(orderedHandles);

  // 3. Check: All handles belong to the host chainId
  assertHandlesBelongToSameChainId(orderedHandles, BigInt(context.chain.id) as Uint64BigInt);

  // 4. Check: ACL permissions
  await checkAllowedForDecryption(context, {
    aclAddress: context.chain.fhevm.contracts.acl.address as ChecksummedAddress,
    handles: orderedHandles,
  });

  // 5. Call relayer. From here on, a failure may come from a KMS context
  // switch: record the context used.
  recordUsedCacheVersion(
    usedVersions,
    'kmsContext',
    kmsContextVersion(requestedKmsSignersContext.id, requestedKmsSignersContext.epochId),
  );
  const {
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    // ignore returned relayer extraData as we never trust the relayer
    // extraData: relayerExtraDataBytesHex,
  } = await context.runtime.relayer.fetchPublicDecrypt(context, {
    payload: {
      orderedHandles,
      extraData: requestedExtraData.bytesHex,
    },
    options: relayerOptions,
    fhevmContext,
  });

  // 6. Verify and Compute PublicDecryptionProof
  const publicDecryptionProof: PublicDecryptionProof = await createPublicDecryptionProof(context, {
    originToken,
    orderedHandles: orderedHandles,
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEip712Signatures,
    kmsSignersContext: requestedKmsSignersContext,
  });

  return publicDecryptionProof;
}
