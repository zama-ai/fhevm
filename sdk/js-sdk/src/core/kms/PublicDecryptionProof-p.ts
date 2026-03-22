import type { BytesHex } from "../types/primitives.js";
import type { PublicDecryptionProof } from "../types/publicDecryptionProof.js";
import type { DecryptedFhevmHandle } from "../types/decryptedFhevmHandle.js";

//////////////////////////////////////////////////////////////////////////////
// PublicDecryptionProof class
//////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 */
export class PublicDecryptionProofImpl implements PublicDecryptionProof {
  // numSigners + KMS signatures + extraData
  readonly #decryptionProof: BytesHex;
  readonly #orderedDecryptedHandles: readonly DecryptedFhevmHandle[];
  readonly #orderedAbiEncodedClearValues: BytesHex;
  readonly #extraData: BytesHex;

  constructor(params: {
    readonly decryptionProof: BytesHex;
    readonly orderedDecryptedHandles: readonly DecryptedFhevmHandle[];
    readonly orderedAbiEncodedClearValues: BytesHex;
    readonly extraData: BytesHex;
  }) {
    this.#decryptionProof = params.decryptionProof;
    this.#orderedDecryptedHandles = Object.freeze([
      ...params.orderedDecryptedHandles,
    ]);
    this.#extraData = params.extraData;
    this.#orderedAbiEncodedClearValues = params.orderedAbiEncodedClearValues;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Getters
  //////////////////////////////////////////////////////////////////////////////

  public get decryptionProof(): BytesHex {
    return this.#decryptionProof;
  }

  public get orderedDecryptedHandles(): readonly DecryptedFhevmHandle[] {
    return this.#orderedDecryptedHandles;
  }

  public get orderedAbiEncodedClearValues(): BytesHex {
    return this.#orderedAbiEncodedClearValues;
  }

  public get extraData(): BytesHex {
    return this.#extraData;
  }
}

Object.freeze(PublicDecryptionProofImpl);
Object.freeze(PublicDecryptionProofImpl.prototype);
