import type { ethers as EthersT } from 'ethers';
import type { TrustedClient } from '../../core/modules/ethereum/types.js';
import type { FhevmRuntime, FhevmRuntimeConfig } from '../../core/types/coreFhevmRuntime.js';
import type { CreateFhevmRuntimeParameters } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { createFhevmRuntime as createFhevmRuntime_ } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { verifyTrustedValue } from '../../core/base/trustedValue.js';
import { createTrustedClient } from '../../core/modules/ethereum/createTrustedClient.js';
import { ethereumModule } from './ethereum.js';
import { relayerModule } from '../../core/modules/relayer/module/index.js';

////////////////////////////////////////////////////////////////////////////////

// Will leak in js
export const PRIVATE_ETHERS_TOKEN = Symbol('ethers.token');

////////////////////////////////////////////////////////////////////////////////

let ethersFhevmRuntime: FhevmRuntime | undefined;
let ethersFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

////////////////////////////////////////////////////////////////////////////////

/**
 * Sets the global {@link FhevmRuntimeConfig} used by the ethers adapter.
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
    ethersFhevmRuntimeConfig = { ...config };
    return;
  }
  if (
    ethersFhevmRuntimeConfig.logger !== config.logger ||
    ethersFhevmRuntimeConfig.locateFile !== config.locateFile ||
    ethersFhevmRuntimeConfig.singleThread !== config.singleThread ||
    ethersFhevmRuntimeConfig.numberOfThreads !== config.numberOfThreads
  ) {
    throw new Error(
      'FhevmRuntime config has already been set and cannot be changed. ' +
        'Ensure setFhevmRuntimeConfig is called only once, or with identical parameters.',
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function getEthersRuntime(): FhevmRuntime {
  if (ethersFhevmRuntimeConfig === undefined) {
    throw new Error('Call setFhevmRuntimeConfig first.');
  }

  const em = ethereumModule();
  const rm = relayerModule();

  ethersFhevmRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    relayer: rm.relayer,
    config: ethersFhevmRuntimeConfig,
  });
  return ethersFhevmRuntime;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Seals an ethers `ContractRunner` into an opaque {@link TrustedClient}.
 *
 * The returned value can be passed through the core layer without exposing
 * the underlying ethers instance. Only the ethers adapter can later recover
 * the original runner via {@link trustedClientToEthersContractRunner}.
 *
 * @param runner - The ethers contract runner to seal.
 * @returns An opaque {@link TrustedClient} bound to the ethers origin token.
 */
export function ethersContractRunnerToTrustedClient<client extends EthersT.ContractRunner>(
  runner: client,
): TrustedClient<client> {
  return createTrustedClient(runner, PRIVATE_ETHERS_TOKEN);
}

/**
 * Verifies that the {@link TrustedClient} was created by the ethers adapter
 * and extracts the original `ContractRunner`.
 *
 * @param trustedClient - The host client to verify.
 * @returns The original ethers `ContractRunner`.
 * @throws {Error} If the client was not created by {@link ethersContractRunnerToTrustedClient}.
 */
export function trustedClientToEthersContractRunner<client extends EthersT.ContractRunner>(
  trustedClient: TrustedClient<client>,
): client {
  return verifyTrustedValue(trustedClient, PRIVATE_ETHERS_TOKEN);
}

////////////////////////////////////////////////////////////////////////////////

export function createFhevmRuntime(parameters: CreateFhevmRuntimeParameters): FhevmRuntime {
  return createFhevmRuntime_(PRIVATE_ETHERS_TOKEN, parameters);
}
