import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { SolanaDecryptActions } from './decorators/decrypt.js';
import type { SolanaDecryptTrust, SolanaPermitDecryptActions } from './decorators/permitDecrypt.js';
import type { Fhevm } from '../../core/types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { solanaDecryptActions } from './decorators/decrypt.js';
import { solanaPermitDecryptActions } from './decorators/permitDecrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmSolanaDecryptClient<chain extends FhevmSolanaChain = FhevmSolanaChain> = Fhevm<
  undefined,
  FhevmRuntime,
  undefined
> & { readonly solanaChain: chain } & SolanaDecryptActions;

export type FhevmSolanaPermitDecryptClient<chain extends FhevmSolanaChain = FhevmSolanaChain> =
  FhevmSolanaDecryptClient<chain> & SolanaPermitDecryptActions;

/**
 * Creates a Solana decrypt-side client.
 *
 * Without `trust`, the client carries the public-decrypt actions and nothing else. With it, the
 * permit path is wired in: `signPermit` takes a permit to the wallet once through the sRFC-38
 * channel, and `userDecrypt` runs requests under that one signature — evidence from the chain's
 * RPC and proof service, transport to its relayer, verification under the trust configuration.
 * The chain must then also name `rpcUrl`, `proofServiceUrl` and `verifyingProgramId`.
 *
 * There is deliberately no `generateTransportKeyPair` action: the pair a permit commits to comes
 * from `generateSolanaTransportKeyPair` (`solana/userDecrypt`), and `signPermit` generates it
 * itself; the core EVM-blob pair the retired action produced cannot be consumed by this path.
 *
 * @param parameters.chain - The Solana host chain definition.
 * @param parameters.options - Optional client options.
 * @param parameters.trust - The permit path's trust configuration; omit for public-decrypt only.
 */
export function createFhevmDecryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmSolanaDecryptClient<chain>;
export function createFhevmDecryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
  readonly trust: SolanaDecryptTrust;
}): FhevmSolanaPermitDecryptClient<chain>;
export function createFhevmDecryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
  readonly trust?: SolanaDecryptTrust | undefined;
}): FhevmSolanaDecryptClient<chain> | FhevmSolanaPermitDecryptClient<chain> {
  const c = createFhevmBaseClient({ chain: parameters.chain, options: parameters.options }).extend(
    solanaDecryptActions,
  );
  const trust = parameters.trust;
  if (trust === undefined) {
    return c;
  }
  return c.extend((fhevm) => ({
    actions: solanaPermitDecryptActions(parameters.chain, trust, fhevm.runtime),
    runtime: fhevm.runtime,
  }));
}
