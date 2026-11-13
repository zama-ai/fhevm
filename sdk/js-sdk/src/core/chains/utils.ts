import type { FhevmChain } from '../types/fhevmChain.js';
import { simpleDeepFreeze } from '../base/object.js';
import { isAddress } from '../base/address.js';

const REQUIRED_CONTRACTS = ['acl', 'inputVerifier', 'kmsVerifier', 'protocolConfig'] as const;

/**
 * Runtime guard for chains built from untyped input (JS, JSON config): checks
 * that every required host contract is configured with an address, so a
 * missing field fails with a descriptive error instead of a `TypeError`.
 *
 * @internal
 */
export function assertFhevmChainContracts(fhevmChain: FhevmChain): void {
  const contracts = (fhevmChain as { fhevm?: { contracts?: Record<string, { address?: unknown } | undefined> } }).fhevm
    ?.contracts;
  for (const name of REQUIRED_CONTRACTS) {
    const address = contracts?.[name]?.address;
    if (!isAddress(address)) {
      throw new Error(
        `Invalid FHEVM chain (id: ${String(fhevmChain.id)}): missing or invalid required 'fhevm.contracts.${name}.address'.`,
      );
    }
  }
}

export function defineFhevmChain<const chain extends FhevmChain>(fhevmChain: chain): chain {
  assertFhevmChainContracts(fhevmChain);
  return simpleDeepFreeze(fhevmChain);
}
