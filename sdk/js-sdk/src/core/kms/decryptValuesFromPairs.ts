////////////////////////////////////////////////////////////////////////////////

import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { SDK_PROTOCOL_API_MAJOR_VERSION, SDK_PROTOCOL_API_MINOR_VERSION } from '../runtime/sdkProtocolApiVersion.js';
import { decryptKmsSigncryptedShares } from './decryptKmsSigncryptedShares-p.js';
import { fetchKmsSigncryptedSharesV1 } from './fetchKmsSigncryptedSharesV1-p.js';
import { fetchKmsSigncryptedSharesV2 } from './fetchKmsSigncryptedSharesV2-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithDecrypt;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

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

export async function decryptValuesFromPairs(fhevm: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, fhevmContext } = parameters;

  let kmsSigncryptedShares: KmsSigncryptedShares;

  // Compile-time fast path for the protocol API version this SDK is using.
  // At v0.13.x the SDK only ever produces/handles V1 permits, so the V2 fetch
  // path is statically unreachable here. This guard is a foldable literal
  // comparison, so bundlers evaluate it at build time and tree-shake
  // fetchKmsSigncryptedSharesV2 (and its V2-only deps) out of the bundle.
  // When the SDK adopts protocol API v0.14+, this branch stops folding to true
  // and the permit-version routing below takes over.
  if (SDK_PROTOCOL_API_MAJOR_VERSION === 0 && SDK_PROTOCOL_API_MINOR_VERSION <= 13) {
    // A V2 permit cannot be served by this build: routing it through the V1
    // path would silently reach the wrong relayer endpoint, so fail loudly.
    if (parameters.signedPermit.version > 1) {
      throw new Error(
        `Unsupported permit version ${parameters.signedPermit.version}: this SDK uses protocol API v0.13.x, which only supports V1 decryption permits. V2 permits require an SDK using protocol API v0.14.0 or later.`,
      );
    }
    kmsSigncryptedShares = await fetchKmsSigncryptedSharesV1(fhevm, parameters);
  } else {
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
      kmsSigncryptedShares = await fetchKmsSigncryptedSharesV1(fhevm, parameters);
    } else {
      kmsSigncryptedShares = await fetchKmsSigncryptedSharesV2(fhevm, {
        ...parameters,
        options: parameters.options as RelayerUserDecryptOptions | undefined,
      });
    }
  }

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  return decryptKmsSigncryptedShares(fhevm, {
    kmsSigncryptedShares,
    transportKeyPair,
    fhevmContext,
  });
}
