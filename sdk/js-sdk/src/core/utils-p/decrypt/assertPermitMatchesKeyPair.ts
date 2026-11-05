import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { assertIsTransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { assertIsSignedDecryptionPermit } from '../../kms/SignedDecryptionPermit-p.js';
import { InvalidSignedPermitError } from '../../errors/InvalidSignedPermitError.js';

/**
 * Asserts that the permit's publicKey matches the key pair's publicKey.
 *
 * Both values are validated via `instanceof` before comparison.
 *
 * @throws {InvalidSignedPermitError} If the permit is not a signed decryption permit created by the SDK.
 * @throws {InvalidTypeError} If the key pair is not a TransportKeyPair.
 * @throws {InvalidSignedPermitError} If the public keys do not match.
 */
export function assertPermitMatchesKeyPair(
  signedPermit: SignedDecryptionPermit,
  transportKeyPair: TransportKeyPair,
): void {
  assertIsSignedDecryptionPermit(signedPermit, {});
  assertIsTransportKeyPair(transportKeyPair, {});

  if (signedPermit.transportPublicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
    throw new InvalidSignedPermitError({
      message:
        "The permit's publicKey does not match the TransportKeyPair's publicKey. " +
        'Ensure the permit was signed with the same key pair.',
    });
  }
}
