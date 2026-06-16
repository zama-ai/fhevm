import { ed25519 } from '@noble/curves/ed25519.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Minimal ed25519 signer the Solana user-decrypt client needs.
 *
 * The shape mirrors `@solana/kit`'s `MessagePartialSigner`: it exposes the 32-byte ed25519
 * public key (the user's `identity`) and signs an arbitrary message. A Kit
 * `MessagePartialSigner` satisfies this interface once its `signMessages` result is adapted
 * to return the raw 64-byte signature (see {@link solanaSignerFromKitPartialSigner}); we keep
 * the surface intentionally small so no `@solana/kit` dependency is required.
 *
 * `@solana/kit` is **not** a dependency of this SDK, so it is not imported here.
 */
export interface SolanaUserDecryptSigner {
  /** The signer's 32-byte ed25519 public key — the user-decrypt `identity`. */
  readonly publicKey: Uint8Array;
  /**
   * Signs `preimage` with the ed25519 secret key and resolves to the raw 64-byte signature.
   * The returned signature must verify against {@link SolanaUserDecryptSigner.publicKey}.
   */
  sign(preimage: Uint8Array): Promise<Uint8Array>;
}

////////////////////////////////////////////////////////////////////////////////

const ED25519_SEED_LEN = 32;

/**
 * Wraps a raw 32-byte ed25519 seed into a {@link SolanaUserDecryptSigner}.
 *
 * Uses the same `@noble/curves/ed25519` primitive as the core
 * `buildSolanaUserDecryptRequest`, so a request signed via this adapter is byte-identical to
 * the one the KMS connector re-derives and verifies.
 *
 * @param seed - The 32-byte ed25519 seed (secret key).
 */
export function solanaSignerFromSecretKey(seed: Uint8Array): SolanaUserDecryptSigner {
  if (seed.length !== ED25519_SEED_LEN) {
    throw new Error(`seed must be a 32-byte ed25519 seed, got ${seed.length}`);
  }
  const publicKey = ed25519.getPublicKey(seed);
  return Object.freeze({
    publicKey,
    sign: (preimage: Uint8Array): Promise<Uint8Array> => Promise.resolve(ed25519.sign(preimage, seed)),
  });
}
