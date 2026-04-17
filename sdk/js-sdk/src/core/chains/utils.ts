import type { FhevmChain } from '../types/fhevmChain.js';
import { simpleDeepFreeze } from '../base/object.js';

export function defineFhevmChain<const chain extends FhevmChain>(fhevmChain: chain): chain {
  return simpleDeepFreeze(fhevmChain);
}
