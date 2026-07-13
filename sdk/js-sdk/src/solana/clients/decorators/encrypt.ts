import type { Bytes32Hex } from '../../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmChain } from '../../../core/types/fhevmChain.js';
import type {
  Fhevm,
  FhevmBase,
  FhevmExtension,
  OptionalNativeClient,
  WithTfheVersion,
} from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime, WithEncrypt } from '../../../core/types/coreFhevmRuntime.js';
import type { SolanaEncryptInputParameters, SolanaEncryptInputResult } from '../../actions/encryptInput.js';
import { asFhevmWith, setResolvedTfheVersion } from '../../../core/runtime/CoreFhevm-p.js';
import { encryptModule } from '../../../core/modules/encrypt/module/index.js';
import { DEFAULT_TFHE_VERSION } from '../../../wasm/tfhe/loadTfheLib.js';
import { encryptInput } from '../../actions/encryptInput.js';

////////////////////////////////////////////////////////////////////////////////

export type SolanaEncryptActions = {
  /** Builds a Solana input ZK proof (RFC-021 bytes32 identities + 128-byte aux). */
  readonly buildInputProof: (parameters: SolanaEncryptInputParameters) => Promise<SolanaEncryptInputResult>;
};

////////////////////////////////////////////////////////////////////////////////

type SolanaClientBase = FhevmBase<undefined, FhevmRuntime, undefined>;

async function _initEncrypt(fhevm: FhevmBase<undefined, FhevmRuntime, OptionalNativeClient>): Promise<void> {
  const f = asFhevmWith(fhevm, 'encrypt');

  // The Solana input-proof prover MUST match the host coprocessor's pinned tfhe (=1.6.2): the
  // zkproof-worker verifies the proof with that exact version. Use the manifest default
  // (DEFAULT_TFHE_VERSION) rather than protocol-context auto-resolution — Solana has no on-chain
  // protocol context, and the EVM-derived context maps to a different tfhe version.
  const tfheVersion = DEFAULT_TFHE_VERSION;

  await f.runtime.encrypt.initTfheModule({ tfheVersion });

  setResolvedTfheVersion(fhevm, tfheVersion);
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

    // `buildSolana` reads `.chain` (id, relayerUrl, acl address), `.runtime`, and `.tfheVersion`.
    // Build the minimal EVM-shaped adapter from the Solana chain + ACL program id. `tfheVersion`
    // is resolved and stored on `fhevm` by `_initEncrypt`; `buildInputProof` awaits `fhevm.ready`
    // so the version is always set before this getter fires.
    const encryptFhevm = {
      chain: {
        id: solanaChain.id,
        fhevm: {
          relayerUrl: solanaChain.fhevm.relayerUrl,
          contracts: { acl: { address: aclProgramAddress } },
        },
      },
      get tfheVersion() {
        // After fhevm.ready the CoreFhevmImpl exposes tfheVersion directly; read it to avoid
        // a second set/get round-trip through the stored-value path.
        return (fhevm as unknown as WithTfheVersion).tfheVersion;
      },
      runtime,
    } as unknown as Fhevm<FhevmChain, WithEncrypt> & WithTfheVersion;

    return {
      actions: {
        // Auto-init: await fhevm.ready so _initEncrypt has run and tfheVersion is set before
        // encryptInput accesses encryptFhevm.tfheVersion. Idempotent — ready memoises the promise.
        // Cast: createFhevmBaseClient always returns a CoreFhevmImpl which satisfies Fhevm (has `ready`).
        buildInputProof: async (parameters) => {
          await (fhevm as Fhevm<undefined, FhevmRuntime, undefined>).ready;
          return encryptInput(encryptFhevm, parameters);
        },
      },
      runtime,
      init: _initEncrypt as (fhevm: FhevmBase) => Promise<void>,
    };
  };
}
