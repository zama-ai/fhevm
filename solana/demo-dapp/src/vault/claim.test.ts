import { SYSTEM_PROGRAM_ADDRESS } from '@solana-program/system';
import { prepareTransientStore } from '@fhevm/sdk/solana';
import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildClaimInstruction } from './claim.js';
import {
  CLAIM_DISCRIMINATOR,
  getClaimInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/claim.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

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
  const SPL_TOKEN = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
  const ASSOCIATED_TOKEN = address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
  const ata = (owner: Address, mint: Address): Promise<Address> =>
    pda(ASSOCIATED_TOKEN, [base58.decode(owner), base58.decode(SPL_TOKEN), base58.decode(mint)]);

  it('derives every non-root account exactly as claim.rs validates them', async () => {
    const transientStore = await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS });
    const instruction = await buildClaimInstruction({
      transientStore: transientStore,
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
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
    const record = await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [
      utf8('join-record'),
      base58.decode(batch),
      base58.decode(user),
    ]);
    const state = await batcherValuePda(batch, record);
    const expectedTransientStore = await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('transient'), base58.decode(payer.address)]);
    const expected: Address[] = [
      payer.address,
      user,
      batcher,
      batch,
      batchAuthority,
      await pda(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, [utf8('join-record'), base58.decode(batch), base58.decode(user)]),
      state,
      expectedTransientStore,
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
      await pda(ZAMA_HOST_PROGRAM_ADDRESS, [utf8('host-config')]),
      await pda(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, [utf8('__event_authority')]),
      CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
      SYSTEM_PROGRAM_ADDRESS,
      // The optional HCU accounts, absent: Anchor reads the program id as None.
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
      CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS,
    ];
    expect(instruction.accounts!.map((a) => a.address)).toEqual(expected);

    // user carries no signer role (0x02/0x03 are the signer roles).
    expect(instruction.accounts![1]!.role & 0b10).toBe(0);

    const decoded = getClaimInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(CLAIM_DISCRIMINATOR));
  });

  // Golden pins for the fixed fixture: the encrypted stores are re-pinned from the RFC 035 seed
  // derivation (`encrypted_store_seeds`, mirrored and pinned in the SDK's encryptedStore
  // test), the rest carried over unchanged, the event authorities from
  // `solana find-program-derived-address <program> string:__event_authority`.
  it('matches the golden derived addresses for the fixed fixture', async () => {
    const instruction = await buildClaimInstruction({
      transientStore: await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS }),
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
    });
    const addresses = instruction.accounts!.map((a) => a.address);
    expect(addresses[13]).toBe('4MxNx3UFs82BQ349hySkRZ4YTLuuT77jTpXc1ohbXYnA'); // batchPayoutTokenAccount
    expect(addresses[15]).toBe('4pn8uFyj9EnVWa8g8YGQedU4sCLBCNEQnhcJBZRnkwtw'); // batchPayoutBalanceStore
    expect(addresses[17]).toBe('CAspHyipvqeHA78sMa73uD2ThP84Zw4ywXG71dyrNpXp'); // zamaEventAuthority
    expect(addresses[20]).toBe('FmW1wCB2eZQFwLVuALBH2Y3yh9uscwcExz1i4zFGYZgp'); // tokenEventAuthority
  });

  it('appends, under the deny list, the batch then the payout mint deny record', async () => {
    const input = {
      transientStore: await prepareTransientStore({ payer, host: ZAMA_HOST_PROGRAM_ADDRESS }),
      payer,
      user,
      batcher,
      batch,
      payoutConfidentialMint,
      payoutUnderlyingMint,
      tokenProgram: SPL_TOKEN,
    };
    const plain = await buildClaimInstruction(input);
    const instruction = await buildClaimInstruction({ ...input, denyListEnabled: true });
    const [batchRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS, scope: batch });
    const [payoutMintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: payoutConfidentialMint });
    // The MulDiv runs as the batch; the payout transfer runs as the payout mint.
    expect(instruction.accounts!.slice(plain.accounts!.length)).toEqual([
      { address: batchRecord, role: 0 },
      { address: payoutMintRecord, role: 0 },
    ]);
  });
});
