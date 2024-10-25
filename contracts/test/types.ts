import type { FhevmInstance } from 'fhevmjs';

import { BlindAuction, Comp, EncryptedERC20, GovernorZama, IdentifiedERC20, IdentityRegistry, Rand } from '../types';
import type { Signers } from './signers';

declare module 'mocha' {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
    erc20: EncryptedERC20;
    blindAuction: BlindAuction;
    rand: Rand;
    identityRegistry: IdentityRegistry;
    identifiedErc20: IdentifiedERC20;
    comp: Comp;
    governor: GovernorZama;
  }
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
  eve: FhevmInstance;
}
