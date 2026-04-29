import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { BytesHex, ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import { asBytesHex } from '../base/bytes.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';
import { createZkProof } from './ZkProofBuilder-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithEncrypt;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly contractAddress: ChecksummedAddress;
  readonly userAddress: ChecksummedAddress;
  readonly values: readonly TypedValue[];
  readonly options?: RelayerInputProofOptions | undefined;
};

type ReturnType = {
  readonly inputHandles: readonly InputHandle[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encrypt(context: Context, parameters: Parameters): Promise<ReturnType> {
  const hardCodedExtraData = '0x00' as BytesHex;

  const zkProof = await createZkProof(context, parameters);

  const inputProof = await fetchVerifiedInputProof(context, {
    zkProof,
    extraData: asBytesHex(hardCodedExtraData),
    options: parameters.options,
  });

  return {
    inputHandles: inputProof.inputHandles,
    inputProof: inputProof.bytesHex,
  };
}
