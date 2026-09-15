import type { KmsPublicDecryptEip712 } from '../types/kms.js';
import type { Handle } from '../types/encryptedTypes-p.js';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { concatBytes, hexToBytes } from '../base/bytes.js';
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
// createKmsPublicDecryptEip712
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

const text = new TextEncoder();
const hashText = (value: string): Uint8Array => keccak_256(text.encode(value));
const uint256 = (value: bigint): Uint8Array => hexToBytes(value.toString(16).padStart(64, '0'));

// This fixed schema is shared with EVM. Derive its type strings from the canonical definition,
// without requiring an EVM wallet/provider dependency in the Solana package.
export function publicDecryptDigest(eip712: KmsPublicDecryptEip712): Uint8Array {
  const typeHash = (typeName: keyof typeof eip712.types): Uint8Array =>
    hashText(`${typeName}(${eip712.types[typeName].map(({ name, type }) => `${type} ${name}`).join(',')})`);
  const domain = keccak_256(
    concatBytes(
      typeHash('EIP712Domain'),
      hashText(eip712.domain.name),
      hashText(eip712.domain.version),
      uint256(BigInt(eip712.domain.chainId)),
      concatBytes(new Uint8Array(12), hexToBytes(eip712.domain.verifyingContract)),
    ),
  );
  const message = keccak_256(
    concatBytes(
      typeHash('PublicDecryptVerification'),
      keccak_256(concatBytes(...eip712.message.ctHandles.map(hexToBytes))),
      keccak_256(hexToBytes(eip712.message.decryptedResult)),
      keccak_256(hexToBytes(eip712.message.extraData)),
    ),
  );
  return keccak_256(concatBytes(new Uint8Array([0x19, 0x01]), domain, message));
}
