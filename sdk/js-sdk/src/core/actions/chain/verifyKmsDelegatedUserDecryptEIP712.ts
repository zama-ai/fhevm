import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import {
  verifyKmsDelegatedUserDecryptEIP712 as verifyKmsDelegatedUserDecryptEIP712_,
  type VerifyKmsDelegatedUserDecryptEIP712Parameters,
} from '../../utils-p/decrypt/verifyKmsDelegatedUserDecryptEIP712.js';

export type { VerifyKmsDelegatedUserDecryptEIP712Parameters };

export async function verifyKmsDelegatedUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: VerifyKmsDelegatedUserDecryptEIP712Parameters,
): Promise<void> {
  await verifyKmsDelegatedUserDecryptEIP712_(fhevm, parameters);
}
