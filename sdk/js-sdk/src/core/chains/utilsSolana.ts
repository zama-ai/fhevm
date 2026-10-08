import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { simpleDeepFreeze } from '../base/object.js';
import { assertValidSolanaChainId } from './hostChainId.js';

export function defineFhevmSolanaChain<const chain extends FhevmSolanaChain>(fhevmSolanaChain: chain): chain {
  assertValidSolanaChainId(fhevmSolanaChain.id);
  return simpleDeepFreeze(fhevmSolanaChain);
}
