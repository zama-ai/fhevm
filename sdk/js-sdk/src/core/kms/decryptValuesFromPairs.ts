////////////////////////////////////////////////////////////////////////////////

import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import { isSemverStrictlyBefore } from '../base/semver.js';
import { getResolvedProtocolVersion } from '../runtime/CoreFhevm-p.js';
import { decryptKmsSigncryptedShares } from './decryptKmsSigncryptedShares-p.js';
import { fetchKmsSigncryptedSharesV1 } from './fetchKmsSigncryptedSharesV1-p.js';
import { fetchKmsSigncryptedSharesV2 } from './fetchKmsSigncryptedSharesV2-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithDecrypt;
  readonly client: NonNullable<object>;
  readonly tkmsVersion: TkmsVersion;
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
};

export type ReturnType = readonly TypedValue[];

////////////////////////////////////////////////////////////////////////////////

export async function decryptValuesFromPairs(fhevm: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair } = parameters;

  const protocolVersion = getResolvedProtocolVersion(fhevm);
  if (protocolVersion === undefined) {
    throw new Error(
      'Unable to resolve protocol version from context, ensure proper initialization of the FhevmRuntime and FhevmChain.',
    );
  }

  let kmsSigncryptedShares: KmsSigncryptedShares;
  if (isSemverStrictlyBefore(protocolVersion.version, '0.14.0')) {
    kmsSigncryptedShares = await fetchKmsSigncryptedSharesV1(fhevm, parameters);
  } else {
    kmsSigncryptedShares = await fetchKmsSigncryptedSharesV2(fhevm, {
      ...parameters,
      options: parameters.options as RelayerUserDecryptOptions | undefined,
    });
  }

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  return decryptKmsSigncryptedShares(fhevm, {
    kmsSigncryptedShares,
    transportKeyPair: transportKeyPair,
  });
}
