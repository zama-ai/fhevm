import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import { describe, expect, it } from 'vitest';
import {
  assertExtraDataMatchesKmsSingersContext,
  createKmsSignersContext,
  extraDataMatchesKmsSingersContext,
  kmsSignersContextToExtraData,
} from './KmsSignersContext-p.js';
import { EXTRA_DATA_V0, EXTRA_DATA_V1, EXTRA_DATA_V2 } from '../kms/kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/host-contracts/KmsSignersContext-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const word = (value: bigint): string => value.toString(16).padStart(64, '0');

const KMS_VERIFIER_ADDRESS = '0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC' as ChecksummedAddress;
const KMS_SIGNER = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;

// A permit signed on protocol v0.11 embeds extraData v0 (`0x00`): the chain had
// no KMS context id at all. Permits are cached client-side for their whole
// validity window (days), so this exact payload can come back long after the
// chain has migrated to v12/v13 (context id, extraData v1) or v14 (context id +
// epoch id, extraData v2).
const STALE_V11_PERMIT_EXTRA_DATA = '0x00';

// A permit signed on protocol v0.12/v0.13 embeds extraData v1 (version byte +
// context id, no epoch).
const staleV12V13PermitExtraData = (kmsContextId: bigint): string => `0x01${word(kmsContextId)}`;

/**
 * Builds a {@link KmsSignersContext} shaped like what
 * `readCurrentKmsSignersContext` returns on each protocol era:
 * - v0.11: `kmsContextId = 0`, `kmsEpochId = 0`
 * - v0.12/v0.13: `kmsContextId != 0`, `kmsEpochId = 0`
 * - v0.14+: `kmsContextId != 0`, `kmsEpochId != 0`
 */
function makeContext(kmsContextId: bigint, kmsEpochId: bigint): KmsSignersContext {
  return createKmsSignersContext(new WeakRef({} as FhevmRuntime), {
    kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
    kmsContextId: kmsContextId as Uint256BigInt,
    kmsEpochId: kmsEpochId as Uint256BigInt,
    kmsSigners: [KMS_SIGNER],
    kmsSignerThreshold: 1 as Uint8Number,
  });
}

const contextV11 = (): KmsSignersContext => makeContext(0n, 0n);
const contextV12V13 = (kmsContextId: bigint): KmsSignersContext => makeContext(kmsContextId, 0n);
const contextV14 = (kmsContextId: bigint, kmsEpochId: bigint): KmsSignersContext =>
  makeContext(kmsContextId, kmsEpochId);

describe('kmsSignersContextToExtraData', () => {
  it('derives the extraData version from the context shape (v11/v12-13/v14 eras)', () => {
    expect(kmsSignersContextToExtraData(contextV11()).version).toBe(EXTRA_DATA_V0);
    expect(kmsSignersContextToExtraData(contextV12V13(7n)).version).toBe(EXTRA_DATA_V1);
    expect(kmsSignersContextToExtraData(contextV14(7n, 3n)).version).toBe(EXTRA_DATA_V2);
  });
});

describe('assertExtraDataMatchesKmsSingersContext — same-era permits (no migration)', () => {
  it('accepts a v11 permit extraData against a v11 context', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: STALE_V11_PERMIT_EXTRA_DATA, kmsSignersContext: contextV11() },
        {},
      ),
    ).not.toThrow();
  });

  it('accepts a v12/v13 permit extraData against a v12/v13 context with the same context id', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: staleV12V13PermitExtraData(7n), kmsSignersContext: contextV12V13(7n) },
        {},
      ),
    ).not.toThrow();
  });

  it('accepts a v14 permit extraData against a v14 context with the same context and epoch ids', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: `0x02${word(7n)}${word(3n)}`, kmsSignersContext: contextV14(7n, 3n) },
        {},
      ),
    ).not.toThrow();
  });

  it('rejects a permit extraData whose context id differs from the context (rotation)', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: staleV12V13PermitExtraData(7n), kmsSignersContext: contextV12V13(8n) },
        {},
      ),
    ).toThrow('does not match KmsSignersContext extraData');
    expect(
      extraDataMatchesKmsSingersContext({
        extraData: staleV12V13PermitExtraData(7n),
        kmsSignersContext: contextV12V13(8n),
      }),
    ).toBe(false);
  });
});

// ---------------------------------------------------------------------------
// Cross-era extraData — stale cached permit across a protocol migration.
//
// Scenario: the user signed a decryption permit while the chain was on an older
// protocol version, cached it (e.g. localStorage), never refreshed the cache,
// and the chain has since migrated (v11 -> v12 -> v13 -> v14). The permit's
// EIP-712 message embeds the OLD extraData encoding, and the signature covers
// it, so the SDK cannot rewrite it.
//
// `assertExtraDataMatchesKmsSingersContext` is byte-strict BY DESIGN and
// rejects cross-era encodings — these tests pin that down. Migration support
// is the decrypt flow's job: it must resolve the signer context FROM the
// permit's extraData (`readKmsSignersContextFromExtraData`, where v0 is the
// "current context" sentinel, mirroring `KMSVerifier._extractKmsContextId`)
// instead of calling this strict helper against the current context (PR #3204).
// The end-to-end migration behavior is covered by the
// `clientDecrypt.stalePermitMigration` suites in `test/fheTest/{viem,ethers}`.
// ---------------------------------------------------------------------------
describe('assertExtraDataMatchesKmsSingersContext — cross-era encodings are rejected (byte-strict by design)', () => {
  it('rejects a stale v11 permit extraData (v0 = 0x00) against a v12/v13-era context', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: STALE_V11_PERMIT_EXTRA_DATA, kmsSignersContext: contextV12V13(7n) },
        {},
      ),
    ).toThrow('does not match KmsSignersContext extraData');
  });

  it('rejects a stale v11 permit extraData (v0 = 0x00) against a v14-era context', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: STALE_V11_PERMIT_EXTRA_DATA, kmsSignersContext: contextV14(7n, 3n) },
        {},
      ),
    ).toThrow('does not match KmsSignersContext extraData');
  });

  it('rejects a stale v12/v13 permit extraData (v1) against a v14-era context even when the context id matches', () => {
    expect(() =>
      assertExtraDataMatchesKmsSingersContext(
        { extraData: staleV12V13PermitExtraData(7n), kmsSignersContext: contextV14(7n, 3n) },
        {},
      ),
    ).toThrow('does not match KmsSignersContext extraData');
  });
});
