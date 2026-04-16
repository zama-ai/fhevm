import type { Uint64BigInt } from '../types/primitives.js';
import type { CoprocessorEip712 } from '../types/coprocessor.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import { addressToChecksummedAddress, assertIsAddress } from '../base/address.js';
import { assertIsUint64 } from '../base/uint.js';
import { assertIsBytesHex } from '../base/bytes.js';
import { coprocessorEip712Types } from './coprocessorEip712Types.js';
import { createCoprocessorEip712Domain } from './createCoprocessorEip712Domain.js';
import { assertIsInputHandle } from '../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateCoprocessorEip712Parameters = {
  readonly gatewayChainId: number | bigint;
  readonly verifyingContractAddressInputVerification: string;
  readonly inputHandles: readonly InputHandle[];
  readonly contractChainId: number | bigint;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly extraData: string;
};

////////////////////////////////////////////////////////////////////////////////
// createCoprocessorEip712
////////////////////////////////////////////////////////////////////////////////

export function createCoprocessorEip712({
  gatewayChainId,
  verifyingContractAddressInputVerification,
  inputHandles,
  contractChainId,
  contractAddress,
  userAddress,
  extraData,
}: CreateCoprocessorEip712Parameters): CoprocessorEip712 {
  assertIsAddress(userAddress, {});
  assertIsAddress(contractAddress, {});
  assertIsUint64(contractChainId, {});
  assertIsBytesHex(extraData, {});

  const domain = createCoprocessorEip712Domain({
    gatewayChainId,
    verifyingContractAddressInputVerification,
  });

  const eip712 = {
    domain,
    types: coprocessorEip712Types,
    message: {
      ctHandles: inputHandles.map((inputHandle) => {
        assertIsInputHandle(inputHandle, {});
        return inputHandle.bytes32Hex;
      }),
      userAddress: addressToChecksummedAddress(userAddress),
      contractAddress: addressToChecksummedAddress(contractAddress),
      contractChainId: BigInt(contractChainId) as Uint64BigInt,
      extraData,
    },
  };

  Object.freeze(eip712);
  Object.freeze(eip712.domain);
  Object.freeze(eip712.types);
  Object.freeze(eip712.types.CiphertextVerification);
  Object.freeze(eip712.message);
  Object.freeze(eip712.message.ctHandles);

  return eip712;
}
