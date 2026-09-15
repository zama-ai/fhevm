import type { EncryptedValue } from '../../../core/types/encryptedTypes.js';
import { asEncryptedValue } from '../../../core/handle/EncryptedValue.js';
import type { RelayerInputProofOptions } from '../../../core/types/relayer.js';
import type { Bytes32Hex } from '../../../core/types/primitives.js';
import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { FhevmBase, FhevmExtension, OptionalNativeClient } from '../../../core/types/coreFhevmClient.js';
import type { FhevmRuntime, WithEncrypt } from '../../../core/types/coreFhevmRuntime.js';
import type { SolanaEncryptInputParameters, SolanaEncryptInputResult } from '../../actions/encryptInput.js';
import type { SolanaSubmitInputProofParameters, SolanaSubmitInputProofResult } from '../../actions/submitInputProof.js';
import {
  asFhevmWith,
  getFrozenContext,
  initPublicAction,
  setFrozenContext,
  setResolvedTfheVersion,
} from '../../../core/runtime/CoreFhevm-p.js';
import { createFhevmClientFrozenContext } from '../../../core/frozenContext/fhevmClientFrozenContext-p.js';
import { encryptModule } from '../../../core/modules/encrypt/module/index.js';
import { DEFAULT_TFHE_VERSION } from '../../../wasm/tfhe/loadTfheLib.js';
import { encryptInput } from '../../actions/encryptInput.js';
import { submitInputProof } from '../../actions/submitInputProof.js';

////////////////////////////////////////////////////////////////////////////////

/** Submitted coprocessor attestation, ready to encode in a Solana instruction. */
export type SolanaInputProof = SolanaSubmitInputProofResult & {
  readonly chainId: bigint;
  readonly aclContractAddress: Bytes32Hex;
  readonly contractAddress: Bytes32Hex;
  readonly userAddress: Bytes32Hex;
};

export type SolanaEncryptValuesResult = {
  readonly encryptedValues: readonly EncryptedValue[];
  readonly inputProof: SolanaInputProof;
};

export type SolanaEncryptActions = {
  readonly encryptValues: (
    parameters: SolanaEncryptInputParameters & { readonly options?: RelayerInputProofOptions | undefined },
  ) => Promise<SolanaEncryptValuesResult>;
  /** Builds a Solana input ZK proof (RFC-021 bytes32 identities + 128-byte aux). */
  readonly generateZkProof: (parameters: SolanaEncryptInputParameters) => Promise<SolanaEncryptInputResult>;
  /** Submits a built Solana input proof and verifies the returned handles. */
  readonly submitInputProof: (parameters: SolanaSubmitInputProofParameters) => Promise<SolanaSubmitInputProofResult>;
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

  if (getFrozenContext(fhevm) === undefined) {
    setFrozenContext(fhevm, createFhevmClientFrozenContext({ tfheVersion }));
  } else {
    setResolvedTfheVersion(fhevm, tfheVersion);
  }
}

/**
 * Attaches the Solana `generateZkProof` action to a base Solana client, extending the runtime
 * with the TFHE encrypt module (the ZK prover). Mirrors the EVM encrypt decorator.
 *
 * @param aclProgramAddress - The zama-host program id as bytes32 (the Solana ACL identity).
 */
export function solanaEncryptActions(
  aclProgramAddress: Bytes32Hex,
): (fhevm: SolanaClientBase) => FhevmExtension<SolanaEncryptActions, WithEncrypt> {
  return (fhevm: SolanaClientBase): FhevmExtension<SolanaEncryptActions, WithEncrypt> => {
    const runtime = fhevm.runtime.extend(encryptModule);
    const solanaChain = (fhevm as SolanaClientBase & { readonly solanaChain: FhevmSolanaChain }).solanaChain;

    const generateZkProof: SolanaEncryptActions['generateZkProof'] = async (parameters) => {
      const fhevmContext = await initPublicAction(fhevm);
      return encryptInput(
        {
          chain: solanaChain,
          aclProgramAddress,
          runtime,
          tfheVersion: fhevmContext.tfheVersion,
        },
        parameters,
      );
    };

    return {
      actions: {
        generateZkProof,
        encryptValues: async (parameters) => {
          const inputProof = await generateZkProof(parameters);
          const result = await submitInputProof({ runtime, solanaChain }, { inputProof, options: parameters.options });
          return {
            encryptedValues: result.handles.map(asEncryptedValue),
            inputProof: {
              ...result,
              chainId: inputProof.chainId,
              aclContractAddress: inputProof.aclContractAddress,
              contractAddress: inputProof.contractAddress,
              userAddress: inputProof.userAddress,
            },
          };
        },
        submitInputProof: async (parameters) => {
          await initPublicAction(fhevm);
          return submitInputProof({ runtime, solanaChain }, parameters);
        },
      },
      runtime,
      init: _initEncrypt as (fhevm: FhevmBase) => Promise<void>,
    };
  };
}
