import { SYSTEM_PROGRAM_ADDRESS } from '@solana-program/system';
import { prepareTransientStore } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, isWritableRole, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildQuitInstruction } from './quit.js';
import {
  QUIT_DISCRIMINATOR,
  getQuitInstructionDataDecoder,
  parseQuitInstruction,
} from './internal/generated/confidentialBatcher/instructions/quit.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';
import { findDenyScopeRecordPda } from '@fhevm/solana-zama-host';
import { batchApp, tokenApp } from './internal/hostPolicy.js';
import { hcuSlots, testHostPolicy } from './testHostPolicy.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

const SPL_TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');

async function quitInput() {
  return {
    transientStore: await prepareTransientStore({ payer: signer(addr(2)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
    user: signer(addr(1)),
    payer: signer(addr(2)),
    batcher: addr(3),
    batch: addr(4),
    joinConfidentialMint: addr(5),
    joinUnderlyingMint: addr(16),
    tokenProgram: SPL_TOKEN,
    host: testHostPolicy(false),
  };
}

// Independent restatement of the seeds and labels quit.rs validates, sharing no code with the builder.
const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
const encryptedStore = (program: Address, authority: Address, scope: Address): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-state'),
    base58.decode(program),
    base58.decode(authority),
    base58.decode(scope),
  ]);

describe('buildQuitInstruction', () => {
  it('derives every non-root account exactly as quit.rs validates them, for a plain-address user', async () => {
    const input = { ...(await quitInput()), user: addr(1) };
    const { batch, joinConfidentialMint: mint, joinUnderlyingMint } = input;
    const instruction = await buildQuitInstruction(input);

    const ata = (owner: Address): Promise<Address> =>
      pda(address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'), [
        base58.decode(owner),
        base58.decode(SPL_TOKEN),
        base58.decode(joinUnderlyingMint),
      ]);
    const tokenAccount = (owner: Address): Promise<Address> =>
      pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('token-account'), base58.decode(mint), base58.decode(owner)]);
    const batchAuthority = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('batch-authority'), base58.decode(batch)]);
    const joinRecord = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('join-record'),
      base58.decode(batch),
      base58.decode(addr(1)),
    ]);
    const batchJoinTokenAccount = await tokenAccount(batchAuthority);
    const userTokenAccount = await tokenAccount(addr(1));
    expect(instruction.accounts!.map((a) => a.address)).toEqual([
      addr(1),
      addr(2),
      input.batcher,
      batch,
      batchAuthority,
      joinRecord,
      mint,
      joinUnderlyingMint,
      await ata(batchAuthority),
      await ata(addr(1)),
      batchJoinTokenAccount,
      userTokenAccount,
      await encryptedStore(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, batchJoinTokenAccount, mint),
      await encryptedStore(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, userTokenAccount, mint),
      await encryptedStore(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, joinRecord, batch),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('transient'), base58.decode(addr(2))]),
      address('Sysvar1nstructions1111111111111111111111111'),
      ZAMA_HOST_PROGRAM_ADDRESS,
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('host-config')]),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      SYSTEM_PROGRAM_ADDRESS,
      ...Array(4).fill(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS),
    ]);
    // A refunding quit's user carries no signer role (0x02/0x03 are the signer roles).
    expect(instruction.accounts![0]!.role & 0b10).toBe(0);
  });

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

  it('carries each application\'s HCU accounts and, under the deny list, its deny records in program order', async () => {
    const input = await quitInput();
    const instruction = await buildQuitInstruction({ ...input, host: testHostPolicy(true, true) });
    const { actual, expected } = await hcuSlots(parseQuitInstruction(instruction as never).accounts, {
      joinMint: tokenApp(input.joinConfidentialMint),
      batch: batchApp(input.batch),
    });
    expect(actual).toEqual(expected);
    const accounts = instruction.accounts!;
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
