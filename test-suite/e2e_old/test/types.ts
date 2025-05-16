import type { HTTPZInstance } from "@httpz/sdk/node";

import type { Signers } from "./signers";

declare module "mocha" {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    httpz: HTTPZInstance;
    contract: any;
  }
}

export interface FhevmInstances {
  alice: HTTPZInstance;
  bob: HTTPZInstance;
  carol: HTTPZInstance;
  dave: HTTPZInstance;
  eve: HTTPZInstance;
}
