import { describe, it, expect, vi } from 'vitest';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ClearValue, Handle } from '../types/encryptedTypes-p.js';
import {
  isClearValue,
  assertIsClearValue,
  isClearValueArray,
  assertIsClearValueArray,
  createClearValue,
  createClearValueArray,
  clearValueToTypedValue,
  abiEncodeClearValues,
} from './ClearValue.js';
import { buildHandle } from './FhevmHandle.js';
import { asUint8Number, asUint64BigInt } from '../base/uint.js';
import { asAddress } from '../base/address.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/handle/ClearValue.test.ts
////////////////////////////////////////////////////////////////////////////////

const HASH21 = `0x${'ab'.repeat(21)}`;
const CHAIN_ID = 12345n;
const ADDRESS = asAddress(`0x${'ab'.repeat(20)}`);

const EUINT8_HANDLE: Handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: 2 });
const EUINT64_HANDLE: Handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: 5, index: 0 });
const EBOOL_HANDLE: Handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: 0, index: 1 });
const EADDRESS_HANDLE: Handle = buildHandle({ chainId: CHAIN_ID, hash21: HASH21, fheTypeId: 7, index: 2 });

describe('ClearValue', () => {
  //////////////////////////////////////////////////////////////////////////////
  // createClearValue
  //////////////////////////////////////////////////////////////////////////////

  it('createClearValue builds a ClearValue exposing value/type/handle', () => {
    const token = Symbol('token');
    const clearValue = createClearValue({ value: asUint8Number(42), handle: EUINT8_HANDLE, originToken: token });

    expect(clearValue.value).toBe(42);
    expect(clearValue.type).toBe('uint8');
    expect(clearValue.handle).toBe(EUINT8_HANDLE);
  });

  it('createClearValue supports ebool, euint64, and eaddress', () => {
    const token = Symbol('token');

    const boolValue = createClearValue({ value: true, handle: EBOOL_HANDLE, originToken: token });
    expect(boolValue.type).toBe('bool');
    expect(boolValue.value).toBe(true);

    const bigIntValue = createClearValue({ value: asUint64BigInt(123n), handle: EUINT64_HANDLE, originToken: token });
    expect(bigIntValue.type).toBe('uint64');
    expect(bigIntValue.value).toBe(123n);

    const addressValue = createClearValue({ value: ADDRESS, handle: EADDRESS_HANDLE, originToken: token });
    expect(addressValue.type).toBe('address');
    expect(addressValue.value).toBe(ADDRESS);
  });

  it('createClearValue throws when the value has the wrong JS type for the handle fheType', () => {
    const token = Symbol('token');

    expect(() =>
      createClearValue({
        // A string is not a valid euint8 value
        value: 'not-a-number' as never,
        handle: EUINT8_HANDLE,
        originToken: token,
      }),
    ).toThrow();
  });

  it('createClearValue enforces the euint8/euint64 upper bound', () => {
    const token = Symbol('token');

    expect(() =>
      createClearValue({
        value: 999999 as never, // exceeds MAX_UINT8 (255)
        handle: EUINT8_HANDLE,
        originToken: token,
      }),
    ).toThrow();

    expect(() =>
      createClearValue({
        value: (2n ** 64n) as never, // exceeds MAX_UINT64
        handle: EUINT64_HANDLE,
        originToken: token,
      }),
    ).toThrow();
  });

  it('createClearValue returns a frozen instance', () => {
    const token = Symbol('token');
    const clearValue = createClearValue({ value: asUint8Number(1), handle: EUINT8_HANDLE, originToken: token });

    expect(Object.isFrozen(clearValue)).toBe(true);
    expect(() => {
      'use strict';
      // @ts-expect-error - testing runtime immutability
      clearValue.injected = 'evil';
    }).toThrow();
  });

  it('createClearValue toString/toJSON do not leak the value', () => {
    const token = Symbol('token');
    const clearValue = createClearValue({ value: asUint8Number(42), handle: EUINT8_HANDLE, originToken: token });

    expect(String(clearValue)).toBe('ClearValue<euint8>');
    expect(String(clearValue)).not.toContain('42');
    expect(clearValue.toString()).not.toContain('42');
    expect(JSON.stringify(clearValue)).toBe(
      JSON.stringify({ handle: EUINT8_HANDLE.bytes32Hex, fheType: 'euint8' }),
    );
  });

  //////////////////////////////////////////////////////////////////////////////
  // isClearValue / assertIsClearValue
  //////////////////////////////////////////////////////////////////////////////

  it('isClearValue / assertIsClearValue only pass for the matching origin token', () => {
    const token = Symbol('token');
    const otherToken = Symbol('other-token');
    const clearValue = createClearValue({ value: asUint8Number(1), handle: EUINT8_HANDLE, originToken: token });

    expect(isClearValue(clearValue, token)).toBe(true);
    expect(() => assertIsClearValue(clearValue, { originToken: token })).not.toThrow();

    expect(isClearValue(clearValue, otherToken)).toBe(false);
    expect(() => assertIsClearValue(clearValue, { originToken: otherToken })).toThrow();
  });

  it('isClearValue / assertIsClearValue reject non-ClearValue values', () => {
    const token = Symbol('token');

    expect(isClearValue({ value: 1 }, token)).toBe(false);
    expect(isClearValue(null, token)).toBe(false);
    expect(isClearValue(42, token)).toBe(false);
    expect(() => assertIsClearValue('not-a-clear-value', { originToken: token })).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // isClearValueArray / assertIsClearValueArray
  //////////////////////////////////////////////////////////////////////////////

  it('isClearValueArray / assertIsClearValueArray validate every element', () => {
    const token = Symbol('token');
    const a = createClearValue({ value: asUint8Number(1), handle: EUINT8_HANDLE, originToken: token });
    const b = createClearValue({ value: true, handle: EBOOL_HANDLE, originToken: token });

    expect(isClearValueArray([a, b], token)).toBe(true);
    expect(() => assertIsClearValueArray([a, b], { originToken: token })).not.toThrow();

    expect(isClearValueArray([a, 'not-a-clear-value'], token)).toBe(false);
    expect(() => assertIsClearValueArray([a, 'not-a-clear-value'], { originToken: token })).toThrow();
  });

  it('assertIsClearValueArray rejects a non-array value', () => {
    const token = Symbol('token');
    expect(() => assertIsClearValueArray('not-an-array', { originToken: token })).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // clearValueToTypedValue
  //////////////////////////////////////////////////////////////////////////////

  it('clearValueToTypedValue converts a ClearValue to a TypedValue', () => {
    const token = Symbol('token');
    const clearValue = createClearValue({ value: asUint8Number(7), handle: EUINT8_HANDLE, originToken: token });

    const typedValue = clearValueToTypedValue(clearValue, token);
    expect(typedValue.type).toBe('uint8');
    expect(typedValue.value).toBe(7);
  });

  it('clearValueToTypedValue throws for a non-ClearValue or a mismatched token', () => {
    const token = Symbol('token');
    const otherToken = Symbol('other-token');
    const clearValue = createClearValue({ value: asUint8Number(7), handle: EUINT8_HANDLE, originToken: token });

    expect(() => clearValueToTypedValue(clearValue, otherToken)).toThrow();
    expect(() => clearValueToTypedValue('not-a-clear-value', token)).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // createClearValueArray
  //////////////////////////////////////////////////////////////////////////////

  it('createClearValueArray builds a frozen array of ClearValues in order', () => {
    const token = Symbol('token');
    const result = createClearValueArray({
      orderedValues: [asUint8Number(1), true],
      orderedHandles: [EUINT8_HANDLE, EBOOL_HANDLE],
      originToken: token,
    });

    expect(result).toHaveLength(2);
    expect(result[0]?.value).toBe(1);
    expect(result[0]?.handle).toBe(EUINT8_HANDLE);
    expect(result[1]?.value).toBe(true);
    expect(result[1]?.handle).toBe(EBOOL_HANDLE);
    expect(Object.isFrozen(result)).toBe(true);
  });

  it('createClearValueArray throws when array lengths do not match', () => {
    const token = Symbol('token');

    expect(() =>
      createClearValueArray({
        orderedValues: [asUint8Number(1)],
        orderedHandles: [EUINT8_HANDLE, EBOOL_HANDLE],
        originToken: token,
      }),
    ).toThrow();

    expect(() =>
      createClearValueArray({
        orderedValues: [asUint8Number(1), true],
        orderedHandles: [EUINT8_HANDLE],
        originToken: token,
      }),
    ).toThrow();
  });

  //////////////////////////////////////////////////////////////////////////////
  // abiEncodeClearValues
  //////////////////////////////////////////////////////////////////////////////

  function makeRuntimeStub() {
    const encode = vi.fn(() => '0xencoded');
    const runtime = { ethereum: { encode } } as unknown as FhevmRuntime;
    return { runtime, encode };
  }

  it('abiEncodeClearValues encodes ebool/euint/eaddress values as uint256', () => {
    const token = Symbol('token');
    const { runtime, encode } = makeRuntimeStub();

    const orderedClearValues = [
      createClearValue({ value: true, handle: EBOOL_HANDLE, originToken: token }),
      createClearValue({ value: false, handle: EBOOL_HANDLE, originToken: token }),
      createClearValue({ value: asUint8Number(5), handle: EUINT8_HANDLE, originToken: token }),
      createClearValue({ value: asUint64BigInt(123n), handle: EUINT64_HANDLE, originToken: token }),
      createClearValue({ value: ADDRESS, handle: EADDRESS_HANDLE, originToken: token }),
    ];

    const result = abiEncodeClearValues({ runtime }, { orderedClearValues });

    expect(result.abiTypes).toEqual(['uint256', 'uint256', 'uint256', 'uint256', 'uint256']);
    expect(result.abiValues).toEqual([1n, 0n, 5n, 123n, ADDRESS]);
    expect(result.abiEncodedClearValues).toBe('0xencoded');
    expect(encode).toHaveBeenCalledWith({ types: result.abiTypes, values: result.abiValues });
  });

  it('abiEncodeClearValues throws for an invalid ebool clear value', () => {
    const { runtime } = makeRuntimeStub();

    const forgedClearValue = { handle: { fheTypeId: 0 }, value: 2 } as unknown as ClearValue;

    expect(() => abiEncodeClearValues({ runtime }, { orderedClearValues: [forgedClearValue] })).toThrow(
      /Invalid ebool clear text value/,
    );
  });

  it('abiEncodeClearValues throws for an unsupported fheTypeId', () => {
    const { runtime } = makeRuntimeStub();

    const forgedClearValue = { handle: { fheTypeId: 99 }, value: 1 } as unknown as ClearValue;

    expect(() => abiEncodeClearValues({ runtime }, { orderedClearValues: [forgedClearValue] })).toThrow(
      /Unsupported Fhevm primitive type id: 99/,
    );
  });
});
