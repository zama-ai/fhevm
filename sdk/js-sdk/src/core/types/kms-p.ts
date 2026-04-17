import type { Handle } from './encryptedTypes-p.js';
import type { KmsEip712Domain } from './kms.js';
import type { KmsSignersContext } from './kmsSignersContext.js';
import type { Bytes65Hex, Bytes65HexNo0x, BytesHexNo0x, ChecksummedAddress } from './primitives.js';

export interface KmsSigncryptedSharesMetadata {
  readonly kmsSignersContext: KmsSignersContext;
  readonly eip712Domain: KmsEip712Domain;
  readonly eip712Signature: Bytes65Hex;
  readonly eip712SignerAddress: ChecksummedAddress;
  readonly handles: readonly Handle[];
}

export interface KmsSigncryptedShare {
  readonly payload: BytesHexNo0x;
  readonly signature: Bytes65HexNo0x;
  readonly extraData: BytesHexNo0x;
}
