import type { PublicClient } from 'viem';
import type { TrustedClient } from '../../core/modules/ethereum/types.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { CreateFhevmRuntimeParameters } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { verifyTrustedValue } from '../../core/base/trustedValue.js';
import { createFhevmRuntime as createFhevmRuntime_ } from '../../core/runtime/CoreFhevmRuntime-p.js';
import { createTrustedClient } from '../../core/modules/ethereum/createTrustedClient.js';

////////////////////////////////////////////////////////////////////////////////

// Will leak in js
export const PRIVATE_VIEM_TOKEN = Symbol('viem.token');

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
