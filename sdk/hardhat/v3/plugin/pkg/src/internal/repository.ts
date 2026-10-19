// The contracts repository of a connection: the stack it deployed, wrapped by ABI for revert decoding
// and reads. Development connections only, until public networks settle which network group serves
// a chain listed under two.

import type { Deployed } from '@fhevm/host-contracts-cleartext/ts';
import type { NetworkConnection } from 'hardhat/types/network';

import { developmentChain, developmentPublicClient } from './clients.js';
import { FhevmCleartextContractsRepository, type FhevmContractsRepository } from './contracts.js';

export async function createRepository(
  connection: NetworkConnection<string>,
  stack: Deployed | undefined,
): Promise<FhevmContractsRepository | undefined> {
  if (stack === undefined) return undefined;
  const chain = await developmentChain(connection.provider);
  const client = developmentPublicClient(connection.provider, chain);
  const { fhevmAddresses, cleartextAddresses } = stack;
  return new FhevmCleartextContractsRepository(client, {
    aclAddress: fhevmAddresses.aclAddress,
    fhevmExecutorAddress: fhevmAddresses.fhevmExecutorAddress,
    inputVerifierAddress: fhevmAddresses.inputVerifierAddress,
    kmsVerifierAddress: fhevmAddresses.kmsVerifierAddress,
    hcuLimitAddress: fhevmAddresses.hcuLimitAddress,
    protocolConfigAddress: fhevmAddresses.protocolConfigAddress,
    kmsGenerationAddress: fhevmAddresses.kmsGenerationAddress,
    pauserSetAddress: stack.pauserSetAddress,
    cleartextArithmeticAddress: cleartextAddresses.cleartextArithmeticAddress,
    cleartextDbAddress: cleartextAddresses.cleartextDbAddress,
  });
}
