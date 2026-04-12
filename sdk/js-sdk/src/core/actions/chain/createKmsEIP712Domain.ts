import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { createKmsEIP712Domain as createKmsEIP712Domain_ } from '../../kms/createKmsEIP712Domain.js';
import type { KmsEIP712Domain } from '../../types/kms.js';

export type CreateKmsEIP712DomainReturnType = KmsEIP712Domain;

export function createKmsEIP712Domain(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
): CreateKmsEIP712DomainReturnType {
  return createKmsEIP712Domain_({
    chainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption:
      fhevm.chain.fhevm.gateway.contracts.decryption.address,
  });
}
