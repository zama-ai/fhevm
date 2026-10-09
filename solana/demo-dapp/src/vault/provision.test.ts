import { prepareTransientStore } from '@fhevm/sdk/solana';
import { findDenyScopeRecordPda, ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { describe, expect, it } from 'vitest';
import { AccountRole, address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildInitializeMintInstruction } from './initializeMint.js';
import { buildInitializeTokenAccountInstruction } from './initializeTokenAccount.js';
import { buildWrapUsdcInstruction } from './wrapUsdc.js';
import { testHostPolicy } from './testHostPolicy.js';
import { INITIALIZE_MINT_DISCRIMINATOR, getInitializeMintInstructionDataDecoder, INITIALIZE_TOKEN_ACCOUNT_DISCRIMINATOR, getInitializeTokenAccountInstructionDataDecoder, WRAP_USDC_DISCRIMINATOR, getWrapUsdcInstructionDataDecoder, CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '@fhevm/confidential-token';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('vault provisioning builders', () => {
  it('initialize_mint: right program + discriminator (encrypted store/event PDAs derived internally)', async () => {
    const instruction = await buildInitializeMintInstruction({
      transientStore: await prepareTransientStore({ payer: signer(addr(1)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
      authority: signer(addr(1)),
      mint: signer(addr(2)),
      underlyingMint: addr(3),
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const decoded = getInitializeMintInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_MINT_DISCRIMINATOR));
  });

  it('initialize_token_account: right program + discriminator (always zero balance)', async () => {
    const payer = signer(addr(1));
    const owner = addr(2);
    const instruction = await buildInitializeTokenAccountInstruction({
      transientStore: await prepareTransientStore({ payer: signer(addr(1)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
      payer,
      owner,
      mint: addr(3),
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    expect(instruction.accounts?.[0]?.address).toBe(payer.address);
    expect(instruction.accounts?.[1]?.address).toBe(owner);
    const decoded = getInitializeTokenAccountInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_TOKEN_ACCOUNT_DISCRIMINATOR));
  });

  it('wrap_usdc: public amount, no proof; encodes the u64 amount', async () => {
    const instruction = await buildWrapUsdcInstruction({
      transientStore: await prepareTransientStore({ payer: signer(addr(1)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
      owner: signer(addr(1)),
      mint: addr(2),
      underlyingMint: addr(3),
      tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
      amount: 1_000_000n,
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const decoded = getWrapUsdcInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(WRAP_USDC_DISCRIMINATOR));
    expect(decoded.amount).toBe(1_000_000n);
  });

  it.each([
    [
      'initialize_token_account',
      async (host?: ReturnType<typeof testHostPolicy>) =>
        buildInitializeTokenAccountInstruction({
          transientStore: await prepareTransientStore({ payer: signer(addr(1)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
          payer: signer(addr(1)),
          owner: addr(2),
          mint: addr(3),
          host,
        }),
    ],
    [
      'wrap_usdc',
      async (host?: ReturnType<typeof testHostPolicy>) =>
        buildWrapUsdcInstruction({
          transientStore: await prepareTransientStore({ payer: signer(addr(1)), host: ZAMA_HOST_PROGRAM_ADDRESS }),
          owner: signer(addr(1)),
          mint: addr(3),
          underlyingMint: addr(4),
          tokenProgram: address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
          amount: 1n,
          host,
        }),
    ],
  ])('%s: carries the mint\'s HCU accounts and, under the deny list, its deny record', async (_, build) => {
    const plain = (await build()).accounts!;
    const accounts = (await build(testHostPolicy(true, { [addr(3)]: { hcuBlockMeter: addr(40), hcuTrustedAppRecord: addr(41) } })))
      .accounts!;
    const [mintRecord] = await findDenyScopeRecordPda({ appProgram: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, scope: addr(3) });

    // Absent optional accounts hold the program id; the host policy fills the two HCU slots.
    const filled = accounts.slice(0, plain.length).filter((account, index) => account.address !== plain[index]!.address);
    expect(filled).toEqual([
      { address: addr(40), role: AccountRole.WRITABLE },
      { address: addr(41), role: AccountRole.READONLY },
    ]);
    expect(accounts.slice(plain.length)).toEqual([{ address: mintRecord, role: AccountRole.READONLY }]);
  });
});
