import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { simpleDeepFreeze } from '../base/object.js';

/**
 * High byte of the eight-byte chain-id field. `0x00` is EVM, `0x01` is Solana.
 */
export const EVM_CHAIN_TYPE = 0x00n;
export const SOLANA_CHAIN_TYPE = 0x01n;
const CHAIN_TYPE_SHIFT = 56n;
const CLUSTER_TAG_MASK = 0x00ffffffffffffffn;
const U64_MAX = (1n << 64n) - 1n;

export function chainTypeByte(chainId: bigint): bigint {
  return (chainId >> CHAIN_TYPE_SHIFT) & 0xffn;
}

export function isEvmHostChainId(chainId: bigint | number): boolean {
  return chainTypeByte(BigInt(chainId)) === EVM_CHAIN_TYPE;
}

export function isSolanaHostChainId(chainId: bigint | number): boolean {
  return chainTypeByte(BigInt(chainId)) === SOLANA_CHAIN_TYPE;
}

export function solanaHostChainId(clusterTag: bigint | number): bigint {
  return (SOLANA_CHAIN_TYPE << CHAIN_TYPE_SHIFT) | (BigInt(clusterTag) & CLUSTER_TAG_MASK);
}

export function assertValidSolanaChainId(chainId: bigint): void {
  if (typeof chainId !== 'bigint' || chainId > U64_MAX || !isSolanaHostChainId(chainId)) {
    throw new Error('Solana chain id must be a u64 bigint with type byte 0x01');
  }
}

export function defineFhevmSolanaChain<const chain extends FhevmSolanaChain>(fhevmSolanaChain: chain): chain {
  assertValidSolanaChainId(fhevmSolanaChain.id);
  return simpleDeepFreeze(fhevmSolanaChain);
}
