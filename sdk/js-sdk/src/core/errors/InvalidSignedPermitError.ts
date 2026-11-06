import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// InvalidSignedPermitError
////////////////////////////////////////////////////////////////////////////////

export type InvalidSignedPermitErrorType = InvalidSignedPermitError & {
  name: 'InvalidSignedPermitError';
};

export type InvalidSignedPermitErrorParams = Prettify<Omit<FhevmErrorBaseParams, 'name'>>;

/**
 * Thrown when a signed decryption permit cannot be used for the requested
 * decryption (e.g. an extraData version the protocol no longer supports).
 * A new permit must be signed.
 */
export class InvalidSignedPermitError extends FhevmErrorBase {
  constructor(params: InvalidSignedPermitErrorParams) {
    super({
      ...params,
      name: 'InvalidSignedPermitError',
    });
  }
}
