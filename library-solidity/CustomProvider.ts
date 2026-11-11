import dotenv from 'dotenv';
import { ethers } from 'ethers';
import * as fs from 'fs';
import { ProviderWrapper } from 'hardhat/plugins';
import type { EIP1193Provider, RequestArguments } from 'hardhat/types';

// Returns the first RPC parameter of `args`, throwing if it is not a number.
function getFirstNumberParam(args: RequestArguments): number {
  const first: unknown = Array.isArray(args.params) ? args.params[0] : undefined;
  if (typeof first !== 'number') {
    throw new Error(`${args.method}: expected a number as first parameter, got ${String(first)}`);
  }
  return first;
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

  // override public abstract request(args: RequestArguments): Promise<unknown>;
  async request(args: RequestArguments): Promise<unknown> {
    // eth_estimateGas
    if (args.method === 'eth_estimateGas') {
      const estimatedGas: unknown = await this._wrappedProvider.request(args);
      if (typeof estimatedGas !== 'string' || !ethers.isHexString(estimatedGas)) {
        throw new Error(`eth_estimateGas: expected a hex quantity, got ${String(estimatedGas)}`);
      }
      const estimatedGasLimit = BigInt(estimatedGas);
      const increasedGasLimit = ethers.toBeHex((estimatedGasLimit * 120n) / 100n); // override estimated gasLimit by 120%, to avoid some edge case with ethermint gas estimation
      return increasedGasLimit;
    }

    // evm_revert
    if (args.method === 'evm_revert') {
      const result = await this._wrappedProvider.request(args);
      const blockNumberHex: unknown = await this._wrappedProvider.request({ method: 'eth_blockNumber' });
      if (typeof blockNumberHex !== 'string' || !ethers.isHexString(blockNumberHex)) {
        throw new Error(`eth_blockNumber: expected a hex quantity, got ${String(blockNumberHex)}`);
      }

      this.lastBlockSnapshot = parseInt(blockNumberHex);
      this.lastBlockSnapshotForDecrypt = parseInt(blockNumberHex);

      const coprocAdd = dotenv.parse(
        fs.readFileSync(require.resolve('./fhevmTemp/addresses/.env.host')),
      ).FHEVM_EXECUTOR_CONTRACT_ADDRESS;

      const counterRandSlot: unknown = await this._wrappedProvider.request({
        method: 'eth_getStorageAt',
        params: [coprocAdd, '0xa436a06f0efce5ea38c956a21e24202a59b3b746d48a23fb52b4a5bc33fe3e00', 'latest'],
      });
      if (typeof counterRandSlot !== 'string' || !ethers.isHexString(counterRandSlot)) {
        throw new Error(`eth_getStorageAt: expected a hex value, got ${String(counterRandSlot)}`);
      }

      // The storage slot is a 32-byte hex value; toNumber throws if it does not fit in a safe integer.
      this.lastCounterRand = ethers.toNumber(counterRandSlot);

      return result;
    }

    // get_lastBlockSnapshot
    if (args.method === 'get_lastBlockSnapshot') {
      return [this.lastBlockSnapshot, this.lastCounterRand];
    }

    // get_lastBlockSnapshotForDecrypt
    if (args.method === 'get_lastBlockSnapshotForDecrypt') {
      return this.lastBlockSnapshotForDecrypt;
    }

    // set_lastBlockSnapshot
    if (args.method === 'set_lastBlockSnapshot') {
      this.lastBlockSnapshot = getFirstNumberParam(args);
      return this.lastBlockSnapshot;
    }

    // set_lastBlockSnapshotForDecrypt
    if (args.method === 'set_lastBlockSnapshotForDecrypt') {
      this.lastBlockSnapshotForDecrypt = getFirstNumberParam(args);
      return this.lastBlockSnapshotForDecrypt;
    }

    const result = this._wrappedProvider.request(args);

    return result;
  }
}

export default CustomProvider;
