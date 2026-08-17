// Derives the FHEVM host-contract addresses a cleartext deploy produces, purely
// from (deployer mnemonic, account index). The deploy creates each contract via
// CREATE from the deployer at a fixed nonce (see the derivation comment in
// contracts/scripts/v*/DeployCleartextFHEVMHost.s.sol), so address = f(deployer,
// nonce). This lets the gateway serve each slot's addresses without parsing the
// deploy's generated Solidity (which we restore post-deploy) — the single source
// of truth is the same (mnemonic, index) the deploy itself uses.
//
// Verified: deriveFhevmHostAddresses(DEFAULT_DEPLOYER_MNEMONIC, 5).acl ===
// 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D (matches the committed file).

import { getContractAddress } from 'viem';
import { mnemonicToAccount } from 'viem/accounts';
import { DEFAULT_DEPLOYER_INDEX } from '../config.js';

// CREATE nonce of each host contract, fixed by the deploy's creation order.
const HOST_NONCE = {
  acl: 1,
  fhevmExecutor: 3,
  kmsVerifier: 4,
  inputVerifier: 5,
  hcuLimit: 6,
  protocolConfig: 7,
  kmsGeneration: 8,
  pauserSet: 9,
} as const;

export type FhevmHostAddresses = {
  readonly acl: string;
  readonly inputVerifier: string;
  readonly kmsVerifier: string;
  readonly protocolConfig: string;
};

/** Host-stack addresses for a deployer (mnemonic, index). */
export function deriveFhevmHostAddresses(mnemonic: string, index: number = DEFAULT_DEPLOYER_INDEX): FhevmHostAddresses {
  const from = mnemonicToAccount(mnemonic, { addressIndex: index }).address;
  const at = (nonce: number): string => getContractAddress({ from, nonce: BigInt(nonce) });
  return {
    acl: at(HOST_NONCE.acl),
    inputVerifier: at(HOST_NONCE.inputVerifier),
    kmsVerifier: at(HOST_NONCE.kmsVerifier),
    protocolConfig: at(HOST_NONCE.protocolConfig),
  };
}
