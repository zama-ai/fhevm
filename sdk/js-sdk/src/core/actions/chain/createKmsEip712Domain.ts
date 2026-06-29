import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { KmsEip712Domain } from '../../types/kms.js';
import { createKmsEip712Domain as createKmsEip712Domain_ } from '../../kms/createKmsEip712Domain.js';

export type CreateKmsEip812DomainReturnType = KmsEip712Domain;

export function createKmsEip712Domain(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
): CreateKmsEip812DomainReturnType {
  return createKmsEip712Domain_({
    chainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption: fhevm.chain.fhevm.gateway.contracts.decryption.address,
  });
}
