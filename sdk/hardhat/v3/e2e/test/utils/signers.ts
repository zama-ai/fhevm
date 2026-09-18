import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import type { NetworkConnection } from 'hardhat/types/network';
import { type HDAccount, mnemonicToAccount } from 'viem/accounts';

import { HD_PATH, MNEMONIC } from '../../hardhat.config.ts';

export const ACCOUNT_NAMES = ['alice', 'bob', 'carol', 'dave', 'eve', 'fred', 'greg', 'hugo', 'ian', 'jane'] as const;

type AccountNames = (typeof ACCOUNT_NAMES)[number];

export type Signers = Record<AccountNames, HardhatEthersSigner>;
export type Accounts = Record<AccountNames, HDAccount>;

// Named signers of ONE connection: hardhat 3 scopes accounts to the connection, so there is no
// process-wide cache to fill the way the v2 helper did.
export async function getSigners(connection: NetworkConnection): Promise<Signers> {
  const ethSigners = await connection.ethers.getSigners();
  const entries = ACCOUNT_NAMES.map((name, index) => {
    const signer = ethSigners[index];
    if (signer === undefined)
      throw new Error(`Network provides ${ethSigners.length} signers; '${name}' needs #${index}`);
    return [name, signer] as const;
  });
  return Object.fromEntries(entries) as Signers;
}

// The same ten accounts as viem local accounts: the plugin speaks viem, so a decrypting USER is a viem
// account (or wallet client), while transactions keep going through the ethers signers above.
export function getAccounts(): Accounts {
  const entries = ACCOUNT_NAMES.map(
    (name, index) => [name, mnemonicToAccount(MNEMONIC, { path: `${HD_PATH}${index}` })] as const,
  );
  return Object.fromEntries(entries) as Accounts;
}

// The viem account behind an ethers signer: fixtures send transactions with the signer and decrypt as
// the same user, and the plugin wants the user as a viem account.
export function accountFor(signer: { readonly address: string }): HDAccount {
  const account = Object.values(getAccounts()).find((a) => a.address.toLowerCase() === signer.address.toLowerCase());
  if (account === undefined) throw new Error(`No suite account for ${signer.address}`);
  return account;
}
