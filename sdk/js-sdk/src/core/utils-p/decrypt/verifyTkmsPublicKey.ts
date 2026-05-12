import type { TkmsPrivateKey } from '../../types/tkms-p.js';
import type { FhevmRuntime, WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { BytesHex } from '../../types/primitives.js';

/**
 * Verifies that a TKMS public key matches the given private key
 * by re-deriving the public key and comparing.
 *
 * @throws If the derived public key does not match `tkmsPublicKeyBytesHex`.
 */
export async function verifyTkmsPublicKey(
  context: { readonly runtime: FhevmRuntime<WithDecrypt> },
  parameters: {
    readonly tkmsPrivateKey: TkmsPrivateKey;
    readonly tkmsPublicKeyBytesHex: BytesHex;
  },
): Promise<void> {
  const { tkmsPrivateKey, tkmsPublicKeyBytesHex } = parameters;

  const expectedTkmsPublicKeyBytesHex = await context.runtime.decrypt.getTkmsPublicKeyHex({
    tkmsPrivateKey,
  });

  if (expectedTkmsPublicKeyBytesHex !== tkmsPublicKeyBytesHex) {
    throw new Error('invalid TransportKeyPairKeyPair');
  }
}
