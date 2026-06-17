import type { Bytes32Hex } from '../../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmChain } from '../../../core/types/fhevmChain.js';
import type { Fhevm, FhevmBase, FhevmExtension, OptionalNativeClient } from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime, WithEncrypt } from '../../../core/types/coreFhevmRuntime.js';
import type { SolanaEncryptInputParameters, SolanaEncryptInputResult } from '../../actions/encryptInput.js';
import { asFhevmWith } from '../../../core/runtime/CoreFhevm-p.js';
import { encryptModule } from '../../../core/modules/encrypt/module/index.js';
import { encryptInput } from '../../actions/encryptInput.js';

////////////////////////////////////////////////////////////////////////////////

export type SolanaEncryptActions = {
  /** Builds a Solana input ZK proof (RFC-021 bytes32 identities + 128-byte aux). */
  readonly buildInputProof: (parameters: SolanaEncryptInputParameters) => Promise<SolanaEncryptInputResult>;
};

////////////////////////////////////////////////////////////////////////////////

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

function _initEncrypt(fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>): Promise<void> {
  const f = asFhevmWith(fhevm, 'encrypt');
  return f.runtime.encrypt.initTfheModule();
}

/**
 * Attaches the Solana `buildInputProof` action to a base Solana client, extending the runtime
 * with the TFHE encrypt module (the ZK prover). Mirrors the EVM encrypt decorator.
 *
 * `buildSolana` reads an EVM-shaped chain (`id`, `fhevm.relayerUrl`, `fhevm.contracts.acl.address`),
 * but {@link FhevmSolanaChain} carries no `contracts` — the Solana host's "ACL contract" is the
 * zama-host program, supplied here as `aclProgramAddress`. We adapt the two into the context
 * `buildSolana` expects; per-host identity validation happens inside `buildInputProofMetaData`.
 *
 * @param aclProgramAddress - The zama-host program id as bytes32 (the Solana ACL identity).
 */
export function solanaEncryptActions(
  aclProgramAddress: Bytes32Hex,
): (fhevm: SolanaClientBase) => FhevmExtension<SolanaEncryptActions, WithEncrypt> {
  return (fhevm: SolanaClientBase): FhevmExtension<SolanaEncryptActions, WithEncrypt> => {
    const runtime = fhevm.runtime.extend(encryptModule);
    const solanaChain = (fhevm as SolanaClientBase & { readonly solanaChain: FhevmSolanaChain }).solanaChain;

    // `buildSolana` only reads `.chain` (id, relayerUrl, acl address) and `.runtime` off the client;
    // build the minimal EVM-shaped adapter it expects from the Solana chain + ACL program id.
    const encryptFhevm = {
      chain: {
        id: solanaChain.id,
        fhevm: {
          relayerUrl: solanaChain.fhevm.relayerUrl,
          contracts: { acl: { address: aclProgramAddress } },
        },
      },
      runtime,
    } as unknown as Fhevm<FhevmChain, WithEncrypt>;

    return {
      actions: {
        buildInputProof: (parameters) => encryptInput(encryptFhevm, parameters),
      },
      runtime,
      init: _initEncrypt,
    };
  };
}
