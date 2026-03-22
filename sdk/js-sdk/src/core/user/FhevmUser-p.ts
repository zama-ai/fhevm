import type { ErrorMetadataParams } from "../base/errors/ErrorBase.js";
import { InvalidTypeError } from "../base/errors/InvalidTypeError.js";
import type { FhevmDecryptionKey } from "./FhevmDecryptionKey-p.js";
import {
  createFhevmDecryptionKey,
  isFhevmDecryptionKey,
} from "./FhevmDecryptionKey-p.js";
import type { FhevmRuntime } from "../types/coreFhevmRuntime.js";
import type { FhevmUser } from "../types/fhevmUser.js";
import type { ChecksummedAddress, Bytes } from "../types/primitives.js";
import type { TkmsPrivateKey } from "../types/tkms-p.js";
import type { WithDecryptModule } from "../modules/decrypt/types.js";

////////////////////////////////////////////////////////////////////////////////
// FhevmUserImpl
//
// Unexported class wrapping an address and a FhevmDecryptionKey.
// - Class: enables instanceof checks (isFhevmUser)
// - Frozen: instance, class, and prototype are all immutable
// - Tree-shakable: unused exports are eliminated by bundlers
// - No this pitfalls: properties are plain readonly values

class FhevmUserImpl implements FhevmUser {
  readonly address: ChecksummedAddress;
  readonly decryptionKey: FhevmDecryptionKey;

  constructor(parameters: {
    address: ChecksummedAddress;
    decryptionKey: FhevmDecryptionKey;
  }) {
    this.address = parameters.address;
    this.decryptionKey = parameters.decryptionKey;
    Object.freeze(this);
  }
}

Object.freeze(FhevmUserImpl);
Object.freeze(FhevmUserImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/** Type guard: returns true if value was created by {@link createFhevmUser}. */
export function isFhevmUser(value: unknown): value is FhevmUser {
  return value instanceof FhevmUserImpl;
}

////////////////////////////////////////////////////////////////////////////////

/** Throws {@link InvalidTypeError} if value is not a valid {@link FhevmUser}. */
export function assertIsFhevmUser(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is FhevmUser {
  if (!isFhevmUser(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: "FhevmUser",
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/** Creates a {@link FhevmUser} by binding an address and a private key into an immutable object. */
export async function createFhevmUser(
  fhevmRuntime: FhevmRuntime<WithDecryptModule>,
  parameters: {
    address: ChecksummedAddress;
    privateKey: Bytes | TkmsPrivateKey | FhevmDecryptionKey;
  },
): Promise<FhevmUser> {
  const decryptionKey = isFhevmDecryptionKey(parameters.privateKey)
    ? parameters.privateKey
    : await createFhevmDecryptionKey(fhevmRuntime, {
        tkmsPrivateKey: parameters.privateKey,
      });

  return new FhevmUserImpl({
    address: parameters.address,
    decryptionKey,
  });
}

////////////////////////////////////////////////////////////////////////////////
