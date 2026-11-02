import { type Provider, type TransactionRequest, type TransactionResponse, Wallet } from 'ethers';

import type { ChainGas } from '../../../config/profiles/types.js';
import { bufferedGasLimit } from './gas.js';

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
