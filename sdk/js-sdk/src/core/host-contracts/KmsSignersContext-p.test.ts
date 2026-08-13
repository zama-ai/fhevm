import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { ChecksummedAddress, Uint256BigInt, Uint8Number } from '../types/primitives.js';
import { describe, expect, it } from 'vitest';
import {
  assertKmsSignerThreshold,
  createKmsSignersContext,
  kmsSignersContextToExtraData,
} from './KmsSignersContext-p.js';
import { EXTRA_DATA_V0, EXTRA_DATA_V1, EXTRA_DATA_V2 } from '../kms/kmsExtraData-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/host-contracts/KmsSignersContext-p.test.ts
////////////////////////////////////////////////////////////////////////////////

const KMS_VERIFIER_ADDRESS = '0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC' as ChecksummedAddress;
const KMS_SIGNER = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;

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

// ---------------------------------------------------------------------------
// Threshold arithmetic during a staggered KMS rollout.
//
// Production KMS upgrades are staged: first a subset of nodes below the
// signing threshold runs the new version, then a subset above it, then all
// (e.g. 5-of-13 new → 9-of-13 new → all). The signer SET and threshold come
// from the on-chain context and do not change during those phases — but the
// number of responsive/valid shares can dip while nodes restart. These tests
// pin the acceptance boundary: exactly-threshold valid shares from distinct
// known signers pass; anything below, duplicated, or from an unknown signer
// is rejected.
// ---------------------------------------------------------------------------
describe('assertKmsSignerThreshold — mixed-version rollout boundaries', () => {
  const SIGNER_2 = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;
  const SIGNER_3 = '0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC' as ChecksummedAddress;
  const UNKNOWN_SIGNER = '0x90F79bf6EB2c4f870365E785982E1f101E93b906' as ChecksummedAddress;

  function makeThresholdContext(threshold: number): KmsSignersContext {
    return createKmsSignersContext(new WeakRef({} as FhevmRuntime), {
      kmsVerifierAddress: KMS_VERIFIER_ADDRESS,
      kmsContextId: 7n as Uint256BigInt,
      kmsEpochId: 0n as Uint256BigInt,
      kmsSigners: [KMS_SIGNER, SIGNER_2, SIGNER_3],
      kmsSignerThreshold: threshold as Uint8Number,
    });
  }

  it('accepts exactly-threshold distinct known signers (above-threshold rollout phase)', () => {
    expect(() => assertKmsSignerThreshold(makeThresholdContext(2), [KMS_SIGNER, SIGNER_2])).not.toThrow();
  });

  it('accepts more than threshold signers (all nodes responsive)', () => {
    expect(() => assertKmsSignerThreshold(makeThresholdContext(2), [KMS_SIGNER, SIGNER_2, SIGNER_3])).not.toThrow();
  });

  it('rejects below-threshold responses (below-threshold rollout phase must not decrypt alone)', () => {
    expect(() => assertKmsSignerThreshold(makeThresholdContext(2), [KMS_SIGNER])).toThrow('threshold is not reached');
  });

  it('rejects duplicated signers — one node cannot vote twice to fake a threshold', () => {
    expect(() => assertKmsSignerThreshold(makeThresholdContext(2), [KMS_SIGNER, KMS_SIGNER])).toThrow(
      'appears multiple times',
    );
  });

  it('rejects shares signed by an address outside the context signer set', () => {
    expect(() => assertKmsSignerThreshold(makeThresholdContext(2), [KMS_SIGNER, UNKNOWN_SIGNER])).toThrow(
      'is not in the list of kms signers',
    );
  });

  it('signer membership is case-insensitive (checksummed vs lowercase recovery)', () => {
    expect(() =>
      assertKmsSignerThreshold(makeThresholdContext(2), [
        KMS_SIGNER.toLowerCase() as ChecksummedAddress,
        SIGNER_2.toLowerCase() as ChecksummedAddress,
      ]),
    ).not.toThrow();
  });
});
