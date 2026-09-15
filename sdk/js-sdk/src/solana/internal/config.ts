import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import { authsAreEqual, loggersAreEqual, moduleVersionsAreEqual } from '../../core/runtimeConfig-p.js';

export type SolanaRuntimeConfig = Omit<FhevmRuntimeConfig, 'moduleVersions'>;

let solanaFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

////////////////////////////////////////////////////////////////////////////////

function runtimeConfigsAreEqual(a: FhevmRuntimeConfig, b: FhevmRuntimeConfig): boolean {
  return (
    loggersAreEqual(a.logger, b.logger) &&
    a.locateFile === b.locateFile &&
    a.wasmAssetLoadMode === b.wasmAssetLoadMode &&
    moduleVersionsAreEqual(a.moduleVersions, b.moduleVersions) &&
    a.singleThread === b.singleThread &&
    a.numberOfThreads === b.numberOfThreads &&
    authsAreEqual(a.auth, b.auth)
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Sets the global {@link FhevmRuntimeConfig} used by the Solana adapter.
 *
 * Must be called before any runtime or client is created.
 * May be called multiple times with identical parameters (idempotent).
 * Throws if called again with different parameters.
 *
 * @param config - The runtime configuration.
 * @throws If a different config has already been set.
 */
export function setFhevmRuntimeConfig(config: SolanaRuntimeConfig): void {
  if ('moduleVersions' in config && config.moduleVersions !== undefined) {
    throw new Error('Solana uses the SDK deployment-pinned WASM versions; moduleVersions is unsupported');
  }
  if (solanaFhevmRuntimeConfig === undefined) {
    solanaFhevmRuntimeConfig = Object.freeze<FhevmRuntimeConfig>({
      ...config,
      logger: config.logger ? Object.freeze({ ...config.logger }) : undefined,
      auth: config.auth ? Object.freeze({ ...config.auth }) : undefined,
    });
    return;
  }

  if (!runtimeConfigsAreEqual(solanaFhevmRuntimeConfig, config)) {
    throw new Error(
      'FhevmRuntime config has already been set and cannot be changed. ' +
        'Ensure setFhevmRuntimeConfig is called only once, or with identical parameters.',
    );
  }
}

export function hasFhevmRuntimeConfig(): boolean {
  return solanaFhevmRuntimeConfig !== undefined;
}

export function getFhevmRuntimeConfig(): FhevmRuntimeConfig {
  if (solanaFhevmRuntimeConfig === undefined) {
    throw new Error(
      'FhevmRuntime config has not been set. ' + 'Call setFhevmRuntimeConfig before creating any runtime or client.',
    );
  }
  return solanaFhevmRuntimeConfig;
}
