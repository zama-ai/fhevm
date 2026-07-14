import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { simpleDeepFreeze } from '../base/object.js';

const SOLANA_CHAIN_TYPE_BIT = 1n << 63n;
const U64_MAX = (1n << 64n) - 1n;

export function assertValidSolanaChainId(chainId: bigint): void {
  if (typeof chainId !== 'bigint' || chainId < SOLANA_CHAIN_TYPE_BIT || chainId > U64_MAX) {
    throw new Error('Solana chain id must be a u64 bigint with bit 63 set');
  }
}

export function defineFhevmSolanaChain<const chain extends FhevmSolanaChain>(fhevmSolanaChain: chain): chain {
  assertValidSolanaChainId(fhevmSolanaChain.id);
  return simpleDeepFreeze(fhevmSolanaChain);
}
