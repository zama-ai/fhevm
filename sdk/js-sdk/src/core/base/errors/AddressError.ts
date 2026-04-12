import type { ErrorMetadataParams } from './ErrorBase.js';
import { ErrorBase } from './ErrorBase.js';

export type AddressErrorType = AddressError & {
  name: 'AddressError';
};

export type AddressErrorParams = Readonly<{
  address?: string;
}>;

export class AddressError extends ErrorBase {
  constructor(params: AddressErrorParams, options: ErrorMetadataParams) {
    super({
      ...options,
      message:
        params.address !== undefined
          ? `Address "${params.address}" is invalid.`
          : 'Address is invalid.',
      name: 'AddressError',
    });
  }
}
