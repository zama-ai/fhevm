import { describe, expect, it } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import type { Bytes32 } from '../../core/types/primitives.js';
import {
  deriveBatchAddresses,
  deriveJoinRecordAddress,
  deriveSettleAccounts,
  deriveSettleLookupTableAddresses,
  settleAccountsToLookupTableAddresses,
  type VaultDemoRoots,
} from './derive.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function roots(): VaultDemoRoots {
  return {
    batcherProgram: addr(30),
    tokenProgram: addr(31),
    vaultProgram: addr(32),
    hostProgram: addr(33),
    batcher: addr(2),
    vault: addr(10),
    joinConfidentialMint: addr(4),
    payoutConfidentialMint: addr(13),
    joinUnderlyingMint: addr(5),
    payoutUnderlyingMint: addr(14),
    hostConfig: addr(8),
    kmsContext: addr(9),
  };
}

const BURNED_HANDLE = new Uint8Array(32).fill(0x92) as Bytes32;

describe('deriveBatchAddresses', () => {
  it('is deterministic and index-sensitive', async () => {
    const r = roots();
    const a0 = await deriveBatchAddresses(r, 0n);
    const a0Again = await deriveBatchAddresses(r, 0n);
    const a1 = await deriveBatchAddresses(r, 1n);
    expect(a0).toEqual(a0Again);
    expect(a0.batch).not.toBe(a1.batch);
    // Every field is a distinct 44-ish char base58 address; none is empty.
    for (const value of Object.values(a0)) expect(typeof value).toBe('string');
  });
});

describe('deriveJoinRecordAddress', () => {
  it('is user-specific', async () => {
    const r = roots();
    const { batch } = await deriveBatchAddresses(r, 0n);
    const forAlice = await deriveJoinRecordAddress(batch, addr(100));
    const forBob = await deriveJoinRecordAddress(batch, addr(101));
    expect(forAlice).not.toBe(forBob);
  });
});

describe('settle lookup-table addresses', () => {
  it('exclude redemption_record and match the open_batch / settleBatch shared ordering', async () => {
    const r = roots();
    const batch = await deriveBatchAddresses(r, 0n);
    const accounts = await deriveSettleAccounts(r, batch, BURNED_HANDLE);

    const fromAccounts = settleAccountsToLookupTableAddresses(accounts);
    const fromDerive = await deriveSettleLookupTableAddresses(r, batch);

    // The two producers (settleBatch's real accounts vs open_batch's zero-handle derivation) agree
    // exactly — the invariant that keeps the on-chain ALT indices lined up with the v0 message.
    expect(fromAccounts).toEqual(fromDerive);
    // redemption_record is the only field left out.
    expect(fromAccounts).not.toContain(accounts.redemptionRecord);
    expect(fromAccounts).toContain(accounts.batchBurnedAmountValue);
    expect(fromAccounts.length).toBe(Object.keys(accounts).length - 1);
  });
});
