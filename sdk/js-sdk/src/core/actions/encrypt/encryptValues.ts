import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { createTypedValue } from '../../base/typedValue.js';
import { encrypt as encrypt_ } from '../../coprocessor/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValuesParameters = {
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type EncryptValuesReturnType = {
  readonly encryptedValues: readonly EncryptedValue[];
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptValues(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptValuesParameters,
): Promise<EncryptValuesReturnType> {
  const { contractAddress, userAddress, options } = parameters;
  // Validates `values`
  const values = parameters.values.map(createTypedValue);

  assertIsAddress(contractAddress, {});
  assertIsAddress(userAddress, {});

  const result = await encrypt_(fhevm, {
    contractAddress: addressToChecksummedAddress(contractAddress),
    userAddress: addressToChecksummedAddress(userAddress),
    values,
    options,
  });

  return {
    encryptedValues: result.inputHandles.map(
      (encryptedValue) => encryptedValue.bytes32Hex as unknown as EncryptedValue,
    ),
    inputProof: result.inputProof,
  };
}
