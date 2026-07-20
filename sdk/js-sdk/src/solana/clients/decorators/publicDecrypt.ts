import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmBase, FhevmExtension } from '../../../core/types/coreFhevmClient.js';
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

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

/** Attaches the signer-free Solana public-decrypt certificate action. */
export function solanaPublicDecryptActions(
  fhevm: SolanaClientBase,
): FhevmExtension<SolanaPublicDecryptActions> {
  const chain = (fhevm as SolanaClientBase & { readonly solanaChain: FhevmSolanaChain }).solanaChain;
  return {
    actions: {
      publicDecryptCertificate: (parameters) =>
        publicDecryptCertificate({ chain, runtime: fhevm.runtime }, parameters),
    },
    runtime: fhevm.runtime,
  };
}
