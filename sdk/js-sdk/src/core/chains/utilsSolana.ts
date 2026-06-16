import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { simpleDeepFreeze } from '../base/object.js';

export function defineFhevmSolanaChain<const chain extends FhevmSolanaChain>(fhevmSolanaChain: chain): chain {
  return simpleDeepFreeze(fhevmSolanaChain);
}
