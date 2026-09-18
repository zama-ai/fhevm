// @fhevm/sdk chain definitions. Public chains come from the SDK itself (it is the source of truth for
// them); the local cleartext chain is the one the SDK does not ship, so it is built here from the
// stack the connection actually deployed plus the gateway the cleartext package bootstraps against.

import {
  CLEARTEXT_DECRYPTION_ADDRESS,
  CLEARTEXT_GATEWAY_CHAIN_ID,
  CLEARTEXT_INPUT_VERIFICATION_ADDRESS,
  type Deployed,
} from '@fhevm/host-contracts-cleartext/ts';
import { type FhevmChain, defineFhevmChain } from '@fhevm/sdk/chains';
import { type Address, getAddress } from 'viem';

function contract(address: string): { address: Address } {
  return { address: getAddress(address) };
}

/** The cleartext chain a development connection runs on. `relayerUrl` is required by the type and unused in cleartext mode. */
export function cleartextChain(chainId: number, deployed: Deployed): FhevmChain {
  const { fhevmAddresses } = deployed;
  return defineFhevmChain({
    id: chainId,
    fhevm: {
      contracts: {
        acl: contract(fhevmAddresses.aclAddress),
        inputVerifier: contract(fhevmAddresses.inputVerifierAddress),
        kmsVerifier: contract(fhevmAddresses.kmsVerifierAddress),
        protocolConfig: contract(fhevmAddresses.protocolConfigAddress),
      },
      relayerUrl: 'http://localhost:8545',
      gateway: {
        id: Number(CLEARTEXT_GATEWAY_CHAIN_ID),
        contracts: {
          decryption: contract(CLEARTEXT_DECRYPTION_ADDRESS),
          inputVerification: contract(CLEARTEXT_INPUT_VERIFICATION_ADDRESS),
        },
      },
    },
  });
}
