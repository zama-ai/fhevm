import { simpleDeepFreeze } from "../base/object.js";
import type { FhevmChain } from "../types/fhevmChain.js";

export function defineFhevmChain<const chain extends FhevmChain>(
  fhevmChain: chain,
): chain {
  return simpleDeepFreeze(fhevmChain);
}
