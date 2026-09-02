import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import type { NetworkConnection } from 'hardhat/types/network';

export const ACCOUNT_NAMES = ['alice', 'bob', 'carol', 'dave', 'eve', 'fred', 'greg', 'hugo', 'ian', 'jane'] as const;

type AccountNames = (typeof ACCOUNT_NAMES)[number];

export type Signers = Record<AccountNames, HardhatEthersSigner>;

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
