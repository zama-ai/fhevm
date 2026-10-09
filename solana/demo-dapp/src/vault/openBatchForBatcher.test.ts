import { prepareTransientStore } from '@fhevm/sdk/solana';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { openBatchForBatcher } from './openBatchForBatcher.js';

const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));
const signer = (value: Address): TransactionSigner =>
  ({ address: value, signTransactions: async () => [] }) as unknown as TransactionSigner;

describe('openBatchForBatcher', () => {
  it('appends, under the deny list, the join then the payout mint deny record to open_batch', async () => {
    const payer = signer(addr(1));
    const input = {
      transientStore: await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS }),
      roots: {
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
      },
      batchIndex: 0n,
      payer,
      authorityFundingLamports: 0n,
    };
    const plain = await openBatchForBatcher(input);
    const instruction = await openBatchForBatcher({ ...input, denyListEnabled: true });
    const [joinMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: addr(4) });
    const [payoutMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: addr(13) });
    // Each token account initialization runs as its mint.
    expect(instruction.accounts!.slice(plain.accounts!.length)).toEqual([
      { address: joinMintRecord, role: 0 },
      { address: payoutMintRecord, role: 0 },
    ]);
  });
});
