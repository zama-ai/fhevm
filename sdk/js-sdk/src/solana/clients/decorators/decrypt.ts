import type { SolanaUserDecryptSigner } from '../../signer.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmChain } from '../../../core/types/fhevmChain.js';
import type { FhevmBase, FhevmExtension, OptionalNativeClient } from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime, WithDecrypt } from '../../../core/types/coreFhevmRuntime.js';
import type { SolanaUserDecryptParameters, SolanaUserDecryptResult } from '../../actions/userDecrypt.js';
import type { GenerateTransportKeyPairReturnType } from '../../../core/actions/decrypt/generateTransportKeyPair.js';
import { asFhevmWith } from '../../../core/runtime/CoreFhevm-p.js';
import { generateTransportKeyPair } from '../../../core/kms/TransportKeyPair-p.js';
import { decryptModule } from '../../../core/modules/decrypt/module/index.js';
import { userDecrypt } from '../../actions/userDecrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type SolanaDecryptActions = {
  /** Runs the full Solana ed25519 user-decrypt round-trip and returns the decrypted clear values. */
  readonly userDecrypt: (parameters: SolanaUserDecryptParameters) => Promise<SolanaUserDecryptResult>;
  /** Generates a fresh E2E transport (ML-KEM) key pair for decryption. */
  readonly generateTransportKeyPair: () => Promise<GenerateTransportKeyPairReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

function _initDecrypt(fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>): Promise<void> {
  const f = asFhevmWith(fhevm, 'decrypt');
  return f.runtime.decrypt.initTkmsModule();
}

/**
 * Attaches the Solana `userDecrypt` action (and `generateTransportKeyPair`) to a base Solana
 * client, extending the runtime with the TKMS decrypt module used to generate the ML-KEM transport
 * key pair. The {@link SolanaUserDecryptSigner} is captured here, mirroring the EVM decrypt decorator.
 */
export function solanaDecryptActions(
  signer: SolanaUserDecryptSigner,
): (fhevm: SolanaClientBase) => FhevmExtension<SolanaDecryptActions, WithDecrypt> {
  return (fhevm: SolanaClientBase): FhevmExtension<SolanaDecryptActions, WithDecrypt> => {
    const runtime = fhevm.runtime.extend(decryptModule);
    const context = {
      chain: (fhevm as SolanaClientBase & { readonly solanaChain: FhevmSolanaChain }).solanaChain,
      runtime,
      options: fhevm.options,
    };
    return {
      actions: {
        userDecrypt: (parameters) => userDecrypt(context, signer, parameters),
        generateTransportKeyPair: () => generateTransportKeyPair({ runtime }),
      },
      runtime,
      init: _initDecrypt,
    };
  };
}
