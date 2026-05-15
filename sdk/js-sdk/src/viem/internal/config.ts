import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';

let viemFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

////////////////////////////////////////////////////////////////////////////////

/**
 * Sets the global {@link FhevmRuntimeConfig} used by the viem adapter.
 *
 * Must be called before any runtime or client is created.
 * May be called multiple times with identical parameters (idempotent).
 * Throws if called again with different parameters.
 *
 * @param config - The runtime configuration.
 * @throws If a different config has already been set.
 */
export function setFhevmRuntimeConfig(config: FhevmRuntimeConfig): void {
  if (viemFhevmRuntimeConfig === undefined) {
    viemFhevmRuntimeConfig = Object.freeze({
      ...config,
      logger: config.logger ? Object.freeze({ ...config.logger }) : undefined,
    });
    return;
  }

  if (
    viemFhevmRuntimeConfig.logger !== config.logger ||
    viemFhevmRuntimeConfig.locateFile !== config.locateFile ||
    viemFhevmRuntimeConfig.singleThread !== config.singleThread ||
    viemFhevmRuntimeConfig.numberOfThreads !== config.numberOfThreads
  ) {
    throw new Error(
      'FhevmRuntime config has already been set and cannot be changed. ' +
        'Ensure setFhevmRuntimeConfig is called only once, or with identical parameters.',
    );
  }
}

export function hasFhevmRuntimeConfig(): boolean {
  return viemFhevmRuntimeConfig !== undefined;
}

export function getFhevmRuntimeConfig(): FhevmRuntimeConfig {
  if (viemFhevmRuntimeConfig === undefined) {
    throw new Error(
      'FhevmRuntime config has not been set. ' + 'Call setFhevmRuntimeConfig before creating any runtime or client.',
    );
  }
  return viemFhevmRuntimeConfig;
}
