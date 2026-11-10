import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import { retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';
import { permitRequestStaleContextError } from '../host-contracts/readKmsSignersContext-p.js';
import { createKmsExtraDataFromBytesHex } from './kmsExtraData-p.js';
import { decryptKmsSigncryptedShares } from './decryptKmsSigncryptedShares-p.js';
import { fetchKmsSigncryptedSharesV1 } from './fetchKmsSigncryptedSharesV1-p.js';
import { fetchKmsSigncryptedSharesV2 } from './fetchKmsSigncryptedSharesV2-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain, WithDecrypt>;

type Parameters = {
  readonly pairs: ReadonlyArray<{
    readonly handle: Handle;
    readonly contractAddress: ChecksummedAddress;
    // ownerAddress is only relevant for V2 (protocol >= 0.14.0). When omitted,
    // fetchKmsSigncryptedSharesV2 defaults to userAddress (direct-access path).
    // Provide it explicitly only for delegated handles where ownerAddress !== userAddress.
    readonly ownerAddress?: ChecksummedAddress | undefined;
  }>;
  readonly signedPermit: SignedDecryptionPermit;
  readonly transportKeyPair: TransportKeyPair;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
  readonly fhevmContext: FhevmClientFrozenContext;
};

export type ReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairsWithGateway(fhevm: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, fhevmContext } = parameters;

  // A failure once the request is sent checks the KMS context and caches a
  // switched one, for the next permit. Not run again: the permit names the old
  // context.
  const kmsSigncryptedShares: KmsSigncryptedShares = await retryOnStaleCache(
    fhevm,
    fhevmContext,
    (usedVersions) => _fetchKmsSigncryptedShares(fhevm, parameters, usedVersions),
    {
      createStaleError: (_stale, error) =>
        permitRequestStaleContextError(
          fhevm,
          createKmsExtraDataFromBytesHex(parameters.signedPermit.eip712.message.extraData),
          error,
        ),
    },
  );

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  return decryptKmsSigncryptedShares(fhevm, {
    kmsSigncryptedShares,
    transportKeyPair,
    fhevmContext,
  });
}

////////////////////////////////////////////////////////////////////////////////

/** The request: the shares, from the route of the permit's version (which records the KMS context used). */
function _fetchKmsSigncryptedShares(
  fhevm: Context,
  parameters: Parameters,
  usedVersions: UsedCacheVersions,
): Promise<KmsSigncryptedShares> {
  // Route on the permit's OWN version, never the resolved protocol version. A signed
  // permit is a self-describing artifact: its version fixes the EIP-712 message shape,
  // so only the matching route can read it and reach the matching relayer endpoint.
  // Keying off protocol version instead is indirect and breaks the moment the two
  // diverge (e.g. a v1 permit used against a chain that now resolves to a newer
  // protocol) — the artifact you are consuming is the source of truth, not the context.
  //
  //   v1: message.contractAddresses              -> fetchKmsSigncryptedSharesV1 -> POST v2/user-decrypt
  //   v2: message.userAddress + allowedContracts -> fetchKmsSigncryptedSharesV2 -> POST v3/user-decrypt
  if (parameters.signedPermit.version === 1) {
    return fetchKmsSigncryptedSharesV1(fhevm, { ...parameters, usedVersions });
  }
  return fetchKmsSigncryptedSharesV2(fhevm, {
    ...parameters,
    usedVersions,
    options: parameters.options as RelayerUserDecryptOptions | undefined,
  });
}
