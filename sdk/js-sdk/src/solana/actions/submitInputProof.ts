import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FetchInputProofResult, RelayerInputProofOptions } from '../../core/types/relayer.js';
import type { ZkProof } from '../../core/types/zkProof-p.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import { hexToBytes32 } from '../../core/base/bytes.js';
import { InputProofError } from '../../core/errors/InputProofError.js';
import { assertHandleArrayEquals } from '../../core/handle/FhevmHandle.js';

import { base58 } from '@scure/base';

////////////////////////////////////////////////////////////////////////////////

export type SolanaSubmitInputProofParameters = {
  readonly inputProof: SolanaZkProof;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type SolanaSubmitInputProofResult = FetchInputProofResult;

type SolanaSubmitInputProofContext = {
  readonly runtime: FhevmRuntime;
  readonly solanaChain: FhevmSolanaChain;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Submits a previously built Solana input proof and verifies the returned handles.
 * The returned coprocessor signatures are verified by the Solana host program when consumed, not
 * by this SDK action.
 */
export async function submitInputProof(
  fhevm: SolanaSubmitInputProofContext,
  parameters: SolanaSubmitInputProofParameters,
): Promise<SolanaSubmitInputProofResult> {
  const { inputProof, options } = parameters;
  const expectedHandles = inputProof.getInputHandles();

  if (expectedHandles.length === 0) {
    throw new InputProofError({
      message: 'Input proof must contain at least one external handle',
    });
  }
  if (inputProof.chainId !== fhevm.solanaChain.id) {
    throw new InputProofError({
      message: `Input proof chain id ${inputProof.chainId} does not match Solana client chain id ${fhevm.solanaChain.id}`,
    });
  }

  // The relayer payload predates RFC-021 and carries Solana host identities as base58 strings.
  // Keep that wire adaptation here so callers only handle canonical bytes32 identities.
  const relayerProof = {
    chainId: inputProof.chainId,
    aclContractAddress: inputProof.aclContractAddress,
    contractAddress: base58.encode(hexToBytes32(inputProof.contractAddress)),
    userAddress: base58.encode(hexToBytes32(inputProof.userAddress)),
    ciphertextWithZkProof: inputProof.ciphertextWithZkProof,
    encryptionBits: inputProof.encryptionBits,
    getInputHandles: () => expectedHandles,
    getExtraData: () => '0x00',
  } as unknown as ZkProof;
  const relayerOptions: RelayerInputProofOptions = {
    auth: fhevm.runtime.config.auth,
    ...options,
  };

  const result = await fhevm.runtime.relayer.fetchCoprocessorSignatures(
    {
      chain: {
        id: fhevm.solanaChain.id,
        fhevm: {
          relayerUrl: fhevm.solanaChain.fhevm.relayerUrl,
        },
      },
      runtime: fhevm.runtime,
      client: {},
      options: { batchRpcCalls: false },
    } as unknown as Parameters<FhevmRuntime['relayer']['fetchCoprocessorSignatures']>[0],
    { payload: { zkProof: relayerProof }, options: relayerOptions },
  );

  assertHandleArrayEquals(result.handles, expectedHandles, {
    actualName: 'relayer response',
    expectedName: 'input proof',
  });

  return {
    handles: result.handles,
    signatures: result.coprocessorEip712Signatures,
    extraData: result.extraData,
  };
}
