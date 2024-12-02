import type { BaseContract } from "ethers";
import type { FhevmInstance } from "fhevmjs/node";

import type { Signers } from "./signers";

declare module "mocha" {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    fhevm: FhevmInstance;
    contract: any;
  }
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
  eve: FhevmInstance;
}
