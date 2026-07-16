import type { HostContractVersion } from '../types/hostContract.js';
import type { BytesHex, Uint256BigInt, UintNumber } from '../types/primitives.js';
import { describe, expect, it } from 'vitest';
import {
  assertIsKmsExtraData,
  assertIsKmsExtraDataBytesHex,
  createKmsExtraData,
  createKmsExtraDataV0,
  createKmsExtraDataV1,
  createKmsExtraDataV2,
  equalsKmsExtraData,
  EXTRA_DATA_V0,
  EXTRA_DATA_V1,
  EXTRA_DATA_V2,
  fromKmsExtraDataBytesHex,
  isKmsExtraData,
  isKmsExtraDataCompatibleWithKmsVerifier,
  KmsExtraDataImpl,
  validateKmsExtraDataParams,
} from './kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/kms/kmsExtraData-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const word = (value: bigint): string => value.toString(16).padStart(64, '0');
const kmsVerifierVersion = (major: number, minor: number, patch: number): HostContractVersion<'KMSVerifier'> => ({
  version: `KMSVerifier v${major}.${minor}.${patch}`,
  contractName: 'KMSVerifier',
  major: major as UintNumber,
  minor: minor as UintNumber,
  patch: patch as UintNumber,
});

describe('kmsExtraData', () => {
  it('encodes and decodes v0 extraData', () => {
    const fromFactory = createKmsExtraDataV0();
    const fromZeroByte = fromKmsExtraDataBytesHex('0x00' as BytesHex);
    const fromEmptyBytes = fromKmsExtraDataBytesHex('0x' as BytesHex);

    for (const extraData of [fromFactory, fromZeroByte, fromEmptyBytes]) {
      expect(extraData.version).toBe(EXTRA_DATA_V0);
      expect(extraData.kmsContextId).toBe(0n);
      expect(extraData.kmsEpochId).toBe(0n);
      expect(extraData.toBytesHex()).toBe('0x00');
    }
  });

  it('encodes and decodes v1 extraData with a context id', () => {
    const kmsContextId = 1n as Uint256BigInt;
    const expectedBytes = `0x01${word(kmsContextId)}` as BytesHex;

    const fromFactory = createKmsExtraDataV1({ kmsContextId });
    const fromBytes = fromKmsExtraDataBytesHex(expectedBytes);

    expect(fromFactory.toBytesHex()).toBe(expectedBytes);
    expect(fromBytes.version).toBe(EXTRA_DATA_V1);
    expect(fromBytes.kmsContextId).toBe(kmsContextId);
    expect(fromBytes.kmsEpochId).toBe(0n);
    expect(equalsKmsExtraData(fromFactory, fromBytes)).toBe(true);
  });

  it('encodes and decodes v2 extraData with a context id and epoch id', () => {
    const kmsContextId = 1n as Uint256BigInt;
    const kmsEpochId = 2n as Uint256BigInt;
    const expectedBytes = `0x02${word(kmsContextId)}${word(kmsEpochId)}` as BytesHex;

    const fromFactory = createKmsExtraDataV2({ kmsContextId, kmsEpochId });
    const fromBytes = fromKmsExtraDataBytesHex(expectedBytes);

    expect(fromFactory.toBytesHex()).toBe(expectedBytes);
    expect(fromBytes.version).toBe(EXTRA_DATA_V2);
    expect(fromBytes.kmsContextId).toBe(kmsContextId);
    expect(fromBytes.kmsEpochId).toBe(kmsEpochId);
    expect(equalsKmsExtraData(fromFactory, fromBytes)).toBe(true);
  });

  it('selects the smallest extraData version that can encode the provided ids', () => {
    expect(createKmsExtraData({ kmsContextId: 0n as Uint256BigInt, kmsEpochId: 0n as Uint256BigInt }).version).toBe(
      EXTRA_DATA_V0,
    );
    expect(createKmsExtraData({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 0n as Uint256BigInt }).version).toBe(
      EXTRA_DATA_V1,
    );
    expect(createKmsExtraData({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt }).version).toBe(
      EXTRA_DATA_V2,
    );
  });

  it('rejects invalid version/id combinations', () => {
    expect(() => createKmsExtraDataV1({ kmsContextId: 0n as Uint256BigInt })).toThrow(
      'kmsContextId cannot be 0 for v1 kms extraData',
    );
    expect(() => createKmsExtraDataV2({ kmsContextId: 0n as Uint256BigInt, kmsEpochId: 1n as Uint256BigInt })).toThrow(
      'kmsContextId cannot be 0 for v2 kms extraData',
    );
    expect(() => createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 0n as Uint256BigInt })).toThrow(
      'kmsEpochId cannot be 0 for v2 kms extraData',
    );
    expect(() =>
      validateKmsExtraDataParams({ kmsContextId: 0n as Uint256BigInt, kmsEpochId: 1n as Uint256BigInt }),
    ).toThrow('kmsContextId cannot be 0 if kmsEpochId is not 0');
  });

  it('rejects malformed extraData bytes', () => {
    expect(() => fromKmsExtraDataBytesHex('0x01' as BytesHex)).toThrow('Unsupported kms extraData length');
    expect(() => fromKmsExtraDataBytesHex(`0x01${word(1n)}00` as BytesHex)).toThrow(
      'Invalid kms extraData length for v1',
    );
    expect(() => fromKmsExtraDataBytesHex(`0x02${word(1n)}` as BytesHex)).toThrow('Invalid extraData length for v2');
    expect(() => fromKmsExtraDataBytesHex(`0x03${word(1n)}` as BytesHex)).toThrow(
      'Unsupported kms extraData version 3',
    );
  });

  it('narrows valid extraData objects and bytes', () => {
    const v2Bytes = `0x02${word(1n)}${word(2n)}` as BytesHex;
    const extraData = fromKmsExtraDataBytesHex(v2Bytes);

    expect(isKmsExtraData(extraData)).toBe(true);
    expect(isKmsExtraData({})).toBe(false);
    expect(() => assertIsKmsExtraData(extraData, {})).not.toThrow();
    expect(() => assertIsKmsExtraData({}, {})).toThrow('expected KmsExtraData');
    expect(() => assertIsKmsExtraDataBytesHex('0x', {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex('0x00', {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex(v2Bytes, {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex('0x02', {})).toThrow('Unsupported kms extraData length');
  });

  it('keeps KmsExtraDataImpl constructor private to the module', () => {
    expect(
      () =>
        new KmsExtraDataImpl(Symbol('wrong'), {
          version: EXTRA_DATA_V0,
          kmsContextId: 0n as Uint256BigInt,
          kmsEpochId: 0n as Uint256BigInt,
        }),
    ).toThrow('Unauthorized');
  });

  it('compares extraData values by version, context id, and epoch id', () => {
    const a = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt });
    const b = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt });
    const c = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 3n as Uint256BigInt });

    expect(equalsKmsExtraData(a, b)).toBe(true);
    expect(equalsKmsExtraData(a, c)).toBe(false);
  });

  it('keeps old extraData versions compatible with newer KMSVerifier versions (stale-permit migration)', () => {
    // A permit cached on an older protocol keeps its original extraData
    // encoding (the EIP-712 signature covers it). Migration v11 -> v12 -> v13
    // -> v14 only works if an OLD extraData version is never rejected by the
    // compatibility check against a NEWER KMSVerifier.
    const v0 = createKmsExtraDataV0();
    const v1 = createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt });

    // v11-era extraData (v0) against every later protocol's KMSVerifier
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v0, kmsVerifierVersion(0, 2, 0))).toBe(true); // v12
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v0, kmsVerifierVersion(0, 3, 0))).toBe(true); // v13
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v0, kmsVerifierVersion(0, 4, 0))).toBe(true); // v14
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v0, kmsVerifierVersion(0, 5, 0))).toBe(true); // future

    // v12/v13-era extraData (v1) against v14+ KMSVerifiers
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v1, kmsVerifierVersion(0, 4, 0))).toBe(true);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v1, kmsVerifierVersion(0, 5, 0))).toBe(true);
  });

  it('checks compatibility with KMSVerifier versions', () => {
    const v0 = createKmsExtraDataV0();
    const v1 = createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt });
    const v2 = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt });

    expect(isKmsExtraDataCompatibleWithKmsVerifier(v0, kmsVerifierVersion(0, 1, 0))).toBe(true);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v1, kmsVerifierVersion(0, 1, 0))).toBe(false);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v1, kmsVerifierVersion(0, 2, 0))).toBe(true);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v2, kmsVerifierVersion(0, 3, 9))).toBe(false);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v2, kmsVerifierVersion(0, 4, 0))).toBe(true);
    expect(isKmsExtraDataCompatibleWithKmsVerifier(v2, kmsVerifierVersion(0, 4, 1))).toBe(true);
  });
});
