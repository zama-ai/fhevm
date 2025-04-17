import dotenv from 'dotenv';
import { ethers } from 'ethers';
import * as fs from 'fs';
import { ProviderWrapper } from 'hardhat/plugins';

class CustomProvider extends ProviderWrapper {
  public lastBlockSnapshot: number;
  public lastCounterRand: number;
  public lastBlockSnapshotForDecrypt: number;

  constructor(protected readonly _wrappedProvider: any) {
    super(_wrappedProvider);
    this.lastBlockSnapshot = 0; // Initialize the variable
    this.lastCounterRand = 0;
    this.lastBlockSnapshotForDecrypt = 0;
  }

  async request(args: { method: string; params?: any[] }) {
    if (args.method === 'eth_estimateGas') {
      const estimatedGasLimit = BigInt(await this._wrappedProvider.request(args));
      const increasedGasLimit = ethers.toBeHex((estimatedGasLimit * 120n) / 100n); // override estimated gasLimit by 120%, to avoid some edge case with ethermint gas estimation
      return increasedGasLimit;
    }
    if (args.method === 'evm_revert') {
      const result = await this._wrappedProvider.request(args);
      const blockNumberHex = await this._wrappedProvider.request({ method: 'eth_blockNumber' });
      this.lastBlockSnapshot = parseInt(blockNumberHex);
      this.lastBlockSnapshotForDecrypt = parseInt(blockNumberHex);
      const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('addresses/.env.exec'));
      const coprocAdd = parsedEnvCoprocessor.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
      this.lastCounterRand = await this._wrappedProvider.request({
        method: 'eth_getStorageAt',
        params: [coprocAdd, '0xa436a06f0efce5ea38c956a21e24202a59b3b746d48a23fb52b4a5bc33fe3e00', 'latest'],
      });
      return result;
    }
    if (args.method === 'get_lastBlockSnapshot') {
      return [this.lastBlockSnapshot, this.lastCounterRand];
    }
    if (args.method === 'get_lastBlockSnapshotForDecrypt') {
      return this.lastBlockSnapshotForDecrypt;
    }
    if (args.method === 'set_lastBlockSnapshot') {
      this.lastBlockSnapshot = args.params![0];
      return this.lastBlockSnapshot;
    }
    if (args.method === 'set_lastBlockSnapshotForDecrypt') {
      this.lastBlockSnapshotForDecrypt = args.params![0];
      return this.lastBlockSnapshotForDecrypt;
    }
    const result = this._wrappedProvider.request(args);
    return result;
  }
}

export default CustomProvider;
