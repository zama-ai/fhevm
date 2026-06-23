import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { BytesHex, ChecksummedAddress, TypedValue } from '../types/primitives.js';
import type { RelayerInputProofOptions } from '../types/relayer.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { TfheVersion } from '../../wasm/tfhe/TfheApi.js';
import { fetchVerifiedInputProof } from './fetchVerifiedInputProof.js';
import { createZkProof } from './ZkProofBuilder-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithEncrypt;
  readonly client: NonNullable<object>;
  readonly tfheVersion: TfheVersion;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  readonly contractAddress: ChecksummedAddress;
  readonly userAddress: ChecksummedAddress;
  readonly values: readonly TypedValue[];
  readonly options?: RelayerInputProofOptions | undefined;
  /**
   * Optional seed for deterministic ("seeded") public encryption. When provided,
   * the ciphertext is byte-for-byte reproducible from the same seed + inputs.
   * Requires TFHE version 1.6.1 and a seed of at least 16 bytes.
   */
  readonly seed?: Uint8Array | undefined;
};

type ReturnType = {
  readonly inputHandles: readonly InputHandle[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encrypt(context: Context, parameters: Parameters): Promise<ReturnType> {
  const hardCodedExtraData = '0x00' as BytesHex;

  const zkProof = await createZkProof(
    { chain: context.chain, runtime: context.runtime, tfheVersion: context.tfheVersion },
    { ...parameters, extraData: hardCodedExtraData },
  );

  const inputProof = await fetchVerifiedInputProof(context, {
    zkProof,
    options: parameters.options,
  });

  return {
    inputHandles: inputProof.inputHandles,
    inputProof: inputProof.bytesHex,
  };
}
