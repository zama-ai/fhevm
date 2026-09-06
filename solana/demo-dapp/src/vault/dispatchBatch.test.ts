import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildDispatchBatchInstruction } from './dispatchBatch.js';
import {
  DISPATCH_DISCRIMINATOR,
  getDispatchInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

// Independent restatement of every seed / label the builder's derivations must reproduce, so the
// expected list below shares no code with the implementation (dispatch.rs is the common reference).
const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
// PDA(zamaHost, ["encrypted-value", token program, authority, mint, label]) — the crate's
// `encrypted_value_seeds` for a token-program value scoped to its mint.
const tokenValuePda = (mint: Address, authority: Address, label: Uint8Array): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-value'),
    base58.decode(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    base58.decode(authority),
    base58.decode(mint),
    label,
  ]);

describe('buildDispatchBatchInstruction', () => {
  // Fixture aligned with derive.test.ts's consensus-critical golden: batcher = addr(2),
  // batch = the golden batch PDA for index 0, mint = addr(13) — so the token-account, balance and
  // total-supply expectations below can be pinned to the same golden base58 strings.
  const payer = signer(addr(1));
  const batcher = addr(2);
  const batch = address('Dm6gzuvv47gSSeMyV72nVs9N79AQA7sczD5GBw3XwXHX');
  const joinConfidentialMint = addr(13);
  const joinUnderlyingMint = addr(14);
  const hostConfig = addr(8);
  const SPL_TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
  const ASSOCIATED_TOKEN = address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
  const ata = (owner: Address, mint: Address): Promise<Address> =>
    pda(ASSOCIATED_TOKEN, [base58.decode(owner), base58.decode(SPL_TOKEN), base58.decode(mint)]);

  it('derives every non-root account exactly as dispatch.rs validates them', async () => {
    const instruction = await buildDispatchBatchInstruction({
      payer,
      batcher,
      batch,
      joinConfidentialMint,
      joinUnderlyingMint,
      tokenProgram: SPL_TOKEN,
      hostConfig,
    });

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);

    const batchAuthority = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('batch-authority'),
      base58.decode(batch),
    ]);
    const batchJoinTokenAccount = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('token-account'),
      base58.decode(joinConfidentialMint),
      base58.decode(batchAuthority),
    ]);
    const totalSupplyAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('total-supply'),
      base58.decode(joinConfidentialMint),
    ]);
    const expected: Address[] = [
      payer.address,
      batcher,
      batch,
      batchAuthority,
      joinConfidentialMint,
      joinUnderlyingMint,
      await ata(batchAuthority, joinUnderlyingMint),
      totalSupplyAuthority,
      batchJoinTokenAccount,
      await tokenValuePda(joinConfidentialMint, batchJoinTokenAccount, utf8('balance_________________________')),
      await tokenValuePda(joinConfidentialMint, totalSupplyAuthority, utf8('total_supply____________________')),
      await tokenValuePda(joinConfidentialMint, batchJoinTokenAccount, utf8('burned_amount___________________')),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
        utf8('pending-burn'),
        base58.decode(joinConfidentialMint),
        base58.decode(batchJoinTokenAccount),
      ]),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      ZAMA_HOST_PROGRAM_ADDRESS,
      hostConfig,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      address('11111111111111111111111111111111'),
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    const decoded = getDispatchInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(DISPATCH_DISCRIMINATOR));
  });

  // Golden pins for the fixed fixture: the value accounts are re-pinned from the RFC 035 seed
  // derivation (`encrypted_value_seeds`, mirrored and pinned in the SDK's encryptedValueAccount
  // test), the rest carried over unchanged, the event authorities from
  // `solana find-program-derived-address <program> string:__event_authority`.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildDispatchBatchInstruction({
      payer,
      batcher,
      batch,
      joinConfidentialMint,
      joinUnderlyingMint,
      tokenProgram: SPL_TOKEN,
      hostConfig,
    });
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses[7]).toBe('W4dfnWqZVyik2iMYeP2jHGDfRJbZxzbXfgysxQS1VYK'); // totalSupplyAuthority
    expect(addresses[8]).toBe('8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z'); // batchJoinTokenAccount
    expect(addresses[9]).toBe('3i11PrkLtKRVttNh4XhLcrvVyZp4yfyZroUJeL1ijZrM'); // batchBalanceValue
    expect(addresses[10]).toBe('EHNVHNm2M214V2QXYwrXVPHFbCTCi9uPGBVcECRKBgqg'); // totalSupplyValue
    // addresses[11] = batchBurnedAmountValue; addresses[12] = pendingBurn
    expect(addresses[13]).toBe('7usNGbH9WupMAsyDeqdUEoKrjisKcgusGjDiju4vNog'); // zamaEventAuthority
    expect(addresses[16]).toBe('2KQ5N8YEUTk8hQWXBnkGjsvKPzm2rh2nFH6PeoVt7q8U'); // tokenEventAuthority
  });
});
