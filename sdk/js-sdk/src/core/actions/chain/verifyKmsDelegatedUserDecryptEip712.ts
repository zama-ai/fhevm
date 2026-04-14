import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  verifyKmsDelegatedUserDecryptEip712 as verifyKmsDelegatedUserDecryptEip712_,
  type VerifyKmsDelegatedUserDecryptEip712Parameters,
} from '../../utils-p/decrypt/verifyKmsDelegatedUserDecryptEip712.js';

export type { VerifyKmsDelegatedUserDecryptEip712Parameters as VerifyKmsDelegatedUserDecryptEIP712Parameters };

export async function verifyKmsDelegatedUserDecryptEip712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: VerifyKmsDelegatedUserDecryptEip712Parameters,
): Promise<void> {
  await verifyKmsDelegatedUserDecryptEip712_(fhevm, parameters);
}
