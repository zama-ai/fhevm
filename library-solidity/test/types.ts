import type { EncryptedERC20, Rand } from '../typechain-types/index.js';
import type { FhevmInstance } from './legacyRelayerSdk/types.js';
import type { Signers } from './signers.js';

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
