import type { KmsDelegatedUserDecryptEip712V1, KmsUserDecryptEip712V1, KmsUserDecryptEip712V2 } from './kms.js';
import type { Bytes65Hex, BytesHex, ChecksummedAddress } from './primitives.js';

/**
 * Common fields shared by all signed decryption permit versions.
 *
 * @typeParam Signature - The permit signature hex type. Defaults to the strict
 *   65-byte {@link Bytes65Hex} used by the EOA path; the unified V2 permit widens
 *   it to a variable-length {@link BytesHex} to carry ERC-1271 blobs.
 */
export type SignedDecryptionPermitBase<Signature extends BytesHex = Bytes65Hex> = {
  /** The EIP-712 signature. */
  readonly signature: Signature;
  /** The account that signed the permit. */
  readonly signerAddress: ChecksummedAddress;
  /**
   * The account that owns the encrypted data this permit covers.
   *
   * - **Self permit:** equals `signerAddress`.
   * - **Delegated permit:** differs from `signerAddress`. The owner has granted
   *   the signer permission via `FHE.delegateUserDecryption()` on-chain.
   */
  readonly encryptedDataOwnerAddress: ChecksummedAddress;
  /** The E2E transport public key embedded in the EIP-712 message. */
  readonly transportPublicKey: BytesHex;
  /** `true` when the signer is decrypting on behalf of another account (delegated permit). */
  readonly isDelegated: boolean;

  /** @throws If the permit has expired. */
  assertNotExpired(): void;
};

////////////////////////////////////////////////////////////////////////////////
// V1 — protocol v13 and below (two separate EIP-712 shapes)
////////////////////////////////////////////////////////////////////////////////

/**
 * Signed decryption permit produced by protocol v13 and below.
 *
 * Narrow to self vs delegated via `eip712.primaryType`:
 * - `'UserDecryptRequestVerification'` → self permit (`encryptedDataOwnerAddress === signerAddress`)
 * - `'DelegatedUserDecryptRequestVerification'` → delegated permit
 */
export type SignedDecryptionPermitV1 = SignedDecryptionPermitBase & {
  readonly version: 1;
  readonly eip712: KmsUserDecryptEip712V1 | KmsDelegatedUserDecryptEip712V1;
};

////////////////////////////////////////////////////////////////////////////////
// V2 — protocol v14 and above (unified EIP-712 shape)
////////////////////////////////////////////////////////////////////////////////

/**
 * Signed decryption permit produced by protocol v14 and above.
 *
 * Uses the unified `KmsUserDecryptEip712V2` EIP-712 shape — there is no longer
 * a separate delegated variant. The `encryptedDataOwnerAddress` is read from
 * `eip712.message.userAddress` and may differ from `signerAddress`.
 *
 * The `signature` is widened to a variable-length {@link BytesHex} so it can
 * carry an ERC-1271 smart-contract-wallet blob; a normal EOA permit still uses
 * the strict 65-byte shape.
 *
 * Discriminate from {@link SignedDecryptionPermitV1} with `permit.version === 2`.
 */
export type SignedDecryptionPermitV2 = SignedDecryptionPermitBase<BytesHex> & {
  readonly version: 2;
  readonly eip712: KmsUserDecryptEip712V2;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * A signed decryption permit, covering all protocol versions.
 *
 * Narrow by version: `permit.version === 1` → {@link SignedDecryptionPermitV1},
 * `permit.version === 2` → {@link SignedDecryptionPermitV2}.
 */
export type SignedDecryptionPermit = SignedDecryptionPermitV1 | SignedDecryptionPermitV2;
