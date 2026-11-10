import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { BytesHex, ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from '../caching/usedCacheVersions-p.js';
import { fetchInputProofWithCurrentSigners } from './fetchInputProofWithCurrentSigners.js';
import { createZkProof } from './ZkProofBuilder-p.js';
import { retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';
import { fetchVersionedFheEncryptionKeyWasm } from '../key/fetchFheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

// The SDK client itself, not a copy of its fields: on a key cache miss, the
// key check reads KMSGeneration through the client's trusted host-chain
// client, which only a genuine client instance carries.
type Context = FhevmBase<FhevmChain, WithEncrypt>;

type Parameters = {
  readonly contractAddress: ChecksummedAddress;
  readonly userAddress: ChecksummedAddress;
  readonly values: readonly TypedValue[];
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerInputProofOptions | undefined;
};

type ReturnType = {
  readonly inputHandles: readonly InputHandle[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encrypt(context: Context, parameters: Parameters): Promise<ReturnType> {
  // Two nested retries, one per cached value: a rotated key (input proof
  // rejected) rebuilds the proof (outer); a changed coprocessor signer set only
  // sends the same proof again (inner, in _encrypt). Each retry checks only the
  // values it recorded, so an error the inner one cannot explain reaches the
  // outer one.
  return retryOnStaleCache(context, parameters.fhevmContext, (keyVersions) =>
    _encrypt(context, parameters, keyVersions),
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The outer request: the key (its version recorded), the ZK proof built with
 * it, then the input proof, sent with its own retry on the coprocessor signers.
 */
async function _encrypt(context: Context, parameters: Parameters, keyVersions: UsedCacheVersions): Promise<ReturnType> {
  const { fhevmContext } = parameters;
  const hardCodedExtraData = '0x00' as BytesHex;

  // 1. The key, then the ZK proof built with it
  const { key: fheEncryptionKey, version: keyVersion } = await fetchVersionedFheEncryptionKeyWasm(context, {
    fhevmContext,
  });
  recordUsedCacheVersion(keyVersions, 'keyMaterial', keyVersion);

  const zkProof = await createZkProof(context, {
    contractAddress: parameters.contractAddress,
    userAddress: parameters.userAddress,
    values: parameters.values,
    extraData: hardCodedExtraData,
    fheEncryptionKey,
  });

  // 2. The input proof, checked against the current coprocessor signers
  const inputProof = await retryOnStaleCache(context, fhevmContext, (signerVersions) =>
    fetchInputProofWithCurrentSigners(context, { zkProof, options: parameters.options, fhevmContext }, signerVersions),
  );

  return {
    inputHandles: inputProof.inputHandles,
    inputProof: inputProof.bytesHex,
  };
}
