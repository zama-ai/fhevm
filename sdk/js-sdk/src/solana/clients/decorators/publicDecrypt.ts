import type {
  SolanaPublicDecryptCertificateClaim,
  SolanaPublicDecryptCertificateParameters,
} from '../../actions/publicDecryptCertificate.js';

export type SolanaPublicDecryptActions = {
  /** Returns a certificate claim that must still be verified on-chain (`disclose_secp` or host `verify_public_decrypt`). */
  readonly publicDecryptCertificate: (
    parameters: SolanaPublicDecryptCertificateParameters,
  ) => Promise<SolanaPublicDecryptCertificateClaim>;
};
