import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { KmsDelegatedUserDecryptEip712 } from '../../types/kms.js';
import { createKmsDelegatedUserDecryptEip712 as createKmsDelegatedUserDecryptEip712_ } from '../../kms/createKmsDelegatedUserDecryptEip712.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsDelegatedUserDecryptEip712Parameters = {
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
  readonly delegatorAddress: string;
};

export type CreateKmsDelegatedUserDecryptEip712ReturnType = KmsDelegatedUserDecryptEip712;

export function createKmsDelegatedUserDecryptEip712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateKmsDelegatedUserDecryptEip712Parameters,
): CreateKmsDelegatedUserDecryptEip712ReturnType {
  return createKmsDelegatedUserDecryptEip712_({
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts.decryption.address as ChecksummedAddress,
    chainId: fhevm.chain.id,
    contractAddresses: parameters.contractAddresses,
    durationDays: parameters.durationDays,
    startTimestamp: parameters.startTimestamp,
    extraData: parameters.extraData,
    publicKey: parameters.publicKey,
    delegatorAddress: parameters.delegatorAddress,
  });
}

////////////////////////////////////////////////////////////////////////////////
