import {
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from '@solana/kit';

import {
  CLOSE_SCRATCH_DISCRIMINATOR,
  getCloseScratchInstruction,
} from './internal/generated/zamaHost/instructions/closeScratch.js';
import {
  OPEN_SCRATCH_DISCRIMINATOR,
  getOpenScratchInstruction,
} from './internal/generated/zamaHost/instructions/openScratch.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from './internal/generated/zamaHost/programAddress.js';

/** Forward these accounts through every FHE instruction and CPI in the transaction. */
export type SolanaFheTransactionAccounts = {
  readonly scratch: Address;
  readonly instructions: Address;
};

export type SolanaFheTransaction = {
  readonly accounts: SolanaFheTransactionAccounts;
  /** Surround the complete transaction body with one open and one final close. */
  readonly wrap: (body: readonly Instruction[]) => readonly Instruction[];
};

/**
 * Prepares a shared transaction context. The payer funds its refundable rent; each program still
 * authenticates its own encrypted State. Build all participating instructions with `accounts`,
 * then call `wrap` once. This helper does not sign, simulate, or send the transaction.
 */
export async function createSolanaFheTransaction(parameters: {
  readonly payer: TransactionSigner;
}): Promise<SolanaFheTransaction> {
  const programAddress = ZAMA_HOST_PROGRAM_ADDRESS;
  const [scratch] = await getProgramDerivedAddress({
    programAddress,
    seeds: [new TextEncoder().encode('transient'), getAddressEncoder().encode(parameters.payer.address)],
  });
  const accounts = {
    scratch,
    instructions: 'Sysvar1nstructions1111111111111111111111111' as Address,
  };
  const open = getOpenScratchInstruction({ payer: parameters.payer, ...accounts }, { programAddress });
  const close = getCloseScratchInstruction({ ...accounts, refund: parameters.payer.address }, { programAddress });
  return {
    accounts,
    wrap(body) {
      if (
        body.some(
          (ix) =>
            ix.programAddress === programAddress &&
            [OPEN_SCRATCH_DISCRIMINATOR, CLOSE_SCRATCH_DISCRIMINATOR].some((tag) =>
              tag.every((byte, index) => ix.data?.[index] === byte),
            ),
        )
      ) {
        throw new Error('The FHE transaction body must not open or close scratch');
      }
      return [open, ...body, close];
    },
  };
}
