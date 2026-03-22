import type {
  Fhevm,
  OptionalNativeClient,
} from "../../../types/coreFhevmClient.js";
import type { FhevmRuntime } from "../../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../../types/fhevmChain.js";
import type { ChecksummedAddress } from "../../../types/primitives.js";
import type { KmsDelegatedUserDecryptEIP712 } from "../../../types/kms.js";
import { createKmsDelegatedUserDecryptEIP712 } from "../../../kms/createKmsDelegatedUserDecryptEIP712.js";

////////////////////////////////////////////////////////////////////////////////

export type CreateDelegatedUserDecryptEIP712Parameters = {
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
  readonly delegatedAccount: string;
};
export type CreateDelegatedUserDecryptEIP712ReturnType =
  KmsDelegatedUserDecryptEIP712;

export function createDelegatedUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateDelegatedUserDecryptEIP712Parameters,
): CreateDelegatedUserDecryptEIP712ReturnType {
  return createKmsDelegatedUserDecryptEIP712({
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts
      .decryption.address as ChecksummedAddress,
    chainId: fhevm.chain.id,
    contractAddresses: parameters.contractAddresses,
    durationDays: parameters.durationDays,
    startTimestamp: parameters.startTimestamp,
    extraData: parameters.extraData,
    publicKey: parameters.publicKey,
    delegatedAccount: parameters.delegatedAccount,
  });
}

////////////////////////////////////////////////////////////////////////////////
