import type { InputVerifierContractData } from './coprocessor.js';
import type { KmsVerifierContractData } from './kms.js';
import type { ChecksummedAddress, Uint32BigInt, Uint64BigInt } from './primitives.js';

////////////////////////////////////////////////////////////////////////////////
//
// Fhevm Configs
//
////////////////////////////////////////////////////////////////////////////////

export interface FhevmHostChainConfig {
  readonly chainId: Uint64BigInt;
  readonly aclContractAddress: ChecksummedAddress;
  readonly kmsVerifierContractAddress: ChecksummedAddress;
  readonly inputVerifierContractAddress: ChecksummedAddress;
}

export interface FhevmGatewayChainConfig {
  readonly chainId: Uint32BigInt;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;
  readonly verifyingContractAddressInputVerification: ChecksummedAddress;
}

export interface FhevmConfig {
  readonly hostChainConfig: FhevmHostChainConfig;
  readonly gatewayChainConfig: FhevmGatewayChainConfig;
  readonly inputVerifier: InputVerifierContractData;
  readonly kmsVerifier: KmsVerifierContractData;
}
