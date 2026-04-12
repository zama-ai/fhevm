import { asAddress } from './address.js';
import { toBoolean } from './boolean.js';
import type { ErrorMetadataParams } from './errors/ErrorBase.js';
import { InvalidTypeError } from './errors/InvalidTypeError.js';
import type {
  AddressValueLike,
  BoolValueLike,
  TypedValue,
  TypedValueFrom,
  TypedValueLike,
  TypedValueOfBase,
  Uint128ValueLike,
  Uint16ValueLike,
  Uint256ValueLike,
  Uint32ValueLike,
  Uint64ValueLike,
  Uint8ValueLike,
  ValueType,
  ValueTypeMap,
  ValueTypeName,
} from '../types/primitives.js';
import { asUintForType, normalizeUintForType } from './uint.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Internal implementation. Not exported — external code cannot instantiate.
 *
 * Security relies on:
 * - Class not being exported (no `new` from outside)
 * - `Object.freeze` on every instance (immutability)
 * - Private fields (`#type`, `#value`) inaccessible from outside
 * - `Object.freeze` on prototype (no prototype pollution)
 */
class TypedValueImpl<T extends ValueTypeName> implements TypedValueOfBase<T> {
  readonly #type: T;
  readonly #value: ValueTypeMap[T];

  constructor(value: ValueTypeMap[T], type: T) {
    this.#value = value;
    this.#type = type;
  }

  public get type(): T {
    return this.#type;
  }

  public get value(): ValueTypeMap[T] {
    return this.#value;
  }

  /**
   * Safe string representation that does not expose the value.
   */
  public toString(): string {
    return `TypedValue<${this.#type}>`;
  }

  /**
   * Safe JSON serialization that does not expose the value.
   */
  public toJSON(): { type: T } {
    return { type: this.#type };
  }
}

Object.freeze(TypedValueImpl.prototype);

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns `true` if `value` was created via {@link createTypedValue}.
 *
 * Uses `instanceof` against the non-exported `TypedValueImpl` class,
 * which is unforgeable in same-realm contexts.
 */
export function isTypedValue<T extends ValueTypeName>(
  value: unknown,
  options: { type: T },
): value is TypedValue & { readonly type: T };
export function isTypedValue(value: unknown): value is TypedValue;
export function isTypedValue(
  value: unknown,
  options?: { type: ValueTypeName },
): value is TypedValue {
  if (!(value instanceof TypedValueImpl)) {
    return false;
  }
  if (options?.type !== undefined) {
    return value.type === options.type;
  }
  return true;
}

/**
 * Asserts that `value` was created via {@link createTypedValue}.
 *
 * @throws {InvalidTypeError} If the value is not a `TypedValue` instance.
 */
export function assertIsTypedValue<T extends ValueTypeName>(
  value: unknown,
  options: { type: T; subject?: string } & ErrorMetadataParams,
): asserts value is TypedValue & { readonly type: T };
export function assertIsTypedValue(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is TypedValue;
export function assertIsTypedValue(
  value: unknown,
  options: { type?: ValueTypeName; subject?: string } & ErrorMetadataParams,
): asserts value is TypedValue {
  const expectedType =
    options.type !== undefined ? `TypedValue<${options.type}>` : 'TypedValue';

  const isValid =
    options.type !== undefined
      ? isTypedValue(value, { type: options.type })
      : isTypedValue(value);

  if (!isValid) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType,
      },
      options,
    );
  }
}

/**
 * Asserts that `values` is an array where every element was created
 * via {@link createTypedValue}.
 *
 * @throws {InvalidTypeError} If the value is not an array, or if any
 * element is not a `TypedValue` instance (error includes the index).
 */
export function assertIsTypedValueArray(
  values: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts values is TypedValue[] {
  if (!Array.isArray(values)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof values,
        expectedType: 'TypedValue[]',
      },
      options,
    );
  }
  for (let i = 0; i < values.length; ++i) {
    if (!isTypedValue(values[i])) {
      throw new InvalidTypeError(
        {
          subject: options.subject,
          index: i,
          type: typeof values[i],
          expectedType: 'TypedValue',
        },
        options,
      );
    }
  }
}

/**
 * Creates a validated and immutable {@link TypedValueOf}.
 *
 * Validation steps:
 * 1. **Boolean values:** Validated via `asBoolean()`
 * 2. **Addresses:** Validated and checksummed via `asChecksummedAddress()` (EIP-55)
 * 3. **Uint values:** Validated via `asUintForType()` (range check)
 *
 * @param input - The loose typed value to validate
 * @returns A validated and frozen `TypedValue` with proper type narrowing
 * @throws {InvalidTypeError} If validation fails
 *
 * @example
 * ```typescript
 * const uint8 = createTypedValue({ type: 'uint8', value: 42 });
 * // Type: TypedValue<'uint8', Uint8>
 *
 * const addr = createTypedValue({
 *   type: 'address',
 *   value: '0x742d35cc6634c0532925a3b844bc9e7595f0beb'
 * });
 * // Type: TypedValue<'address', ChecksummedAddress>
 * ```
 */
export function createTypedValue<InputType extends TypedValueLike>(
  input: InputType,
): TypedValue & { readonly type: InputType['type'] } {
  if ((input as unknown) == null || typeof input !== 'object') {
    throw new InvalidTypeError(
      {
        subject: 'input',
        type: typeof input,
        expectedType: 'InputTypedValue ({ type, value })',
      },
      {},
    );
  }

  if (isTypedValue(input)) {
    return input;
  }

  const expectedType = input.type;

  let validatedValue: ValueType;

  if (expectedType === 'bool') {
    validatedValue = toBoolean(input.value, {});
  } else if (expectedType === 'address') {
    validatedValue = asAddress(input.value);
  } else {
    validatedValue = normalizeUintForType(
      asUintForType(input.value, expectedType, {}),
      expectedType,
    );
  }

  const v: TypedValueOfBase<typeof expectedType> = new TypedValueImpl(
    validatedValue,
    expectedType,
  );
  Object.freeze(v);
  return v as TypedValueFrom<InputType>;
}

/**
 * Mapped tuple type that preserves per-element type narrowing.
 * @internal
 */
type TypedValueArrayFrom<T extends readonly TypedValueLike[]> = {
  [K in keyof T]: TypedValueFrom<T[K]>;
};

/**
 * Creates an array of validated {@link TypedValueOf}s from a tuple of inputs.
 *
 * Preserves per-element type narrowing:
 * ```typescript
 * const [b, n] = createTypedValueArray([
 *   { type: 'bool', value: true },
 *   { type: 'uint8', value: 42 },
 * ]);
 * // b: BoolValue, n: Uint8Value
 * ```
 */
export function createTypedValueArray<T extends readonly TypedValueLike[]>(
  inputs: [...T],
): TypedValueArrayFrom<T> {
  return inputs.map(createTypedValue) as unknown as TypedValueArrayFrom<T>;
}

/**
 * Returns `true` if every element was created via {@link createTypedValue}.
 */
export function isTypedValueArray(
  arr: readonly unknown[],
): arr is TypedValue[] {
  return arr.every((v) => isTypedValue(v));
}

export class TypedValueArrayBuilder {
  readonly #arr: TypedValue[] = [];

  public addBool(value: boolean | number | bigint | BoolValueLike): this {
    return this.#push('bool', value);
  }

  public addUint8(value: number | bigint | Uint8ValueLike): this {
    return this.#push('uint8', value);
  }

  public addUint16(value: number | bigint | Uint16ValueLike): this {
    return this.#push('uint16', value);
  }

  public addUint32(value: number | bigint | Uint32ValueLike): this {
    return this.#push('uint32', value);
  }

  public addUint64(value: number | bigint | Uint64ValueLike): this {
    return this.#push('uint64', value);
  }

  public addUint128(value: number | bigint | Uint128ValueLike): this {
    return this.#push('uint128', value);
  }

  public addUint256(value: number | bigint | Uint256ValueLike): this {
    return this.#push('uint256', value);
  }

  public addAddress(value: string | AddressValueLike): this {
    return this.#push('address', value);
  }

  public addTypedValue(typedValue: TypedValue): this {
    if (!isTypedValue(typedValue)) {
      throw new InvalidTypeError(
        {
          subject: 'typedValue',
          type: typeof typedValue,
          expectedType: 'TypedValue',
        },
        {},
      );
    }
    this.#arr.push(typedValue);
    return this;
  }

  #push(typeName: ValueTypeName, value: unknown): this {
    if (isTypedValue(value, { type: typeName })) {
      this.#arr.push(value);
    } else if (typeof value === 'object' && value !== null) {
      const tv = value as TypedValue;
      if (tv.type !== typeName) {
        throw new InvalidTypeError(
          {
            subject: 'value',
            type: tv.type,
            expectedType: typeName,
          },
          {},
        );
      }
      this.#arr.push(
        createTypedValue({ type: typeName, value: tv.value } as TypedValueLike),
      );
    } else {
      this.#arr.push(
        createTypedValue({ type: typeName, value } as TypedValueLike),
      );
    }
    return this;
  }

  public build(): readonly TypedValue[] {
    return Object.freeze([...this.#arr]);
  }
}
