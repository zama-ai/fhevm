import type { FhevmUserDecryptionPermit } from "../types/fhevmUserDecryptionPermit.js";
import type { KmsUserDecryptEIP712 } from "../types/kms.js";
import type { Bytes65Hex, ChecksummedAddress } from "../types/primitives.js";

type VerifiedUserDecryptionPermit = FhevmUserDecryptionPermit & {
  readonly verified: true;
};

/**
 * Private implementation of {@link FhevmUserDecryptionPermit}.
 * Immutable by design — all fields are stored as private properties
 * and exposed via readonly getters. Instances are only created through
 * SDK-internal factory functions that guarantee the signature has been verified.
 */
class FhevmUserDecryptionPermitImpl implements VerifiedUserDecryptionPermit {
  readonly #eip712: KmsUserDecryptEIP712;
  readonly #signature: Bytes65Hex;
  readonly #signerAddress: ChecksummedAddress;
  readonly verified = true as const;

  constructor(parameters: {
    readonly eip712: KmsUserDecryptEIP712;
    readonly signature: Bytes65Hex;
    readonly signerAddress: ChecksummedAddress;
  }) {
    this.#eip712 = parameters.eip712;
    this.#signature = parameters.signature;
    this.#signerAddress = parameters.signerAddress;
  }

  public get eip712(): KmsUserDecryptEIP712 {
    return this.#eip712;
  }

  public get signature(): Bytes65Hex {
    return this.#signature;
  }

  public get signerAddress(): ChecksummedAddress {
    return this.#signerAddress;
  }
}

////////////////////////////////////////////////////////////////////////////////
// isVerifiedUserDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

export function isVerifiedUserDecryptionPermit(
  value: unknown,
): value is VerifiedUserDecryptionPermit {
  return value instanceof FhevmUserDecryptionPermitImpl;
}

////////////////////////////////////////////////////////////////////////////////
// createFhevmUserDecryptionPermit
////////////////////////////////////////////////////////////////////////////////

export function createFhevmUserDecryptionPermit(parameters: {
  readonly signerAddress: ChecksummedAddress;
  readonly eip712: KmsUserDecryptEIP712;
  readonly signature: Bytes65Hex;
}): VerifiedUserDecryptionPermit {
  return new FhevmUserDecryptionPermitImpl(parameters);
}
