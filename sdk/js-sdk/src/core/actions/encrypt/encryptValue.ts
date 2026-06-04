import type { RelayerInputProofOptions } from '../../types/relayer.js';
import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { BytesHex } from '../../types/primitives.js';
import type { EncryptedValue } from '../../types/encryptedTypes.js';
import { addressToChecksummedAddress, assertIsAddress } from '../../base/address.js';
import { toArray } from '../../base/object.js';
import { createTypedValue } from '../../base/typedValue.js';
import { encrypt as encrypt_ } from '../../coprocessor/encrypt.js';
import { hyperWasmResolveTfheModuleVersion } from '../../runtime/HyperWasmSolver-p.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptValueParameters = {
  readonly value: { readonly type: string; readonly value: boolean | bigint | number | string };
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly options?: RelayerInputProofOptions | undefined;
};

export type EncryptValueReturnType = {
  readonly encryptedValue: EncryptedValue;
  readonly inputProof: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptValue(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
  parameters: EncryptValueParameters,
): Promise<EncryptValueReturnType> {
  const { contractAddress, userAddress, options } = parameters;

  // Validates `value`
  const values = toArray(parameters.value).map(createTypedValue);

  assertIsAddress(contractAddress, {});
  assertIsAddress(userAddress, {});

  const tfheVersion = await hyperWasmResolveTfheModuleVersion(fhevm);

  const result = await encrypt_(
    {
      chain: fhevm.chain,
      runtime: fhevm.runtime,
      client: fhevm.client,
      tfheVersion,
      options: fhevm.options,
    },
    {
      contractAddress: addressToChecksummedAddress(contractAddress),
      userAddress: addressToChecksummedAddress(userAddress),
      values,
      options,
    },
  );

  return {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    encryptedValue: result.inputHandles[0]!.bytes32Hex as unknown as EncryptedValue,
    inputProof: result.inputProof,
  };
}
