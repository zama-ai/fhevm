// TypeScript 6 no longer loads every installed @types package, so load the chai-as-promised
// assertions (`rejected`, `rejectedWith`) that hardhat-chai-matchers registers at runtime.
/// <reference types="chai-as-promised" />
import { EncryptedERC20, Rand } from '../types';
import type { FhevmInstance } from './legacyRelayerSdk/types';
import type { Signers } from './signers';

declare module 'mocha' {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
    erc20: EncryptedERC20;
    rand: Rand;
  }
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
  eve: FhevmInstance;
}
