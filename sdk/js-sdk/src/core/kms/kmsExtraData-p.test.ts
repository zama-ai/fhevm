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
  createKmsExtraDataFromBytesHex,
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
    const fromZeroByte = createKmsExtraDataFromBytesHex('0x00' as BytesHex);
    const fromEmptyBytes = createKmsExtraDataFromBytesHex('0x' as BytesHex);

    for (const extraData of [fromFactory, fromZeroByte, fromEmptyBytes]) {
      expect(extraData.version).toBe(EXTRA_DATA_V0);
      expect(extraData.kmsContextId).toBe(0n);
      expect(extraData.kmsEpochId).toBe(0n);
      expect(extraData.bytesHex).toBe('0x00');
    }
  });

  it('encodes and decodes v1 extraData with a context id', () => {
    const kmsContextId = 1n as Uint256BigInt;
    const expectedBytes = `0x01${word(kmsContextId)}` as BytesHex;

    const fromFactory = createKmsExtraDataV1({ kmsContextId });
    const fromBytes = createKmsExtraDataFromBytesHex(expectedBytes);

    expect(fromFactory.bytesHex).toBe(expectedBytes);
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
    const fromBytes = createKmsExtraDataFromBytesHex(expectedBytes);

    expect(fromFactory.bytesHex).toBe(expectedBytes);
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
    // Too short: no version byte at all.
    expect(() => createKmsExtraDataFromBytesHex('0x0' as BytesHex)).toThrow();

    // v1 declared but wrong length (missing / extra bytes).
    expect(() => createKmsExtraDataFromBytesHex('0x01' as BytesHex)).toThrow();
    expect(() => createKmsExtraDataFromBytesHex(`0x01${word(1n)}00` as BytesHex)).toThrow();

    // v2 declared but wrong length (missing / extra bytes).
    expect(() => createKmsExtraDataFromBytesHex('0x02' as BytesHex)).toThrow();
    expect(() => createKmsExtraDataFromBytesHex(`0x02${word(1n)}` as BytesHex)).toThrow();
    expect(() => createKmsExtraDataFromBytesHex(`0x02${word(1n)}${word(2n)}00` as BytesHex)).toThrow();

    // Correct length, but the payload is not valid hex.
    expect(() => createKmsExtraDataFromBytesHex(`0x01${'z'.repeat(64)}` as BytesHex)).toThrow();

    // Version byte itself is not valid hex.
    expect(() => createKmsExtraDataFromBytesHex(`0xzz${word(1n)}` as BytesHex)).toThrow();
  });

  it('classifies any 0x00-prefixed payload as v0, ignoring trailing bytes (matches KMSVerifier)', () => {
    // KMSVerifier._extractKmsContextId treats an empty payload or a leading
    // 0x00 byte as "current context" (v0) and ignores trailing bytes; the SDK
    // decoder must mirror that so it never disagrees with the chain.
    for (const bytes of ['0x', '0x00', '0x0000', `0x00${word(7n)}`] as BytesHex[]) {
      const extraData = createKmsExtraDataFromBytesHex(bytes);
      expect(extraData.version).toBe(EXTRA_DATA_V0);
      expect(extraData.kmsContextId).toBe(0n);
      expect(extraData.kmsEpochId).toBe(0n);
      expect(extraData.isFutureVersion).toBe(false);
    }
  });

  it('decodes an unrecognized version as a future (unknown) extraData', () => {
    const future = createKmsExtraDataFromBytesHex(`0x03${word(1n)}` as BytesHex);

    expect(future.version).toBeUndefined();
    expect(future.isFutureVersion).toBe(true);
    // The SDK cannot decode a future layout: the ids are not trusted (neutral 0),
    // but the raw bytes are preserved so the encoding can still be forwarded to
    // the chain verbatim (the chain is the authority on future versions).
    expect(future.kmsContextId).toBe(0n);
    expect(future.kmsEpochId).toBe(0n);
    expect(future.bytesHex).toBe(`0x03${word(1n)}`);
  });

  it('reports isFutureVersion only for unrecognized versions', () => {
    expect(createKmsExtraDataV0().isFutureVersion).toBe(false);
    expect(createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt }).isFutureVersion).toBe(false);
    expect(
      createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt }).isFutureVersion,
    ).toBe(false);
    expect(createKmsExtraDataFromBytesHex(`0x03${word(1n)}` as BytesHex).isFutureVersion).toBe(true);
  });

  it('compares known versions with lt/le/gt/ge numerically', () => {
    const v0 = createKmsExtraDataV0();
    const v1 = createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt });
    const v2 = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt });

    // v1 (version 1) against each threshold.
    expect(v1.lt(EXTRA_DATA_V2)).toBe(true);
    expect(v1.lt(EXTRA_DATA_V1)).toBe(false);
    expect(v1.le(EXTRA_DATA_V1)).toBe(true);
    expect(v1.le(EXTRA_DATA_V0)).toBe(false);
    expect(v1.gt(EXTRA_DATA_V0)).toBe(true);
    expect(v1.gt(EXTRA_DATA_V1)).toBe(false);
    expect(v1.ge(EXTRA_DATA_V1)).toBe(true);
    expect(v1.ge(EXTRA_DATA_V2)).toBe(false);

    // v0 and v2 boundary spot-checks.
    expect(v0.lt(EXTRA_DATA_V1)).toBe(true);
    expect(v0.ge(EXTRA_DATA_V1)).toBe(false);
    expect(v2.gt(EXTRA_DATA_V1)).toBe(true);
    expect(v2.ge(EXTRA_DATA_V2)).toBe(true);
    expect(v2.lt(EXTRA_DATA_V2)).toBe(false);
  });

  it('treats a future (unknown) version as greater than every version', () => {
    const future = createKmsExtraDataFromBytesHex(`0x03${word(1n)}` as BytesHex);

    // Never less-than / less-or-equal to any version.
    expect(future.lt(EXTRA_DATA_V0)).toBe(false);
    expect(future.lt(EXTRA_DATA_V2)).toBe(false);
    expect(future.lt(255)).toBe(false);
    expect(future.le(EXTRA_DATA_V2)).toBe(false);
    expect(future.le(255)).toBe(false);

    // Always greater-than / greater-or-equal to any version.
    expect(future.gt(EXTRA_DATA_V0)).toBe(true);
    expect(future.gt(EXTRA_DATA_V2)).toBe(true);
    expect(future.gt(255)).toBe(true);
    expect(future.ge(EXTRA_DATA_V0)).toBe(true);
    expect(future.ge(EXTRA_DATA_V2)).toBe(true);
  });

  it('gates "v2 or later" via lt(EXTRA_DATA_V2): rejects v0/v1, accepts v2 and future', () => {
    // This is the exact predicate the unified (V2) permit builder relies on:
    // `extraData.lt(EXTRA_DATA_V2)` === true means "reject".
    const v0 = createKmsExtraDataV0();
    const v1 = createKmsExtraDataV1({ kmsContextId: 1n as Uint256BigInt });
    const v2 = createKmsExtraDataV2({ kmsContextId: 1n as Uint256BigInt, kmsEpochId: 2n as Uint256BigInt });
    const future = createKmsExtraDataFromBytesHex(`0x03${word(1n)}` as BytesHex);

    expect(v0.lt(EXTRA_DATA_V2)).toBe(true); // reject
    expect(v1.lt(EXTRA_DATA_V2)).toBe(true); // reject
    expect(v2.lt(EXTRA_DATA_V2)).toBe(false); // accept
    expect(future.lt(EXTRA_DATA_V2)).toBe(false); // accept
  });

  it('narrows valid extraData objects and bytes', () => {
    const v2Bytes = `0x02${word(1n)}${word(2n)}` as BytesHex;
    const extraData = createKmsExtraDataFromBytesHex(v2Bytes);

    expect(isKmsExtraData(extraData)).toBe(true);
    expect(isKmsExtraData({})).toBe(false);
    expect(() => assertIsKmsExtraData(extraData, {})).not.toThrow();
    expect(() => assertIsKmsExtraData({}, {})).toThrow('expected KmsExtraData');
    expect(() => assertIsKmsExtraDataBytesHex('0x', {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex('0x00', {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex(v2Bytes, {})).not.toThrow();
    expect(() => assertIsKmsExtraDataBytesHex('0x02', {})).toThrow('Invalid extraData length for v2');
  });

  it('keeps KmsExtraDataImpl constructor private to the module', () => {
    expect(
      () =>
        new KmsExtraDataImpl(Symbol('wrong'), {
          version: EXTRA_DATA_V0,
          kmsContextId: 0n as Uint256BigInt,
          kmsEpochId: 0n as Uint256BigInt,
          kmsExtraData: '0x00' as BytesHex,
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
