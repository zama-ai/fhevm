import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { BytesHex, ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';
import { createZkProof } from './ZkProofBuilder-p.js';
import { retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';
import { recordCacheUsage } from '../caching/cacheUsage-p.js';
import { fetchVersionedFheEncryptionKeyWasm } from '../key/fetchFheEncryptionKey.js';
import { readVersionedCoprocessorSignersContext } from '../host-contracts/readCoprocessorSignersContext-p.js';

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
  const { fhevmContext } = parameters;
  const hardCodedExtraData = '0x00' as BytesHex;

  // A rotated key (input proof rejected) or a changed coprocessor signer set
  // is refreshed and the whole request runs once more: a new key needs a new
  // proof. The request reads each cached value it uses, records its version
  // and passes the value down.
  return retryOnStaleCache(context, fhevmContext, async (cacheUsage) => {
    // 1. The key, then the ZK proof built with it
    const fheEncryptionKey = await fetchVersionedFheEncryptionKeyWasm(context, { fhevmContext });
    recordCacheUsage(cacheUsage, 'keyMaterial', fheEncryptionKey.version);

    const zkProof = await createZkProof(context, {
      contractAddress: parameters.contractAddress,
      userAddress: parameters.userAddress,
      values: parameters.values,
      extraData: hardCodedExtraData,
      fheEncryptionKey: fheEncryptionKey.value,
    });

    // 2. The coprocessor signers, then the input proof checked against them
    const coprocessorSigners = await readVersionedCoprocessorSignersContext(context, {
      address: context.chain.fhevm.contracts.inputVerifier.address as ChecksummedAddress,
      fhevmContext,
    });
    recordCacheUsage(cacheUsage, 'coprocessorSigners', coprocessorSigners.version);

    const inputProof = await fetchVerifiedInputProof(context, {
      zkProof,
      options: parameters.options,
      fhevmContext,
      coprocessorSigners: coprocessorSigners.value,
    });

    return {
      inputHandles: inputProof.inputHandles,
      inputProof: inputProof.bytesHex,
    };
  });
}
