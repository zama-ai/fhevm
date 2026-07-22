import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildClaimInstruction } from './claim.js';
import {
  CLAIM_DISCRIMINATOR,
  getClaimInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/claim.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('buildClaimInstruction', () => {
  it('builds the permissionless claim with user as an unchecked (non-signer) account', async () => {
    const payer = signer(addr(1));
    const user = addr(2); // NOT a signer — permissionless pull
    const instruction = await buildClaimInstruction({
      payer,
      user,
      batcher: addr(3),
      batch: addr(4),
      pendingJoinValue: addr(5),
      claimAmountValue: addr(6),
      payoutConfidentialMint: addr(7),
      payoutComputeSigner: addr(8),
      batchPayoutTokenAccount: addr(9),
      userPayoutTokenAccount: addr(10),
      batchPayoutBalanceValue: addr(11),
      userPayoutBalanceValue: addr(12),
      batchPayoutTransferredValue: addr(13),
      zamaEventAuthority: addr(14),
      hostConfig: addr(15),
      confidentialTokenEventAuthority: addr(16),
    });

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    const accounts = instruction.accounts!;
    expect(accounts).toHaveLength(21);
    expect(accounts[0]!.address).toBe(payer.address); // payer signs
    expect(accounts[1]!.address).toBe(user); // user is unchecked
    // user carries no signer role (0x02/0x03 are the signer roles).
    expect(accounts[1]!.role & 0b10).toBe(0);

    const decoded = getClaimInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(CLAIM_DISCRIMINATOR));
  });
});
