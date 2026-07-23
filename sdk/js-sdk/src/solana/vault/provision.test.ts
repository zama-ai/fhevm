import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import { buildInitializeVaultInstruction } from './initializeVault.js';
import { buildInitializeBatcherInstruction, BatchDirection } from './initializeBatcher.js';
import { buildInitializeMintInstruction } from './initializeMint.js';
import { buildInitializeTokenAccountInstruction } from './initializeTokenAccount.js';
import { buildWrapUsdcInstruction } from './wrapUsdc.js';
import {
  INITIALIZE_VAULT_DISCRIMINATOR,
  getInitializeVaultInstructionDataDecoder,
} from './internal/generated/demoVault/instructions/initializeVault.js';
import { DEMO_VAULT_PROGRAM_ADDRESS } from './internal/generated/demoVault/programAddress.js';
import {
  INITIALIZE_BATCHER_DISCRIMINATOR,
  getInitializeBatcherInstructionDataDecoder,
} from './internal/generated/confidentialBatcher/instructions/initializeBatcher.js';
import { CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS } from './internal/generated/confidentialBatcher/programAddress.js';
import {
  INITIALIZE_MINT_DISCRIMINATOR,
  getInitializeMintInstructionDataDecoder,
} from '../internal/generated/confidentialToken/instructions/initializeMint.js';
import {
  INITIALIZE_TOKEN_ACCOUNT_DISCRIMINATOR,
  getInitializeTokenAccountInstructionDataDecoder,
} from '../internal/generated/confidentialToken/instructions/initializeTokenAccount.js';
import {
  WRAP_USDC_DISCRIMINATOR,
  getWrapUsdcInstructionDataDecoder,
} from '../internal/generated/confidentialToken/instructions/wrapUsdc.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}
function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

const HOST_CONFIG = addr(200);

describe('vault provisioning builders', () => {
  it('initialize_vault: correct program, discriminator, and the eight vault accounts', async () => {
    const payer = signer(addr(1));
    const instruction = await buildInitializeVaultInstruction({
      payer,
      vault: signer(addr(2)),
      underlyingMint: addr(3),
    });
    expect(instruction.programAddress).toBe(DEMO_VAULT_PROGRAM_ADDRESS);
    expect(instruction.accounts).toHaveLength(8);
    expect(instruction.accounts![0].address).toBe(payer.address);
    const decoded = getInitializeVaultInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_VAULT_DISCRIMINATOR));
  });

  it('initialize_batcher: encodes the min-age slots and the direction enum', async () => {
    const instruction = buildInitializeBatcherInstruction({
      payer: signer(addr(1)),
      batcher: signer(addr(2)),
      joinConfidentialMint: addr(3),
      payoutConfidentialMint: addr(4),
      vault: addr(5),
      minBatchAgeSlots: 25,
      direction: BatchDirection.Deposit,
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_BATCHER_PROGRAM_ADDRESS);
    const decoded = getInitializeBatcherInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_BATCHER_DISCRIMINATOR));
    expect(decoded.minBatchAgeSlots).toBe(25n);
    expect(decoded.direction).toBe(BatchDirection.Deposit);
  });

  it('initialize_mint: right program + discriminator (lineage/event PDAs derived internally)', async () => {
    const instruction = await buildInitializeMintInstruction({
      authority: signer(addr(1)),
      mint: signer(addr(2)),
      underlyingMint: addr(3),
      hostConfig: HOST_CONFIG,
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const decoded = getInitializeMintInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_MINT_DISCRIMINATOR));
  });

  it('initialize_token_account: right program + discriminator + initial balance', async () => {
    const instruction = await buildInitializeTokenAccountInstruction({
      owner: signer(addr(1)),
      mint: addr(2),
      hostConfig: HOST_CONFIG,
      initialBalance: 0,
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const decoded = getInitializeTokenAccountInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(INITIALIZE_TOKEN_ACCOUNT_DISCRIMINATOR));
    expect(decoded.initialBalance).toBe(0n);
  });

  it('wrap_usdc: public amount, no proof; encodes the u64 amount', async () => {
    const instruction = await buildWrapUsdcInstruction({
      owner: signer(addr(1)),
      mint: addr(2),
      underlyingMint: addr(3),
      hostConfig: HOST_CONFIG,
      amount: 1_000_000n,
    });
    expect(instruction.programAddress).toBe(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS);
    const decoded = getWrapUsdcInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(WRAP_USDC_DISCRIMINATOR));
    expect(decoded.amount).toBe(1_000_000n);
  });
});
