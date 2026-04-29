import type { KmsDelegatedUserDecryptEip712, KmsUserDecryptEip712 } from './kms.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from './primitives.js';

/**
 * A signed EIP-712 authorization that allows the signer's ephemeral key
 * to request decryption of the signer's own FHE handles from the KMS.
 *
 * Created once, serializable, and reusable across multiple decrypt calls.
 */
export type SignedDecryptionPermitBase = {
  /** The EIP-712 signature. */
  readonly signature: Bytes65Hex;
  /** The account that signed the permit. */
  readonly signerAddress: ChecksummedAddress;
  /**
   * The account that owns the encrypted data this permit covers.
   *
   * - **Self permit:** equals `signerAddress` — the signer authorizes decryption
   *   of their own data.
   *
   * - **Delegated permit:** differs from `signerAddress`. Step by step:
   *   1. Alice owns encrypted values on-chain (ebool, euint8, etc.).
   *   2. Alice calls the ACL contract to grant Bob permission to decrypt her data.
   *   3. Bob signs an EIP-712 delegated decryption permit referencing Alice as the
   *      `encryptedDataOwnerAddress` (= `delegatorAddress` in the EIP-712 message).
   *   4. The KMS verifies Bob's signature (`signerAddress`) and that Alice
   *      (`encryptedDataOwnerAddress`) has granted him on-chain permission.
   *   5. Bob can now decrypt Alice's encrypted values using this permit.
   */
  readonly encryptedDataOwnerAddress: ChecksummedAddress;
  /** The E2E transport public key embedded in the EIP-712 message. */
  readonly e2eTransportPublicKey: BytesHex;

  /** @throws If the permit has expired based on `startTimestamp` + `durationDays`. */
  assertNotExpired(): void;
};

export type SignedSelfDecryptionPermit = SignedDecryptionPermitBase & {
  readonly eip712: KmsUserDecryptEip712;
  /** Always `false` for non-delegated permits. `encryptedDataOwnerAddress === signerAddress`. */
  readonly isDelegated: false;
};

export type SignedDelegatedDecryptionPermit = SignedDecryptionPermitBase & {
  readonly eip712: KmsDelegatedUserDecryptEip712;
  /** Always `true` for delegated permits. `v` is the delegated account. (see `SenderCannotBeDelegate` is ACL.sol) */
  readonly isDelegated: true;
};

export type SignedDecryptionPermit = SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
