import { createSolanaFheTransaction } from '@fhevm/sdk/solana';
import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildClaimInstruction } from './claim.js';
import {
  CLAIM_DISCRIMINATOR,
  getClaimInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/claim.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@sdk-src/solana/internal/generated/zamaHost/programAddress.js';
import {
  CLOSE_TRANSIENT_STORE_DISCRIMINATOR,
  getCloseTransientStoreInstructionDataDecoder,
} from '@sdk-src/solana/internal/generated/zamaHost/instructions/closeTransientStore.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

// Independent restatement of every seed / label the builder's derivations must reproduce, so the
// expected list below shares no code with the implementation (claim.rs is the common reference).
const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
const tokenValuePda = (mint: Address, authority: Address): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-state'),
    base58.decode(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    base58.decode(authority),
    base58.decode(mint),
  ]);
// A batcher value: the batcher program scoped to the batch, controlled by the batch authority.
const batcherValuePda = (batch: Address, batchAuthority: Address): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-state'),
    base58.decode(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS),
    base58.decode(batchAuthority),
    base58.decode(batch),
  ]);
describe('buildClaimInstruction', () => {
  // Fixture aligned with derive.test.ts's consensus-critical golden: batcher = addr(2),
  // batch = the golden batch PDA for index 0, payout mint = addr(13) — so the batch-side payout
  // token-account / balance expectations can be pinned to the same golden base58 strings.
  const payer = signer(addr(1));
  const user = addr(100); // NOT a signer — permissionless pull
  const batcher = addr(2);
  const batch = address('Dm6gzuvv47gSSeMyV72nVs9N79AQA7sczD5GBw3XwXHX');
  const payoutConfidentialMint = addr(13);
  const payoutUnderlyingMint = addr(14);
  const hostConfig = addr(8);
  const SPL_TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
  const ASSOCIATED_TOKEN = address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
  const ata = (owner: Address, mint: Address): Promise<Address> =>
    pda(ASSOCIATED_TOKEN, [base58.decode(owner), base58.decode(SPL_TOKEN), base58.decode(mint)]);

  it('derives every non-root account exactly as claim.rs validates them', async () => {
    const fhe = await createSolanaFheTransaction({ payer });
    const instruction = await buildClaimInstruction({
      fhe: fhe.accounts,
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
      hostConfig,
    });
    const instructions = fhe.wrap([instruction]);
    expect(instructions).toHaveLength(3);
    const close = instructions[2]!;

    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);

    const batchAuthority = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('batch-authority'),
      base58.decode(batch),
    ]);
    const tokenAccount = (owner: Address): Promise<Address> =>
      pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [
        utf8('token-account'),
        base58.decode(payoutConfidentialMint),
        base58.decode(owner),
      ]);
    const batchPayoutTokenAccount = await tokenAccount(batchAuthority);
    const userPayoutTokenAccount = await tokenAccount(user);
    const record = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('join-record'),
      base58.decode(batch),
      base58.decode(user),
    ]);
    const state = await batcherValuePda(batch, record);
    const transientStore = await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('transient'), base58.decode(payer.address)]);
    const expected: Address[] = [
      payer.address,
      user,
      batcher,
      batch,
      batchAuthority,
      await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('join-record'), base58.decode(batch), base58.decode(user)]),
      state,
      transientStore,
      address('Sysvar1nstructions1111111111111111111111111'),
      payoutConfidentialMint,
      payoutUnderlyingMint,
      await ata(batchAuthority, payoutUnderlyingMint),
      await ata(user, payoutUnderlyingMint),
      batchPayoutTokenAccount,
      userPayoutTokenAccount,
      await tokenValuePda(payoutConfidentialMint, batchPayoutTokenAccount),
      await tokenValuePda(payoutConfidentialMint, userPayoutTokenAccount),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      ZAMA_HOST_PROGRAM_ADDRESS,
      hostConfig,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      address('11111111111111111111111111111111'),
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    expect(close).toBeDefined();
    expect(close!.programAddress).toBe(ZAMA_HOST_PROGRAM_ADDRESS);
    expect(close!.accounts!.map((a) => a.address)).toEqual([
      address('Sysvar1nstructions1111111111111111111111111'),
      transientStore,
      payer.address,
    ]);
    expect(Array.from(getCloseTransientStoreInstructionDataDecoder().decode(close!.data!).discriminator)).toEqual(
      Array.from(CLOSE_TRANSIENT_STORE_DISCRIMINATOR),
    );
    // user carries no signer role (0x02/0x03 are the signer roles).
    expect(instruction.accounts![1]!.role & 0b10).toBe(0);

    const decoded = getClaimInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(CLAIM_DISCRIMINATOR));
  });

  // Golden pins for the fixed fixture: the encrypted States are re-pinned from the RFC 035 seed
  // derivation (`encrypted_state_seeds`, mirrored and pinned in the SDK's encryptedState
  // test), the rest carried over unchanged, the event authorities from
  // `solana find-program-derived-address <program> string:__event_authority`.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildClaimInstruction({
      fhe: (await createSolanaFheTransaction({ payer })).accounts,
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
      hostConfig,
    });
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses[13]).toBe('8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z'); // batchPayoutTokenAccount
    expect(addresses[15]).toBe('Fc46oMpQnJjHqM1YNvc6TYgqRjTRyqu71rVKXAedUt4B'); // batchPayoutBalanceState
    expect(addresses[17]).toBe('7usNGbH9WupMAsyDeqdUEoKrjisKcgusGjDiju4vNog'); // zamaEventAuthority
    expect(addresses[20]).toBe('2KQ5N8YEUTk8hQWXBnkGjsvKPzm2rh2nFH6PeoVt7q8U'); // tokenEventAuthority
  });
});
