import { HDNodeWallet } from 'ethers';

/** Derivation path of an account: the profile's `hdPath` followed by the account index. */
export const accountPath = (hdPath: string, index: number): string => `${hdPath}/${index}`;

/** Address of the account at `index`, derived like Hardhat derives the e2e signers. */
export function deriveAddress(mnemonic: string, hdPath: string, index: number): string {
  return HDNodeWallet.fromPhrase(mnemonic, undefined, accountPath(hdPath, index)).address;
}

/**
 * The account of every wallet role. The mnemonic seed (a costly PBKDF2) is derived once, then each
 * account is a child of `hdPath`.
 */
export function deriveAccounts(
  mnemonic: string,
  hdPath: string,
  wallets: Record<string, number>,
): Record<string, HDNodeWallet> {
  const root = HDNodeWallet.fromPhrase(mnemonic, undefined, hdPath);
  return Object.fromEntries(Object.entries(wallets).map(([role, index]) => [role, root.deriveChild(index)]));
}
