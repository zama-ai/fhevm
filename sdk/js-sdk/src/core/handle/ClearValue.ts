import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { BytesHex, TypedValue, TypedValueOf, ValueType } from '../types/primitives.js';
import type { FheTypeId, FheType, ClearValueType } from '../types/fheType.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ClearValue, ClearValueOfFheType, ClearValueTypeName, Handle } from '../types/encryptedTypes-p.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';
import { asClearValueType } from './FheType.js';
import { assertNever } from '../base/errors/utils.js';
import { createTypedValue } from '../base/typedValue.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('ClearValue.token');

////////////////////////////////////////////////////////////////////////////////

/**
 * Module-scoped symbol used as the method key for origin verification.
 * Never exported — invisible to IDE autocomplete and external code.
 * @internal
 */
const VERIFY_ORIGIN_FUNC = Symbol('ClearValue.verifyOrigin');

/**
 * Internal implementation. Not exported — external code cannot instantiate.
 *
 * Security relies on:
 * - Class not being exported (no `new` from outside)
 * - `Object.freeze` on every instance (immutability)
 * - Private fields (`#handle`, `#value`, `#originToken`) inaccessible from outside
 * - `Object.freeze` on prototype (no prototype pollution)
 * - Symbol-keyed `[VERIFY_ORIGIN]` method invisible to IDE and external code
 */
class ClearValueImpl<etype extends FheType> implements ClearValueOfFheType<etype> {
  readonly #value: ClearValueType<etype>;
  readonly #handle: Handle<etype>;
  readonly #originToken: symbol;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly value: ClearValueType<etype>;
      readonly handle: Handle<etype>;
      readonly originToken: symbol;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#handle = parameters.handle;
    this.#value = parameters.value;
    this.#originToken = parameters.originToken;
  }

  public get value(): ValueType<ClearValueTypeName<etype>> {
    return this.#value;
  }

  public get type(): ClearValueTypeName<etype> {
    // FheType is always "e" + ValueTypeName (e.g. "euint8" → "uint8")
    return this.#handle.fheType.substring(1) as ClearValueTypeName<etype>;
  }

  public get handle(): Handle<etype> {
    return this.#handle;
  }

  public get typedValue(): TypedValueOf<ClearValueTypeName<etype>> {
    return {
      type: this.type,
      value: this.value,
    };
  }

  /**
   * Checks that this instance was created with the given origin token.
   * Symbol-keyed — invisible to IDE autocomplete and inaccessible without
   * the module-scoped {@link VERIFY_ORIGIN_FUNC} symbol.
   */
  public [VERIFY_ORIGIN_FUNC](token: symbol): boolean {
    return this.#originToken === token;
  }

  /**
   * Safe string representation that does not expose the value.
   */
  public toString(): string {
    return `ClearValue<${this.#handle.fheType}>`;
  }

  /**
   * Safe JSON serialization that does not expose the value.
   */
  public toJSON(): { handle: string; fheType: FheType } {
    return {
      handle: this.#handle.bytes32Hex,
      fheType: this.#handle.fheType,
    };
  }
}

Object.freeze(ClearValueImpl.prototype);

// ============================================================================
// Public API — Guards & Assertions
// ============================================================================

/**
 * Returns `true` if `value` was created via {@link createClearValue}
 * and its origin matches the given `originToken`.
 *
 * Uses `instanceof` against the non-exported `ClearValueImpl` class
 * (unforgeable in same-realm contexts), then verifies the origin token.
 *
 * @param value - The value to check
 * @param originToken - Origin symbol held privately by the decrypt flow
 */
export function isClearValue(value: unknown, originToken: symbol): value is ClearValue {
  if (!(value instanceof ClearValueImpl)) return false;
  return value[VERIFY_ORIGIN_FUNC](originToken);
}

export function clearValueToTypedValue(value: unknown, originToken: symbol): TypedValue {
  assertIsClearValue(value, { originToken });
  return createTypedValue((value as ClearValueImpl<FheType>).typedValue);
}

/**
 * Asserts that `value` was created via {@link createClearValue}
 * and its origin matches the given `originToken`.
 *
 * @throws {InvalidTypeError} If the value is not a `ClearValue`
 * instance, or if it fails origin verification.
 */
export function assertIsClearValue(
  value: unknown,
  options: { subject?: string; originToken: symbol } & ErrorMetadataParams,
): asserts value is ClearValue {
  if (!isClearValue(value, options.originToken)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'ClearValue',
      },
      options,
    );
  }
}

/**
 * Returns `true` if every element was created via
 * {@link createClearValue} and its origin matches the given
 * `originToken`.
 */
export function isClearValueArray(values: readonly unknown[], originToken: symbol): values is readonly ClearValue[] {
  return values.every((v) => isClearValue(v, originToken));
}

/**
 * Asserts that `values` is an array where every element was created via
 * {@link createClearValue} and its origin matches the given
 * `originToken`.
 *
 * @throws {InvalidTypeError} If the value is not an array, or if any element
 * is not a `ClearValue` instance (error includes the index).
 */
export function assertIsClearValueArray(
  values: unknown,
  options: { subject?: string; originToken: symbol } & ErrorMetadataParams,
): asserts values is readonly ClearValue[] {
  if (!Array.isArray(values)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof values,
        expectedType: 'ClearValue[]',
      },
      options,
    );
  }
  for (let i = 0; i < values.length; ++i) {
    if (!isClearValue(values[i], options.originToken)) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof values[i],
          expectedType: 'ClearValue',
        },
        options,
      );
    }
  }
}

// ============================================================================
// Public API — Factory
// ============================================================================

/**
 * Creates a validated, immutable {@link ClearValue}.
 *
 * The `originToken` parameter acts as access control: only code that holds
 * a private `Symbol` (e.g. `publicDecrypt`, `decrypt`) can produce
 * instances that pass {@link isClearValue} with origin verification.
 *
 * @param value - The decrypted plaintext value (validated against `handle.fheType`)
 * @param handle - A validated {@link Handle}
 * @param originToken - Private symbol owned by the calling decrypt flow
 * @returns A frozen `ClearValue` instance
 * @throws {InvalidTypeError} If the value doesn't match the handle's FHE type
 */
export function createClearValue<etype extends FheType>(parameters: {
  readonly value: ClearValueType<etype>;
  readonly handle: Handle<etype>;
  readonly originToken: symbol;
}): ClearValue<etype> {
  const v = new ClearValueImpl<etype>(PRIVATE_TOKEN, {
    handle: parameters.handle,
    value: asClearValueType(parameters.handle.fheType, parameters.value),
    originToken: parameters.originToken,
  });
  Object.freeze(v);
  return v;
}

/**
 * Creates an array of {@link ClearValue}s from parallel arrays of
 * encrypted values and clear values.
 *
 * @param orderedValues - Corresponding decrypted values (same length & order)
 * @param orderedEncryptedValues - Validated FHEVM handles
 * @param originToken - Private symbol owned by the calling decrypt flow
 * @returns A frozen array of frozen `ClearValue` instances
 */
export function createClearValueArray(parameters: {
  readonly orderedValues: ClearValueType[];
  readonly orderedHandles: readonly Handle[];
  readonly originToken: symbol;
}): readonly ClearValue[] {
  const { orderedValues: orderedClearValues, orderedHandles: orderedEncryptedValues, originToken } = parameters;
  if (orderedEncryptedValues.length !== orderedClearValues.length) {
    throw new InvalidTypeError(
      {
        subject: 'orderedClearValues',
        type: `Array(${orderedClearValues.length})`,
        expectedType: `Array(${orderedEncryptedValues.length}) — must match orderedHandles length`,
      },
      {},
    );
  }

  const result = orderedEncryptedValues.map((handle, i) =>
    createClearValue({
      handle: handle,
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      value: orderedClearValues[i]!,
      originToken,
    }),
  );

  return Object.freeze(result);
}

function _clearValueTypeToBigInt(value: ClearValueType): bigint {
  if (typeof value === 'boolean') {
    return value ? BigInt('0x01') : BigInt('0x00');
  }
  return BigInt(value);
}

export function abiEncodeClearValues(
  context: { readonly runtime: FhevmRuntime },
  args: {
    readonly orderedClearValues: readonly ClearValue[];
  },
): {
  abiTypes: Array<'uint256'>;
  abiValues: Array<string | bigint>;
  abiEncodedClearValues: BytesHex;
} {
  const orderedClearValues = args.orderedClearValues;
  const abiTypes: Array<'uint256'> = [];
  const abiValues: Array<string | bigint> = [];

  for (const clearValue of orderedClearValues) {
    const handleType: FheTypeId = clearValue.handle.fheTypeId;

    const clearTextValueBigInt = _clearValueTypeToBigInt(clearValue.value);

    //abiTypes.push(fhevmTypeInfo.solidityTypeName);
    abiTypes.push('uint256');

    switch (handleType) {
      // eaddress
      case 7: {
        // string
        abiValues.push(`0x${clearTextValueBigInt.toString(16).padStart(40, '0')}`);
        break;
      }
      // ebool
      case 0: {
        // bigint (0 or 1)
        if (clearTextValueBigInt !== BigInt(0) && clearTextValueBigInt !== BigInt(1)) {
          throw new Error(`Invalid ebool clear text value ${clearTextValueBigInt}. Expecting 0 or 1.`);
        }
        abiValues.push(clearTextValueBigInt);
        break;
      }
      case 2: //euint8
      case 3: //euint16
      case 4: //euint32
      case 5: //euint64
      case 6: //euint128
      case 8: {
        //euint256
        // bigint
        abiValues.push(clearTextValueBigInt);
        break;
      }
      default: {
        assertNever(handleType, `Unsupported Fhevm primitive type id: ${handleType}`);
      }
    }
  }

  // ABI encode the decryptedResult as done in the KMS, since all decrypted values
  // are native static types, they have same abi-encoding as uint256:
  const abiEncodedClearValues: BytesHex = context.runtime.ethereum.encode({
    types: abiTypes,
    values: abiValues,
  });

  return {
    abiTypes,
    abiValues,
    abiEncodedClearValues,
  };
}
