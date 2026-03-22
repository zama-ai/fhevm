import type {
  KmsDelegatedUserDecryptEIP712,
  KmsUserDecryptEIP712,
} from "./kms.js";
import type { Bytes65Hex, ChecksummedAddress } from "./primitives.js";

/**
 * A signed EIP-712 authorization that allows the signer's ephemeral key
 * to request decryption of the signer's own FHE handles from the KMS.
 *
 * Created once, serializable, and reusable across multiple decrypt calls.
 *
 * @see {@link FhevmDelegatedUserDecryptionPermit} for decrypting another user's handles.
 */
export type FhevmUserDecryptionPermit = {
  readonly eip712: KmsUserDecryptEIP712;
  readonly signature: Bytes65Hex;
  readonly signerAddress: ChecksummedAddress;
};

export type FhevmDelegatedUserDecryptionPermit = {
  readonly eip712: KmsDelegatedUserDecryptEIP712;
  readonly signature: Bytes65Hex;
  readonly signerAddress: ChecksummedAddress;
};
