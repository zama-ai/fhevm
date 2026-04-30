import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { FhevmRuntime, WithDecrypt } from '../types/coreFhevmRuntime.js';
import type { Bytes, BytesHex } from '../types/primitives.js';
import type { TkmsPrivateKey } from '../types/tkms-p.js';
import { assertIsBytesOrBytesHex, bytesToHexLarge, hexToBytesFaster } from '../base/bytes.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { assertRecordNonNullableProperty } from '../base/record.js';
import { asFhevmRuntimeWith } from '../runtime/CoreFhevmRuntime-p.js';
import { verifyTkmsPublicKey } from '../utils-p/decrypt/verifyTkmsPublicKey.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('TransportKeyPair.token');

////////////////////////////////////////////////////////////////////////////////

declare const TransportKeyPairBrand: unique symbol;

export type TransportKeyPair = {
  readonly publicKey: BytesHex;
  readonly [TransportKeyPairBrand]: true;
};

////////////////////////////////////////////////////////////////////////////////
// TransportKeyPairImpl
//
// Unexported class wrapping closures that bind a tkmsPrivateKey.
// - Class: enables instanceof checks (isTransportKeyPair)
// - Closures: methods capture privateKey without exposing it
// - Frozen: instance, class, and prototype are all immutable
// - Tree-shakable: unused exports are eliminated by bundlers
// - No this pitfalls: methods are own properties, not prototype-bound
// - tamper-resistant: class + private token + frozen prototype

const GetTkmsPrivateKeyFn = Symbol();
const SerializeFn = Symbol();

class TransportKeyPairImpl implements TransportKeyPair {
  declare readonly [TransportKeyPairBrand]: true;

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
  async [GetTkmsPrivateKeyFn](privateToken: symbol, expectedRuntime: FhevmRuntime): Promise<TkmsPrivateKey> {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    // If the runtime was GC'd, deref() returns undefined, which !== expectedRuntime → throws.
    // This is intentional: a GC'd runtime means the key pair is stale.
    if (expectedRuntime !== this.#runtime.deref()) {
      throw new Error('Invalid runtime');
    }
    if (this.#tkmsPrivateKey !== undefined) {
      return this.#tkmsPrivateKey;
    }

    const runtimeWithDecrypt = asFhevmRuntimeWith(expectedRuntime, 'decrypt');

    this.#tkmsPrivateKey = await runtimeWithDecrypt.decrypt.deserializeTkmsPrivateKey({
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
   * Hidden method (Symbol key) that serializes the key pair including the private key.
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

Object.freeze(TransportKeyPairImpl);
Object.freeze(TransportKeyPairImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/** Type guard. */
export function isTransportKeyPair(value: unknown): value is TransportKeyPair {
  return value instanceof TransportKeyPairImpl;
}

////////////////////////////////////////////////////////////////////////////////

/** Throws {@link InvalidTypeError} if value is not a valid {@link TransportKeyPair}. */
export function assertIsTransportKeyPair(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is TransportKeyPair {
  if (!isTransportKeyPair(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'TransportKeyPair',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/** Generates a fresh {@link TransportKeyPair}. */
export async function generateTransportKeyPair(context: { readonly runtime: WithDecrypt }): Promise<TransportKeyPair> {
  const tkmsPrivateKey = await context.runtime.decrypt.generateTkmsPrivateKey();
  const tkmsPrivateKeyBytes = await context.runtime.decrypt.serializeTkmsPrivateKey({ tkmsPrivateKey });
  const tkmsPublicKeyBytesHex = await context.runtime.decrypt.getTkmsPublicKeyHex({
    tkmsPrivateKey,
  });
  return new TransportKeyPairImpl(PRIVATE_TOKEN, context.runtime, {
    privateKeyBytes: tkmsPrivateKeyBytes,
    publicKeyBytesHex: tkmsPublicKeyBytesHex,
    tkmsPrivateKey,
  });
}

/**
 * Converts an unknown value into a {@link TransportKeyPair}.
 *
 * Accepted inputs:
 * - An existing {@link TransportKeyPair} (returned as-is)
 * - A plain object `{ publicKey, privateKey }` where each is `Bytes` or `BytesHex`
 *
 * @throws {InvalidTypeError} If `value` is not a recognized key pair shape.
 */
export async function toTransportKeyPair(
  context: { readonly runtime: FhevmRuntime },
  value: unknown,
): Promise<TransportKeyPair> {
  if (isTransportKeyPair(value)) {
    // Force realize
    await (value as TransportKeyPairImpl)[GetTkmsPrivateKeyFn](PRIVATE_TOKEN, context.runtime);
    return value;
  }

  const name = 'TransportKeyPair';
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
    typeof rawPrivateKey === 'string' ? hexToBytesFaster(rawPrivateKey, { strict: true }) : rawPrivateKey;

  const tkmsPublicKeyBytesHex: BytesHex =
    typeof rawPublicKey === 'string' ? rawPublicKey : bytesToHexLarge(rawPublicKey, false /* no0x */);

  let tkmsPrivateKey: TkmsPrivateKey | undefined;

  // If the "decrypt" module is available, deserialize and verify the key
  let runtimeWithDecrypt: WithDecrypt | undefined;
  try {
    runtimeWithDecrypt = asFhevmRuntimeWith(context.runtime, 'decrypt');
  } catch {
    // there's no decrypt module
  }

  if (runtimeWithDecrypt !== undefined) {
    tkmsPrivateKey = await runtimeWithDecrypt.decrypt.deserializeTkmsPrivateKey({
      tkmsPrivateKeyBytes,
    });
    await verifyTkmsPublicKey({ runtime: runtimeWithDecrypt }, { tkmsPrivateKey, tkmsPublicKeyBytesHex });
  }

  return new TransportKeyPairImpl(PRIVATE_TOKEN, context.runtime, {
    privateKeyBytes: tkmsPrivateKeyBytes,
    publicKeyBytesHex: tkmsPublicKeyBytesHex,
    tkmsPrivateKey,
  });
}

/**
 * Serializes an {@link TransportKeyPair} including the private key.
 *
 * **The output contains sensitive key material — handle and store securely.**
 *
 * @throws {InvalidTypeError} If `value` is not a valid {@link TransportKeyPair}.
 */
export function serializeTransportKeyPair(value: TransportKeyPair): {
  publicKey: BytesHex;
  privateKey: BytesHex;
} {
  assertIsTransportKeyPair(value, {});
  return (value as TransportKeyPairImpl)[SerializeFn](PRIVATE_TOKEN);
}

/**
 * Extracts the deserialized TKMS private key from an {@link TransportKeyPair}.
 * Lazily deserializes and verifies the key on first access.
 *
 * @throws {InvalidTypeError} If `value` is not a valid {@link TransportKeyPair}.
 * @throws If the runtime does not match or the key verification fails.
 */
export async function transportKeyPairToTkmsPrivateKey(
  context: { readonly runtime: FhevmRuntime },
  value: TransportKeyPair,
): Promise<TkmsPrivateKey> {
  assertIsTransportKeyPair(value, {});
  return await (value as TransportKeyPairImpl)[GetTkmsPrivateKeyFn](PRIVATE_TOKEN, context.runtime);
}
