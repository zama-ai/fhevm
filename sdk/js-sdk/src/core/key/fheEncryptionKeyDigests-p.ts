import { shake256 } from '@noble/hashes/sha3.js';

////////////////////////////////////////////////////////////////////////////////
//
// Self-contained on purpose: scripts/build/codegen-key-digests-worker.mjs
// bundles this module, with @noble/hashes, into the worker code of
// fheEncryptionKeyDigestsWorkerCode-p.ts. Import nothing else here.
//
////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyDigests = {
  readonly publicKey: Uint8Array;
  readonly crs: Uint8Array;
};

// Domain separators the KMS prepends before hashing (SHAKE256, 32 bytes).
const PUBLIC_KEY_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_KEY');
const CRS_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_CRS');

/**
 * SHAKE256(`"PDAT_KEY"` ‖ public key), SHAKE256(`"PDAT_CRS"` ‖ CRS), 32 bytes
 * each. Blocking: ~140ms for the ~4.6MB CRS on desktop (~33ms/MiB with
 * @noble/hashes).
 */
export function computeFheEncryptionKeyDigests(input: {
  readonly publicKey: Uint8Array;
  readonly crs: Uint8Array;
}): FheEncryptionKeyDigests {
  return {
    publicKey: _digest(PUBLIC_KEY_DIGEST_DOMAIN, input.publicKey),
    crs: _digest(CRS_DIGEST_DOMAIN, input.crs),
  };
}

function _digest(domain: Uint8Array, bytes: Uint8Array): Uint8Array {
  return shake256.create({ dkLen: 32 }).update(domain).update(bytes).digest();
}
