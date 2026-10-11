import { prepareTransientStore } from '@fhevm/sdk/solana';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildInitializeBatcherInstruction } from './initializeBatcher.js';
import { parseInitializeBatcherInstruction } from './internal/generated/confidentialBatcher/instructions/initializeBatcher.js';
import { BatchDirection } from './internal/generated/confidentialBatcher/types/batchDirection.js';
import { tokenApp } from './internal/hostPolicy.js';
import { hcuSlots, testHostPolicy } from './testHostPolicy.js';

const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));
const signer = (value: Address): TransactionSigner =>
  ({ address: value, signTransactions: async () => [] }) as unknown as TransactionSigner;

describe('buildInitializeBatcherInstruction', () => {
  const payer = signer(addr(1));
  // derive.test.ts's roots fixture, whose batch 0 it pins.
  const input = async (host = testHostPolicy(false)) => ({
    transientStore: await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS }),
    payer,
    batcher: signer(addr(2)),
    roots: {
      batcherProgram: addr(30),
      tokenProgram: addr(31),
      vaultProgram: addr(32),
      hostProgram: addr(33),
      vault: addr(10),
      joinConfidentialMint: addr(4),
      payoutConfidentialMint: addr(13),
      joinUnderlyingMint: addr(5),
      payoutUnderlyingMint: addr(14),
    },
    minBatchAgeSecs: 10n,
    settleDeadlineSecs: 120n,
    direction: BatchDirection.Deposit,
    authorityFundingLamports: 0n,
    tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
    host,
  });

  it('opens batch 0 of the batcher', async () => {
    const { accounts } = parseInitializeBatcherInstruction((await buildInitializeBatcherInstruction(await input())) as never);
    expect(accounts.batcher.address).toBe(addr(2));
    expect(accounts.batch.address).toBe('6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN');
    expect(accounts.batchAuthority.address).toBe('7r6dp55LMSc1dKRiMzgizmrogFcB4bkkbC7ia9jpk1Fo');
    expect(accounts.batchPayoutTokenAccount.address).toBe('33Z5S6e7F6Cc9JpJ4aA5sFTSmD63QcJtd4rqgkAquhXQ');
    expect(accounts.batchPayoutBalanceStore.address).toBe('yjr2mdF8iyACXP41AAugk3NqyTmfa8HqjtRaNYQTj56');
    expect(accounts.batchJoinUnderlying.address).toBe('Gv5fMGCo3efrXa83JnBJf1jTE2sJqvmDp4wSDAT16U4C');
    expect(accounts.batchPayoutUnderlying.address).toBe('CW2QJ6TvQLBzm9T8zT26YMPZZMvC1pwkEUh2zGV7qUAp');
  });

  it('appends, under the deny list, the join then the payout mint deny record', async () => {
    const plain = await buildInitializeBatcherInstruction(await input());
    const instruction = await buildInitializeBatcherInstruction(await input(testHostPolicy(true)));
    const [joinMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: addr(4) });
    const [payoutMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: addr(13) });
    // Each token account initialization runs as its mint.
    expect(instruction.accounts!.slice(plain.accounts!.length)).toEqual([
      { address: joinMintRecord, role: 0 },
      { address: payoutMintRecord, role: 0 },
    ]);
  });

  it("puts the join and the payout mint's HCU accounts in their slots", async () => {
    const instruction = await buildInitializeBatcherInstruction(await input(testHostPolicy(false, true)));
    const { actual, expected } = await hcuSlots(parseInitializeBatcherInstruction(instruction as never).accounts, {
      joinMint: tokenApp(addr(4)),
      payoutMint: tokenApp(addr(13)),
    });
    expect(actual).toEqual(expected);
  });
});
