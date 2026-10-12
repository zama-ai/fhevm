import { prepareTransientStore } from '@fhevm/sdk/solana';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { deriveBatchAddresses } from './derive.js';
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
    host,
  });

  it('opens batch 0 of the batcher', async () => {
    const parameters = await input();
    const { accounts } = parseInitializeBatcherInstruction((await buildInitializeBatcherInstruction(parameters)) as never);
    const batch = await deriveBatchAddresses({ ...parameters.roots, batcher: addr(2) }, 0n);
    expect(accounts.batcher.address).toBe(addr(2));
    expect(accounts.batch.address).toBe(batch.batch);
    expect(accounts.batchAuthority.address).toBe(batch.batchAuthority);
    expect(accounts.batchJoinTokenAccount.address).toBe(batch.batchJoinTokenAccount);
    expect(accounts.batchJoinBalanceStore.address).toBe(batch.batchBurnedAmountStore);
    expect(accounts.batchPayoutTokenAccount.address).toBe(batch.batchPayoutTokenAccount);
    expect(accounts.batchPayoutBalanceStore.address).toBe(batch.batchPayoutBalanceStore);
    expect(accounts.batchJoinUnderlying.address).toBe(batch.batchJoinUnderlying);
    expect(accounts.batchPayoutUnderlying.address).toBe(batch.batchPayoutUnderlying);
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
