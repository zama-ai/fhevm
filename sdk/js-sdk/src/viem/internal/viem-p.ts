import type { PublicClient } from 'viem';
import type { TrustedClient } from '../../core/modules/ethereum/types.js';
import type { FhevmRuntime, FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import type { CreateFhevmRuntimeParameters } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { verifyTrustedValue } from '../../core/base/trustedValue.js';
import { createFhevmRuntime as createFhevmRuntime_ } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { createTrustedClient } from '../../core/modules/ethereum/createTrustedClient.js';
import { ethereumModule } from './ethereum.js';
import { relayerModule } from '../../core/modules/relayer/module/index.js';

////////////////////////////////////////////////////////////////////////////////

// Will leak in js
export const PRIVATE_VIEM_TOKEN = Symbol('viem.token');

////////////////////////////////////////////////////////////////////////////////

let viemFhevmRuntime: FhevmRuntime | undefined;
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
    viemFhevmRuntimeConfig = { ...config };
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

////////////////////////////////////////////////////////////////////////////////

export function getViemRuntime(): FhevmRuntime {
  if (viemFhevmRuntimeConfig === undefined) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = ethereumModule();
  const rm = relayerModule();

  viemFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: viemFhevmRuntimeConfig,
  });
  return viemFhevmRuntime;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Seals a viem `PublicClient` into an opaque {@link TrustedClient}.
 *
 * The returned value can be passed through the core layer without exposing
 * the underlying viem instance. Only the viem adapter can later recover
 * the original public client via {@link trustedClientToViemPublicClient}.
 *
 * @param publicClient - The viem public client to seal.
 * @returns An opaque {@link TrustedClient} bound to the viem origin token.
 */
export function viemPublicClientToTrustedClient<client extends PublicClient>(
  publicClient: client,
): TrustedClient<client> {
  return createTrustedClient(publicClient, PRIVATE_VIEM_TOKEN);
}

/**
 * Verifies that the {@link TrustedClient} was created by the viem adapter
 * and extracts the original `PublicClient`.
 *
 * @param trustedClient - The host client to verify.
 * @returns The original viem `PublicClient`.
 * @throws {Error} If the client was not created by {@link viemPublicClientToTrustedClient}.
 */
export function trustedClientToViemPublicClient<client extends PublicClient>(
  trustedClient: TrustedClient<client>,
): client {
  return verifyTrustedValue(trustedClient, PRIVATE_VIEM_TOKEN);
}

////////////////////////////////////////////////////////////////////////////////

export function createFhevmRuntime(parameters: CreateFhevmRuntimeParameters): FhevmRuntime {
  return createFhevmRuntime_(PRIVATE_VIEM_TOKEN, parameters);
}
