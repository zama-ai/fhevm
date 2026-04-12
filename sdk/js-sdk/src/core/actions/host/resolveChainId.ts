import { assertIsUint64, asUint64BigInt } from '../../base/uint.js';
import { getTrustedClient } from '../../runtime/CoreFhevm-p.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { Uint64BigInt } from '../../types/primitives.js';

export type ResolveChainIdParameters = {
  readonly id?: number | bigint | undefined;
  readonly verify?: boolean;
};

export type ResolveChainIdReturnType = Uint64BigInt;

export async function resolveChainId(
  fhevm: Fhevm,
  parameters: ResolveChainIdParameters,
): Promise<ResolveChainIdReturnType> {
  const trustedClient = getTrustedClient(fhevm);
  const { id, verify } = parameters;

  // No id provided → fetch from chain
  if (id === undefined) {
    return asUint64BigInt(
      await fhevm.runtime.ethereum.getChainId(trustedClient),
    );
  }

  assertIsUint64(id, {});
  const resolvedId = asUint64BigInt(BigInt(id));

  // Id provided, no verification requested → return as-is
  // By default, do not verify
  if (verify !== true) {
    return resolvedId;
  }

  // Id provided + verify → cross-check with chain
  const chainId = asUint64BigInt(
    await fhevm.runtime.ethereum.getChainId(trustedClient),
  );

  if (resolvedId !== chainId) {
    throw new Error(
      `Chain id mismatch: connected to chain ${chainId}, but expected chain ${resolvedId}`,
    );
  }

  return resolvedId;
}
