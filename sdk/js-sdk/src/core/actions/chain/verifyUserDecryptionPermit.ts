import { verifyKmsUserDecryptEIP712 } from "./verifyKmsUserDecryptEIP712.js";
import { isVerifiedUserDecryptionPermit } from "../../kms/FhevmUserDecryptionPermit-p.js";
import type { FhevmUserDecryptionPermit } from "../../types/fhevmUserDecryptionPermit.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";

export type VerifyUserDecryptionPermitParameters = {
  readonly permit: FhevmUserDecryptionPermit;
};

export async function verifyUserDecryptionPermit(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: VerifyUserDecryptionPermitParameters,
): Promise<void> {
  const { permit } = parameters;

  if (isVerifiedUserDecryptionPermit(permit)) {
    return;
  }

  await verifyKmsUserDecryptEIP712(fhevm, {
    signer: permit.signerAddress,
    message: permit.eip712.message,
    signature: permit.signature,
  });
}
