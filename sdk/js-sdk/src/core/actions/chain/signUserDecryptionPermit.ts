import { assertIsChecksummedAddress } from "../../base/address.js";
import {
  createKmsUserDecryptEIP712,
  type CreateKmsUserDecryptEIP712Parameters,
} from "../../kms/createKmsUserDecryptEIP712.js";
import { createFhevmUserDecryptionPermit } from "../../kms/FhevmUserDecryptionPermit-p.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { FhevmUserDecryptionPermit } from "../../types/fhevmUserDecryptionPermit.js";
import type { Bytes65Hex } from "../../types/primitives.js";
import type { Prettify } from "../../types/utils.js";
import { verifyKmsUserDecryptEIP712 } from "./verifyKmsUserDecryptEIP712.js";

export type SignUserDecryptionPermitParameters = Prettify<
  {
    readonly signerAddress: string;
  } & CreateKmsUserDecryptEIP712Parameters
>;

export type SignUserDecryptionPermitReturnType = FhevmUserDecryptionPermit;

export type WalletSigner = {
  signTypedData: (eip712: Record<string, unknown>) => Promise<Bytes65Hex>;
};

export async function signUserDecryptionPermit(
  signer: WalletSigner,
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: SignUserDecryptionPermitParameters,
): Promise<SignUserDecryptionPermitReturnType> {
  assertIsChecksummedAddress(parameters.signerAddress, {});

  const eip712 = createKmsUserDecryptEIP712(parameters);

  const signature = await signer.signTypedData(eip712);

  await verifyKmsUserDecryptEIP712(fhevm, {
    signer: parameters.signerAddress,
    message: eip712.message,
    signature,
  });

  return createFhevmUserDecryptionPermit({
    eip712,
    signature,
    signerAddress: parameters.signerAddress,
  });
}
