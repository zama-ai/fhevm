import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { assertIsTransportKeypair } from '../../kms/TransportKeypair-p.js';
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
  transportKeypair: TransportKeypair,
): void {
  assertIsSignedDecryptionPermit(signedPermit, {});
  assertIsTransportKeypair(transportKeypair, {});

  if (signedPermit.e2eTransportPublicKey.toLowerCase() !== transportKeypair.publicKey.toLowerCase()) {
    throw new Error(
      "The permit's publicKey does not match the E2eTransportKeypair's publicKey. " +
        'Ensure the permit was signed with the same keypair.',
    );
  }
}
