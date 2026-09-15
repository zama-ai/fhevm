import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../../core/types/coreFhevmRuntime.js';
import type {
  SolanaPublicDecryptCertificateClaim,
  SolanaPublicDecryptCertificateParameters,
} from '../../actions/publicDecryptCertificate.js';
import { publicDecryptCertificate } from '../../actions/publicDecryptCertificate.js';

export type SolanaPublicDecryptActions = {
  /** Returns a certificate claim that must still be verified on-chain (`disclose_secp` or host `verify_public_decrypt`). */
  readonly publicDecryptCertificate: (
    parameters: SolanaPublicDecryptCertificateParameters,
  ) => Promise<SolanaPublicDecryptCertificateClaim>;
};

/** Binds raw certificate requests to their deployment and shared runtime. */
export function solanaPublicDecryptActions(chain: FhevmSolanaChain, runtime: FhevmRuntime): SolanaPublicDecryptActions {
  return {
    publicDecryptCertificate: (parameters) => publicDecryptCertificate({ chain, runtime }, parameters),
  };
}
