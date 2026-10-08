import { prepareTransientStore } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { address, isWritableRole, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildQuitInstruction } from './quit.js';
import {
  QUIT_DISCRIMINATOR,
  getQuitInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/quit.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { findDenyScopeRecordPda } from '@fhevm/solana-zama-host';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

async function quitInput() {
  return {
    transientStore: await prepareTransientStore({ payer: signer(addr(2)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
    user: signer(addr(1)),
    payer: signer(addr(2)),
    batcher: addr(3),
    batch: addr(4),
    joinConfidentialMint: addr(5),
    joinUnderlyingMint: addr(16),
    batchAuthorityAta: addr(17),
    userAta: addr(18),
    batchJoinTokenAccount: addr(7),
    userTokenAccount: addr(8),
    batchBalanceStore: addr(9),
    userBalanceStore: addr(10),
    joinStore: addr(12),
    confidentialTokenEventAuthority: addr(15),
  };
}

describe('buildQuitInstruction', () => {
  it('builds the batcher quit instruction (from-value refund) with the right program, accounts, and data', async () => {
    const input = await quitInput();
    const user = input.user;
    const instruction = await buildQuitInstruction(input);

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    const addresses = instruction.accounts!.map((a) => a.address);
    // 23 accounts, then the four optional HCU accounts, absent: Anchor reads the program id as None.
    expect(addresses).toHaveLength(27);
    expect(addresses.slice(23)).toEqual(Array(4).fill(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS));
    expect(addresses[0]).toBe(user.address); // user signer first
    expect(addresses[3]).toBe(addr(4)); // batch

    const decoded = getQuitInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(QUIT_DISCRIMINATOR));
  });

  it('carries the HCU accounts it is given and, under the deny list, its deny records in program order', async () => {
    const input = await quitInput();
    const instruction = await buildQuitInstruction({
      ...input,
      joinMintHcuBlockMeter: addr(20),
      joinMintHcuTrustedAppRecord: addr(21),
      batchHcuBlockMeter: addr(22),
      batchHcuTrustedAppRecord: addr(23),
      denyListEnabled: true,
    });
    const accounts = instruction.accounts!;
    expect(accounts.slice(23, 27).map((a) => a.address)).toEqual([addr(20), addr(21), addr(22), addr(23)]);
    // The meters are written; the trust records are read.
    expect(accounts.slice(23, 27).map((a) => isWritableRole(a.role))).toEqual([true, false, true, false]);
    const [joinMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: input.joinConfidentialMint });
    const [batchRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, scope: input.batch });
    // The refund touches the join mint and the batch; the reset touches the batch.
    expect(accounts.slice(27).map((a) => [a.address, a.role])).toEqual([
      [joinMintRecord, 0],
      [batchRecord, 0],
      [batchRecord, 0],
    ]);
  });
});
