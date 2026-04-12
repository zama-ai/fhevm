import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// ZKProofError
////////////////////////////////////////////////////////////////////////////////

export type ZkProofErrorType = ZkProofError & {
  name: 'ZkProofError';
};

export type ZkProofErrorParams = Prettify<Omit<FhevmErrorBaseParams, 'name'>>;

export class ZkProofError extends FhevmErrorBase {
  constructor(params: ZkProofErrorParams) {
    super({
      ...params,
      message: params.message ?? `FHEVM ZkProof is invalid.`,
      name: 'ZkProofError',
    });
  }
}
