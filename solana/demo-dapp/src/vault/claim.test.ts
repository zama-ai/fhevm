import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { buildClaimInstruction } from './claim.js';
import {
  CLAIM_DISCRIMINATOR,
  getClaimInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/claim.js';
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
// expected list below shares no code with the implementation (claim.rs is the common reference).
const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const pda = async (programAddress: Address, seeds: Uint8Array[]): Promise<Address> =>
  (await getProgramDerivedAddress({ programAddress, seeds }))[0];
const concat = (...parts: Uint8Array[]): Uint8Array => {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
};
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
// A batcher value: the batcher program scoped to the batch, controlled by the batch authority.
const batcherValuePda = (batch: Address, batchAuthority: Address, label: Uint8Array): Promise<Address> =>
  pda(ZAMA_HOST_PROGRAM_ADDRESS, [
    utf8('encrypted-value'),
    base58.decode(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS),
    base58.decode(batchAuthority),
    base58.decode(batch),
    label,
  ]);
// Batcher per-user labels: sha256(purpose_prefix || user) — encrypted_pending_join_label / encrypted_claim_amount_label.
const userLabel = (purposePrefix: string, user: Address): Uint8Array =>
  sha256(concat(utf8(purposePrefix), base58.decode(user)));

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
    const instruction = await buildClaimInstruction({
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
      hostConfig,
    });

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
    const ENCRYPTED_BALANCE_LABEL = utf8('balance_________________________');
    const expected: Address[] = [
      payer.address,
      user,
      batcher,
      batch,
      batchAuthority,
      await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('join-record'), base58.decode(batch), base58.decode(user)]),
      await batcherValuePda(batch, batchAuthority, userLabel('batcher-pending-join', user)),
      await batcherValuePda(batch, batchAuthority, userLabel('batcher-claim-amount', user)),
      payoutConfidentialMint,
      payoutUnderlyingMint,
      await ata(batchAuthority, payoutUnderlyingMint),
      await ata(user, payoutUnderlyingMint),
      batchPayoutTokenAccount,
      userPayoutTokenAccount,
      await tokenValuePda(payoutConfidentialMint, batchPayoutTokenAccount, ENCRYPTED_BALANCE_LABEL),
      await tokenValuePda(payoutConfidentialMint, userPayoutTokenAccount, ENCRYPTED_BALANCE_LABEL),
      await tokenValuePda(payoutConfidentialMint, batchPayoutTokenAccount, utf8('transferred_amount______________')),
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      ZAMA_HOST_PROGRAM_ADDRESS,
      hostConfig,
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      address('11111111111111111111111111111111'),
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    // user carries no signer role (0x02/0x03 are the signer roles).
    expect(instruction.accounts![1]!.role & 0b10).toBe(0);

    const decoded = getClaimInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(CLAIM_DISCRIMINATOR));
  });

  // Golden pins for the fixed fixture: the value accounts are re-pinned from the RFC 035 seed
  // derivation (`encrypted_value_seeds`, mirrored and pinned in the SDK's encryptedValueAccount
  // test), the rest carried over unchanged, the event authorities from
  // `solana find-program-derived-address <program> string:__event_authority`.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildClaimInstruction({
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
    expect(addresses[12]).toBe('8iRxqzbzVoCDyN5ruCrtDs3HEJXL6S5khbmijMta8j6z'); // batchPayoutTokenAccount
    expect(addresses[14]).toBe('3i11PrkLtKRVttNh4XhLcrvVyZp4yfyZroUJeL1ijZrM'); // batchPayoutBalanceValue
    expect(addresses[17]).toBe('7usNGbH9WupMAsyDeqdUEoKrjisKcgusGjDiju4vNog'); // zamaEventAuthority
    expect(addresses[20]).toBe('2KQ5N8YEUTk8hQWXBnkGjsvKPzm2rh2nFH6PeoVt7q8U'); // tokenEventAuthority
  });
});
