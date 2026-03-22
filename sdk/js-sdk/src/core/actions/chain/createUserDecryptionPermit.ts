import { createFhevmUserDecryptionPermit } from "../../kms/FhevmUserDecryptionPermit-p.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { FhevmUserDecryptionPermit } from "../../types/fhevmUserDecryptionPermit.js";
import type { KmsUserDecryptEIP712 } from "../../types/kms.js";
import type { Bytes65Hex, ChecksummedAddress } from "../../types/primitives.js";
import { verifyKmsUserDecryptEIP712 } from "./verifyKmsUserDecryptEIP712.js";

export type CreateUserDecryptionPermitParameters = {
  readonly signerAddress: ChecksummedAddress;
  readonly eip712: KmsUserDecryptEIP712;
  readonly signature: Bytes65Hex;
};

export type CreateUserDecryptionPermitReturnType = FhevmUserDecryptionPermit;

export async function createUserDecryptionPermit(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateUserDecryptionPermitParameters,
): Promise<CreateUserDecryptionPermitReturnType> {
  const { signerAddress, signature, eip712 } = parameters;
  await verifyKmsUserDecryptEIP712(fhevm, {
    signer: signerAddress,
    message: eip712.message,
    signature,
  });
  return createFhevmUserDecryptionPermit(parameters);
}
