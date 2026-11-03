import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { BytesHex, ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';
import { createZkProof } from './ZkProofBuilder-p.js';

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
  const hardCodedExtraData = '0x00' as BytesHex;

  const zkProof = await createZkProof(context, {
    ...parameters,
    extraData: hardCodedExtraData,
    fhevmContext: parameters.fhevmContext,
  });

  const inputProof = await fetchVerifiedInputProof(context, {
    zkProof,
    options: parameters.options,
    fhevmContext: parameters.fhevmContext,
  });

  return {
    inputHandles: inputProof.inputHandles,
    inputProof: inputProof.bytesHex,
  };
}
