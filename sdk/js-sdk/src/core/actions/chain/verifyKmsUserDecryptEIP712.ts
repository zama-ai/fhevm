import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  verifyKmsUserDecryptEIP712 as verifyKmsUserDecryptEIP712_,
  type VerifyKmsUserDecryptEIP712Parameters,
} from '../../utils-p/decrypt/verifyKmsUserDecryptEIP712.js';

export type { VerifyKmsUserDecryptEIP712Parameters };

export async function verifyKmsUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: VerifyKmsUserDecryptEIP712Parameters,
): Promise<void> {
  await verifyKmsUserDecryptEIP712_(fhevm, parameters);
}
