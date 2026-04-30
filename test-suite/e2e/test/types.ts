import type { FhevmInstance } from "@zama-fhe/relayer-sdk/node";

import { EncryptedERC20, Rand } from "../types";
import { SdkInstance } from "./sdk/types";
import type { Signers } from "./signers";

declare module "mocha" {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
    erc20: EncryptedERC20;
    rand: Rand;
  }
}

export interface FhevmInstances {
  alice: SdkInstance;
  bob: SdkInstance;
  carol: SdkInstance;
  dave: SdkInstance;
  eve: SdkInstance;
}
