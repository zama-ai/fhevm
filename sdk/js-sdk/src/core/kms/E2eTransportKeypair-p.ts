import {
  assertIsBytesOrBytesHex,
  bytesToHexLarge,
  hexToBytesFaster,
} from '../base/bytes.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import type { FhevmRuntime, WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Bytes, BytesHex } from '../types/primitives.js';
import type { TkmsPrivateKey } from '../types/tkms-p.js';
import { asFhevmRuntimeWith } from '../runtime/CoreFhevmRuntime-p.js';
import { verifyTkmsPublicKey } from '../utils-p/decrypt/verifyTkmsPublicKey.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('E2eTransportKeypair.token');

////////////////////////////////////////////////////////////////////////////////

declare const E2eTransportKeypairBrand: unique symbol;

export type E2eTransportKeypair = {
  readonly publicKey: BytesHex;
  readonly [E2eTransportKeypairBrand]: true;
};

////////////////////////////////////////////////////////////////////////////////
// E2eTransportKeypairImpl
//
// Unexported class wrapping closures that bind a tkmsPrivateKey.
// - Class: enables instanceof checks (isE2eTransportKeypair)
// - Closures: methods capture privateKey without exposing it
// - Frozen: instance, class, and prototype are all immutable
// - Tree-shakable: unused exports are eliminated by bundlers
// - No this pitfalls: methods are own properties, not prototype-bound
// - tamper-resistant: class + private token + frozen prototype

const GetTkmsPrivateKeyFn = Symbol();
const SerializeFn = Symbol();

class E2eTransportKeypairImpl implements E2eTransportKeypair {
  declare readonly [E2eTransportKeypairBrand]: true;

  readonly #publicKeyBytesHex: BytesHex;
  readonly #privateKeyBytes: Bytes;

  readonly #runtime: WeakRef<FhevmRuntime>;
  #tkmsPrivateKey: TkmsPrivateKey | undefined;

  constructor(
    privateToken: symbol,
    runtime: FhevmRuntime,
    parameters: {
      readonly publicKeyBytesHex: BytesHex;
      readonly privateKeyBytes: Bytes;
      readonly tkmsPrivateKey?: TkmsPrivateKey | undefined;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#runtime = new WeakRef(runtime);
    this.#publicKeyBytesHex = parameters.publicKeyBytesHex;
    this.#privateKeyBytes = parameters.privateKeyBytes;
    this.#tkmsPrivateKey = parameters.tkmsPrivateKey;
  }

  public get publicKey(): BytesHex {
    return this.#publicKeyBytesHex;
  }

  /**
   * Hidden method (Symbol key) that returns the deserialized TKMS private key.
   * Lazily deserializes and verifies on first call.
   *
   * Access is doubly protected:
   * - The Symbol key is not exported from the SDK, making the method invisible to external code.
   * - The `privateToken` argument must match the module-scoped token, preventing calls
   *   even if the Symbol is discovered via `Object.getOwnPropertySymbols`.
   *
   * @throws If `privateToken` is invalid or `expectedRuntime` does not match.
   */
  async [GetTkmsPrivateKeyFn](
    privateToken: symbol,
    expectedRuntime: FhevmRuntime,
  ): Promise<TkmsPrivateKey> {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    // If the runtime was GC'd, deref() returns undefined, which !== expectedRuntime → throws.
    // This is intentional: a GC'd runtime means the keypair is stale.
    if (expectedRuntime !== this.#runtime.deref()) {
      throw new Error('Invalid runtime');
    }
    if (this.#tkmsPrivateKey !== undefined) {
      return this.#tkmsPrivateKey;
    }

    const runtimeWithDecrypt = asFhevmRuntimeWith(expectedRuntime, 'decrypt');

    this.#tkmsPrivateKey =
      await runtimeWithDecrypt.decrypt.deserializeTkmsPrivateKey({
        tkmsPrivateKeyBytes: this.#privateKeyBytes,
      });

    // Verify the key is valid
    await verifyTkmsPublicKey(
      { runtime: runtimeWithDecrypt },
      {
        tkmsPrivateKey: this.#tkmsPrivateKey,
        tkmsPublicKeyBytesHex: this.#publicKeyBytesHex,
      },
    );

    return this.#tkmsPrivateKey;
  }

  /**
   * Hidden method (Symbol key) that serializes the keypair including the private key.
   *
   * Access is protected by the Symbol key (not exported) and the private token.
   */
  [SerializeFn](privateToken: symbol): {
    publicKey: BytesHex;
    privateKey: BytesHex;
  } {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    return {
      publicKey: this.#publicKeyBytesHex,
      privateKey: bytesToHexLarge(this.#privateKeyBytes, false /* no0x */),
    };
  }

  /**
   * Prevents accidental private key exposure via `JSON.stringify`.
   * Only the public key is included in the output.
   */
  public toJSON(): { publicKey: string } {
    return { publicKey: this.#publicKeyBytesHex };
  }
}

Object.freeze(E2eTransportKeypairImpl);
Object.freeze(E2eTransportKeypairImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/** Type guard. */
export function isE2eTransportKeypair(
  value: unknown,
): value is E2eTransportKeypair {
  return value instanceof E2eTransportKeypairImpl;
}

////////////////////////////////////////////////////////////////////////////////

/** Throws {@link InvalidTypeError} if value is not a valid {@link E2eTransportKeypair}. */
export function assertIsE2eTransportKeypair(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is E2eTransportKeypair {
  if (!isE2eTransportKeypair(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'E2eTransportKeypair',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/** Generates a fresh {@link E2eTransportKeypair}. */
export async function generateE2eTransportKeypair(context: {
  readonly runtime: WithDecrypt;
}): Promise<E2eTransportKeypair> {
  const tkmsPrivateKey = await context.runtime.decrypt.generateTkmsPrivateKey();
  const tkmsPrivateKeyBytes =
    await context.runtime.decrypt.serializeTkmsPrivateKey({ tkmsPrivateKey });
  const tkmsPublicKeyBytesHex =
    await context.runtime.decrypt.getTkmsPublicKeyHex({
      tkmsPrivateKey,
    });
  return new E2eTransportKeypairImpl(PRIVATE_TOKEN, context.runtime, {
    privateKeyBytes: tkmsPrivateKeyBytes,
    publicKeyBytesHex: tkmsPublicKeyBytesHex,
    tkmsPrivateKey,
  });
}

/**
 * Converts an unknown value into a {@link E2eTransportKeypair}.
 *
 * Accepted inputs:
 * - An existing {@link E2eTransportKeypair} (returned as-is)
 * - A plain object `{ publicKey, privateKey }` where each is `Bytes` or `BytesHex`
 *
 * @throws {InvalidTypeError} If `value` is not a recognized keypair shape.
 */
export async function toE2eTransportKeypair(
  context: { readonly runtime: FhevmRuntime },
  value: unknown,
): Promise<E2eTransportKeypair> {
  if (isE2eTransportKeypair(value)) {
    // Force realize
    await (value as E2eTransportKeypairImpl)[GetTkmsPrivateKeyFn](
      PRIVATE_TOKEN,
      context.runtime,
    );
    return value;
  }

  const name = 'E2eTransportKeypair';
  const options = {};

  assertRecordNonNullableProperty(value, 'publicKey', name, {
    expectedType: 'Bytes | BytesHex',
    ...options,
  });
  assertRecordNonNullableProperty(value, 'privateKey', name, {
    expectedType: 'Bytes | BytesHex',
    ...options,
  });

  const rawPublicKey = value.publicKey;
  const rawPrivateKey = value.privateKey;

  assertIsBytesOrBytesHex(rawPublicKey, { subject: `${name}.publicKey` });
  assertIsBytesOrBytesHex(rawPrivateKey, { subject: `${name}.privateKey` });

  const tkmsPrivateKeyBytes: Bytes =
    typeof rawPrivateKey === 'string'
      ? hexToBytesFaster(rawPrivateKey, { strict: true })
      : rawPrivateKey;

  const tkmsPublicKeyBytesHex: BytesHex =
    typeof rawPublicKey === 'string'
      ? rawPublicKey
      : bytesToHexLarge(rawPublicKey, false /* no0x */);

  let tkmsPrivateKey: TkmsPrivateKey | undefined;

  // If the "decrypt" module is available, deserialize and verify the key
  let runtimeWithDecrypt: WithDecrypt | undefined;
  try {
    runtimeWithDecrypt = asFhevmRuntimeWith(context.runtime, 'decrypt');
  } catch {
    // there's no decrypt module
  }

  if (runtimeWithDecrypt !== undefined) {
    tkmsPrivateKey = await runtimeWithDecrypt.decrypt.deserializeTkmsPrivateKey(
      {
        tkmsPrivateKeyBytes,
      },
    );
    await verifyTkmsPublicKey(
      { runtime: runtimeWithDecrypt },
      { tkmsPrivateKey, tkmsPublicKeyBytesHex },
    );
  }

  return new E2eTransportKeypairImpl(PRIVATE_TOKEN, context.runtime, {
    privateKeyBytes: tkmsPrivateKeyBytes,
    publicKeyBytesHex: tkmsPublicKeyBytesHex,
    tkmsPrivateKey,
  });
}

/**
 * Serializes an {@link E2eTransportKeypair} including the private key.
 *
 * **The output contains sensitive key material — handle and store securely.**
 *
 * @throws {InvalidTypeError} If `value` is not a valid {@link E2eTransportKeypair}.
 */
export function serializeE2eTransportKeypair(value: E2eTransportKeypair): {
  publicKey: BytesHex;
  privateKey: BytesHex;
} {
  assertIsE2eTransportKeypair(value, {});
  return (value as E2eTransportKeypairImpl)[SerializeFn](PRIVATE_TOKEN);
}

/**
 * Extracts the deserialized TKMS private key from an {@link E2eTransportKeypair}.
 * Lazily deserializes and verifies the key on first access.
 *
 * @throws {InvalidTypeError} If `value` is not a valid {@link E2eTransportKeypair}.
 * @throws If the runtime does not match or the key verification fails.
 */
export async function e2eTransportKeypairToTkmsPrivateKey(
  context: { readonly runtime: FhevmRuntime },
  value: E2eTransportKeypair,
): Promise<TkmsPrivateKey> {
  assertIsE2eTransportKeypair(value, {});
  return await (value as E2eTransportKeypairImpl)[GetTkmsPrivateKeyFn](
    PRIVATE_TOKEN,
    context.runtime,
  );
}
