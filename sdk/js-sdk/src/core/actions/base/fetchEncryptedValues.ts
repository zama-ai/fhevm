import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { ZkProofLike } from '../../types/zkProof-p.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import { fetchVerifiedInputProof as fetchVerifiedInputProof_ } from '../../coprocessor/fetchVerifiedInputProof.js';
import { assertIsZkProof } from '../../coprocessor/ZkProof-p.js';

////////////////////////////////////////////////////////////////////////////////

export type FetchEncryptedValuesParameters = {
  readonly zkProof: ZkProofLike;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type FetchEncryptedValuesReturnType = {
  readonly encryptedValues: readonly EncryptedValue[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function fetchEncryptedValues(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchEncryptedValuesParameters,
): Promise<FetchEncryptedValuesReturnType> {
  const { zkProof, ...rest } = parameters;

  assertIsZkProof(zkProof, {
    subject: 'zkProof parameter',
    metaMessages: ['Use generateZkProof() to create a valid ZkProof.'],
  });

  const result = await fetchVerifiedInputProof_(fhevm, { zkProof, ...rest });
  return {
    encryptedValues: result.inputHandles.map(
      (encryptedValue) => encryptedValue.bytes32Hex as unknown as EncryptedValue,
    ),
    inputProof: result.bytesHex,
  };
}
