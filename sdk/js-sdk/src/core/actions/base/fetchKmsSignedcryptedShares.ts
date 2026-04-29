/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { fetchKmsSignedcryptedShares as fetchKmsSignedcryptedShares_ } from '../../kms/fetchKmsSignedcryptedShares-p.js';
import { assertIsEncryptedValueLike, toFhevmHandle } from '../../handle/FhevmHandle.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';

////////////////////////////////////////////////////////////////////////////////

type FetchKmsSignedcryptedSharesParametersBase = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
};

export type FetchSelfKmsSignedcryptedSharesParameters = FetchKmsSignedcryptedSharesParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | undefined;
};

export type FetchDelegatedKmsSignedcryptedSharesParameters = FetchKmsSignedcryptedSharesParametersBase & {
  readonly signedPermit: SignedDelegatedDecryptionPermit;
  readonly options?: RelayerDelegatedUserDecryptOptions | undefined;
};

export type FetchKmsSignedcryptedSharesReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSignedcryptedShares
////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches KMS signcrypted decryption shares from the relayer.
 *
 * Performs the following verifications before making the request:
 * 1. Encrypted values are valid and non-empty.
 * 2. Contract addresses are present and within the max limit.
 * 3. All encrypted values belong to the host chain.
 * 4. Cumulative decryption bit limit (2048 bits) is not exceeded.
 * 5. Permit has not expired.
 * 6. ACL on-chain permissions: the encrypted data owner is allowed to decrypt.
 * 7. EIP-712 signature — skipped (guaranteed valid by {@link SignedDecryptionPermit} construction).
 * 8. Permit `extraData` matches the current on-chain {@link KmsSignersContext}.
 *
 * The returned {@link KmsSigncryptedShares} is fully validated (see
 * {@link KmsSigncryptedSharesImpl} invariants).
 */
export async function fetchKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchSelfKmsSignedcryptedSharesParameters,
): Promise<FetchKmsSignedcryptedSharesReturnType>;

export async function fetchKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchDelegatedKmsSignedcryptedSharesParameters,
): Promise<FetchKmsSignedcryptedSharesReturnType>;

export async function fetchKmsSignedcryptedShares(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchSelfKmsSignedcryptedSharesParameters | FetchDelegatedKmsSignedcryptedSharesParameters,
): Promise<FetchKmsSignedcryptedSharesReturnType> {
  const { pairs } = parameters;

  // Validate & sanitize `pairs` parameter
  const sanitizedPairs = pairs.map((p, i) => {
    assertIsEncryptedValueLike(p.encryptedValue, { subject: `encryptedValue[${i}]` });
    assertIsAddress(p.contractAddress, {});
    return {
      handle: toFhevmHandle(p.encryptedValue),
      contractAddress: addressToChecksummedAddress(p.contractAddress),
    };
  });

  return fetchKmsSignedcryptedShares_(fhevm, { ...parameters, pairs: sanitizedPairs });
}
