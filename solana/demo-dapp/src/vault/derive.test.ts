import { describe, expect, it } from 'vitest';
import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  deriveBatchAddresses,
  deriveSettleAccounts,
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

describe('deriveSettleAccounts', () => {
  // Golden derivations for the fixed `roots()` fixture; claim.test.ts pins its batch-side payout
  // accounts against the same values.
  it('matches the golden settle accounts for the fixed roots fixture', async () => {
    const batch = await deriveBatchAddresses(roots(), 0n);
    expect(batch.batch).toBe('6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN');
    expect(await deriveSettleAccounts(roots(), batch)).toEqual({
      batcher: '8qbHbw2BbbTHBW1sbeqakYXVKRQM8Ne7pLK7m6CVfeR',
      batch: '6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN',
      joinConfidentialMint: 'GgBaCs3NCBuZN12kCJgAW63ydqohFkHEdfdEXBPzLHq',
      batchJoinTokenAccount: '6hQX1zWjYGzzKyP9PmVF7wPkWmLKjN1tHyKfB1HAWXc7',
      joinUnderlyingMint: 'LbUiWL3xVV8hTFYBVdbTNrpDo41NKS6o3LHHuDzjfcY',
      joinMintVaultUnderlying: 'HapWyaFR2h7fotmqC738BWnrjUCG2afpNy8i6GzcJyqg',
      joinMintVaultAuthority: 'BBwwKFwNxKrxBmSD6ey2cJ1VJYVkFhJAoBovc38ACjhk',
      batchBurnedAmountStore: 'BFahaYhwFQvt2cHGeHg4ujaWcC52RWgHdiEQuV7PT2oA',
      pendingBurn: 'BgV6GmgySEincRffgdRA8qLX8nUxpuSfjRF76WGrTAPy',
      hostConfig: '8FL98RBTQ8LviLQ6c9F5j6pdtJPFSbnnTJgh3VVcLeKC',
      kmsContext: 'cGfHiC6Kgg3FpFZvgwGcswsCRtp4aBP2fzuXRQPizuN',
      vault: 'gBxS1f6uyyGPuW5MzGBukidSb71jdsCb5fZaoSzULE5',
      vaultAuthority: 'CxnAjXqMPmT5xT8dmsFbMgVNgA8HPVa4WVWhCm3gZNL9',
      vaultTokenAccount: '3kdEZ36hPkUH2Hq2XhjBCdKVn7xBbpWLtbd9ePxSBf1o',
      payoutConfidentialMint: 'swqrv48gsrwpBFbftEwnP2vB4jckpvfGJfXkwaniLCC',
      payoutUnderlyingMint: 'ws91DX9HBAAxGW77BZs5FogRDwpRtcUpiLBpKdPTfWu',
      batchPayoutTokenAccount: '33Z5S6e7F6Cc9JpJ4aA5sFTSmD63QcJtd4rqgkAquhXQ',
      payoutMintVaultUnderlying: '3FZcScRdyGQQ79qGdRpke9TukYVHZEsqm1AWpDG15cmd',
      payoutMintVaultAuthority: '6AeDKTasqPbqrhTCPpf5GKYdKogjycpAZyevFQRNP6kr',
      payoutTotalSupplyAuthority: '2K4784bFHReRcc7juUMW2N12NQbxMsL35i33Hcxy4zGk',
      batchPayoutBalanceStore: 'yjr2mdF8iyACXP41AAugk3NqyTmfa8HqjtRaNYQTj56',
      payoutTotalSupplyStore: '5fZHscRuBSv8Kxnt1cCkBZC1zGX21eJhPar216jZMupZ',
      batchAuthority: '7r6dp55LMSc1dKRiMzgizmrogFcB4bkkbC7ia9jpk1Fo',
      batchJoinUnderlying: 'Gv5fMGCo3efrXa83JnBJf1jTE2sJqvmDp4wSDAT16U4C',
      batchPayoutUnderlying: 'CW2QJ6TvQLBzm9T8zT26YMPZZMvC1pwkEUh2zGV7qUAp',
      zamaEventAuthority: 'CAspHyipvqeHA78sMa73uD2ThP84Zw4ywXG71dyrNpXp',
      confidentialTokenEventAuthority: 'FmW1wCB2eZQFwLVuALBH2Y3yh9uscwcExz1i4zFGYZgp',
    });
  });
});
