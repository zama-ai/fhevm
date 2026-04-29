import { describe, it, expect } from 'vitest';
import {
  createTypedValue,
  isTypedValue,
  assertIsTypedValue,
  assertIsTypedValueArray,
  createTypedValueArray,
  typedValueToBytes32Hex,
  TypedValueArrayBuilder,
} from './typedValue.js';

///////////////////////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/typedValue.test.ts
///////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////
// createTypedValue
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('createTypedValue', () => {
  it('bool', () => {
    // Valid
    expect(createTypedValue({ type: 'bool', value: true }).value).toBe(true);
    expect(createTypedValue({ type: 'bool', value: false }).value).toBe(false);
    expect(createTypedValue({ type: 'bool', value: true }).type).toBe('bool');
    expect(isTypedValue(createTypedValue({ type: 'bool', value: true }))).toBe(true);
  });

  it('uint8', () => {
    // Valid: number and bigint both normalize to number
    expect(createTypedValue({ type: 'uint8', value: 0 }).value).toBe(0);
    expect(createTypedValue({ type: 'uint8', value: 42 }).value).toBe(42);
    expect(createTypedValue({ type: 'uint8', value: 255 }).value).toBe(255);
    expect(createTypedValue({ type: 'uint8', value: 42n }).value).toBe(42);
    expect(createTypedValue({ type: 'uint8', value: 42 }).type).toBe('uint8');

    // Throws: out of range
    expect(() => createTypedValue({ type: 'uint8', value: 256 })).toThrow();
    expect(() => createTypedValue({ type: 'uint8', value: -1 })).toThrow();

    // Throws: wrong value type
    expect(() => createTypedValue({ type: 'uint8', value: null as unknown as number })).toThrow();
    expect(() => createTypedValue({ type: 'uint8', value: true as unknown as number })).toThrow();
    expect(() => createTypedValue({ type: 'uint8', value: '42' as unknown as number })).toThrow();
  });

  it('uint16 / uint32', () => {
    // Valid: normalized to number
    expect(createTypedValue({ type: 'uint16', value: 1000 }).value).toBe(1000);
    expect(createTypedValue({ type: 'uint16', value: 65535 }).value).toBe(65535);
    expect(createTypedValue({ type: 'uint32', value: 100_000 }).value).toBe(100_000);
    expect(createTypedValue({ type: 'uint32', value: 2 ** 32 - 1 }).value).toBe(2 ** 32 - 1);

    // Throws: out of range
    expect(() => createTypedValue({ type: 'uint16', value: 65536 })).toThrow();
    expect(() => createTypedValue({ type: 'uint32', value: 2 ** 32 })).toThrow();
  });

  it('uint64 / uint128 / uint256', () => {
    // Valid: normalized to bigint
    expect(createTypedValue({ type: 'uint64', value: 2n ** 32n }).value).toBe(2n ** 32n);
    expect(createTypedValue({ type: 'uint64', value: 2n ** 64n - 1n }).value).toBe(2n ** 64n - 1n);
    expect(createTypedValue({ type: 'uint128', value: 2n ** 64n }).value).toBe(2n ** 64n);
    expect(createTypedValue({ type: 'uint256', value: 2n ** 128n }).value).toBe(2n ** 128n);
    expect(createTypedValue({ type: 'uint64', value: 2n ** 32n }).type).toBe('uint64');

    // Throws: out of range
    expect(() => createTypedValue({ type: 'uint64', value: 2n ** 64n })).toThrow();
    expect(() => createTypedValue({ type: 'uint128', value: 2n ** 128n })).toThrow();
    expect(() => createTypedValue({ type: 'uint256', value: 2n ** 256n })).toThrow();
  });

  it('address', () => {
    const addr = '0xaaaabbbbccccddddeeeeffffaaaabbbbccccdddd';

    // Valid: lowercase 0x string stored as-is
    expect(createTypedValue({ type: 'address', value: addr }).type).toBe('address');
    expect(createTypedValue({ type: 'address', value: addr }).value).toBe(addr);

    // Valid: uint160 is an alias — type is canonicalized to 'address'
    expect(createTypedValue({ type: 'uint160' as unknown as 'address', value: addr }).type).toBe('address');
    expect(createTypedValue({ type: 'uint160' as unknown as 'address', value: addr }).value).toBe(addr);

    // Valid: bigint via uint160 is converted to a 0x address string
    const bigintAddr = BigInt(addr);
    expect(createTypedValue({ type: 'uint160' as unknown as 'address', value: bigintAddr as unknown as string }).type).toBe('address');
    expect(createTypedValue({ type: 'uint160' as unknown as 'address', value: bigintAddr as unknown as string }).value).toBe(addr);

    // Throws: missing 0x prefix
    expect(() => createTypedValue({ type: 'address', value: 'aaaabbbbccccddddeeeeffffaaaabbbbccccdddd' })).toThrow();
  });

  it('null / undefined input', () => {
    // Throws
    expect(() => createTypedValue(null as unknown as { type: 'uint8'; value: number })).toThrow();
    expect(() => createTypedValue(undefined as unknown as { type: 'uint8'; value: number })).toThrow();
  });

  it('invalid type name', () => {
    // Throws
    expect(() => createTypedValue({ type: 'string' as unknown as 'uint8', value: 'x' as unknown as number })).toThrow();
    expect(() => createTypedValue({ type: 'int8' as unknown as 'uint8', value: 1 })).toThrow();
  });

  it('result is frozen', () => {
    expect(Object.isFrozen(createTypedValue({ type: 'uint8', value: 42 }))).toBe(true);
    expect(Object.isFrozen(createTypedValue({ type: 'bool', value: true }))).toBe(true);
  });

  it('identity pass-through', () => {
    // Passing an existing TypedValue returns the same instance unchanged
    const v = createTypedValue({ type: 'uint8', value: 42 });
    expect(createTypedValue(v as unknown as { type: 'uint8'; value: number })).toBe(v);
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
// assertIsTypedValue
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('assertIsTypedValue', () => {
  it('assertIsTypedValue', () => {
    const v = createTypedValue({ type: 'uint8', value: 42 });

    // Valid
    expect(() => assertIsTypedValue(v, {})).not.toThrow();
    expect(() => assertIsTypedValue(v, { type: 'uint8' })).not.toThrow();

    // Throws: not a TypedValue
    expect(() => assertIsTypedValue(42, {})).toThrow();
    expect(() => assertIsTypedValue('hello', {})).toThrow();
    expect(() => assertIsTypedValue(null, {})).toThrow();
    expect(() => assertIsTypedValue({ type: 'uint8', value: 42 }, {})).toThrow();

    // Throws: type mismatch
    expect(() => assertIsTypedValue(v, { type: 'uint16' })).toThrow();
    expect(() => assertIsTypedValue(v, { type: 'bool' })).toThrow();
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
// assertIsTypedValueArray
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('assertIsTypedValueArray', () => {
  it('assertIsTypedValueArray', () => {
    const v1 = createTypedValue({ type: 'uint8', value: 1 });
    const v2 = createTypedValue({ type: 'bool', value: true });

    // Valid
    expect(() => assertIsTypedValueArray([], {})).not.toThrow();
    expect(() => assertIsTypedValueArray([v1, v2], {})).not.toThrow();

    // Throws: not an array
    expect(() => assertIsTypedValueArray(42, {})).toThrow();
    expect(() => assertIsTypedValueArray('hello', {})).toThrow();
    expect(() => assertIsTypedValueArray(null, {})).toThrow();

    // Throws: element is a plain object, not a TypedValue
    expect(() => assertIsTypedValueArray([v1, { type: 'uint8', value: 2 }], {})).toThrow();
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
// createTypedValueArray
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('createTypedValueArray', () => {
  it('createTypedValueArray', () => {
    const addr = '0xaaaabbbbccccddddeeeeffffaaaabbbbccccdddd';

    // Valid: empty input
    expect(createTypedValueArray([])).toEqual([]);

    // Valid: mixed types, all elements are TypedValues
    const arr = createTypedValueArray([
      { type: 'bool', value: true },
      { type: 'uint8', value: 42 },
      { type: 'address', value: addr },
    ]);
    expect(arr[0]!.type).toBe('bool');
    expect(arr[0]!.value).toBe(true);
    expect(arr[1]!.type).toBe('uint8');
    expect(arr[1]!.value).toBe(42);
    expect(arr[2]!.type).toBe('address');
    expect(arr[2]!.value).toBe(addr);
    expect(arr.every(isTypedValue)).toBe(true);

    // Throws: any invalid element propagates the error
    expect(() => createTypedValueArray([{ type: 'uint8', value: 256 }])).toThrow();
    expect(() => createTypedValueArray([{ type: 'uint8', value: 42 }, { type: 'uint8', value: 256 }])).toThrow();
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
// typedValueToBytes32Hex
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('typedValueToBytes32Hex', () => {
  it('typedValueToBytes32Hex', () => {
    // bool: true → ...0001, false → ...0000
    expect(typedValueToBytes32Hex(createTypedValue({ type: 'bool', value: true }))).toBe(
      '0x0000000000000000000000000000000000000000000000000000000000000001',
    );
    expect(typedValueToBytes32Hex(createTypedValue({ type: 'bool', value: false }))).toBe(
      '0x0000000000000000000000000000000000000000000000000000000000000000',
    );

    // uint: right-aligned in 32 bytes
    expect(typedValueToBytes32Hex(createTypedValue({ type: 'uint8', value: 42 }))).toBe(
      '0x000000000000000000000000000000000000000000000000000000000000002a',
    );
    expect(typedValueToBytes32Hex(createTypedValue({ type: 'uint256', value: 2n ** 128n }))).toBe(
      '0x0000000000000000000000000000000100000000000000000000000000000000',
    );

    // address: right-aligned, left-padded with 12 zero bytes
    expect(typedValueToBytes32Hex(createTypedValue({ type: 'address', value: '0xaaaabbbbccccddddeeeeffffaaaabbbbccccdddd' }))).toBe(
      '0x000000000000000000000000aaaabbbbccccddddeeeeffffaaaabbbbccccdddd',
    );

    // Throws: not a TypedValue
    expect(() => typedValueToBytes32Hex({ type: 'uint8', value: 42 } as unknown as ReturnType<typeof createTypedValue>)).toThrow();
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
// TypedValueArrayBuilder
///////////////////////////////////////////////////////////////////////////////////////////////////

describe('TypedValueArrayBuilder', () => {
  const addr = '0xaaaabbbbccccddddeeeeffffaaaabbbbccccdddd';

  it('build', () => {
    // Empty builder produces empty frozen array
    const empty = new TypedValueArrayBuilder().build();
    expect(empty).toEqual([]);
    expect(Object.isFrozen(empty)).toBe(true);

    // Non-empty build is also frozen
    expect(Object.isFrozen(new TypedValueArrayBuilder().addBool(false).build())).toBe(true);

    // build() snapshots state — further adds do not affect prior snapshots
    const builder = new TypedValueArrayBuilder().addUint8(1);
    const first = builder.build();
    builder.addUint8(2);
    expect(first.length).toBe(1);
    expect(builder.build().length).toBe(2);
  });

  it('method chaining', () => {
    const builder = new TypedValueArrayBuilder();
    expect(builder.addBool(true)).toBe(builder);
    expect(builder.addUint8(1)).toBe(builder);
    expect(builder.addUint16(2)).toBe(builder);
    expect(builder.addAddress(addr)).toBe(builder);
  });

  it('addBool / addUint8 / addUint16 / addUint32', () => {
    // Valid: uint8/16/32 normalize to number
    const arr = new TypedValueArrayBuilder()
      .addBool(true)
      .addUint8(42)
      .addUint16(1000)
      .addUint32(100_000)
      .build();
    expect(arr[0]!.type).toBe('bool');
    expect(arr[0]!.value).toBe(true);
    expect(arr[1]!.value).toBe(42);
    expect(arr[2]!.value).toBe(1000);
    expect(arr[3]!.value).toBe(100_000);

    // Valid: addBool also accepts a TypedValueLike object
    const [v] = new TypedValueArrayBuilder().addBool({ type: 'bool' as const, value: false }).build();
    expect(v!.type).toBe('bool');
    expect(v!.value).toBe(false);
    expect(isTypedValue(v)).toBe(true);
  });

  it('addUint64 / addUint128 / addUint256', () => {
    // Valid: normalized to bigint
    const arr = new TypedValueArrayBuilder()
      .addUint64(2n ** 32n)
      .addUint128(2n ** 64n)
      .addUint256(2n ** 128n)
      .build();
    expect(arr[0]!.value).toBe(2n ** 32n);
    expect(arr[1]!.value).toBe(2n ** 64n);
    expect(arr[2]!.value).toBe(2n ** 128n);
  });

  it('addAddress', () => {
    // Valid
    const [v] = new TypedValueArrayBuilder().addAddress(addr).build();
    expect(v!.type).toBe('address');
    expect(v!.value).toBe(addr);

    // Throws: missing 0x prefix
    expect(() => new TypedValueArrayBuilder().addAddress('aaaabbbbccccddddeeeeffffaaaabbbbccccdddd')).toThrow();
  });

  it('addTypedValue', () => {
    const existing = createTypedValue({ type: 'uint8', value: 7 });

    // Valid: appends and returns the same instance
    const [v] = new TypedValueArrayBuilder().addTypedValue(existing).build();
    expect(v).toBe(existing);

    // Throws: plain object is not a TypedValue
    expect(() => new TypedValueArrayBuilder().addTypedValue({ type: 'uint8', value: 7 } as unknown as ReturnType<typeof createTypedValue>)).toThrow();

    // Throws: TypedValue with mismatched type
    const uint16 = createTypedValue({ type: 'uint16', value: 1000 });
    expect(() => new TypedValueArrayBuilder().addUint8(uint16 as unknown as number)).toThrow();
  });
});

///////////////////////////////////////////////////////////////////////////////////////////////////
