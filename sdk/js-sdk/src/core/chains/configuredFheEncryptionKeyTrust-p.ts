import type { FhevmChain } from '../types/fhevmChain.js';
import type { ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { addressToChecksummedAddress, asAddress } from '../base/address.js';
import { asUint64BigInt } from '../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

/** Declarative host-chain KMSGeneration anchor from the selected chain config. */
export type ConfiguredFheEncryptionKeyTrust = {
  readonly chainId: Uint64BigInt;
  readonly kmsGenerationAddress: ChecksummedAddress;
};

/**
 * Reads the optional KMSGeneration trust anchor from the exact chain config the
 * caller selected. Custom chains are authenticated when they configure the
 * address; chains without it intentionally fall back to diagnostic-only warning.
 */
export function getConfiguredFheEncryptionKeyTrust(
  chain: FhevmChain | undefined,
): ConfiguredFheEncryptionKeyTrust | undefined {
  if (chain === undefined) {
    return undefined;
  }

  const kmsGeneration = chain.fhevm.contracts.kmsGeneration;
  if (kmsGeneration === undefined) {
    return undefined;
  }

  return Object.freeze({
    chainId: asUint64BigInt(BigInt(chain.id), { subject: 'FHE encryption-key trust chain ID' }),
    kmsGenerationAddress: addressToChecksummedAddress(asAddress(kmsGeneration.address)),
  });
}
