import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  type EncryptValuesWithZkProofParameters,
  type EncryptValuesWithZkProofReturnType,
  encryptValuesWithZkProof,
} from './encryptValuesWithZkProof.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValuesParameters = EncryptValuesWithZkProofParameters;

export type EncryptValuesReturnType = EncryptValuesWithZkProofReturnType;

////////////////////////////////////////////////////////////////////////////////

/** Encrypts `values` with a ZK proof of knowledge: {@link encryptValuesWithZkProof}. */
export async function encryptValues(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptValuesParameters,
): Promise<EncryptValuesReturnType> {
  return encryptValuesWithZkProof(fhevm, parameters);
}
