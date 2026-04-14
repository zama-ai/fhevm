import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { E2eTransportKeypair } from '../../kms/E2eTransportKeypair-p.js';
import { parseSignedDecryptionPermit as parseSignedDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseSignedDecryptionPermitParameters = {
  /** The serialized permit — a JSON string or a previously parsed object. */
  readonly serialized: string | Record<string, unknown>;
  /** The e2e transport keypair that was used when signing the permit. */
  readonly e2eTransportKeypair: E2eTransportKeypair;
};

export type ParseSignedDecryptionPermitReturnType = SignedDecryptionPermit;

/**
 * Parses and verifies a previously serialized signed decryption permit.
 *
 * Validates the EIP-712 structure, verifies the signature against the on-chain
 * verifier, and checks the permit's public key matches the provided keypair.
 *
 * @throws If the permit is malformed, expired, or the signature is invalid.
 */
export async function parseSignedDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: ParseSignedDecryptionPermitParameters,
): Promise<ParseSignedDecryptionPermitReturnType> {
  const { serialized, e2eTransportKeypair } = parameters;

  const parsed = typeof serialized === 'string' ? (JSON.parse(serialized) as unknown) : serialized;

  return parseSignedDecryptionPermit_(fhevm, e2eTransportKeypair, parsed);
}
