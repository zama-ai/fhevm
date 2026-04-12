import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { KmsDelegatedUserDecryptEIP712 } from '../../types/kms.js';
import { createKmsDelegatedUserDecryptEIP712 as createKmsDelegatedUserDecryptEIP712_ } from '../../kms/createKmsDelegatedUserDecryptEIP712.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsDelegatedUserDecryptEIP712Parameters = {
  readonly publicKey: string | Uint8Array;
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly extraData: string;
  readonly delegatorAddress: string;
};

export type CreateKmsDelegatedUserDecryptEIP712ReturnType =
  KmsDelegatedUserDecryptEIP712;

export function createKmsDelegatedUserDecryptEIP712(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: CreateKmsDelegatedUserDecryptEIP712Parameters,
): CreateKmsDelegatedUserDecryptEIP712ReturnType {
  return createKmsDelegatedUserDecryptEIP712_({
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts
      .decryption.address as ChecksummedAddress,
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
