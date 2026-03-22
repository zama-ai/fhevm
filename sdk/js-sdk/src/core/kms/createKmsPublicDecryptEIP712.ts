//////////////////////////////////////////////////////////////////////////////
// createKmsPublicDecryptEIP712
//////////////////////////////////////////////////////////////////////////////

import { assertIsBytesHex } from "../base/bytes.js";
import {
  assertIsFhevmHandleLikeArray,
  fhevmHandleLikeToFhevmHandle,
} from "../handle/FhevmHandle.js";
import type { FhevmHandleLike } from "../types/fhevmHandle.js";
import type { KmsPublicDecryptEIP712 } from "../types/kms.js";
import { createKmsEIP712Domain } from "./createKmsEIP712Domain.js";
import { kmsPublicDecryptEIP712Types } from "./kmsPublicDecryptEIP712Types.js";

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsPublicDecryptEIP712Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly handles: readonly FhevmHandleLike[];
  readonly decryptedResult: string;
  readonly extraData: string;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsPublicDecryptEIP712
////////////////////////////////////////////////////////////////////////////////

export function createKmsPublicDecryptEIP712({
  verifyingContractAddressDecryption,
  chainId,
  handles,
  decryptedResult,
  extraData,
}: CreateKmsPublicDecryptEIP712Parameters): KmsPublicDecryptEIP712 {
  assertIsFhevmHandleLikeArray(handles, {});
  assertIsBytesHex(decryptedResult, {});
  assertIsBytesHex(extraData, {});

  const primaryType: KmsPublicDecryptEIP712["primaryType"] =
    "PublicDecryptVerification";

  const domain = createKmsEIP712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712: KmsPublicDecryptEIP712 = {
    types: kmsPublicDecryptEIP712Types,
    primaryType,
    domain,
    message: {
      ctHandles: handles.map((h) => {
        return fhevmHandleLikeToFhevmHandle(h).bytes32Hex;
      }),
      decryptedResult,
      extraData,
    },
  };

  Object.freeze(eip712);
  Object.freeze(eip712.domain);
  Object.freeze(eip712.types);
  Object.freeze(eip712.types.EIP712Domain);
  Object.freeze(eip712.types.PublicDecryptVerification);
  Object.freeze(eip712.message);
  Object.freeze(eip712.message.ctHandles);

  return eip712;
}
