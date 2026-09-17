import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FetchInputProofResult, RelayerInputProofOptions } from '../../core/types/relayer.js';
import type { SolanaZkProof } from '../../core/types/zkProof-p.js';
import { submitInputProofPayload } from '../../core/modules/relayer/module/fetchCoprocessorSignatures.js';
import { u64ToHex0x } from '../../core/base/uint.js';
import { asBytesHex, bytesToHexNo0x, hexToBytes32 } from '../../core/base/bytes.js';
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
  const relayerOptions: RelayerInputProofOptions = {
    auth: fhevm.runtime.config.auth,
    ...options,
  };

  const result = await submitInputProofPayload({
    relayerUrl: fhevm.solanaChain.fhevm.relayerUrl,
    payload: {
      ciphertextWithInputVerification: bytesToHexNo0x(inputProof.ciphertextWithZkProof),
      contractAddress: base58.encode(hexToBytes32(inputProof.contractAddress)),
      contractChainId: u64ToHex0x(inputProof.chainId),
      extraData: asBytesHex('0x00'),
      userAddress: base58.encode(hexToBytes32(inputProof.userAddress)),
    },
    options: relayerOptions,
    logger: fhevm.runtime.config.logger,
  });

  assertHandleArrayEquals(result.handles, expectedHandles, {
    actualName: 'relayer response',
    expectedName: 'input proof',
  });

  return {
    handles: result.handles,
    signatures: result.signatures,
    extraData: result.extraData,
  };
}
