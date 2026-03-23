import type { PublicClient, Chain, Transport } from "viem";
import { verifyTrustedValue } from "../../core/base/trustedValue.js";
import type { TrustedClient } from "../../core/modules/ethereum/types.js";
import {
  createFhevmRuntime as createFhevmRuntime_,
  type CreateFhevmRuntimeParameters,
} from "../../core/runtime/CoreFhevmRuntime-p.js";
import type {
  FhevmRuntime,
  FhevmRuntimeConfig,
} from "../../core/types/coreFhevmRuntime.js";
import { createTrustedClient } from "../../core/modules/ethereum/createTrustedClient.js";
import { ethereumModule } from "./ethereum.js";

////////////////////////////////////////////////////////////////////////////////

// Will leak in js
export const PRIVATE_VIEM_TOKEN = Symbol("viem.token");

////////////////////////////////////////////////////////////////////////////////

let cachedViemRuntime: FhevmRuntime | undefined;
let globalFhevmRuntimeConfig: FhevmRuntimeConfig | undefined;

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
  if (globalFhevmRuntimeConfig === undefined) {
    globalFhevmRuntimeConfig = { ...config };
    return;
  }
  if (
    globalFhevmRuntimeConfig.logger !== config.logger ||
    globalFhevmRuntimeConfig.locateFile !== config.locateFile ||
    globalFhevmRuntimeConfig.singleThread !== config.singleThread ||
    globalFhevmRuntimeConfig.numberOfThreads !== config.numberOfThreads
  ) {
    throw new Error(
      "FhevmRuntime config has already been set and cannot be changed. " +
        "Ensure setFhevmRuntimeConfig is called only once, or with identical parameters.",
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function getViemRuntime(): FhevmRuntime {
  if (globalFhevmRuntimeConfig === undefined) {
    throw new Error("Call setFhevmRuntimeConfig first.");
  }

  const em = ethereumModule();
  cachedViemRuntime ??= createFhevmRuntime({
    ethereum: em.ethereum,
    config: globalFhevmRuntimeConfig,
  });
  return cachedViemRuntime;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Seals a viem `PublicClient` into an opaque {@link TrustedClient}.
 *
 * The returned value can be passed through the core layer without exposing
 * the underlying viem instance. Only the viem adapter can later recover
 * the original client via {@link trustedClientToViemPublicClient}.
 *
 * @param client - The viem public client to seal.
 * @returns An opaque {@link TrustedClient} bound to the viem origin token.
 */
export function viemPublicClientToTrustedClient<
  client extends PublicClient<Transport, Chain>,
>(publicClient: client): TrustedClient<client> {
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
export function trustedClientToViemPublicClient<
  client extends PublicClient<Transport, Chain>,
>(trustedClient: TrustedClient<client>): client {
  return verifyTrustedValue(trustedClient, PRIVATE_VIEM_TOKEN);
}

export function createFhevmRuntime(
  parameters: CreateFhevmRuntimeParameters,
): FhevmRuntime {
  return createFhevmRuntime_(PRIVATE_VIEM_TOKEN, parameters);
}
