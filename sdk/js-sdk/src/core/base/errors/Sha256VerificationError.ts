import type { ErrorMetadataParams } from './ErrorBase.js';
import { ErrorBase } from './ErrorBase.js';

export type Sha256VerificationErrorType = Sha256VerificationError & {
  name: 'Sha256VerificationError';
};

export type Sha256VerificationErrorParams = Readonly<{
  subject: string;
  expected: string;
  actual: string;
}>;

export class Sha256VerificationError extends ErrorBase {
  constructor(params: Sha256VerificationErrorParams, options: ErrorMetadataParams) {
    super({
      ...options,
      message: `SHA-256 mismatch for ${params.subject}: expected ${params.expected}, got ${params.actual}`,
      name: 'Sha256VerificationError',
    });
  }
}
