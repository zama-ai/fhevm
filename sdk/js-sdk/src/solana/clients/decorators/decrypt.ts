import type { FhevmBase, FhevmExtension } from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime } from '../../../core/types/coreFhevmRuntime.js';
import { solanaPublicDecryptActions, type SolanaPublicDecryptActions } from './publicDecrypt.js';

////////////////////////////////////////////////////////////////////////////////

// The v0 `userDecrypt` action is gone with the rest of the retired ed25519 surface, and with it the
// core TKMS decrypt module this decorator used to load: its `generateTransportKeyPair` produced a
// pair from the EVM-generation blob, which the permit path cannot use — the permit commits to the
// 869-byte MlKem512 container that only `generateSolanaTransportKeyPair` (the vendored Solana blob,
// `solana/userDecrypt`) produces, and a pair from another blob generation fails response
// verification on wasm class identity. No EVM-blob generator is offered here, so a pair the permit
// path cannot consume cannot be produced from the Solana surface at all.
//
// What remains today is the public-decrypt set. Wiring the permit path (`solana/permit`,
// `solana/userDecrypt`) into this client — the transport, the evidence source, the signer-set and
// routing configuration — is the client-assembly work that follows this stage.
export type SolanaDecryptActions = SolanaPublicDecryptActions;

////////////////////////////////////////////////////////////////////////////////

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

/** Attaches the Solana decrypt-side actions — today, the public-decrypt set — to a base client. */
export function solanaDecryptActions(fhevm: SolanaClientBase): FhevmExtension<SolanaDecryptActions> {
  return solanaPublicDecryptActions(fhevm);
}
