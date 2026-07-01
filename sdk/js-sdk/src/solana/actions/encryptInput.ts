import type { Fhevm, WithTfheVersion } from '../../core/types/coreFhevmClient.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import { createTypedValue } from '../../core/base/typedValue.js';
import { createZkProofBuilder } from '../../core/coprocessor/ZkProofBuilder-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SolanaEncryptInputValue = {
  readonly type: string;
  readonly value: boolean | bigint | number | string;
};

export type SolanaEncryptInputParameters = {
  /** The bound contract identity, a 32-byte (bytes32) Solana host identity (RFC-021). */
  readonly contractAddress: string;
  /** The bound user identity, a 32-byte (bytes32) Solana host identity (RFC-021). */
  readonly userAddress: string;
  readonly values: readonly SolanaEncryptInputValue[];
};

export type SolanaEncryptInputResult = SolanaZkProof;

////////////////////////////////////////////////////////////////////////////////

/**
 * Builds a Solana input ZK proof — the public counterpart of the EVM `generateZkProof`.
 * It packs the values, fetches the FHE encryption key, and produces a {@link SolanaZkProof}
 * (RFC-021 bytes32 host identities + 128-byte aux). Submitting the proof to the relayer and
 * reading back the verified handle is the caller's job.
 */
export async function encryptInput(
  fhevm: Fhevm<FhevmChain, WithEncrypt> & WithTfheVersion,
  parameters: SolanaEncryptInputParameters,
): Promise<SolanaEncryptInputResult> {
  const { values, contractAddress, userAddress } = parameters;
  const builder = createZkProofBuilder();
  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }
  const context = { chain: fhevm.chain, runtime: fhevm.runtime, tfheVersion: fhevm.tfheVersion };
  return builder.buildSolana(context, { contractAddress, userAddress });
}
