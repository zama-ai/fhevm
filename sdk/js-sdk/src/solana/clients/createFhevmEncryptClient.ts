import type { Bytes32Hex } from '../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { SolanaEncryptActions } from './decorators/encrypt.js';
import type { Fhevm } from '../../core/types/coreFhevmClient.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { solanaEncryptActions } from './decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type FhevmSolanaEncryptClient<chain extends FhevmSolanaChain = FhevmSolanaChain> = Fhevm<
  undefined,
  WithEncrypt,
  undefined
> & { readonly solanaChain: chain } & SolanaEncryptActions;

/**
 * Creates a Solana encrypt-only client. `.buildInputProof(...)` produces the RFC-021 proof and
 * `.submitInputProof(...)` submits it to the relayer while checking the returned handles.
 *
 * @param parameters.chain - The Solana host chain definition.
 * @param parameters.aclProgramAddress - The zama-host program id as bytes32 (the Solana ACL identity).
 * @param parameters.options - Optional client options.
 */
export function createFhevmEncryptClient<chain extends FhevmSolanaChain>(parameters: {
  readonly chain: chain;
  readonly aclProgramAddress: Bytes32Hex;
  readonly options?: FhevmOptions | undefined;
}): FhevmSolanaEncryptClient<chain> {
  const c = createFhevmBaseClient({ chain: parameters.chain, options: parameters.options });
  return c.extend(solanaEncryptActions(parameters.aclProgramAddress));
}
