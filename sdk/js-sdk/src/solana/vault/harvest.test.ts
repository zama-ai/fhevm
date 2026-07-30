import { describe, expect, it, vi } from 'vitest';

const fetchEncodedAccount = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  fetchEncodedAccount: (...args: unknown[]) => fetchEncodedAccount(...args),
}));

import {
  address,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type TransactionSigner,
} from '@solana/kit';
import { base58 } from '@scure/base';

import { buildHarvestInstruction, getVaultMetrics } from './harvest.js';
import { getVaultEncoder } from './internal/generated/demoVault/accounts/vault.js';
import {
  getHarvestInstructionDataDecoder,
  HARVEST_DISCRIMINATOR,
} from './internal/generated/demoVault/instructions/harvest.js';
import { DEMO_VAULT_PROGRAM_ADDRESS } from './internal/generated/demoVault/programAddress.js';

const SPL_TOKEN_PROGRAM_ADDRESS = address('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = address('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function signer(value: Address): TransactionSigner {
  return { address: value, signTransactions: async () => [] } as unknown as TransactionSigner;
}

const encodedAddress = (value: Address): Uint8Array => new Uint8Array(getAddressEncoder().encode(value));

describe('Solana demo vault harvest', () => {
  const vault = addr(1);
  const donor = signer(addr(2));
  const underlyingMint = addr(3);
  const shareMint = addr(4);
  const vaultTokenAccount = addr(5);

  const setVaultAccount = () => {
    fetchEncodedAccount.mockResolvedValue({
      exists: true,
      address: vault,
      data: getVaultEncoder().encode({
        underlyingMint,
        shareMint,
        vaultTokenAccount,
        authorityBump: 250,
      }),
    });
  };

  it('builds the permissionless donation from semantic roots', async () => {
    setVaultAccount();

    const instruction = await buildHarvestInstruction({} as never, {
      donor,
      vault,
      amount: 25_000_000n,
    });
    const donorUnderlying = (
      await getProgramDerivedAddress({
        programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
        seeds: [
          encodedAddress(donor.address),
          encodedAddress(SPL_TOKEN_PROGRAM_ADDRESS),
          encodedAddress(underlyingMint),
        ],
      })
    )[0];

    expect(instruction.programAddress).toBe(DEMO_VAULT_PROGRAM_ADDRESS);
    expect(instruction.accounts?.map((account) => account.address)).toEqual([
      donor.address,
      vault,
      underlyingMint,
      donorUnderlying,
      vaultTokenAccount,
      SPL_TOKEN_PROGRAM_ADDRESS,
    ]);
    const decoded = getHarvestInstructionDataDecoder().decode(instruction.data!);
    expect(Array.from(decoded.discriminator)).toEqual(Array.from(HARVEST_DISCRIMINATOR));
    expect(decoded.amount).toBe(25_000_000n);
  });

  it('reads assets and share supply from the vault account', async () => {
    setVaultAccount();
    const getTokenAccountBalance = vi.fn(() => ({
      send: async () => ({ value: { amount: '125000000' } }),
    }));
    const getTokenSupply = vi.fn(() => ({
      send: async () => ({ value: { amount: '100000000' } }),
    }));

    const metrics = await getVaultMetrics({ getTokenAccountBalance, getTokenSupply } as never, vault, {
      commitment: 'confirmed',
    });

    expect(metrics).toEqual({
      underlyingMint,
      shareMint,
      vaultTokenAccount,
      totalAssets: 125_000_000n,
      totalShares: 100_000_000n,
    });
    expect(getTokenAccountBalance).toHaveBeenCalledWith(vaultTokenAccount, { commitment: 'confirmed' });
    expect(getTokenSupply).toHaveBeenCalledWith(shareMint, { commitment: 'confirmed' });
  });
});
