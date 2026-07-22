import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildQuitInstruction } from './quit.js';
import {
  QUIT_DISCRIMINATOR,
  getQuitInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/quit.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('buildQuitInstruction', () => {
  it('builds the batcher quit instruction (from-value refund) with the right program, accounts, and data', async () => {
    const user = signer(addr(1));
    const instruction = await buildQuitInstruction({
      user,
      payer: signer(addr(2)),
      batcher: addr(3),
      batch: addr(4),
      joinConfidentialMint: addr(5),
      joinComputeSigner: addr(6),
      batchJoinTokenAccount: addr(7),
      userTokenAccount: addr(8),
      batchBalanceValue: addr(9),
      userBalanceValue: addr(10),
      batchTransferredValue: addr(11),
      pendingJoinValue: addr(12),
      zamaEventAuthority: addr(13),
      hostConfig: addr(14),
      confidentialTokenEventAuthority: addr(15),
    });

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses).toHaveLength(20);
    expect(addresses[0]).toBe(user.address); // user signer first
    expect(addresses[3]).toBe(addr(4)); // batch

    const decoded = getQuitInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(QUIT_DISCRIMINATOR));
  });
});
