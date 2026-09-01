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
  const kms = fhevmSolanaChain.fhevm.kms;
  // Response verification is fail-closed: an empty signer set authenticates nothing, so a chain
  // declaring `kms` with no signers could never decrypt. Refuse it here, at definition time.
  if (kms !== undefined) {
    if (kms.signers.length === 0) {
      throw new Error('fhevm.kms.signers must name at least one registered KMS signer address');
    }
    if (kms.verifyingProgramId.length === 0) {
      throw new Error('fhevm.kms.verifyingProgramId must be the base58 host program address');
    }
  }
  return simpleDeepFreeze(fhevmSolanaChain);
}
