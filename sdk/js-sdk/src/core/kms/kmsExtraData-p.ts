import type { Bytes32Hex, BytesHex, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { HostContractVersion } from '../types/hostContract.js';
import type { KmsExtraData } from '../types/kms-p.js';
import { asUint256BigInt, asUint8Number } from '../base/uint.js';
import { asBytes32Hex, asBytesHex, assertIsBytesHex } from '../base/bytes.js';
import { isVersionEqual, isVersionStrictlyBefore } from '../host-contracts/HostContractVersion-p.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';

export const EXTRA_DATA_V0: Uint8Number = 0x00 as Uint8Number; // 0x00
export const EXTRA_DATA_V1: Uint8Number = 0x01 as Uint8Number; // 1 version byte + 32-byte big-endian context ID = 33 bytes
export const EXTRA_DATA_V2: Uint8Number = 0x02 as Uint8Number; // 1 version byte + 32-byte big-endian context ID + 32-byte big-endian epoch ID = 65 bytes

//////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('KmsExtraData.token');

//////////////////////////////////////////////////////////////////////////////
// ExtraDataImpl class
//////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 */
export class KmsExtraDataImpl implements KmsExtraData {
  readonly #version: Uint8Number | undefined;
  readonly #kmsContextId: Uint256BigInt;
  readonly #kmsEpochId: Uint256BigInt;
  readonly #kmsExtraData: BytesHex;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly version: Uint8Number | undefined;
      readonly kmsContextId: Uint256BigInt;
      readonly kmsEpochId: Uint256BigInt;
      readonly kmsExtraData: BytesHex;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#kmsContextId = asUint256BigInt(parameters.kmsContextId);
    this.#kmsEpochId = asUint256BigInt(parameters.kmsEpochId);
    this.#version = parameters.version === undefined ? undefined : asUint8Number(parameters.version);
    this.#kmsExtraData = asBytesHex(parameters.kmsExtraData);

    this.#validate();
  }

  #validate(): void {
    if (this.#version === undefined) {
      if (this.#kmsContextId !== 0n) {
        throw new Error('kmsContextId must be 0 for unknown kms extraData');
      }
      if (this.#kmsEpochId !== 0n) {
        throw new Error('kmsEpochId must be 0 for unknown kms extraData');
      }
      return;
    }
    switch (this.#version) {
      case EXTRA_DATA_V0: {
        if (this.#kmsContextId !== 0n) {
          throw new Error('kmsContextId must be 0 for v0 kms extraData');
        }
        if (this.#kmsEpochId !== 0n) {
          throw new Error('kmsEpochId must be 0 for v0 kms extraData');
        }
        return;
      }
      case EXTRA_DATA_V1: {
        if (this.#kmsContextId === 0n) {
          throw new Error('kmsContextId cannot be 0 for v1 kms extraData');
        }
        if (this.#kmsEpochId !== 0n) {
          throw new Error('kmsEpochId must be 0 for v1 kms extraData');
        }
        return;
      }
      case EXTRA_DATA_V2: {
        if (this.#kmsContextId === 0n) {
          throw new Error('kmsContextId cannot be 0 for v2 kms extraData');
        }
        if (this.#kmsEpochId === 0n) {
          throw new Error('kmsEpochId cannot be 0 for v2 kms extraData');
        }
        return;
      }
      default:
        throw new Error(`Unsupported kms extraData version ${this.#version}`);
    }
  }

  public get version(): Uint8Number | undefined {
    return this.#version;
  }

  public get isFutureVersion(): boolean {
    return this.#version === undefined;
  }

  public get kmsContextId(): Uint256BigInt {
    return this.#kmsContextId;
  }

  public get kmsEpochId(): Uint256BigInt {
    return this.#kmsEpochId;
  }

  public get bytesHex(): BytesHex {
    return this.#kmsExtraData;
  }

  public lt(version: number): boolean {
    return this.#version !== undefined && this.#version < version;
  }
  public le(version: number): boolean {
    return this.#version !== undefined && this.#version <= version;
  }
  public gt(version: number): boolean {
    return this.#version === undefined || this.#version > version;
  }
  public ge(version: number): boolean {
    return this.#version === undefined || this.#version >= version;
  }
}

//////////////////////////////////////////////////////////////////////////////

export function createKmsExtraDataFromBytesHex(extraDataBytesHex: BytesHex): KmsExtraData {
  const sanitized: string = (extraDataBytesHex as string) === '0x' ? '0x00' : extraDataBytesHex;

  if (sanitized.length < 4) {
    throw new Error(`Unsupported kms extraData length ${extraDataBytesHex.length}: must be more than 4 bytes`);
  }

  // First byte = version (characters 2-3 after '0x')
  const version = asUint8Number(Number('0x' + sanitized.slice(2, 4)));

  if (version === EXTRA_DATA_V0) {
    return new KmsExtraDataImpl(PRIVATE_TOKEN, {
      version: EXTRA_DATA_V0,
      kmsContextId: 0n as Uint256BigInt,
      kmsEpochId: 0n as Uint256BigInt,
      kmsExtraData: '0x00' as BytesHex,
    });
  }

  if (version === EXTRA_DATA_V1) {
    // ExtraData v1 format: 1 version byte + 32-byte big-endian context ID = 33 bytes = 66 hex chars + 2 for '0x' = 68
    // 0x01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    // ↑   ↑                                                               ↑
    // 0   4                                                               68
    // 0         1         2         3         4         5         6
    // 01234567890123456789012345678901234567890123456789012345678901234567

    if (extraDataBytesHex.length != 68) {
      throw new Error(`Invalid kms extraData length for v1: expected 68, got ${extraDataBytesHex.length}`);
    }

    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraDataBytesHex.slice(4, 68)}`);
    const kmsContextId = asUint256BigInt(BigInt(contextIdBytes32Hex));

    return new KmsExtraDataImpl(PRIVATE_TOKEN, {
      version: EXTRA_DATA_V1,
      kmsContextId,
      kmsEpochId: 0n as Uint256BigInt,
      kmsExtraData: extraDataBytesHex,
    });
  }

  if (version === EXTRA_DATA_V2) {
    // ExtraData v2 format: 1 version byte + 32-byte big-endian context ID + 32-byte big-endian epoch ID = 65 bytes = 130 hex chars + 2 for '0x' = 132
    // 0x01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
    // ↑   ↑                                                               ↑                                                               ↑
    // 0   4                                                               68                                                              132
    // 0         1         2         3         4         5         6         7         8         9         0         1         2         3
    // 012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901

    if (extraDataBytesHex.length != 132) {
      throw new Error(`Invalid extraData length for v2: expected 132, got ${extraDataBytesHex.length}`);
    }

    // 32-byte contextId starts at byte 1 (hex chars 4..67)
    const contextIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraDataBytesHex.slice(4, 68)}`);
    const kmsContextId = asUint256BigInt(BigInt(contextIdBytes32Hex));

    // 32-byte epochId starts at byte 33 (hex chars 68..131)
    const epochIdBytes32Hex: Bytes32Hex = asBytes32Hex(`0x${extraDataBytesHex.slice(68, 132)}`);
    const kmsEpochId = asUint256BigInt(BigInt(epochIdBytes32Hex));

    return new KmsExtraDataImpl(PRIVATE_TOKEN, {
      version: EXTRA_DATA_V2,
      kmsContextId,
      kmsEpochId,
      kmsExtraData: extraDataBytesHex,
    });
  }

  return new KmsExtraDataImpl(PRIVATE_TOKEN, {
    version: undefined,
    kmsContextId: 0n as Uint256BigInt,
    kmsEpochId: 0n as Uint256BigInt,
    kmsExtraData: extraDataBytesHex,
  });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the `extraData` bytes exactly as they appear on the wire and inside KMS
 * signatures — i.e. the form the KMS/gateway signs over, NOT the SDK's internal
 * encoding.
 *
 * The only difference is the v0 sentinel: internally the SDK carries it as the
 * one-byte `0x00`, but the KMS signs v0 as EMPTY bytes (`0x`). Every place that
 * reconstructs a message the KMS signed — `UserDecryptResponseVerification`
 * (per-share and wasm request), the public-decryption proof, and the public-decrypt
 * EIP-712 — must use THIS value, or v0 signatures fail to verify. Concrete versions
 * (v1, v2, …) and unknown/future versions pass through unchanged.
 *
 * This centralizes a rule previously inlined at each call site (KmsSigncryptedShares,
 * PublicDecryptionProof, verifyKmsPublicDecryptEip712), where any divergence silently
 * breaks v0 verification.
 */
export function toKmsSignedExtraDataBytesHex(extraData: KmsExtraData): BytesHex {
  return extraData.version === EXTRA_DATA_V0 ? ('0x' as BytesHex) : extraData.bytesHex;
}

////////////////////////////////////////////////////////////////////////////////

export function createKmsExtraDataV0(): KmsExtraData {
  return new KmsExtraDataImpl(PRIVATE_TOKEN, {
    version: EXTRA_DATA_V0,
    kmsContextId: 0n as Uint256BigInt,
    kmsEpochId: 0n as Uint256BigInt,
    kmsExtraData: '0x00' as BytesHex,
  });
}

////////////////////////////////////////////////////////////////////////////////

export function createKmsExtraDataV1(parameters: { readonly kmsContextId: Uint256BigInt }): KmsExtraData {
  const v = EXTRA_DATA_V1.toString(16).padStart(2, '0');
  const contextId = parameters.kmsContextId.toString(16).padStart(64, '0');
  return new KmsExtraDataImpl(PRIVATE_TOKEN, {
    version: EXTRA_DATA_V1,
    kmsContextId: parameters.kmsContextId,
    kmsEpochId: 0n as Uint256BigInt,
    kmsExtraData: `0x${v}${contextId}` as BytesHex,
  });
}

////////////////////////////////////////////////////////////////////////////////

export function createKmsExtraDataV2(parameters: {
  readonly kmsContextId: Uint256BigInt;
  readonly kmsEpochId: Uint256BigInt;
}): KmsExtraData {
  const v = EXTRA_DATA_V2.toString(16).padStart(2, '0');
  const contextId = parameters.kmsContextId.toString(16).padStart(64, '0');
  const epochId = parameters.kmsEpochId.toString(16).padStart(64, '0');
  return new KmsExtraDataImpl(PRIVATE_TOKEN, {
    version: EXTRA_DATA_V2,
    kmsContextId: parameters.kmsContextId,
    kmsEpochId: parameters.kmsEpochId,
    kmsExtraData: `0x${v}${contextId}${epochId}` as BytesHex,
  });
}

////////////////////////////////////////////////////////////////////////////////

export function validateKmsExtraDataParams(parameters: {
  readonly kmsContextId: Uint256BigInt;
  readonly kmsEpochId: Uint256BigInt;
}): void {
  if (parameters.kmsEpochId !== 0n && parameters.kmsContextId === 0n) {
    throw new Error('kmsContextId cannot be 0 if kmsEpochId is not 0');
  }
}

////////////////////////////////////////////////////////////////////////////////

export function createKmsExtraData(parameters: {
  readonly kmsContextId: Uint256BigInt;
  readonly kmsEpochId: Uint256BigInt;
}): KmsExtraData {
  validateKmsExtraDataParams(parameters);

  if (parameters.kmsEpochId === 0n) {
    if (parameters.kmsContextId === 0n) {
      return createKmsExtraDataV0();
    }
    return createKmsExtraDataV1({
      kmsContextId: parameters.kmsContextId,
    });
  }

  return createKmsExtraDataV2({
    kmsContextId: parameters.kmsContextId,
    kmsEpochId: parameters.kmsEpochId,
  });
}

////////////////////////////////////////////////////////////////////////////////

export function equalsKmsExtraData(a: KmsExtraData, b: KmsExtraData): boolean {
  assertIsKmsExtraData(a, {});
  assertIsKmsExtraData(b, {});
  return a.version === b.version && a.kmsContextId === b.kmsContextId && a.kmsEpochId === b.kmsEpochId;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Module-only helper. Narrows an unknown value to the concrete
 * {@link KmsExtraDataImpl} class by verifying it via {@link assertIsKmsExtraData}
 * and returning it typed as the implementation class, so other functions in this
 * module can reach class-only members. Prefer this over a bare cast.
 *
 * Must never be exported: `Impl` classes are internal to the SDK and never leave
 * the module, so a function that hands one back must stay module-private too.
 * The public surface exposes only the {@link KmsExtraData} interface.
 *
 * @param value - The value to narrow.
 * @returns The same value typed as {@link KmsExtraDataImpl}.
 * @throws {InvalidTypeError} If `value` is not a {@link KmsExtraDataImpl} instance.
 */
function _asKmsExtraDataImpl(value: unknown): KmsExtraDataImpl {
  assertIsKmsExtraData(value, {});
  return value as KmsExtraDataImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function isKmsExtraData(value: unknown): value is KmsExtraData {
  return value instanceof KmsExtraDataImpl;
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsExtraData(
  value: unknown,
  options: { readonly subject?: string } & ErrorMetadataParams,
): asserts value is KmsExtraData {
  if (!isKmsExtraData(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'KmsExtraData',
      },
      options,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

export function assertIsKmsExtraDataBytesHex(
  value: unknown,
  options: { subject?: string } & ErrorMetadataParams,
): asserts value is BytesHex {
  if (value === '0x00' || value === '0x') {
    return;
  }

  assertIsBytesHex(value, options);

  // Will valid extraData length too
  createKmsExtraDataFromBytesHex(value);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns `true` if `kmsExtraData`'s version is accepted by the given
 * KMSVerifier contract version.
 *
 * | KMSVerifier version | protocol        | max extraData version |
 * | ------------------- | --------------- | --------------------- |
 * | < 0.2.0             | v0.11.0         | v0                    |
 * | 0.2.0 – 0.3.x       | v0.12.0/v0.13.0 | v1                    |
 * | 0.4.0               | v0.14.0         | v2                    |
 * | > 0.4.0             | v0.14.1+        | not checked (`true`)  |
 */
export function isKmsExtraDataCompatibleWithKmsVerifier(
  kmsExtraData: KmsExtraData,
  kmsVerifierVersion: HostContractVersion,
): boolean {
  const ed = _asKmsExtraDataImpl(kmsExtraData);

  // Protocol v0.11.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 2 })) {
    return ed.version !== undefined && ed.version <= EXTRA_DATA_V0;
  }

  // Protocol v0.12.0/v0.13.0
  if (isVersionStrictlyBefore(kmsVerifierVersion, { major: 0, minor: 4 })) {
    return ed.version !== undefined && ed.version <= EXTRA_DATA_V1;
  }

  // Protocol v0.14.0
  if (isVersionEqual(kmsVerifierVersion, { major: 0, minor: 4, patch: 0 })) {
    return ed.version !== undefined && ed.version <= EXTRA_DATA_V2;
  }

  // Protocol v0.14.1+ (beyond the versions this SDK release knows about).
  //
  // We deliberately return `true` (compatible) instead of guessing an upper
  // bound on `extraData.version`. On v0.14.0 the KMSVerifier reverts for
  // `extraData.version > 2`, but we cannot know the accepted range of future
  // contract versions. Pre-validating here would risk rejecting extraData that
  // a newer contract actually supports. If the on-chain contract does reject it,
  // the call reverts — and surfacing that revert is the source of truth, not a
  // speculative client-side check.
  return true;
}
