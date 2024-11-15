import type { FhevmInstance } from "fhevmjs";

import type { Signers } from "./signers";

declare module "mocha" {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
  }
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
  eve: FhevmInstance;
}
