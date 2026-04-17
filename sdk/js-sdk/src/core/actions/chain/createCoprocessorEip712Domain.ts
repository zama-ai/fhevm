import type { CoprocessorEip712Domain } from '../../types/coprocessor.js';
import type { Fhevm, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import { createCoprocessorEip712Domain as createCoprocessorEip712Domain_ } from '../../coprocessor/createCoprocessorEip712Domain.js';

export type CreateCoprocessorEip712DomainReturnType = CoprocessorEip712Domain;

export function createCoprocessorEip712Domain(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
): CreateCoprocessorEip712DomainReturnType {
  return createCoprocessorEip712Domain_({
    gatewayChainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification: fhevm.chain.fhevm.gateway.contracts.inputVerification.address,
  });
}
