import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { Fhevm } from '../../core/types/coreFhevmClient.js';
import { PRIVATE_SOLANA_TOKEN } from '../internal/solana-p.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { getSolanaRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Base Solana client: a chain + runtime, no native client.
 *
 * Unlike the EVM adapters, the Solana host has no provider to seal into a `TrustedClient`
 * (the user-decrypt path is ed25519 + relayer HTTP, with a static KMS context). The chain is
 * carried as the opaque `chain` field so Solana actions can read the relayer URL, gateway and
 * KMS wiring from it.
 */
export function createFhevmBaseClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): Fhevm<undefined, FhevmRuntime, undefined> & { readonly solanaChain: chain } {
  // The core client is chain-agnostic at runtime (it just freezes and exposes `chain`); the
  // Solana chain shape differs from `FhevmChain`, so it is carried as `solanaChain` instead and
  // the core `chain` is left undefined (no EVM semantics apply).
  const c: Fhevm<undefined, FhevmRuntime, undefined> = createCoreFhevm(PRIVATE_SOLANA_TOKEN, {
    runtime: getSolanaRuntime(),
    options: parameters.options,
  });

  Object.defineProperty(c, 'solanaChain', {
    value: parameters.chain,
    writable: false,
    configurable: false,
    enumerable: true,
  });

  return c as Fhevm<undefined, FhevmRuntime, undefined> & { readonly solanaChain: chain };
}
