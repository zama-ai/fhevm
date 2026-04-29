import type { KmsPublicDecryptEip712 } from '../types/kms.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { assertIsBytesHex } from '../base/bytes.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import { kmsPublicDecryptEip712Types } from './kmsPublicDecryptEip712Types.js';
import { assertIsHandle } from '../handle/FhevmHandle.js';

////////////////////////////////////////////////////////////////////////////////

export type CreateKmsPublicDecryptEip712Parameters = {
  readonly verifyingContractAddressDecryption: string;
  readonly chainId: number | bigint;
  readonly handles: readonly Handle[];
  readonly decryptedResult: string;
  readonly extraData: string;
};

////////////////////////////////////////////////////////////////////////////////
// createKmsPublicDecryptEIP712
////////////////////////////////////////////////////////////////////////////////

export function createKmsPublicDecryptEip712(
  parameters: CreateKmsPublicDecryptEip712Parameters,
): KmsPublicDecryptEip712 {
  const { verifyingContractAddressDecryption, chainId, handles, decryptedResult, extraData } = parameters;
  assertIsBytesHex(decryptedResult, {});
  assertIsBytesHex(extraData, {});

  const primaryType: KmsPublicDecryptEip712['primaryType'] = 'PublicDecryptVerification';

  const domain = createKmsEip712Domain({
    chainId,
    verifyingContractAddressDecryption,
  });

  const eip712: KmsPublicDecryptEip712 = {
    types: kmsPublicDecryptEip712Types,
    primaryType,
    domain,
    message: {
      ctHandles: handles.map((h) => {
        assertIsHandle(h);
        return h.bytes32Hex;
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
