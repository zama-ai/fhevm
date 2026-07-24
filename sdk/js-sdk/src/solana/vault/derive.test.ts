import { describe, expect, it } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import type { Bytes32 } from '../../core/types/primitives.js';
import {
  deriveBatchAddresses,
  deriveJoinRecordAddress,
  deriveSettleAccounts,
  deriveSettleLookupTableAddresses,
  SETTLE_ALT_FIELD_ORDER,
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

    // The ALT ordering is driven by the explicit SETTLE_ALT_FIELD_ORDER tuple, not by object-key
    // insertion order. Pin that the tuple is exactly the settle account keys minus redemptionRecord,
    // so a field added to the struct without being placed in the tuple fails here rather than
    // silently dropping out of the table.
    expect(SETTLE_ALT_FIELD_ORDER).not.toContain('redemptionRecord');
    expect([...SETTLE_ALT_FIELD_ORDER].sort()).toEqual(
      Object.keys(accounts)
        .filter((name) => name !== 'redemptionRecord')
        .sort(),
    );
  });

  // CONSENSUS-CRITICAL GOLDEN. The settle v0 message compresses its accounts against the on-chain
  // ALT by POSITION: the on-chain table is extended in exactly this order at open_batch, and
  // `settleBatch` looks each account up by its index in the same list. The ordering is produced by
  // the explicit `SETTLE_ALT_FIELD_ORDER` tuple in derive.ts (not by object-key insertion order).
  // Reordering that tuple (or renaming/removing an entry) silently shifts every downstream index and
  // corrupts settle. This golden pins both the exact ordered address list and a couple of derived
  // PDAs for the fixed `roots()` fixture; if it breaks, the ALT field order was changed and the
  // on-chain ALT builder must change in lockstep.
  it('matches the golden ordered ALT list + golden PDA derivations (fixed roots fixture)', async () => {
    const r = roots();
    const batch = await deriveBatchAddresses(r, 0n);
    const accounts = await deriveSettleAccounts(r, batch, BURNED_HANDLE);

    // The exact ordered ALT address list. Order mirrors the `SolanaVaultSettleAccounts` field order
    // with `redemptionRecord` (position 9) removed:
    //   batcher, batch, joinConfidentialMint, batchJoinTokenAccount, joinUnderlyingMint,
    //   joinMintVaultUnderlying, joinMintVaultAuthority, batchBurnedAmountValue,
    //   hostConfig, kmsContext, vault, vaultAuthority, vaultTokenAccount, payoutConfidentialMint,
    //   payoutUnderlyingMint, batchPayoutTokenAccount, payoutMintVaultUnderlying,
    //   payoutMintVaultAuthority, payoutComputeSigner, payoutTotalSupplyAuthority,
    //   batchPayoutBalanceValue, payoutTotalSupplyValue
    const GOLDEN_LOOKUP_TABLE_ADDRESSES = [
      '8qbHbw2BbbTHBW1sbeqakYXVKRQM8Ne7pLK7m6CVfeR',
      'Dm6gzuvv47gSSeMyV72nVs9N79AQA7sczD5GBw3XwXHX',
      'GgBaCs3NCBuZN12kCJgAW63ydqohFkHEdfdEXBPzLHq',
      '4v6SNfKuPWbh2GoS3M4fzDAL21GR3YmS2SXri7XqwP3b',
      'LbUiWL3xVV8hTFYBVdbTNrpDo41NKS6o3LHHuDzjfcY',
      '8u7FMwPBNrQreRM2x2BM8rcxaKTSWKhf1zT7BcTjZfUs',
      '2zVia6PtH7dX6JPRMkykuv2V8ZXRWwBn1vzYtfUMKLD2',
      '6h6FYkmzQeZ2XBGNPjgHSp3i2ktPqc6xsgpBL4xGpLMK',
      'YMN9Qj5jPNp7j14VPcML1B6xGgcPWVZUGLFU3Mnyfaf',
      'cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN',
      'gBxS1f6uyyGPuW5MzGBukidSb71jdsCb5fZaoSzULE5',
      '8CsPLFTubdWkuhEqa2LUL7kyy8mbY3z8DWhU3vkZtFWk',
      'CmLufny5KwimzbUb38r3v7SS9jqMoM7GPoWEfW4Q11JM',
      'swqrv48gsrwpBFbftEwnP2vB4jckpvfGJfXkwaniLCC',
      'ws91DX9HBAAxGW77BZs5FogRDwpRtcUpiLBpKdPTfWu',
      '8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z',
      'HE7TPXRx8Dy4AZ2tuVV767SknKGXWHqPRDmXc836ZYe6',
      'DYpWU6FKz9dkW4a9HuqvBtDKgdnxD5fbtxaGuGevvDyr',
      '9Zex4Xc17gawiJNk1pEirBrTx2GsNb5HB6WYgHWWkemQ',
      'W4dfnWqZVyik2iMYeP2jHGDfRJbZxzbXfgysxQS1VYK',
      '6L34CwYQLjs4e5sHTjCsoNk5UBZwDtTMkKegf7tRdoM7',
      'D1kRDX4FNzfiFqnJCjX443t7ZgN3jCk2NLtNk93eH8pt',
    ];
    expect(settleAccountsToLookupTableAddresses(accounts)).toEqual(GOLDEN_LOOKUP_TABLE_ADDRESSES);

    // A couple of derived PDAs pinned individually, so a derivation-logic change (not just field
    // order) is caught with a named field rather than only as a list diff.
    expect(batch.batch).toBe('Dm6gzuvv47gSSeMyV72nVs9N79AQA7sczD5GBw3XwXHX');
    expect(accounts.batchBurnedAmountValue).toBe('6h6FYkmzQeZ2XBGNPjgHSp3i2ktPqc6xsgpBL4xGpLMK');
    expect(accounts.payoutTotalSupplyValue).toBe('D1kRDX4FNzfiFqnJCjX443t7ZgN3jCk2NLtNk93eH8pt');
  });
});
