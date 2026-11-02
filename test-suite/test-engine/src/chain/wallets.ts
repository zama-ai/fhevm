import { HDNodeWallet, type Provider, type TransactionRequest, type TransactionResponse, Wallet } from 'ethers';

import type { ChainGas } from '../profiles/types.js';
import { bufferedGasLimit } from './gas.js';

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

/**
 * Wallet of a named role. Every transaction it sends, contract deployments included, first runs
 * `beforeSend` (the chain ID check) and gets the profile's gas buffer when it has no `gasLimit`.
 */
export class EngineWallet extends Wallet {
  constructor(
    readonly role: string,
    privateKey: string,
    provider: Provider,
    private readonly gas: ChainGas,
    private readonly beforeSend: () => Promise<void>,
  ) {
    super(privateKey, provider);
  }

  /**
   * Not supported: ethers would return a plain `Wallet`, without the gas buffer and the chain ID
   * check. Use `chain.wallet(role)`; it is already connected to the profile's provider.
   */
  override connect(): Wallet {
    throw new Error(`wallet '${this.role}' cannot be reconnected: use chain.wallet('${this.role}')`);
  }

  override async sendTransaction(tx: TransactionRequest): Promise<TransactionResponse> {
    await this.beforeSend();
    if (tx.gasLimit == null) {
      tx = { ...tx, gasLimit: bufferedGasLimit(await this.estimateGas(tx), this.gas) };
    }
    return super.sendTransaction(tx);
  }
}
