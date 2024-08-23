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
      const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('lib/.env.exec'));
      const coprocAdd = parsedEnvCoprocessor.TFHE_EXECUTOR_CONTRACT_ADDRESS;
      const callData = {
        to: coprocAdd,
        data: '0x1f20d85c',
      };
      this.lastCounterRand = await this._wrappedProvider.request({
        method: 'eth_call',
        params: [callData, 'latest'],
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
