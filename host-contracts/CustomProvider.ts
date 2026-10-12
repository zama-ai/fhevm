import dotenv from 'dotenv';
import { ethers } from 'ethers';
import * as fs from 'fs';
import { ProviderWrapper } from 'hardhat/plugins';
import type { EIP1193Provider, RequestArguments } from 'hardhat/types';

// Reads the block number passed as the first param of the custom set_lastBlockSnapshot* methods
function _getBlockNumberParam(args: RequestArguments): number {
  const blockNumber = Array.isArray(args.params) ? args.params[0] : undefined;
  if (typeof blockNumber !== 'number') {
    throw new Error(`${args.method} expects a block number as its first param, got ${String(blockNumber)}`);
  }
  return blockNumber;
}

class CustomProvider extends ProviderWrapper {
  public lastBlockSnapshot: number;
  public lastCounterRand: number;
  public lastBlockSnapshotForDecrypt: number;

  constructor(protected readonly _wrappedProvider: EIP1193Provider) {
    super(_wrappedProvider);
    this.lastBlockSnapshot = 0; // Initialize the variable
    this.lastCounterRand = 0;
    this.lastBlockSnapshotForDecrypt = 0;
  }

  async request(args: RequestArguments) {
    if (args.method === 'eth_estimateGas') {
      const estimatedGasLimitHex = await this._wrappedProvider.request(args);
      if (typeof estimatedGasLimitHex !== 'string') {
        throw new Error(`eth_estimateGas returned a non-string result: ${String(estimatedGasLimitHex)}`);
      }
      const estimatedGasLimit = BigInt(estimatedGasLimitHex);
      const increasedGasLimit = ethers.toBeHex((estimatedGasLimit * 120n) / 100n); // override estimated gasLimit by 120%, to avoid some edge case with ethermint gas estimation
      return increasedGasLimit;
    }

    if (args.method === 'evm_revert') {
      const result = await this._wrappedProvider.request(args);
      const blockNumberHex = await this._wrappedProvider.request({ method: 'eth_blockNumber' });
      if (typeof blockNumberHex !== 'string') {
        throw new Error(`eth_blockNumber returned a non-string result: ${String(blockNumberHex)}`);
      }
      this.lastBlockSnapshot = parseInt(blockNumberHex);
      this.lastBlockSnapshotForDecrypt = parseInt(blockNumberHex);
      const parsedEnvCoprocessor = dotenv.parse(fs.readFileSync('addresses/.env.host'));
      const coprocAdd = parsedEnvCoprocessor.FHEVM_EXECUTOR_CONTRACT_ADDRESS;

      const storedNum = await this._wrappedProvider.request({
        method: 'eth_getStorageAt',
        params: [coprocAdd, '0xa436a06f0efce5ea38c956a21e24202a59b3b746d48a23fb52b4a5bc33fe3e00', 'latest'],
      });
      if (typeof storedNum !== 'string') {
        throw new Error(`eth_getStorageAt returned a non-string result: ${String(storedNum)}`);
      }
      this.lastCounterRand = Number(BigInt(storedNum));
      return result;
    }

    if (args.method === 'get_lastBlockSnapshot') {
      return [this.lastBlockSnapshot, this.lastCounterRand];
    }

    if (args.method === 'get_lastBlockSnapshotForDecrypt') {
      return this.lastBlockSnapshotForDecrypt;
    }

    if (args.method === 'set_lastBlockSnapshot') {
      this.lastBlockSnapshot = _getBlockNumberParam(args);
      return this.lastBlockSnapshot;
    }

    if (args.method === 'set_lastBlockSnapshotForDecrypt') {
      this.lastBlockSnapshotForDecrypt = _getBlockNumberParam(args);
      return this.lastBlockSnapshotForDecrypt;
    }

    const result = this._wrappedProvider.request(args);

    return result;
  }
}

export default CustomProvider;
