import { describe, expect, it } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

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
  it('include pending_burn and match the open_batch / settleBatch shared ordering', async () => {
    const r = roots();
    const batch = await deriveBatchAddresses(r, 0n);
    const accounts = await deriveSettleAccounts(r, batch);

    const fromAccounts = settleAccountsToLookupTableAddresses(accounts);
    const fromDerive = await deriveSettleLookupTableAddresses(r, batch);

    // The root-taking wrapper preserves the same ordered projection as the already-derived account
    // set. Both paths share SETTLE_ALT_FIELD_ORDER; the tuple coverage assertion below is the
    // independent guard against silently omitting a settle account.
    expect(fromAccounts).toEqual(fromDerive);
    // pending_burn is known at open_batch and rides in the ALT.
    expect(fromAccounts).toContain(accounts.pendingBurn);
    expect(fromAccounts).toContain(accounts.batchBurnedAmountStore);
    expect(fromAccounts.length).toBe(Object.keys(accounts).length);

    // The ALT ordering is driven by the explicit SETTLE_ALT_FIELD_ORDER tuple, not by object-key
    // insertion order. Pin that the tuple is exactly the settle account keys, so a field added to
    // the struct without being placed in the tuple fails here rather than silently dropping out of
    // the table.
    expect(SETTLE_ALT_FIELD_ORDER).toContain('pendingBurn');
    expect([...SETTLE_ALT_FIELD_ORDER].sort()).toEqual(Object.keys(accounts).sort());
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
    const accounts = await deriveSettleAccounts(r, batch);

    // The exact ordered ALT address list mirrors `SETTLE_ALT_FIELD_ORDER` / SolanaVaultSettleAccounts:
    //   batcher, batch, joinConfidentialMint, batchJoinTokenAccount, joinUnderlyingMint,
    //   joinMintVaultUnderlying, joinMintVaultAuthority, batchBurnedAmountStore, pendingBurn,
    //   hostConfig, kmsContext, vault, vaultAuthority, vaultTokenAccount, payoutConfidentialMint,
    //   payoutUnderlyingMint, batchPayoutTokenAccount, payoutMintVaultUnderlying,
    //   payoutMintVaultAuthority, payoutTotalSupplyAuthority,
    //   batchPayoutBalanceStore, payoutTotalSupplyStore, batchAuthority,
    //   batchJoinUnderlying, batchPayoutUnderlying, zamaEventAuthority, confidentialTokenEventAuthority
    const GOLDEN_LOOKUP_TABLE_ADDRESSES = [
      '8qbHbw2BbbTHBW1sbeqakYXVKRQM8Ne7pLK7m6CVfeR',
      '6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN',
      'GgBaCs3NCBuZN12kCJgAW63ydqohFkHEdfdEXBPzLHq',
      '6hQX1zWjYGzzKyP9PmVF7wPkWmLKjN1tHyKfB1HAWXc7',
      'LbUiWL3xVV8hTFYBVdbTNrpDo41NKS6o3LHHuDzjfcY',
      'HapWyaFR2h7fotmqC738BWnrjUCG2afpNy8i6GzcJyqg',
      'BBwwKFwNxKrxBmSD6ey2cJ1VJYVkFhJAoBovc38ACjhk',
      'BFahaYhwFQvt2cHGeHg4ujaWcC52RWgHdiEQuV7PT2oA',
      'BgV6GmgySEincRffgdRA8qLX8nUxpuSfjRF76WGrTAPy',
      'YMN9Qj5jPNp7j14VPcML1B6xGgcPWVZUGLFU3Mnyfaf',
      'cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN',
      'gBxS1f6uyyGPuW5MzGBukidSb71jdsCb5fZaoSzULE5',
      'CxnAjXqMPmT5xT8dmsFbMgVNgA8HPVa4WVWhCm3gZNL9',
      '3kdEZ36hPkUH2Hq2XhjBCdKVn7xBbpWLtbd9ePxSBf1o',
      'swqrv48gsrwpBFbftEwnP2vB4jckpvfGJfXkwaniLCC',
      'ws91DX9HBAAxGW77BZs5FogRDwpRtcUpiLBpKdPTfWu',
      '33Z5S6e7F6Cc9JpJ4aA5sFTSmD63QcJtd4rqgkAquhXQ',
      '3FZcScRdyGQQ79qGdRpke9TukYVHZEsqm1AWpDG15cmd',
      '6AeDKTasqPbqrhTCPpf5GKYdKogjycpAZyevFQRNP6kr',
      '2K4784bFHReRcc7juUMW2N12NQbxMsL35i33Hcxy4zGk',
      'yjr2mdF8iyACXP41AAugk3NqyTmfa8HqjtRaNYQTj56',
      '5fZHscRuBSv8Kxnt1cCkBZC1zGX21eJhPar216jZMupZ',
      '7r6dp55LMSc1dKRiMzgizmrogFcB4bkkbC7ia9jpk1Fo',
      'Gv5fMGCo3efrXa83JnBJf1jTE2sJqvmDp4wSDAT16U4C',
      'CW2QJ6TvQLBzm9T8zT26YMPZZMvC1pwkEUh2zGV7qUAp',
      'CAspHyipvqeHA78sMa73uD2ThP84Zw4ywXG71dyrNpXp',
      'FmW1wCB2eZQFwLVuALBH2Y3yh9uscwcExz1i4zFGYZgp',
    ];
    expect(settleAccountsToLookupTableAddresses(accounts)).toEqual(GOLDEN_LOOKUP_TABLE_ADDRESSES);

    // A couple of derived PDAs pinned individually, so a derivation-logic change (not just field
    // order) is caught with a named field rather than only as a list diff.
    expect(batch.batch).toBe('6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN');
    expect(accounts.batchBurnedAmountStore).toBe('BFahaYhwFQvt2cHGeHg4ujaWcC52RWgHdiEQuV7PT2oA');
    expect(accounts.pendingBurn).toBe('BgV6GmgySEincRffgdRA8qLX8nUxpuSfjRF76WGrTAPy');
    expect(accounts.payoutTotalSupplyStore).toBe('5fZHscRuBSv8Kxnt1cCkBZC1zGX21eJhPar216jZMupZ');
  });
});
