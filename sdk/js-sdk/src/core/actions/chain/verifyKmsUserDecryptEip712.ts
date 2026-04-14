import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  verifyKmsUserDecryptEip712 as verifyKmsUserDecryptEIP712_,
  type VerifyKmsUserDecryptEip712Parameters,
} from '../../utils-p/decrypt/verifyKmsUserDecryptEip712.js';

export type { VerifyKmsUserDecryptEip712Parameters };

export async function verifyKmsUserDecryptEip712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: VerifyKmsUserDecryptEip712Parameters,
): Promise<void> {
  await verifyKmsUserDecryptEIP712_(fhevm, parameters);
}
