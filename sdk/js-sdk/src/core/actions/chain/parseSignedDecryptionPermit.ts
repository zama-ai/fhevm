import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import type { TransportKeyPair } from '../../kms/TransportKeyPair-p.js';
import type { KmsDecryptEip712Like } from '../../types/kms.js';
import { parseSignedDecryptionPermit as parseSignedDecryptionPermit_ } from '../../kms/SignedDecryptionPermit-p.js';

////////////////////////////////////////////////////////////////////////////////

export type ParseSignedDecryptionPermitParameters = {
  /** The serialized permit — a previously parsed permit object. */
  readonly serializedPermit: {
    readonly eip712: KmsDecryptEip712Like;
    readonly signature: string;
    readonly signerAddress: string;
  };
  /** The transport key pair that was used when signing the permit. */
  readonly transportKeyPair: TransportKeyPair;
};

export type ParseSignedDecryptionPermitReturnType = SignedDecryptionPermit;

/**
 * Parses and verifies a previously serialized signed decryption permit.
 *
 * Validates the EIP-712 structure, verifies the signature against the on-chain
 * verifier, and checks the permit's public key matches the provided key pair.
 *
 * @throws If the permit is malformed, expired, or the signature is invalid.
 */
export async function parseSignedDecryptionPermit(
  fhevm: Fhevm<FhevmChain>,
  parameters: ParseSignedDecryptionPermitParameters,
): Promise<ParseSignedDecryptionPermitReturnType> {
  const { serializedPermit, transportKeyPair: transportKeyPair } = parameters;

  return parseSignedDecryptionPermit_(fhevm, transportKeyPair, serializedPermit);
}
