import type { RelayerCommonOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncryptTransci } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import type { SignedDecryptionPermit } from '../../types/signedDecryptionPermit.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toArray } from '../../base/object.js';
import { createTypedValue } from '../../base/typedValue.js';
import { encryptTransci } from '../../coprocessor/encryptTransci.js';
import { resolveRawValueTypeName } from '../../handle/FheType.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValueTransciParameters = {
  readonly value: { readonly type: string; readonly value: boolean | bigint | number | string };
  readonly contractAddress: string;
  readonly userAddress: string;
  /** RFC-016 (v2) permit signed by `userAddress`, authenticating the pad decryption request. */
  readonly signedPermit: SignedDecryptionPermit;
  readonly options?: RelayerCommonOptions | undefined;
};

export type EncryptValueTransciReturnType = {
  readonly encryptedValue: EncryptedValue;
  readonly inputPackage: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptValueTransci(
  fhevm: Fhevm<FhevmChain, WithEncryptTransci>,
  parameters: EncryptValueTransciParameters,
): Promise<EncryptValueTransciReturnType> {
  const { contractAddress, userAddress, options } = parameters;

  // Validates `value`
  const values = toArray(parameters.value).map((v) =>
    createTypedValue({ type: resolveRawValueTypeName(v.type), value: v.value }),
  );

  assertIsAddress(contractAddress, {});
  assertIsAddress(userAddress, {});

  const fhevmContext = await initPublicAction(fhevm);

  const result = await encryptTransci(fhevm, {
    contractAddress: addressToChecksummedAddress(contractAddress),
    userAddress: addressToChecksummedAddress(userAddress),
    values,
    signedPermit: parameters.signedPermit,
    fhevmContext,
    options,
  });

  return {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    encryptedValue: result.externalHandles[0]!.bytes32Hex as unknown as EncryptedValue,
    inputPackage: result.inputPackage,
  };
}
