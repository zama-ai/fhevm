import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { ZkProof, ZkProofLike } from '../../types/zkProof-p.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import type { ChecksummedAddress } from '../../types/primitives.js';
import type { VerifiedInputProof } from '../../types/inputProof.js';
import type { FhevmClientFrozenContext } from '../../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from '../../caching/usedCacheVersions-p.js';
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
  const result = await retryOnStaleCache(fhevm, fhevmContext, (usedVersions) =>
    _fetchInputProof(fhevm, { ...rest, zkProof, fhevmContext }, usedVersions),
  );
  return {
    encryptedValues: result.inputHandles.map(
      (encryptedValue) => encryptedValue.bytes32Hex as unknown as EncryptedValue,
    ),
    inputProof: result.bytesHex,
  };
}

////////////////////////////////////////////////////////////////////////////////

/** The request: reads the coprocessor signers, records their version, and checks the proof against them. */
async function _fetchInputProof(
  fhevm: Fhevm<FhevmChain>,
  parameters: FetchEncryptedValuesParameters & {
    readonly zkProof: ZkProof;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
  usedVersions: UsedCacheVersions,
): Promise<VerifiedInputProof> {
  const coprocessorSigners = await readCoprocessorSignersContext(fhevm, {
    address: fhevm.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
    fhevmContext: parameters.fhevmContext,
  });
  recordUsedCacheVersion(usedVersions, 'coprocessorSigners', coprocessorSignersVersion(coprocessorSigners));
  return fetchVerifiedInputProof_(fhevm, { ...parameters, coprocessorSigners });
}
