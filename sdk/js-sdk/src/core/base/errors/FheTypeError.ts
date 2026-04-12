import type { ErrorMetadataParams } from './ErrorBase.js';
import { ErrorBase } from './ErrorBase.js';

////////////////////////////////////////////////////////////////////////////////

export type FheTypeErrorType = FheTypeError & {
  name: 'FheTypeError';
};

export type FheTypeErrorParams = Readonly<{
  message: string;
}>;

////////////////////////////////////////////////////////////////////////////////
// FheTypeError
////////////////////////////////////////////////////////////////////////////////

export class FheTypeError extends ErrorBase {
  constructor(params: FheTypeErrorParams, options: ErrorMetadataParams) {
    super({
      ...options,
      message: params.message,
      name: 'FheTypeError',
    });
  }

  public static throwFheTypeIdError(
    id: unknown,
    options: ErrorMetadataParams,
  ): never {
    let message: string;
    if (id === undefined) {
      message = 'FheTypeId is invalid.';
    } else if (typeof id === 'number') {
      message = `FheTypeId '${id}' is invalid.`;
    } else {
      message = `FheTypeId is invalid, got ${Object.prototype.toString.call(id)}.`;
    }

    throw new FheTypeError(
      {
        message,
      },
      options,
    );
  }

  public static throwFheTypeNameError(
    name: unknown,
    options: ErrorMetadataParams,
  ): never {
    let message: string;
    if (name === undefined) {
      message = 'FheTypeName is invalid.';
    } else if (typeof name === 'string') {
      message = `FheTypeName '${name}' is invalid.`;
    } else {
      message = `FheTypeName is invalid, got ${Object.prototype.toString.call(name)}.`;
    }

    throw new FheTypeError(
      {
        message,
      },
      options,
    );
  }
}
