import type { SolanaUserDecryptSigner } from '../signer.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { SolanaDecryptActions } from './decorators/decrypt.js';
import type { Fhevm } from '../../core/types/coreFhevmClient.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { solanaDecryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmSolanaDecryptClient<chain extends FhevmSolanaChain = FhevmSolanaChain> = Fhevm<
  undefined,
  WithDecrypt,
  undefined
> & { readonly solanaChain: chain } & SolanaDecryptActions;

/**
 * Creates a Solana decrypt-only client. `.userDecrypt(...)` runs the full ed25519 user-decrypt
 * round-trip to cleartext: it signs the request with `signer`, POSTs it to the chain's relayer,
 * fetches the signcrypted shares, and reuses the core de-signcryption (TKMS WASM) to return the
 * plaintext — full parity with the EVM `decryptValue`.
 *
 * @param parameters.signer - The user's ed25519 signer (its public key is the decrypt identity).
 * @param parameters.chain - The Solana host chain definition.
 * @param parameters.options - Optional client options.
 */
export function createFhevmDecryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly signer: SolanaUserDecryptSigner;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmSolanaDecryptClient<chain> {
  const c = createFhevmBaseClient({ chain: parameters.chain, options: parameters.options });
  return c.extend(solanaDecryptActions(parameters.signer));
}
