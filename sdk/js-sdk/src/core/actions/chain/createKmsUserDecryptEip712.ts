import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { KmsUserDecryptEip712 } from '../../types/kms.js';
import { createKmsUserDecryptEip712 as createKmsUserDecryptEip712_ } from '../../kms/createKmsUserDecryptEip712.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsUserDecryptEip712Parameters = {
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
};

export type CreateKmsUserDecryptEip712ReturnType = KmsUserDecryptEip712;

export function createKmsUserDecryptEip712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateKmsUserDecryptEip712Parameters,
): CreateKmsUserDecryptEip712ReturnType {
  return createKmsUserDecryptEip712_({
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts.decryption.address as ChecksummedAddress,
    chainId: fhevm.chain.id,
    contractAddresses: parameters.contractAddresses,
    durationDays: parameters.durationDays,
    startTimestamp: parameters.startTimestamp,
    extraData: parameters.extraData,
    publicKey: parameters.publicKey,
  });
}

////////////////////////////////////////////////////////////////////////////////
