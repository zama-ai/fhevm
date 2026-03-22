import type {
  KmsSigncryptedSharesMetadata,
  KmsSigncryptedShare,
} from "../types/kms-p.js";

import type {
  KmsSigncryptedShares,
  KmsSigncryptedSharesBrand,
} from "../types/kms.js";

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN = Symbol(
  "KmsSigncryptedShares.token",
);

const GET_METADATA_FUNC = Symbol("KmsSigncryptedShares.getMetadata");
const GET_SHARES_FUNC = Symbol("KmsSigncryptedShares.getShares");

////////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 */
class KmsSigncryptedSharesImpl implements KmsSigncryptedShares {
  declare readonly [KmsSigncryptedSharesBrand]: never;
  readonly #metadata: KmsSigncryptedSharesMetadata;
  readonly #shares: readonly KmsSigncryptedShare[];

  constructor(
    metadata: KmsSigncryptedSharesMetadata,
    shares: readonly KmsSigncryptedShare[],
  ) {
    this.#metadata = {
      kmsSignersContext: metadata.kmsSignersContext,
      eip712Domain: metadata.eip712Domain,
      eip712Signature: metadata.eip712Signature,
      eip712SignerAddress: metadata.eip712SignerAddress,
      fhevmHandles: [...metadata.fhevmHandles],
    };
    Object.freeze(this.#metadata);
    Object.freeze(this.#metadata.fhevmHandles);

    this.#shares = [...shares];
    Object.freeze(this.#shares);
    this.#shares.forEach((share) => Object.freeze(share));
  }

  public [GET_SHARES_FUNC](token: symbol): readonly KmsSigncryptedShare[] {
    if (token !== PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN) {
      throw new Error("Unauthorized");
    }
    return this.#shares;
  }

  public [GET_METADATA_FUNC](token: symbol): KmsSigncryptedSharesMetadata {
    if (token !== PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN) {
      throw new Error("Unauthorized");
    }
    return this.#metadata;
  }
}

/**
 * @internal
 */
export function createKmsSigncryptedShares(
  metadata: KmsSigncryptedSharesMetadata,
  shares: readonly KmsSigncryptedShare[],
): KmsSigncryptedShares {
  return new KmsSigncryptedSharesImpl(metadata, shares);
}

/**
 * @internal
 */
export function getShares(
  signcryptedShares: KmsSigncryptedShares,
): readonly KmsSigncryptedShare[] {
  if (!(signcryptedShares instanceof KmsSigncryptedSharesImpl)) {
    throw new Error("Invalid KmsSigncryptedShares");
  }
  return signcryptedShares[GET_SHARES_FUNC](
    PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN,
  );
}

/**
 * @internal
 */
export function getMetadata(
  signcryptedShares: KmsSigncryptedShares,
): KmsSigncryptedSharesMetadata {
  if (!(signcryptedShares instanceof KmsSigncryptedSharesImpl)) {
    throw new Error("Invalid KmsSigncryptedShares");
  }
  return signcryptedShares[GET_METADATA_FUNC](
    PRIVATE_KMS_SIGNCRYPTED_SHARES_TOKEN,
  );
}
