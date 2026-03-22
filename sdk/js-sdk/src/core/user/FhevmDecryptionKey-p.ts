import { isBytes } from "../base/bytes.js";
import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import { InvalidTypeError } from "../base/errors/InvalidTypeError.js";
import type {
  DecryptAndReconstructUserModuleFunction,
  DecryptAndReconstructUserParameters,
  GetTkmsPublicKeyHexUserModuleFunction,
  WithDecryptModule,
} from "../modules/decrypt/types.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import type { Bytes } from "../types/primitives.js";
import type { TkmsPrivateKey } from "../types/tkms-p.js";

////////////////////////////////////////////////////////////////////////////////

export type FhevmDecryptionKey = GetTkmsPublicKeyHexUserModuleFunction &
  DecryptAndReconstructUserModuleFunction;

////////////////////////////////////////////////////////////////////////////////
// FhevmDecryptionKeyImpl
//
// Unexported class wrapping closures that bind a tkmsPrivateKey.
// - Class: enables instanceof checks (isFhevmDecryptionKey)
// - Closures: methods capture privateKey without exposing it
// - Frozen: instance, class, and prototype are all immutable
// - Tree-shakable: unused exports are eliminated by bundlers
// - No this pitfalls: methods are own properties, not prototype-bound

class FhevmDecryptionKeyImpl implements FhevmDecryptionKey {
  readonly decryptAndReconstruct: DecryptAndReconstructUserModuleFunction["decryptAndReconstruct"];
  readonly getTkmsPublicKeyHex: GetTkmsPublicKeyHexUserModuleFunction["getTkmsPublicKeyHex"];

  constructor(parameters: {
    decryptAndReconstruct: DecryptAndReconstructUserModuleFunction["decryptAndReconstruct"];
    getTkmsPublicKeyHex: GetTkmsPublicKeyHexUserModuleFunction["getTkmsPublicKeyHex"];
  }) {
    this.decryptAndReconstruct = parameters.decryptAndReconstruct;
    this.getTkmsPublicKeyHex = parameters.getTkmsPublicKeyHex;
    Object.freeze(this);
  }
}

Object.freeze(FhevmDecryptionKeyImpl);
Object.freeze(FhevmDecryptionKeyImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/** Type guard: returns true if value was created by {@link createFhevmDecryptionKey}. */
export function isFhevmDecryptionKey(
  value: unknown,
): value is FhevmDecryptionKey {
  return value instanceof FhevmDecryptionKeyImpl;
}

////////////////////////////////////////////////////////////////////////////////

/** Throws {@link InvalidTypeError} if value is not a valid {@link FhevmDecryptionKey}. */
export function assertIsFhevmDecryptionKey(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is FhevmDecryptionKey {
  if (!isFhevmDecryptionKey(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "FhevmDecryptionKey",
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/** Creates a {@link FhevmDecryptionKey} by binding a private key (raw bytes or deserialized) into closures. */
export async function createFhevmDecryptionKey(
  fhevmRuntime: FhevmRuntime<WithDecryptModule>,
  parameters: {
    tkmsPrivateKey: Bytes | TkmsPrivateKey;
  },
): Promise<FhevmDecryptionKey> {
  let tkmsPrivateKey: TkmsPrivateKey;

  if (isBytes(parameters.tkmsPrivateKey)) {
    tkmsPrivateKey = await fhevmRuntime.decrypt.deserializeTkmsPrivateKey({
      tkmsPrivateKeyBytes: parameters.tkmsPrivateKey,
    });
  } else {
    tkmsPrivateKey = parameters.tkmsPrivateKey;
    fhevmRuntime.decrypt.verifyTkmsPrivateKey({ tkmsPrivateKey });
  }

  return new FhevmDecryptionKeyImpl({
    async decryptAndReconstruct(
      decryptParameters: DecryptAndReconstructUserParameters,
    ) {
      return fhevmRuntime.decrypt.decryptAndReconstruct({
        tkmsPrivateKey,
        ...decryptParameters,
      });
    },
    async getTkmsPublicKeyHex() {
      return fhevmRuntime.decrypt.getTkmsPublicKeyHex({
        tkmsPrivateKey,
      });
    },
  });
}

////////////////////////////////////////////////////////////////////////////////
