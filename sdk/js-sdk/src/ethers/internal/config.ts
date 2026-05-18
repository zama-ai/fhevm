import type { FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import { cloneModuleVersions } from '../../core/runtimeConfig-p.js';

let ethersFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

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
  if (ethersFhevmRuntimeConfig === undefined) {
    ethersFhevmRuntimeConfig = Object.freeze({
      ...config,
      logger: config.logger ? Object.freeze({ ...config.logger }) : undefined,
      moduleVersions: cloneModuleVersions(config.moduleVersions),
    });
    return;
  }

  if (
    ethersFhevmRuntimeConfig.logger !== config.logger ||
    ethersFhevmRuntimeConfig.locateFile !== config.locateFile ||
    ethersFhevmRuntimeConfig.wasmAssetLoadMode !== config.wasmAssetLoadMode ||
    !moduleVersionsAreEqual(ethersFhevmRuntimeConfig.moduleVersions, config.moduleVersions) ||
    ethersFhevmRuntimeConfig.singleThread !== config.singleThread ||
    ethersFhevmRuntimeConfig.numberOfThreads !== config.numberOfThreads
  ) {
    throw new Error(
      'FhevmRuntime config has already been set and cannot be changed. ' +
        'Ensure setFhevmRuntimeConfig is called only once, or with identical parameters.',
    );
  }
}

export function hasFhevmRuntimeConfig(): boolean {
  return ethersFhevmRuntimeConfig !== undefined;
}

export function getFhevmRuntimeConfig(): FhevmRuntimeConfig {
  if (ethersFhevmRuntimeConfig === undefined) {
    throw new Error(
      'FhevmRuntime config has not been set. ' + 'Call setFhevmRuntimeConfig before creating any runtime or client.',
    );
  }
  return ethersFhevmRuntimeConfig;
}

function moduleVersionsAreEqual(
  a: FhevmRuntimeConfig['moduleVersions'],
  b: FhevmRuntimeConfig['moduleVersions'],
): boolean {
  const normalizedA = normalizeModuleVersions(a);
  const normalizedB = normalizeModuleVersions(b);
  return (
    normalizedA.tfhe === normalizedB.tfhe &&
    normalizedA.kms === normalizedB.kms &&
    normalizedA.checkCompatibility === normalizedB.checkCompatibility
  );
}

function normalizeModuleVersions(moduleVersions: FhevmRuntimeConfig['moduleVersions']): {
  readonly tfhe: 'auto' | NonNullable<Exclude<FhevmRuntimeConfig['moduleVersions'], 'auto'>>['tfhe'];
  readonly kms: 'auto' | NonNullable<Exclude<FhevmRuntimeConfig['moduleVersions'], 'auto'>>['kms'];
  readonly checkCompatibility: NonNullable<Exclude<FhevmRuntimeConfig['moduleVersions'], 'auto'>>['checkCompatibility'];
} {
  if (moduleVersions === undefined || moduleVersions === 'auto') {
    return { tfhe: 'auto', kms: 'auto', checkCompatibility: undefined };
  }
  return {
    tfhe: moduleVersions.tfhe ?? 'auto',
    kms: moduleVersions.kms ?? 'auto',
    checkCompatibility: moduleVersions.checkCompatibility,
  };
}
