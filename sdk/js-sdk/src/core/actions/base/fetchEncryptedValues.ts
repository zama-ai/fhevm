import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { ZkProofLike } from '../../types/zkProof-p.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import { fetchVerifiedInputProof as fetchVerifiedInputProof_ } from '../../coprocessor/fetchVerifiedInputProof.js';
import { assertIsZkProof } from '../../coprocessor/ZkProof-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { retryOnStaleCache } from '../../caching/retryOnStaleCache-p.js';
import { recordUsedCacheVersion } from '../../caching/usedCacheVersions-p.js';
import { readCoprocessorSignersContext } from '../../host-contracts/readCoprocessorSignersContext-p.js';
import { coprocessorSignersVersion } from '../../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';

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

  const fhevmContext = await initPublicAction(fhevm);
  // A changed coprocessor signer set is refreshed and the request sent once
  // more: the proof stays valid.
  const result = await retryOnStaleCache(fhevm, fhevmContext, async (usedVersions) => {
    const coprocessorSigners = await readCoprocessorSignersContext(fhevm, {
      address: fhevm.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
      fhevmContext,
    });
    recordUsedCacheVersion(usedVersions, 'coprocessorSigners', coprocessorSignersVersion(coprocessorSigners));
    return fetchVerifiedInputProof_(fhevm, { zkProof, ...rest, fhevmContext, coprocessorSigners });
  });
  return {
    encryptedValues: result.inputHandles.map(
      (encryptedValue) => encryptedValue.bytes32Hex as unknown as EncryptedValue,
    ),
    inputProof: result.bytesHex,
  };
}
