import type { WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { KmsSigncryptedShares } from '../types/kms.js';
import type { ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../types/relayer.js';
import type { SignedDecryptionPermit } from '../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from './TransportKeyPair-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { decryptKmsSigncryptedShares } from './decryptKmsSigncryptedShares-p.js';
import { fetchKmsSigncryptedSharesV2NoGateway } from './fetchKmsSigncryptedSharesV2NoGateway-p.js';

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
    // fetchKmsSigncryptedSharesV2NoGateway defaults to userAddress (direct-access path).
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

export async function decryptValuesFromPairsNoGateway(fhevm: Context, parameters: Parameters): Promise<ReturnType> {
  const { transportKeyPair: transportKeyPair, fhevmContext } = parameters;

  let kmsSigncryptedShares: KmsSigncryptedShares;

  // relayer-http's POST /v4/user-decrypt only accepts the unified attestation
  // (attestationType "eip712-unified-user-decrypt-v1"), whose payload needs
  // userAddress + allowedContracts — fields a V1 permit's signed message does
  // not carry. So, unlike the WithGateway path, there is no v1 route here: a
  // V1 permit cannot be forwarded to relayer-http at all.
  if (parameters.signedPermit.version >= 2) {
    kmsSigncryptedShares = await fetchKmsSigncryptedSharesV2NoGateway(fhevm, {
      ...parameters,
      options: parameters.options as RelayerUserDecryptOptions | undefined,
    });
  } else {
    throw new Error(
      `Unsupported permit version for no-gateway decryption: ${parameters.signedPermit.version}. ` +
        'relayer-http only accepts unified (V2) permits — sign one with signUnifiedDecryptionPermit.',
    );
  }

  // Using the `KmsSigncryptedShares` decrypt and reconstruct clear values
  return decryptKmsSigncryptedShares(fhevm, {
    kmsSigncryptedShares,
    transportKeyPair,
    fhevmContext,
  });
}
