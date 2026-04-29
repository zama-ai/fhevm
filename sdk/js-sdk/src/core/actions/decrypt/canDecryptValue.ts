/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type {
  SignedDelegatedDecryptionPermit,
  SignedSelfDecryptionPermit,
} from '../../types/signedDecryptionPermit.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { EncryptedValueLike } from '../../types/encryptedTypes.js';
import { canDecryptValuesFromPairs as canDecryptValuesFromPairs_ } from '../../host-contracts/canDecryptValuesFromPairs.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { assertIsEncryptedValueLike, toFhevmHandle } from '../../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

type CanDecryptValueParametersBase = {
  readonly encryptedValue: EncryptedValueLike;
  readonly contractAddress: string;
};

export type CanDecryptValueWithUserAddressParameters = CanDecryptValueParametersBase & {
  readonly userAddress: string;
};

export type CanDecryptValueWithPermitParameters = CanDecryptValueParametersBase & {
  readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
  readonly transportKeypair?: TransportKeypair | undefined;
};

export type CanDecryptValueReturnType = {
  readonly allowed: boolean;
  readonly details: {
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  };
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Preflight check: returns whether `encryptedValue` can be decrypted for
 * `contractAddress` by the target user, without performing actual decryption.
 *
 * The target user is identified either by a plain address or by a
 * `SignedDecryptionPermit`.
 *
 * In all cases, the function checks on-chain ACL authorization for both:
 * - `contractAddress` to access `encryptedValue`
 * - the target user to access `encryptedValue`
 *
 * When a `SignedDecryptionPermit` is provided, the function additionally checks
 * that the permit is valid for the requested decryption context:
 * - the permit is structurally valid
 * - the current time is within the permit validity window
 * - the permit is scoped to the requested `contractAddress`
 *
 * When both a `SignedDecryptionPermit` and an `transportKeypair` are provided,
 * the function also checks that the permit is scoped to the corresponding
 * `transportKeypair.publicKey`.
 *
 * A permit is scoped to:
 * - a user
 * - a contract address
 * - a transport public key
 * - a validity time window
 *
 * A permit is not scoped to individual encryptedValues. EncryptedValue-level authorization is
 * always determined separately through the on-chain ACL.
 *
 * This function is intended as a preflight companion to `decryptValue`.
 * A `true` result means the requested decryption appears authorized at the time
 * of the call, but does not guarantee that a later decryption attempt cannot
 * fail for unrelated reasons.
 *
 * Additional conditions:
 * - all encryptedValues must belong to the same chain as the current `fhevm` client
 * - the target user address and `contractAddress` must be different
 */

export async function canDecryptValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValueWithUserAddressParameters,
): Promise<CanDecryptValueReturnType>;

export async function canDecryptValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValueWithPermitParameters,
): Promise<CanDecryptValueReturnType>;

export async function canDecryptValue(
  fhevm: Fhevm<FhevmChain>,
  parameters: CanDecryptValueWithUserAddressParameters | CanDecryptValueWithPermitParameters,
): Promise<CanDecryptValueReturnType> {
  const { encryptedValue, contractAddress, ...rest } = parameters;

  assertIsAddress(contractAddress, {});
  assertIsEncryptedValueLike(encryptedValue, { subject: `encryptedValue` });

  const results = await canDecryptValuesFromPairs_(fhevm, {
    ...rest,
    pairs: [{ handle: toFhevmHandle(encryptedValue), contractAddress: addressToChecksummedAddress(contractAddress) }],
  });

  return {
    allowed: results.allowed,
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    details: results.details[0]!,
  };
}
