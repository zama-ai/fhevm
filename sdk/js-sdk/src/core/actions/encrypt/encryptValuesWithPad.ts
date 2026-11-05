import type { RelayerCommonOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncryptWithPad } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { createTypedValue } from '../../base/typedValue.js';
import { encryptWithPad } from '../../coprocessor/encryptWithPad.js';
import { resolveRawValueTypeName } from '../../handle/FheType.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValuesWithPadParameters = {
  readonly values: ReadonlyArray<{ readonly type: string; readonly value: boolean | bigint | number | string }>;
  readonly contractAddress: string;
  readonly userAddress: string;
  /** RFC-016 (v2) permit signed by `userAddress`, authenticating the pad decryption request. */
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerCommonOptions | undefined;
};

export type EncryptValuesWithPadReturnType = {
  readonly encryptedValues: readonly EncryptedValue[];
  readonly inputPackage: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptValuesWithPad(
  fhevm: Fhevm<FhevmChain, WithEncryptWithPad>,
  parameters: EncryptValuesWithPadParameters,
): Promise<EncryptValuesWithPadReturnType> {
  const { contractAddress, userAddress, options } = parameters;

  // Validates `values`
  const values = parameters.values.map((v) =>
    createTypedValue({ type: resolveRawValueTypeName(v.type), value: v.value }),
  );

  assertIsAddress(contractAddress, {});
  assertIsAddress(userAddress, {});

  const fhevmContext = await initPublicAction(fhevm);

  const result = await encryptWithPad(fhevm, {
    contractAddress: addressToChecksummedAddress(contractAddress),
    userAddress: addressToChecksummedAddress(userAddress),
    values,
    signedPermit: parameters.signedPermit,
    fhevmContext,
    options,
  });

  return {
    encryptedValues: result.externalHandles.map((handle) => handle.bytes32Hex as unknown as EncryptedValue),
    inputPackage: result.inputPackage,
  };
}
