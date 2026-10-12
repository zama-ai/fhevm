import { afterEach, describe, expect, it, vi } from 'vitest';

const sendAndConfirm = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  sendAndConfirmTransactionFactory: () => sendAndConfirm,
}));

import { SYSTEM_PROGRAM_ADDRESS } from '@solana-program/system';
import { prepareTransientStore } from '@fhevm/sdk/solana';
import {
  address,
  generateKeyPairSigner,
  getProgramDerivedAddress,
  getU64Encoder,
  type Address,
  type Transaction,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { buildDispatchBatchInstruction } from './dispatchBatch.js';
import {
  DISPATCH_DISCRIMINATOR,
  getDispatchInstructionDataDecoder,
  parseDispatchInstruction,
} from './internal/generated/confidentialBatcher/instructions/dispatch.js';
import { tokenApp } from './internal/hostPolicy.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { hcuSlots, testHostPolicy } from './testHostPolicy.js';
import { expectFitsV1, testDemoClient } from '../testDemoClient';
import type { VaultDemoRoots } from './derive.js';

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
const tokenValuePda = (mint: Address, authority: Address): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-state'),
    base58.decode(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    base58.decode(authority),
    base58.decode(mint),
  ]);

describe('buildDispatchBatchInstruction', () => {
  // Fixture aligned with derive.test.ts's golden: batcher = addr(2), batch index 0, join mint =
  // addr(13) (derive.test's payout mint) — so the batch, token-account, balance and total-supply
  // expectations below are pinned to the same golden base58 strings.
  const payer = signer(addr(1));
  const batcher = addr(2);
  const batch = address('6qzexx3j3oQraXoc1Ji8SdrKBLZaorG8HxtPZtprtoUN');
  const joinConfidentialMint = addr(13);
  const joinUnderlyingMint = addr(14);
  const payoutConfidentialMint = addr(15);
  const payoutUnderlyingMint = addr(16);
  const roots: VaultDemoRoots = {
    batcherProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    tokenProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    vaultProgram: addr(30),
    hostProgram: ZAMA_HOST_PROGRAM_ADDRESS,
    batcher,
    vault: addr(10),
    joinConfidentialMint,
    payoutConfidentialMint,
    joinUnderlyingMint,
    payoutUnderlyingMint,
  };
  const SPL_TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
  const ASSOCIATED_TOKEN = address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
  const ata = (owner: Address, mint: Address): Promise<Address> =>
    pda(ASSOCIATED_TOKEN, [base58.decode(owner), base58.decode(SPL_TOKEN), base58.decode(mint)]);
  const tokenAccount = (mint: Address, owner: Address): Promise<Address> =>
    pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('token-account'), base58.decode(mint), base58.decode(owner)]);
  const batchAuthorityOf = (batch: Address): Promise<Address> =>
    pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('batch-authority'), base58.decode(batch)]);
  const input = async (host = testHostPolicy(false), feePayer = payer) => ({
    transientStore: await prepareTransientStore({ payer: feePayer, host: ZAMA_HOST_PROGRAM_ADDRESS }),
    payer: feePayer,
    roots,
    batchIndex: 0n,
    authorityFundingLamports: 5_000_000n,
    host,
  });

  afterEach(() => vi.restoreAllMocks());

  it('derives every non-root account exactly as dispatch.rs validates them', async () => {
    const instruction = await buildDispatchBatchInstruction(await input());

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);

    const batchAuthority = await batchAuthorityOf(batch);
    const batchJoinTokenAccount = await tokenAccount(joinConfidentialMint, batchAuthority);
    const totalSupplyAuthority = await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
      utf8('total-supply'),
      base58.decode(joinConfidentialMint),
    ]);
    const nextBatch = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('batch'),
      base58.decode(batcher),
      new Uint8Array(getU64Encoder().encode(1n)),
    ]);
    const nextBatchAuthority = await batchAuthorityOf(nextBatch);
    const nextJoinTokenAccount = await tokenAccount(joinConfidentialMint, nextBatchAuthority);
    const nextPayoutTokenAccount = await tokenAccount(payoutConfidentialMint, nextBatchAuthority);
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
      await tokenValuePda(joinConfidentialMint, batchJoinTokenAccount),
      await tokenValuePda(joinConfidentialMint, totalSupplyAuthority),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
        utf8('pending-burn'),
        base58.decode(joinConfidentialMint),
        base58.decode(batchJoinTokenAccount),
      ]),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('transient'), base58.decode(payer.address)]),
      address('Sysvar1nstructions1111111111111111111111111'),
      ZAMA_HOST_PROGRAM_ADDRESS,
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('host-config')]),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      SPL_TOKEN,
      SYSTEM_PROGRAM_ADDRESS,
      // The next batch, which this dispatch opens.
      nextBatch,
      nextBatchAuthority,
      nextJoinTokenAccount,
      await tokenValuePda(joinConfidentialMint, nextJoinTokenAccount),
      payoutConfidentialMint,
      payoutUnderlyingMint,
      nextPayoutTokenAccount,
      await tokenValuePda(payoutConfidentialMint, nextPayoutTokenAccount),
      await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('batch-join-underlying'), base58.decode(nextBatch)]),
      await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('batch-payout-underlying'), base58.decode(nextBatch)]),
      // The optional HCU accounts, absent: Anchor reads the program id as None.
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    const decoded = getDispatchInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(DISPATCH_DISCRIMINATOR));
    expect(decoded.authorityFundingLamports).toBe(5_000_000n);
  });

  // Golden pins for the fixed fixture, shared with derive.test.ts; the event authorities from
  // `solana find-program-derived-address <program> string:__event_authority`.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildDispatchBatchInstruction(await input());
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses[2]).toBe(batch);
    expect(addresses[3]).toBe('7r6dp55LMSc1dKRiMzgizmrogFcB4bkkbC7ia9jpk1Fo'); // batchAuthority
    expect(addresses[7]).toBe('2K4784bFHReRcc7juUMW2N12NQbxMsL35i33Hcxy4zGk'); // totalSupplyAuthority
    expect(addresses[8]).toBe('33Z5S6e7F6Cc9JpJ4aA5sFTSmD63QcJtd4rqgkAquhXQ'); // batchJoinTokenAccount
    expect(addresses[9]).toBe('yjr2mdF8iyACXP41AAugk3NqyTmfa8HqjtRaNYQTj56'); // batchBalanceValue
    expect(addresses[10]).toBe('5fZHscRuBSv8Kxnt1cCkBZC1zGX21eJhPar216jZMupZ'); // totalSupplyStore
    // addresses[11] = pendingBurn
    expect(addresses[12]).toBe('CAspHyipvqeHA78sMa73uD2ThP84Zw4ywXG71dyrNpXp'); // zamaEventAuthority
    expect(addresses[17]).toBe('FmW1wCB2eZQFwLVuALBH2Y3yh9uscwcExz1i4zFGYZgp'); // tokenEventAuthority
  });

  it('appends, under the deny list, the join mint deny record for the burn and each mint\'s for the next batch', async () => {
    const plain = await buildDispatchBatchInstruction(await input());
    const instruction = await buildDispatchBatchInstruction(await input(testHostPolicy(true)));
    const [joinMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: joinConfidentialMint });
    const [payoutMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: payoutConfidentialMint });
    expect(instruction.accounts!.slice(plain.accounts!.length)).toEqual([
      { address: joinMintRecord, role: 0 },
      { address: joinMintRecord, role: 0 },
      { address: payoutMintRecord, role: 0 },
    ]);
  });

  it("puts each mint's HCU accounts in its slots", async () => {
    const instruction = await buildDispatchBatchInstruction(await input(testHostPolicy(false, true)));
    const { actual, expected } = await hcuSlots(parseDispatchInstruction(instruction as never).accounts, {
      joinMint: tokenApp(joinConfidentialMint),
      payoutMint: tokenApp(payoutConfidentialMint),
    });
    expect(actual).toEqual(expected);
  });

  // The largest dispatch: both mints' deny records and HCU witnesses.
  it('fits one v1 transaction with every witness', async () => {
    sendAndConfirm.mockReset().mockResolvedValue(undefined);
    const keeper = await generateKeyPairSigner();
    const { client } = testDemoClient(keeper);
    const parameters = await input(testHostPolicy(true, true), keeper);
    await client.sendFheTransaction(parameters.transientStore, [await buildDispatchBatchInstruction(parameters)]);
    expectFitsV1(sendAndConfirm.mock.lastCall![0] as Transaction);
  });
});
