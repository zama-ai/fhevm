import { ethers } from 'ethers';
import { ProviderWrapper } from 'hardhat/plugins';

class CustomProvider extends ProviderWrapper {
  constructor(protected readonly _wrappedProvider: any) {
    super(_wrappedProvider);
  }

  public async request(args: { method: string; params?: any[] }): Promise<any> {
    if (args.method === 'eth_estimateGas') {
      const estimatedGasLimit = BigInt(await this._wrappedProvider.request(args));
      const increasedGasLimit = ethers.toBeHex((estimatedGasLimit * 120n) / 100n); // override estimated gasLimit by 120%, to avoid some edge case with ethermint gas estimation
      return increasedGasLimit;
    }
    return this._wrappedProvider.request(args);
  }
}

export default CustomProvider;
