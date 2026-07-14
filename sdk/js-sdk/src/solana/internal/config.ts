import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import { cloneModuleVersions, moduleVersionsAreEqual } from '../../core/runtimeConfig-p.js';

let solanaFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

////////////////////////////////////////////////////////////////////////////////

function loggersAreEqual(a: FhevmRuntimeConfig['logger'], b: FhevmRuntimeConfig['logger']): boolean {
  return a === b || (a?.debug === b?.debug && a?.warn === b?.warn && a?.error === b?.error);
}

function authConfigsAreEqual(a: FhevmRuntimeConfig['auth'], b: FhevmRuntimeConfig['auth']): boolean {
  if (a === b) {
    return true;
  }
  if (a?.type !== b?.type) {
    return false;
  }
  if (a === undefined || b === undefined) {
    return false;
  }

  switch (a.type) {
    case 'BearerToken':
      return b.type === 'BearerToken' && a.token === b.token;
    case 'ApiKeyHeader':
      return b.type === 'ApiKeyHeader' && a.header === b.header && a.value === b.value;
    case 'ApiKeyCookie':
      return b.type === 'ApiKeyCookie' && a.cookie === b.cookie && a.value === b.value;
  }
}

function runtimeConfigsAreEqual(a: FhevmRuntimeConfig, b: FhevmRuntimeConfig): boolean {
  return (
    loggersAreEqual(a.logger, b.logger) &&
    a.locateFile === b.locateFile &&
    a.wasmAssetLoadMode === b.wasmAssetLoadMode &&
    moduleVersionsAreEqual(a.moduleVersions, b.moduleVersions) &&
    a.singleThread === b.singleThread &&
    a.numberOfThreads === b.numberOfThreads &&
    authConfigsAreEqual(a.auth, b.auth)
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
export function setFhevmRuntimeConfig(config: FhevmRuntimeConfig): void {
  if (solanaFhevmRuntimeConfig === undefined) {
    solanaFhevmRuntimeConfig = Object.freeze<FhevmRuntimeConfig>({
      ...config,
      logger: config.logger ? Object.freeze({ ...config.logger }) : undefined,
      moduleVersions: cloneModuleVersions(config.moduleVersions),
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
