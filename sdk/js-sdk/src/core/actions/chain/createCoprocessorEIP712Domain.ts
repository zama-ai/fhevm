import { createCoprocessorEIP712Domain as createCoprocessorEIP712Domain_ } from '../../coprocessor/createCoprocessorEIP712Domain.js';
import type { CoprocessorEIP712Domain } from '../../types/coprocessor.js';
import type {
  Fhevm,
  OptionalNativeClient,
} from '../../types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';

export type CreateCoprocessorEIP712DomainReturnType = CoprocessorEIP712Domain;

export function createCoprocessorEIP712Domain(
  fhevm: Fhevm<FhevmChain, FhevmRuntime, OptionalNativeClient>,
): CreateCoprocessorEIP712DomainReturnType {
  return createCoprocessorEIP712Domain_({
    gatewayChainId: fhevm.chain.fhevm.gateway.id,
    verifyingContractAddressInputVerification:
      fhevm.chain.fhevm.gateway.contracts.inputVerification.address,
  });
}
