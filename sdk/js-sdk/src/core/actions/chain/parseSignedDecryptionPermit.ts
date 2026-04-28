import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { TransportKeypair } from '../../kms/TransportKeypair-p.js';
import type { SerializeSignedDecryptionPermitReturnType } from './serializeSignedDecryptionPermit.js';
import { parseSignedDecryptionPermit as parseSignedDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseSignedDecryptionPermitParameters = {
  /** The serialized permit — a previously parsed permit object. */
  readonly serializedPermit: SerializeSignedDecryptionPermitReturnType;
  /** The transport keypair that was used when signing the permit. */
  readonly transportKeypair: TransportKeypair;
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
  const { serializedPermit, transportKeypair } = parameters;

  return parseSignedDecryptionPermit_(fhevm, transportKeypair, serializedPermit);
}
