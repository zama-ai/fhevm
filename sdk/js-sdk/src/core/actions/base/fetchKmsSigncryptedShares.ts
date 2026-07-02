import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { RelayerDelegatedUserDecryptOptions, RelayerUserDecryptOptions } from '../../types/relayer.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { fetchKmsSigncryptedSharesV1 as fetchKmsSigncryptedSharesV1_ } from '../../kms/fetchKmsSigncryptedSharesV1-p.js';
import { fetchKmsSigncryptedSharesV2 as fetchKmsSigncryptedSharesV2_ } from '../../kms/fetchKmsSigncryptedSharesV2-p.js';
import { assertIsEncryptedValueLike, toFhevmHandle } from '../../handle/FhevmHandle.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';

////////////////////////////////////////////////////////////////////////////////

export type FetchKmsSigncryptedSharesParameters = {
  readonly pairs: ReadonlyArray<{
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
  }>;
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerUserDecryptOptions | RelayerDelegatedUserDecryptOptions | undefined;
};

export type FetchKmsSigncryptedSharesReturnType = KmsSigncryptedShares;

////////////////////////////////////////////////////////////////////////////////
// fetchKmsSigncryptedShares
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
export async function fetchKmsSigncryptedShares(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchKmsSigncryptedSharesParameters,
): Promise<FetchKmsSigncryptedSharesReturnType> {
  const { pairs, signedPermit } = parameters;

  // Validate & sanitize `pairs` parameter
  const sanitizedPairs = pairs.map((p, i) => {
    assertIsEncryptedValueLike(p.encryptedValue, { subject: `encryptedValue[${i}]` });
    assertIsAddress(p.contractAddress, {});
    return {
      handle: toFhevmHandle(p.encryptedValue),
      contractAddress: addressToChecksummedAddress(p.contractAddress),
    };
  });

  const options = parameters.options as RelayerUserDecryptOptions | undefined;

  if (signedPermit.version === 1) {
    return fetchKmsSigncryptedSharesV1_(fhevm, {
      ...parameters,
      pairs: sanitizedPairs,
      options,
    });
  }

  return fetchKmsSigncryptedSharesV2_(fhevm, {
    ...parameters,
    pairs: sanitizedPairs,
    options,
  });
}
