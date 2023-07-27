import type { EncryptedERC20 } from "../types/contracts/EncryptedERC20";
import type { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/dist/src/signer-with-address";
import type { FhevmInstance } from "fhevmjs";

declare module "mocha" {
  export interface Context {
    signers: Signers;
    contractAddress: string;
    instances: FhevmInstances;
    erc20: EncryptedERC20;
  }
}

export interface Signers {
  alice: SignerWithAddress;
  bob: SignerWithAddress;
  carol: SignerWithAddress;
  dave: SignerWithAddress;
}

export interface FhevmInstances {
  alice: FhevmInstance;
  bob: FhevmInstance;
  carol: FhevmInstance;
  dave: FhevmInstance;
}
