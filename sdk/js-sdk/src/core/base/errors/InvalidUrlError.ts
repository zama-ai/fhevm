import type { ErrorMetadataParams } from './ErrorBase.js';
import { ErrorBase } from './ErrorBase.js';
import { ensureError } from './utils.js';

export type InvalidUrlErrorType = InvalidUrlError & {
  name: 'InvalidUrlError';
};

export type InvalidUrlErrorParams = Readonly<{
  url?: string;
  message?: string;
  cause?: unknown;
}>;

export class InvalidUrlError extends ErrorBase {
  constructor(params: InvalidUrlErrorParams, options: ErrorMetadataParams) {
    super({
      ...options,
      message: params.message ?? (params.url !== undefined ? `Url "${params.url}" is invalid.` : 'Url is invalid.'),
      ...(params.cause !== undefined ? { cause: ensureError(params.cause) } : {}),
      name: 'InvalidUrlError',
    });
  }
}
