import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { VerifiedInputProof } from '../types/inputProof.js';
import type { ZkProof } from '../types/zkProof-p.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import { InputProofError } from '../errors/InputProofError.js';
import { assertHandleArrayEquals } from '../handle/FhevmHandle.js';
import { createInputProofFromInputHandles } from './InputProof-p.js';
import { verifyInputProof } from './verifyInputProof.js';
import { assertIsZkProof } from './ZkProof-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly zkProof: ZkProof;
  readonly options?: RelayerInputProofOptions | undefined;
};

type ReturnType = VerifiedInputProof;

////////////////////////////////////////////////////////////////////////////////

export async function fetchVerifiedInputProof(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { zkProof, options } = parameters;

  assertIsZkProof(zkProof, {});

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerOptions: RelayerInputProofOptions = {
    auth: context.runtime.config.auth,
    ...options,
  };

  // 1. extract FhevmHandles from the given ZK proof
  const inputHandles: readonly InputHandle[] = zkProof.getInputHandles();

  if (inputHandles.length === 0) {
    throw new InputProofError({
      message: `Input proof must contain at least one external handle`,
    });
  }

  // 2. Request coprocessor signatures from the relayer for the given ZK proof
  const { handles: coprocessorSignedHandles, coprocessorEip712Signatures: coprocessorSignatures } =
    await context.runtime.relayer.fetchCoprocessorSignatures(context, {
      payload: {
        zkProof,
      },
      options: relayerOptions,
    });

  // 3. Check that the handles and the one in the fetch result
  // Note: this check is theoretically unnecessary
  // We prefer to perform this test since we do not trust the relayer
  // The purpose is to check if the relayer is possibly malicious
  assertHandleArrayEquals(inputHandles, coprocessorSignedHandles);

  // 4. Verify ZK proof and Compute the final Input proof
  const inputProof = createInputProofFromInputHandles({
    coprocessorEip712Signatures: coprocessorSignatures,
    inputHandles,
    extraData: zkProof.getExtraData(),
    signedHandleAccess: {
      userAddress: zkProof.userAddress,
      contractAddress: zkProof.contractAddress,
    },
  });

  return await verifyInputProof(context, { inputProof });
}
