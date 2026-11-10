import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { VerifiedInputProof } from '../types/inputProof.js';
import type { ZkProof } from '../types/zkProof-p.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';
import { readCoprocessorSignersContext } from '../host-contracts/readCoprocessorSignersContext-p.js';
import { coprocessorSignersVersion } from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly zkProof: ZkProof;
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerInputProofOptions | undefined;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * A request for `retryOnStaleCache`: reads the current coprocessor signers
 * (from the cache), records their version, then sends the ZK proof and checks
 * the coprocessor signatures against them. A changed signer set only needs
 * this request to run again: the proof stays valid.
 */
export async function fetchInputProofWithCurrentSigners(
  context: Context,
  parameters: Parameters,
  usedVersions: UsedCacheVersions,
): Promise<VerifiedInputProof> {
  const coprocessorSigners = await readCoprocessorSignersContext(context, {
    address: context.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
    fhevmContext: parameters.fhevmContext,
  });
  recordUsedCacheVersion(usedVersions, 'coprocessorSigners', coprocessorSignersVersion(coprocessorSigners));

  return fetchVerifiedInputProof(context, { ...parameters, coprocessorSigners });
}
