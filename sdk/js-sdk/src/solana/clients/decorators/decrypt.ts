import type { SolanaUserDecryptSigner } from '../../signer.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { Fhevm, FhevmBase, FhevmExtension, OptionalNativeClient } from '../../../core/types/coreFhevmClient.js';
import type { FhevmProtocolContext } from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime, WithDecrypt } from '../../../core/types/coreFhevmRuntime.js';
import type { SolanaUserDecryptParameters, SolanaUserDecryptResult } from '../../actions/userDecrypt.js';
import type { GenerateTransportKeyPairReturnType } from '../../../core/actions/decrypt/generateTransportKeyPair.js';
import type { WithTkmsVersion } from '../../../core/types/coreFhevmClient.js';
import { asFhevmWith, setResolvedTkmsVersion } from '../../../core/runtime/CoreFhevm-p.js';
import {
  generateTransportKeyPair as generateTransportKeyPair_,
} from '../../../core/kms/TransportKeyPair-p.js';
import { decryptModule } from '../../../core/modules/decrypt/module/index.js';
import { hyperWasmResolveTkmsModuleVersion } from '../../../core/runtime/HyperWasmSolver-p.js';
import { userDecrypt } from '../../actions/userDecrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type SolanaDecryptActions = {
  /** Runs the full Solana ed25519 user-decrypt round-trip and returns the decrypted clear values. */
  readonly userDecrypt: (parameters: SolanaUserDecryptParameters) => Promise<SolanaUserDecryptResult>;
  /** Generates a fresh E2E transport (ML-KEM) key pair for decryption. */
  readonly generateTransportKeyPair: () => Promise<GenerateTransportKeyPairReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

// Solana has no on-chain ACL contract to query for protocol/pubKeyCrs versions. Use the latest
// known canonical context so `hyperWasmResolveTkmsModuleVersion` auto-resolves to the current
// WASM binaries when the caller has not set an explicit `moduleVersions` override. An explicit
// override (via `options.moduleVersions.kms` or `runtime.config.moduleVersions.kms`) wins and
// this context is only used for the compatibility-check pass (which is skipped when `checkCompatibility: 'off'`).
// `eq` comparators pin to the exact rule-3 boundary (protocol==0.13.0, pubKeyCrs==1.6.0) — see
// `semverComparatorImpliesRange`: `eq:V` implies any range that V satisfies, so this matches only rule 3.
const SOLANA_DEFAULT_PROTOCOL_CONTEXT = {
  protocolVersion: { version: '0.13.0', comparator: 'eq' },
  pubKeyCrsVersion: { version: '1.6.0', comparator: 'eq' },
} as const satisfies FhevmProtocolContext;

////////////////////////////////////////////////////////////////////////////////

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

async function _initDecrypt(fhevm: FhevmBase<undefined, FhevmRuntime, OptionalNativeClient>): Promise<void> {
  const f = asFhevmWith(fhevm, 'decrypt');

  // Resolve the TKMS version against the Solana default protocol context.
  // An explicit `moduleVersions.kms` in options or runtime config overrides auto-resolution.
  const tkmsVersion = hyperWasmResolveTkmsModuleVersion(
    { runtime: f.runtime, options: f.options },
    SOLANA_DEFAULT_PROTOCOL_CONTEXT,
  );

  await f.runtime.decrypt.initTkmsModule({ tkmsVersion });

  // `fhevm` is the same CoreFhevmImpl instance `_initDecrypt` was called with.
  // Cast satisfies the FhevmBase<FhevmChain, ..., NativeClient> signature; instanceof check
  // inside setResolvedTkmsVersion validates identity at runtime.
  setResolvedTkmsVersion(fhevm as unknown as FhevmBase, tkmsVersion);
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
    const solanaChain = (fhevm as SolanaClientBase & { readonly solanaChain: FhevmSolanaChain }).solanaChain;
    const context = {
      chain: solanaChain,
      runtime,
      options: fhevm.options,
    };
    return {
      actions: {
        // Auto-init: await fhevm.ready so _initDecrypt has run before userDecrypt needs TKMS.
        // Cast: createFhevmBaseClient always returns a CoreFhevmImpl which satisfies Fhevm (has `ready`).
        userDecrypt: async (parameters) => {
          await (fhevm as Fhevm<undefined, FhevmRuntime, undefined>).ready;
          return userDecrypt(context, signer, parameters);
        },
        generateTransportKeyPair: async () => {
          // Auto-init: await fhevm.ready so tkmsVersion is stored and TKMS WASM is loaded.
          await (fhevm as Fhevm<undefined, FhevmRuntime, undefined>).ready;
          // Solana has no EVM FhevmChain/NativeClient. generateTransportKeyPair's body uses only
          // `runtime` + `tkmsVersion` (chain/client are unused required params), so pass them as
          // never. Read tkmsVersion directly from fhevm after ready has resolved it.
          const tkmsVersion = (fhevm as unknown as WithTkmsVersion).tkmsVersion;
          return generateTransportKeyPair_({ runtime, chain: {} as never, client: {} as never, tkmsVersion });
        },
      },
      runtime,
      init: _initDecrypt as (fhevm: FhevmBase) => Promise<void>,
    };
  };
}
