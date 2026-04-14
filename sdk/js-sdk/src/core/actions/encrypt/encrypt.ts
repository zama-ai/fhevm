import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex, TypedValueLike } from '../../types/primitives.js';
import type { ExternalEncryptedValue } from '../../types/encryptedTypes.js';
import { fetchVerifiedInputProof } from '../base/fetchVerifiedInputProof.js';
import { generateZkProof } from './generateZkProof.js';
import { asBytesHex } from '../../base/bytes.js';
import { toArray } from '../../base/object.js';

////////////////////////////////////////////////////////////////////////////////

type EncryptParametersBase = {
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type EncryptSingleParameters = EncryptParametersBase & {
  readonly values: TypedValueLike;
};

export type EncryptMultipleParameters = EncryptParametersBase & {
  readonly values: readonly TypedValueLike[];
};

////////////////////////////////////////////////////////////////////////////////

export type EncryptSingleReturnType = {
  readonly externalEncryptedValue: ExternalEncryptedValue;
  readonly inputProof: BytesHex;
};

export type EncryptMultipleReturnType = {
  readonly externalEncryptedValues: readonly ExternalEncryptedValue[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export type EncryptParameters = EncryptSingleParameters | EncryptMultipleParameters;
export type EncryptReturnType = EncryptSingleReturnType | EncryptMultipleReturnType;

////////////////////////////////////////////////////////////////////////////////

export async function encrypt(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptSingleParameters,
): Promise<EncryptSingleReturnType>;
export async function encrypt(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptMultipleParameters,
): Promise<EncryptMultipleReturnType>;
export async function encrypt(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptParameters,
): Promise<EncryptReturnType>;
export async function encrypt(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptParameters,
): Promise<EncryptReturnType> {
  const hardCodedExtraData = '0x00' as BytesHex;
  const { contractAddress, userAddress } = parameters;
  const isArray = Array.isArray(parameters.values);
  const values = toArray(parameters.values);

  const zkProof = await generateZkProof(fhevm, {
    contractAddress,
    userAddress,
    values,
  });

  const inputProof = await fetchVerifiedInputProof(fhevm, {
    zkProof,
    extraData: asBytesHex(hardCodedExtraData),
    options: parameters.options,
  });

  if (isArray) {
    return {
      externalEncryptedValues: inputProof.inputHandles,
      inputProof: inputProof.bytesHex,
    };
  }

  return {
    externalEncryptedValue: inputProof.inputHandles[0],
    inputProof: inputProof.bytesHex,
  };
}
