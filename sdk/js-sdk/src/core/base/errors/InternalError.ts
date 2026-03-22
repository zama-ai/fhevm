import { ErrorBase } from "./ErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// InternalError
////////////////////////////////////////////////////////////////////////////////

export type InternalErrorType = InternalError & {
  name: "InternalError";
};

export type InternalErrorParams = Readonly<{
  message?: string;
}>;

////////////////////////////////////////////////////////////////////////////////

export class InternalError extends ErrorBase {
  constructor(params: InternalErrorParams) {
    super({
      ...params,
      name: "InternalError",
      message: params.message ?? "internal error",
    });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assert(
  condition: boolean,
  message?: string,
): asserts condition {
  if (!condition) {
    throw new InternalError({ message: message ?? "Assertion failed" });
  }
}
