import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { VerifiedInputProof } from '../../types/inputProof.js';
import type { BytesHex } from '../../types/primitives.js';
import type { ZkProofLike } from '../../types/zkProof-p.js';
import { fetchVerifiedInputProof as fetchVerifiedInputProof_ } from '../../coprocessor/fetchVerifiedInputProof.js';
import { toZkProof } from '../../coprocessor/ZkProof-p.js';

export type FetchVerifiedInputProofParameters = {
  readonly zkProof: ZkProofLike;
  readonly extraData: BytesHex;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type FetchVerifiedInputProofReturnType = VerifiedInputProof;

export async function fetchVerifiedInputProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchVerifiedInputProofParameters,
): Promise<FetchVerifiedInputProofReturnType> {
  const { zkProof, ...rest } = parameters;
  const sanitizedZkProof = await toZkProof(zkProof);
  return fetchVerifiedInputProof_(fhevm, { zkProof: sanitizedZkProof, ...rest });
}
