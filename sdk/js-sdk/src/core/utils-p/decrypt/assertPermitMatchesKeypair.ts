import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { assertIsTransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import { assertIsSignedDecryptionPermit } from '../../kms/SignedDecryptionPermit-p.js';

/**
 * Asserts that the permit's publicKey matches the key pair's publicKey.
 *
 * Both values are validated via `instanceof` before comparison.
 *
 * @throws {InvalidTypeError} If either value is not the expected type.
 * @throws If the public keys do not match.
 */
export function assertPermitMatchesKeyPair(
  signedPermit: SignedDecryptionPermit,
  transportKeyPair: TransportKeyPair,
): void {
  assertIsSignedDecryptionPermit(signedPermit, {});
  assertIsTransportKeyPair(transportKeyPair, {});

  if (signedPermit.e2eTransportPublicKey.toLowerCase() !== transportKeyPair.publicKey.toLowerCase()) {
    throw new Error(
      "The permit's publicKey does not match the E2eTransportKeyPair's publicKey. " +
        'Ensure the permit was signed with the same key pair.',
    );
  }
}
