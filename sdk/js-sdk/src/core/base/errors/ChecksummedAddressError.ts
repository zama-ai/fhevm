import type { ErrorMetadataParams } from "./ErrorBase.js";
import { ErrorBase } from "./ErrorBase.js";

export type ChecksummedAddressErrorType = ChecksummedAddressError & {
  name: "ChecksummedAddressError";
};

export type ChecksummedAddressErrorParams = Readonly<{
  address?: string;
}>;

export class ChecksummedAddressError extends ErrorBase {
  constructor(
    params: ChecksummedAddressErrorParams,
    options: ErrorMetadataParams,
  ) {
    super({
      ...options,
      message:
        params.address !== undefined
          ? `Checksummed address "${params.address}" is invalid.`
          : "Checksummed address is invalid.",
      name: "ChecksummedAddressError",
    });
  }
}
