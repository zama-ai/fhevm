import type {
  Fhevm,
  OptionalNativeClient,
} from "../../../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";
import type { ChecksummedAddress } from "../../../types/primitives.js";
import type { KmsUserDecryptEIP712 } from "../../../types/kms.js";
import { createKmsUserDecryptEIP712 } from "../../../kms/createKmsUserDecryptEIP712.js";

////////////////////////////////////////////////////////////////////////////////

export type CreateUserDecryptEIP712Parameters = {
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
};
export type CreateUserDecryptEIP712ReturnType = KmsUserDecryptEIP712;

export function createUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateUserDecryptEIP712Parameters,
): CreateUserDecryptEIP712ReturnType {
  return createKmsUserDecryptEIP712({
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts
      .decryption.address as ChecksummedAddress,
    chainId: fhevm.chain.id,
    contractAddresses: parameters.contractAddresses,
    durationDays: parameters.durationDays,
    startTimestamp: parameters.startTimestamp,
    extraData: parameters.extraData,
    publicKey: parameters.publicKey,
  });
}

////////////////////////////////////////////////////////////////////////////////
