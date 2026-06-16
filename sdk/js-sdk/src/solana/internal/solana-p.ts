import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { CreateFhevmRuntimeParameters } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { createFhevmRuntime as createFhevmRuntime_ } from '../../core/runtime/CoreFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

// Will leak in js
export const PRIVATE_SOLANA_TOKEN = Symbol('solana.token');

////////////////////////////////////////////////////////////////////////////////

export function createFhevmRuntime(parameters: CreateFhevmRuntimeParameters): FhevmRuntime {
  return createFhevmRuntime_(PRIVATE_SOLANA_TOKEN, parameters);
}
