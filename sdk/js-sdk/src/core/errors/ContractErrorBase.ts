import type { Prettify } from "../types/utils.js";
import type { Address } from "../types/primitives.js";
import type { FhevmErrorBaseParams } from "./FhevmErrorBase.js";
import { FhevmErrorBase } from "./FhevmErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// ContractErrorBase
////////////////////////////////////////////////////////////////////////////////

export type ContractErrorBaseType = ContractErrorBase & {
  name: "ContractErrorBase";
};

export type ContractErrorBaseParams = Prettify<
  FhevmErrorBaseParams & {
    contractAddress: Address;
    contractName: string;
  }
>;

export abstract class ContractErrorBase extends FhevmErrorBase {
  readonly #contractAddress: Address;
  readonly #contractName: string;

  constructor(params: ContractErrorBaseParams) {
    super(params);
    this.#contractAddress = params.contractAddress;
    this.#contractName = params.contractName;
  }

  public get contractAddress(): Address {
    return this.#contractAddress;
  }

  public get contractName(): string {
    return this.#contractName;
  }
}
