import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { asFhevmWithTfheVersion } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type SupportsSeededEncryptionReturnType = boolean;

////////////////////////////////////////////////////////////////////////////////

export async function supportsSeededEncryption(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
): Promise<SupportsSeededEncryptionReturnType> {
  const f = asFhevmWithTfheVersion(fhevm);

  const ok = await f.runtime.encrypt.canBuildWithProofPackedSeeded({ tfheVersion: f.tfheVersion });

  return ok;
}
