import type { E2eTransportKeypair } from '../../kms/E2eTransportKeypair-p.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { assertIsE2eTransportKeypair } from '../../kms/E2eTransportKeypair-p.js';
import { assertIsSignedDecryptionPermit } from '../../kms/SignedDecryptionPermit-p.js';

/**
 * Asserts that the permit's publicKey matches the keypair's publicKey.
 *
 * Both values are validated via `instanceof` before comparison.
 *
 * @throws {InvalidTypeError} If either value is not the expected type.
 * @throws If the public keys do not match.
 */
export function assertPermitMatchesKeypair(
  signedPermit: SignedDecryptionPermit,
  e2eTransportKeypair: E2eTransportKeypair,
): void {
  assertIsSignedDecryptionPermit(signedPermit, {});
  assertIsE2eTransportKeypair(e2eTransportKeypair, {});

  if (signedPermit.e2eTransportPublicKey.toLowerCase() !== e2eTransportKeypair.publicKey.toLowerCase()) {
    throw new Error(
      "The permit's publicKey does not match the E2eTransportKeypair's publicKey. " +
        'Ensure the permit was signed with the same keypair.',
    );
  }
}
