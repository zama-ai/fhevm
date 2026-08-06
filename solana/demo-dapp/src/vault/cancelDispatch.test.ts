import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { buildCancelDispatchInstruction } from './cancelDispatch.js';
import {
  CANCEL_DISPATCH_DISCRIMINATOR,
  getCancelDispatchInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/cancelDispatch.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';

const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));
const signer = (value: Address): TransactionSigner =>
  ({ address: value, signTransactions: async () => [] }) as unknown as TransactionSigner;
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
const concat = (...parts: Uint8Array[]): Uint8Array => {
  const result = new Uint8Array(parts.reduce((length, part) => length + part.length, 0));
  let offset = 0;
  for (const part of parts) {
    result.set(part, offset);
    offset += part.length;
  }
  return result;
};
const valueAccountPda = (domain: Address, authority: Address, label: string): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-value'),
    sha256(
      concat(
        utf8('zama-encrypted-value-key-v1'),
        base58.decode(domain),
        base58.decode(authority),
        utf8(label),
      ),
    ),
  ]);

describe('buildCancelDispatchInstruction', () => {
  it('pins the wrapper authority, account derivations, roles, and funding argument', async () => {
    const payer = signer(addr(1));
    const batcher = addr(2);
    const batch = addr(3);
    const mint = addr(4);
    const hostConfig = addr(5);
    const instruction = await buildCancelDispatchInstruction({
      payer,
      batcher,
      batch,
      joinConfidentialMint: mint,
      hostConfig,
      authorityFundingLamports: 7n,
    });

    const batchAuthority = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('batch-authority'),
      base58.decode(batch),
    ]);
    const batchJoinTokenAccount = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('token-account'),
      base58.decode(mint),
      base58.decode(batchAuthority),
    ]);
    const totalSupplyAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('total-supply'),
      base58.decode(mint),
    ]);
    const expected: Address[] = [
      payer.address,
      batcher,
      batch,
      batchAuthority,
      mint,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('fhe-compute'), base58.decode(mint)]),
      totalSupplyAuthority,
      batchJoinTokenAccount,
      await valueAccountPda(mint, batchJoinTokenAccount, 'balance_________________________'),
      await valueAccountPda(mint, totalSupplyAuthority, 'total_supply____________________'),
      await valueAccountPda(mint, batchJoinTokenAccount, 'burned_amount___________________'),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
        utf8('pending-burn'),
        base58.decode(mint),
        base58.decode(batchJoinTokenAccount),
      ]),
      hostConfig,
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      ZAMA_HOST_PROGRAM_ADDRESS,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      address('11111111111111111111111111111111'),
    ];

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    expect(instruction.accounts!.map((account) => account.address)).toEqual(expected);
    expect(instruction.accounts!.map((account) => account.role)).toEqual([
      3, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0,
    ]);

    const decoded = getCancelDispatchInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(CANCEL_DISPATCH_DISCRIMINATOR));
    expect(decoded.authorityFundingLamports).toBe(7n);
  });
});
