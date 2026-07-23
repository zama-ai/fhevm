import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildDispatchBatchInstruction } from './dispatchBatch.js';
import {
  DISPATCH_DISCRIMINATOR,
  getDispatchInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('buildDispatchBatchInstruction', () => {
  it('builds the permissionless dispatch instruction with no arguments', async () => {
    const payer = signer(addr(1));
    const instruction = await buildDispatchBatchInstruction({
      payer,
      batcher: addr(2),
      batch: addr(3),
      joinConfidentialMint: addr(4),
      joinComputeSigner: addr(5),
      totalSupplyAuthority: addr(6),
      batchJoinTokenAccount: addr(7),
      batchBalanceValue: addr(8),
      totalSupplyValue: addr(9),
      batchBurnedAmountValue: addr(10),
      zamaEventAuthority: addr(11),
      hostConfig: addr(12),
      confidentialTokenEventAuthority: addr(13),
    });

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses).toHaveLength(17);
    expect(addresses[0]).toBe(payer.address);

    const decoded = getDispatchInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(DISPATCH_DISCRIMINATOR));
  });
});
