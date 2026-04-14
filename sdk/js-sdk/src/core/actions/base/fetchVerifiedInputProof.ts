import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { VerifiedInputProof } from '../../types/inputProof.js';
import type { BytesHex } from '../../types/primitives.js';
import type { ZkProof } from '../../types/zkProof.js';
import type { ExternalEncryptedValue } from '../../types/encryptedTypes.js';
import { InputProofError } from '../../errors/InputProofError.js';
import { assertHandleArrayEquals } from '../../handle/FhevmHandle.js';
import { createVerifiedInputProofFromComponents } from './createVerifiedInputProofFromComponents.js';

export type FetchVerifiedInputProofParameters = {
  readonly zkProof: ZkProof;
  readonly extraData: BytesHex;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type FetchVerifiedInputProofReturnType = VerifiedInputProof;

export async function fetchVerifiedInputProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchVerifiedInputProofParameters,
): Promise<FetchVerifiedInputProofReturnType> {
  const { zkProof, extraData, options } = parameters;

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerInputProofOptions = {
    auth: fhevm.runtime.config.auth,
    ...options,
  };

  // 1. extract FhevmHandles from the given ZK proof
  const fhevmHandles: readonly ExternalEncryptedValue[] = zkProof.getExternalEncryptedValues();

  if (fhevmHandles.length === 0) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  // 2. Request coprocessor signatures from the relayer for the given ZK proof
  const { handles: coprocessorSignedHandles, coprocessorEip712Signatures: coprocessorSignatures } =
    await fhevm.runtime.relayer.fetchCoprocessorSignatures(
      { relayerUrl: fhevm.chain.fhevm.relayerUrl, chainId: fhevm.chain.id },
      {
        payload: {
          zkProof,
          extraData,
        },
        options: relayerOptions,
      },
    );

  // 3. Check that the handles and the one in the fetch result
  // Note: this check is theoretically unecessary
  // We prefer to perform this test since we do not trust the relayer
  // The purpose is to check if the relayer is possibly malicious
  assertHandleArrayEquals(fhevmHandles, coprocessorSignedHandles);

  // 4. Verify ZK proof and Compute the final Input proof
  return await createVerifiedInputProofFromComponents(fhevm, {
    coprocessorEip712Signatures: coprocessorSignatures,
    inputHandles: fhevmHandles,
    extraData: extraData,
    signedHandleAccess: {
      userAddress: zkProof.userAddress,
      contractAddress: zkProof.contractAddress,
    },
  });
}
