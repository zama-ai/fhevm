import type { FhevmErrorBaseParams } from "./FhevmErrorBase.js";
import type { Prettify } from "../types/utils.js";
import { FhevmErrorBase } from "./FhevmErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// TFHEError
////////////////////////////////////////////////////////////////////////////////

export type TFHEErrorType = TFHEError & {
  name: "TFHEError";
};

export type TFHEErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, "name" | "message"> & {
    readonly message: string;
  }
>;

export class TFHEError extends FhevmErrorBase {
  constructor(params: TFHEErrorParams) {
    super({
      ...params,
      name: "TFHEError",
    });
  }
}
