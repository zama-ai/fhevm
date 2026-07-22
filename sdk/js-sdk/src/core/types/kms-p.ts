import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { Handle } from './encryptedTypes-p.js';
import type { KmsEip712Domain } from './kms.js';
import type { KmsSignersContext } from './kmsSignersContext.js';
import type { Bytes65HexNo0x, BytesHex, BytesHexNo0x, ChecksummedAddress } from './primitives.js';

export interface KmsSigncryptedSharesMetadata {
  readonly kmsSignersContext: KmsSignersContext;
  readonly eip712Domain: KmsEip712Domain;
  // Variable length on the unified /v3 route: 65-byte EOA signature or an
  // ERC-1271 smart-contract-wallet blob.
  readonly eip712Signature: BytesHex;
  readonly eip712SignerAddress: ChecksummedAddress;
  readonly handles: readonly Handle[];
  readonly tkmsVersion: TkmsVersion;
}

export interface KmsSigncryptedShare {
  readonly payload: BytesHexNo0x;
  readonly signature: Bytes65HexNo0x;
  readonly extraData: BytesHexNo0x;
}
