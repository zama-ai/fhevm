import { describe, it, expect } from 'vitest';
import {
  isFheTypeId,
  isFheType,
  isEncryptionBits,
  assertIsEncryptionBits,
  assertIsEncryptionBitsArray,
  fheTypeIdFromEncryptionBits,
  fheTypeIdFromName,
  fheTypeNameFromId,
  solidityPrimitiveTypeNameFromFheTypeId,
  encryptionBitsFromFheTypeId,
  encryptionBitsFromFheType,
  asEncryptionBits,
  asFheTypeId,
  asFheType,
  bigintToClearValueType,
  bytesToClearValueType,
  asClearValueType,
  toClearValueType,
  fheTypeNameFromTypeName,
  typeNameFromFheTypeName,
} from './FheType.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';

////////////////////////////////////////////////////////////////////////////////
//
// Jest Command line
// =================
//
// npx jest --colors --passWithNoTests ./src/sdk/FheType.test.ts
// npx jest --colors --passWithNoTests ./src/sdk/FheType.test.ts --testNamePattern=xxx
// npx jest --colors --passWithNoTests --coverage ./src/sdk/FheType.test.ts --collectCoverageFrom=./src/sdk/FheType.ts
//
// Maximum Code Coverage: 98% because of `_assertMinimumEncryptionBitWidth` private function which is never accessible
// This function is here for defensive purpose in case of potential future change
//
////////////////////////////////////////////////////////////////////////////////

describe('FheType', () => {
  //////////////////////////////////////////////////////////////////////////////
  // isFheTypeId
  //////////////////////////////////////////////////////////////////////////////

  describe('isFheTypeId', () => {
    it('returns true for valid FheTypeIds', () => {
      expect(isFheTypeId(0)).toBe(true); // ebool
      expect(isFheTypeId(2)).toBe(true); // euint8
      expect(isFheTypeId(3)).toBe(true); // euint16
      expect(isFheTypeId(4)).toBe(true); // euint32
      expect(isFheTypeId(5)).toBe(true); // euint64
      expect(isFheTypeId(6)).toBe(true); // euint128
      expect(isFheTypeId(7)).toBe(true); // eaddress
      expect(isFheTypeId(8)).toBe(true); // euint256
    });

    it('returns false for deprecated euint4 (id 1)', () => {
      expect(isFheTypeId(1)).toBe(false);
    });

    it('returns false for invalid ids', () => {
      expect(isFheTypeId(-1)).toBe(false);
      expect(isFheTypeId(9)).toBe(false);
      expect(isFheTypeId(100)).toBe(false);
    });

    it('returns false for non-number values', () => {
      expect(isFheTypeId('0')).toBe(false);
      expect(isFheTypeId(null)).toBe(false);
      expect(isFheTypeId(undefined)).toBe(false);
      expect(isFheTypeId({})).toBe(false);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // isFheType
  //////////////////////////////////////////////////////////////////////////////

  describe('isFheType', () => {
    it('returns true for valid FheTypeNames', () => {
      expect(isFheType('ebool')).toBe(true);
      expect(isFheType('euint8')).toBe(true);
      expect(isFheType('euint16')).toBe(true);
      expect(isFheType('euint32')).toBe(true);
      expect(isFheType('euint64')).toBe(true);
      expect(isFheType('euint128')).toBe(true);
      expect(isFheType('eaddress')).toBe(true);
      expect(isFheType('euint256')).toBe(true);
    });

    it('returns false for deprecated euint4', () => {
      expect(isFheType('euint4')).toBe(false);
    });

    it('returns false for invalid names', () => {
      expect(isFheType('euint512')).toBe(false);
      expect(isFheType('uint8')).toBe(false);
      expect(isFheType('bool')).toBe(false);
      expect(isFheType('')).toBe(false);
    });

    it('returns false for non-string values', () => {
      expect(isFheType(0)).toBe(false);
      expect(isFheType(null)).toBe(false);
      expect(isFheType(undefined)).toBe(false);
      expect(isFheType({})).toBe(false);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // isEncryptionBits
  //////////////////////////////////////////////////////////////////////////////

  describe('isEncryptionBits', () => {
    it('returns true for valid encryption bit widths', () => {
      expect(isEncryptionBits(2)).toBe(true); // ebool
      expect(isEncryptionBits(8)).toBe(true); // euint8
      expect(isEncryptionBits(16)).toBe(true); // euint16
      expect(isEncryptionBits(32)).toBe(true); // euint32
      expect(isEncryptionBits(64)).toBe(true); // euint64
      expect(isEncryptionBits(128)).toBe(true); // euint128
      expect(isEncryptionBits(160)).toBe(true); // eaddress
      expect(isEncryptionBits(256)).toBe(true); // euint256
    });

    it('returns false for deprecated euint4 bitwidth', () => {
      expect(isEncryptionBits(4)).toBe(false);
    });

    it('returns false for invalid bit widths', () => {
      expect(isEncryptionBits(0)).toBe(false);
      expect(isEncryptionBits(1)).toBe(false);
      expect(isEncryptionBits(3)).toBe(false);
      expect(isEncryptionBits(512)).toBe(false);
    });

    it('returns false for non-number values', () => {
      expect(isEncryptionBits('8')).toBe(false);
      expect(isEncryptionBits(null)).toBe(false);
      expect(isEncryptionBits(undefined)).toBe(false);
      expect(isEncryptionBits({})).toBe(false);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertIsEncryptionBits
  //////////////////////////////////////////////////////////////////////////////

  describe('assertIsEncryptionBits', () => {
    it('does not throw for valid encryption bits', () => {
      expect(() => assertIsEncryptionBits(2, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(8, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(16, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(32, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(64, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(128, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(160, {})).not.toThrow();
      expect(() => assertIsEncryptionBits(256, {})).not.toThrow();
    });

    it('throws InvalidTypeError for invalid encryption bits', () => {
      expect(() => assertIsEncryptionBits(4, {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBits(0, {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBits(512, {})).toThrow(InvalidTypeError);
    });

    it('throws InvalidTypeError for non-number values', () => {
      expect(() => assertIsEncryptionBits('8', {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBits(null, {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBits(undefined, {})).toThrow(InvalidTypeError);
    });

    it('includes varName in error when provided', () => {
      expect(() => assertIsEncryptionBits(4, { subject: 'myVar' })).toThrow(/myVar/);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // assertIsEncryptionBitsArray
  //////////////////////////////////////////////////////////////////////////////

  describe('assertIsEncryptionBitsArray', () => {
    it('does not throw for valid encryption bits array', () => {
      expect(() => assertIsEncryptionBitsArray([2, 8, 16], {})).not.toThrow();
      expect(() => assertIsEncryptionBitsArray([32, 64, 128, 160, 256], {})).not.toThrow();
      expect(() => assertIsEncryptionBitsArray([], {})).not.toThrow();
    });

    it('throws InvalidTypeError for non-array values', () => {
      expect(() => assertIsEncryptionBitsArray('not-an-array', {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBitsArray(123, {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBitsArray(null, {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBitsArray(undefined, {})).toThrow(InvalidTypeError);
    });

    it('throws InvalidTypeError for array with invalid encryption bits', () => {
      expect(() => assertIsEncryptionBitsArray([8, 4, 16], {})).toThrow(InvalidTypeError);
      expect(() => assertIsEncryptionBitsArray([8, 'invalid', 16], {})).toThrow(InvalidTypeError);
    });

    it('includes varName with index in error when provided', () => {
      expect(() => assertIsEncryptionBitsArray([8, 4], { subject: 'myArray' })).toThrow(/myArray\[1\]/);
    });

    it('throws without varName when not provided', () => {
      expect(() => assertIsEncryptionBitsArray([4], {})).toThrow(InvalidTypeError);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // fheTypeIdFromEncryptionBits
  //////////////////////////////////////////////////////////////////////////////

  describe('fheTypeIdFromEncryptionBits', () => {
    it('returns correct FheTypeId for valid bit widths', () => {
      expect(fheTypeIdFromEncryptionBits(2)).toBe(0); // ebool
      expect(fheTypeIdFromEncryptionBits(8)).toBe(2); // euint8
      expect(fheTypeIdFromEncryptionBits(16)).toBe(3); // euint16
      expect(fheTypeIdFromEncryptionBits(32)).toBe(4); // euint32
      expect(fheTypeIdFromEncryptionBits(64)).toBe(5); // euint64
      expect(fheTypeIdFromEncryptionBits(128)).toBe(6); // euint128
      expect(fheTypeIdFromEncryptionBits(160)).toBe(7); // eaddress
      expect(fheTypeIdFromEncryptionBits(256)).toBe(8); // euint256
    });

    it('undefined id for invalid bit widths', () => {
      expect(fheTypeIdFromEncryptionBits(4 as any)).toBeUndefined();
      expect(fheTypeIdFromEncryptionBits(512 as any)).toBeUndefined();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // asEncryptionBits
  //////////////////////////////////////////////////////////////////////////////

  describe('asEncryptionBits', () => {
    it('throws for invalid bit widths', () => {
      expect(() => asEncryptionBits(4 as any)).toThrow();
      expect(() => asEncryptionBits(4 as any)).toThrow('expected 2|8|16|32|64|128|160|256');
      expect(() => asEncryptionBits(512 as any)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // fheTypeIdFromName
  //////////////////////////////////////////////////////////////////////////////

  describe('fheTypeIdFromName', () => {
    it('returns correct FheTypeId for valid names', () => {
      expect(fheTypeIdFromName('ebool')).toBe(0);
      expect(fheTypeIdFromName('euint8')).toBe(2);
      expect(fheTypeIdFromName('euint16')).toBe(3);
      expect(fheTypeIdFromName('euint32')).toBe(4);
      expect(fheTypeIdFromName('euint64')).toBe(5);
      expect(fheTypeIdFromName('euint128')).toBe(6);
      expect(fheTypeIdFromName('eaddress')).toBe(7);
      expect(fheTypeIdFromName('euint256')).toBe(8);
    });

    it('undefined id for invalid names', () => {
      expect(fheTypeIdFromName('euint4' as any)).toBeUndefined();
      expect(fheTypeIdFromName('invalid' as any)).toBeUndefined();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // asFheType
  //////////////////////////////////////////////////////////////////////////////

  describe('asFheType', () => {
    it('throws for invalid names', () => {
      expect(() => asFheType('euint4' as any)).toThrow();
      expect(() => asFheType('euint4' as any)).toThrow(
        'expected ebool|euint8|euint16|euint32|euint64|euint128|eaddress|euint256',
      );
      expect(() => asFheType('invalid' as any)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // fheTypeNameFromId
  //////////////////////////////////////////////////////////////////////////////

  describe('fheTypeNameFromId', () => {
    it('returns correct FheTypeName for valid ids', () => {
      expect(fheTypeNameFromId(0)).toBe('ebool');
      expect(fheTypeNameFromId(2)).toBe('euint8');
      expect(fheTypeNameFromId(3)).toBe('euint16');
      expect(fheTypeNameFromId(4)).toBe('euint32');
      expect(fheTypeNameFromId(5)).toBe('euint64');
      expect(fheTypeNameFromId(6)).toBe('euint128');
      expect(fheTypeNameFromId(7)).toBe('eaddress');
      expect(fheTypeNameFromId(8)).toBe('euint256');
    });

    it('undefined name for invalid ids', () => {
      expect(fheTypeNameFromId(1 as any)).toBeUndefined();
      expect(fheTypeNameFromId(9 as any)).toBeUndefined();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // asFheTypeId
  //////////////////////////////////////////////////////////////////////////////

  describe('asFheTypeId', () => {
    it('throws for invalid ids', () => {
      expect(() => asFheTypeId(1 as any)).toThrow();
      expect(() => asFheTypeId(1 as any)).toThrow('expected 0|2|3|4|5|6|7|8');
      expect(() => asFheTypeId(9 as any)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // solidityPrimitiveTypeNameFromFheTypeId
  //////////////////////////////////////////////////////////////////////////////

  describe('solidityPrimitiveTypeNameFromFheTypeId', () => {
    it('returns bool for ebool (id 0)', () => {
      expect(solidityPrimitiveTypeNameFromFheTypeId(0)).toBe('bool');
    });

    it('returns address for eaddress (id 7)', () => {
      expect(solidityPrimitiveTypeNameFromFheTypeId(7)).toBe('address');
    });

    it('returns uint256 for all euint types', () => {
      expect(solidityPrimitiveTypeNameFromFheTypeId(2)).toBe('uint256'); // euint8
      expect(solidityPrimitiveTypeNameFromFheTypeId(3)).toBe('uint256'); // euint16
      expect(solidityPrimitiveTypeNameFromFheTypeId(4)).toBe('uint256'); // euint32
      expect(solidityPrimitiveTypeNameFromFheTypeId(5)).toBe('uint256'); // euint64
      expect(solidityPrimitiveTypeNameFromFheTypeId(6)).toBe('uint256'); // euint128
      expect(solidityPrimitiveTypeNameFromFheTypeId(8)).toBe('uint256'); // euint256
    });

    it('undefined solidity type name for invalid ids', () => {
      expect(solidityPrimitiveTypeNameFromFheTypeId(1 as any)).toBeUndefined();
      expect(solidityPrimitiveTypeNameFromFheTypeId(9 as any)).toBeUndefined();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // encryptionBitsFromFheTypeId
  //////////////////////////////////////////////////////////////////////////////

  describe('encryptionBitsFromFheTypeId', () => {
    it('returns correct bit widths for valid ids', () => {
      expect(encryptionBitsFromFheTypeId(0)).toBe(2); // ebool
      expect(encryptionBitsFromFheTypeId(2)).toBe(8); // euint8
      expect(encryptionBitsFromFheTypeId(3)).toBe(16); // euint16
      expect(encryptionBitsFromFheTypeId(4)).toBe(32); // euint32
      expect(encryptionBitsFromFheTypeId(5)).toBe(64); // euint64
      expect(encryptionBitsFromFheTypeId(6)).toBe(128); // euint128
      expect(encryptionBitsFromFheTypeId(7)).toBe(160); // eaddress
      expect(encryptionBitsFromFheTypeId(8)).toBe(256); // euint256
    });

    it('throws for invalid ids', () => {
      expect(() => encryptionBitsFromFheTypeId(1 as any)).toThrow();
      expect(() => encryptionBitsFromFheTypeId(1 as any)).toThrow(
        'Invalid FheType encryption bit width: undefined. Minimum encryption bit width is 2 bits.',
      );
      expect(() => encryptionBitsFromFheTypeId(9 as any)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // encryptionBitsFromFheType
  //////////////////////////////////////////////////////////////////////////////

  describe('encryptionBitsFromFheType', () => {
    it('returns correct bit widths for valid names', () => {
      expect(encryptionBitsFromFheType('ebool')).toBe(2);
      expect(encryptionBitsFromFheType('euint8')).toBe(8);
      expect(encryptionBitsFromFheType('euint16')).toBe(16);
      expect(encryptionBitsFromFheType('euint32')).toBe(32);
      expect(encryptionBitsFromFheType('euint64')).toBe(64);
      expect(encryptionBitsFromFheType('euint128')).toBe(128);
      expect(encryptionBitsFromFheType('eaddress')).toBe(160);
      expect(encryptionBitsFromFheType('euint256')).toBe(256);
    });

    it('throws for invalid names', () => {
      expect(() => encryptionBitsFromFheType('euint4' as any)).toThrow();
      expect(() => encryptionBitsFromFheType('euint4' as any)).toThrow(
        'Invalid FheType encryption bit width: undefined. Minimum encryption bit width is 2 bits.',
      );
      expect(() => encryptionBitsFromFheType('invalid' as any)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // fheTypeNameFromTypeName
  //////////////////////////////////////////////////////////////////////////////

  describe('fheTypeNameFromTypeName', () => {
    it('returns correct FheType for each ValueTypeName', () => {
      expect(fheTypeNameFromTypeName('bool')).toBe('ebool');
      expect(fheTypeNameFromTypeName('uint8')).toBe('euint8');
      expect(fheTypeNameFromTypeName('uint16')).toBe('euint16');
      expect(fheTypeNameFromTypeName('uint32')).toBe('euint32');
      expect(fheTypeNameFromTypeName('uint64')).toBe('euint64');
      expect(fheTypeNameFromTypeName('uint128')).toBe('euint128');
      expect(fheTypeNameFromTypeName('uint256')).toBe('euint256');
      expect(fheTypeNameFromTypeName('address')).toBe('eaddress');
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // typeNameFromFheTypeName
  //////////////////////////////////////////////////////////////////////////////

  describe('typeNameFromFheTypeName', () => {
    it('returns correct ValueTypeName for each FheType', () => {
      expect(typeNameFromFheTypeName('ebool')).toBe('bool');
      expect(typeNameFromFheTypeName('euint8')).toBe('uint8');
      expect(typeNameFromFheTypeName('euint16')).toBe('uint16');
      expect(typeNameFromFheTypeName('euint32')).toBe('uint32');
      expect(typeNameFromFheTypeName('euint64')).toBe('uint64');
      expect(typeNameFromFheTypeName('euint128')).toBe('uint128');
      expect(typeNameFromFheTypeName('euint256')).toBe('uint256');
      expect(typeNameFromFheTypeName('eaddress')).toBe('address');
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // bigintToClearValueType
  //////////////////////////////////////////////////////////////////////////////

  describe('bigintToClearValueType', () => {
    it('converts bigint to boolean for ebool', () => {
      expect(bigintToClearValueType('ebool', 0n)).toBe(false);
      expect(bigintToClearValueType('ebool', 1n)).toBe(true);
      expect(bigintToClearValueType('ebool', 255n)).toBe(true);
    });

    it('converts bigint to checksummed address string for eaddress', () => {
      expect(bigintToClearValueType('eaddress', 1n)).toBe('0x0000000000000000000000000000000000000001');
      expect(bigintToClearValueType('eaddress', 0n)).toBe('0x0000000000000000000000000000000000000000');
    });

    it('converts bigint to number for euint8/16/32', () => {
      expect(bigintToClearValueType('euint8', 0n)).toBe(0);
      expect(bigintToClearValueType('euint8', 255n)).toBe(255);
      expect(bigintToClearValueType('euint16', 65535n)).toBe(65535);
      expect(bigintToClearValueType('euint32', 4294967295n)).toBe(4294967295);
    });

    it('throws for out-of-range bigint for euint8/16/32', () => {
      expect(() => bigintToClearValueType('euint8', 256n)).toThrow();
      expect(() => bigintToClearValueType('euint16', 65536n)).toThrow();
      expect(() => bigintToClearValueType('euint32', 4294967296n)).toThrow();
    });

    it('converts bigint to bigint for euint64/128/256', () => {
      expect(bigintToClearValueType('euint64', 100n)).toBe(100n);
      expect(bigintToClearValueType('euint128', 100n)).toBe(100n);
      expect(bigintToClearValueType('euint256', 100n)).toBe(100n);
    });

    it('throws for out-of-range bigint for euint64/128/256', () => {
      expect(() => bigintToClearValueType('euint64', 2n ** 64n)).toThrow();
      expect(() => bigintToClearValueType('euint128', 2n ** 128n)).toThrow();
      expect(() => bigintToClearValueType('euint256', 2n ** 256n)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // bytesToClearValueType
  //////////////////////////////////////////////////////////////////////////////

  describe('bytesToClearValueType', () => {
    it('converts zero byte to false for ebool', () => {
      expect(bytesToClearValueType('ebool', new Uint8Array([0]))).toBe(false);
    });

    it('converts non-zero byte to true for ebool', () => {
      expect(bytesToClearValueType('ebool', new Uint8Array([1]))).toBe(true);
    });

    it('converts bytes to number for euint8', () => {
      expect(bytesToClearValueType('euint8', new Uint8Array([42]))).toBe(42);
    });

    it('converts bytes to bigint for euint64', () => {
      const bytes = new Uint8Array(8);
      bytes[7] = 42;
      expect(bytesToClearValueType('euint64', bytes)).toBe(42n);
    });

    it('throws for out-of-range bytes for euint8', () => {
      expect(() => bytesToClearValueType('euint8', new Uint8Array([1, 2]))).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // asClearValueType
  //////////////////////////////////////////////////////////////////////////////

  describe('asClearValueType', () => {
    it('accepts boolean for ebool', () => {
      expect(asClearValueType('ebool', true)).toBe(true);
      expect(asClearValueType('ebool', false)).toBe(false);
    });

    it('rejects non-boolean for ebool', () => {
      expect(() => asClearValueType('ebool', 1)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('ebool', 0)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('ebool', 'true')).toThrow(InvalidTypeError);
    });

    it('accepts valid address string for eaddress', () => {
      expect(asClearValueType('eaddress', '0x0000000000000000000000000000000000000001')).toBe(
        '0x0000000000000000000000000000000000000001',
      );
    });

    it('rejects invalid address for eaddress', () => {
      expect(() => asClearValueType('eaddress', 'not-an-address')).toThrow();
      expect(() => asClearValueType('eaddress', 123)).toThrow();
    });

    it('accepts number for euint8/16/32', () => {
      expect(asClearValueType('euint8', 0)).toBe(0);
      expect(asClearValueType('euint8', 255)).toBe(255);
      expect(asClearValueType('euint16', 65535)).toBe(65535);
      expect(asClearValueType('euint32', 4294967295)).toBe(4294967295);
    });

    it('rejects bigint for euint8/16/32', () => {
      expect(() => asClearValueType('euint8', 1n)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint16', 1n)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint32', 1n)).toThrow(InvalidTypeError);
    });

    it('rejects negative number for euint8/16/32', () => {
      expect(() => asClearValueType('euint8', -1)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint16', -1)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint32', -1)).toThrow(InvalidTypeError);
    });

    it('rejects non-integer number for euint8/16/32', () => {
      expect(() => asClearValueType('euint8', 1.5)).toThrow(InvalidTypeError);
    });

    it('accepts bigint for euint64/128/256', () => {
      expect(asClearValueType('euint64', 0n)).toBe(0n);
      expect(asClearValueType('euint64', 100n)).toBe(100n);
      expect(asClearValueType('euint128', 100n)).toBe(100n);
      expect(asClearValueType('euint256', 100n)).toBe(100n);
    });

    it('rejects number for euint64/128/256', () => {
      expect(() => asClearValueType('euint64', 100)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint128', 100)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint256', 100)).toThrow(InvalidTypeError);
    });

    it('rejects negative bigint for euint64/128/256', () => {
      expect(() => asClearValueType('euint64', -1n)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint128', -1n)).toThrow(InvalidTypeError);
      expect(() => asClearValueType('euint256', -1n)).toThrow(InvalidTypeError);
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // toClearValueType
  //////////////////////////////////////////////////////////////////////////////

  describe('toClearValueType', () => {
    it('accepts boolean for ebool', () => {
      expect(toClearValueType('ebool', true)).toBe(true);
      expect(toClearValueType('ebool', false)).toBe(false);
    });

    it('rejects non-boolean for ebool (no coercion)', () => {
      expect(() => toClearValueType('ebool', 1)).toThrow(InvalidTypeError);
      expect(() => toClearValueType('ebool', 0)).toThrow(InvalidTypeError);
      expect(() => toClearValueType('ebool', 'true')).toThrow(InvalidTypeError);
    });

    it('accepts valid address string for eaddress', () => {
      expect(toClearValueType('eaddress', '0x0000000000000000000000000000000000000001')).toBe(
        '0x0000000000000000000000000000000000000001',
      );
    });

    it('rejects invalid address for eaddress', () => {
      expect(() => toClearValueType('eaddress', 'not-an-address')).toThrow();
      expect(() => toClearValueType('eaddress', 123)).toThrow();
    });

    it('coerces number or bigint to number for euint8/16/32', () => {
      expect(toClearValueType('euint8', 100)).toBe(100);
      expect(toClearValueType('euint8', 100n)).toBe(100);
      expect(toClearValueType('euint16', 1000n)).toBe(1000);
      expect(toClearValueType('euint32', 100000n)).toBe(100000);
    });

    it('rejects negative number for euint8/16/32', () => {
      expect(() => toClearValueType('euint8', -1)).toThrow();
      expect(() => toClearValueType('euint16', -1)).toThrow();
    });

    it('rejects non-integer number for euint8/16/32', () => {
      expect(() => toClearValueType('euint8', 1.5)).toThrow();
    });

    it('coerces number or bigint to bigint for euint64/128/256', () => {
      expect(toClearValueType('euint64', 100n)).toBe(100n);
      expect(toClearValueType('euint64', 100)).toBe(100n);
      expect(toClearValueType('euint128', 100n)).toBe(100n);
      expect(toClearValueType('euint256', 100n)).toBe(100n);
    });

    it('rejects negative bigint for euint64/128/256', () => {
      expect(() => toClearValueType('euint64', -1n)).toThrow();
      expect(() => toClearValueType('euint128', -1n)).toThrow();
      expect(() => toClearValueType('euint256', -1n)).toThrow();
    });
  });

  //////////////////////////////////////////////////////////////////////////////
  // Roundtrip tests
  //////////////////////////////////////////////////////////////////////////////

  describe('roundtrip conversions', () => {
    const validTypes = [
      { name: 'ebool', id: 0, bits: 2, solidity: 'bool' },
      { name: 'euint8', id: 2, bits: 8, solidity: 'uint256' },
      { name: 'euint16', id: 3, bits: 16, solidity: 'uint256' },
      { name: 'euint32', id: 4, bits: 32, solidity: 'uint256' },
      { name: 'euint64', id: 5, bits: 64, solidity: 'uint256' },
      { name: 'euint128', id: 6, bits: 128, solidity: 'uint256' },
      { name: 'eaddress', id: 7, bits: 160, solidity: 'address' },
      { name: 'euint256', id: 8, bits: 256, solidity: 'uint256' },
    ] as const;

    it.each(validTypes)('name -> id -> name roundtrip for $name', ({ name, id }) => {
      expect(fheTypeIdFromName(name)).toBe(id);
      expect(fheTypeNameFromId(id)).toBe(name);
    });

    it.each(validTypes)('bits -> id -> bits roundtrip for $name', ({ bits, id }) => {
      expect(fheTypeIdFromEncryptionBits(bits)).toBe(id);
      expect(encryptionBitsFromFheTypeId(id)).toBe(bits);
    });

    it.each(validTypes)('name -> bits roundtrip for $name', ({ name, bits }) => {
      expect(encryptionBitsFromFheType(name)).toBe(bits);
    });
  });
});
