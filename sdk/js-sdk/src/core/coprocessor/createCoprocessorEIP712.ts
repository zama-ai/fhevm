import type { Uint64BigInt } from '../types/primitives.js';
import type { CoprocessorEIP712 } from '../types/coprocessor.js';
import {
  assertIsInputHandleLikeArray,
  handleLikeToHandle,
} from '../handle/FhevmHandle.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
} from '../base/address.js';
import { assertIsUint64 } from '../base/uint.js';
import { assertIsBytesHex } from '../base/bytes.js';
import { coprocessorEIP712Types } from './coprocessorEIP712Types.js';
import { createCoprocessorEIP712Domain } from './createCoprocessorEIP712Domain.js';
import type { InputHandleLike } from '../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateCoprocessorEIP712Parameters = {
  readonly gatewayChainId: number | bigint;
  readonly verifyingContractAddressInputVerification: string;
  readonly handles: readonly InputHandleLike[];
  readonly contractChainId: number | bigint;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly extraData: string;
};

////////////////////////////////////////////////////////////////////////////////
// createCoprocessorEIP712
////////////////////////////////////////////////////////////////////////////////

export function createCoprocessorEIP712({
  gatewayChainId,
  verifyingContractAddressInputVerification,
  handles,
  contractChainId,
  contractAddress,
  userAddress,
  extraData,
}: CreateCoprocessorEIP712Parameters): CoprocessorEIP712 {
  assertIsInputHandleLikeArray(handles, {});
  assertIsAddress(userAddress, {});
  assertIsAddress(contractAddress, {});
  assertIsUint64(contractChainId, {});
  assertIsBytesHex(extraData, {});

  const domain = createCoprocessorEIP712Domain({
    gatewayChainId,
    verifyingContractAddressInputVerification,
  });

  const eip712 = {
    domain,
    types: coprocessorEIP712Types,
    message: {
      ctHandles: handles.map((h) => {
        return handleLikeToHandle(h).bytes32Hex;
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
