import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// FheEncryptionKeyVerificationError
////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyVerificationErrorType = FheEncryptionKeyVerificationError & {
  name: 'FheEncryptionKeyVerificationError';
};

export type FheEncryptionKeyVerificationErrorParams = Prettify<Omit<FhevmErrorBaseParams, 'name'>>;

/**
 * Thrown when a downloaded or provided FHE public key or CRS does not match the
 * active one recorded in `KMSGeneration` (digest or id). The key material is
 * neither used nor cached.
 */
export class FheEncryptionKeyVerificationError extends FhevmErrorBase {
  constructor(params: FheEncryptionKeyVerificationErrorParams) {
    super({
      ...params,
      name: 'FheEncryptionKeyVerificationError',
    });
  }
}
