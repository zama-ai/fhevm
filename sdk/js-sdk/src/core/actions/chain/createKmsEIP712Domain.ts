import { createKmsEIP712Domain as createKmsEIP712Domain_ } from "../../kms/createKmsEIP712Domain.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { KmsEIP712Domain } from "../../types/kms.js";

export type CreateKmsEIP712DomainReturnType = KmsEIP712Domain;

export function createKmsEIP712Domain(fhevm: {
  readonly chain: FhevmChain;
}): KmsEIP712Domain {
  return createKmsEIP712Domain_({
    chainId: fhevm.chain.id,
    verifyingContractAddressDecryption:
      fhevm.chain.fhevm.gateway.contracts.decryption.address,
  });
}
