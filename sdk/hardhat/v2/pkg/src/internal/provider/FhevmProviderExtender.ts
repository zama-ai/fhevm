import { ethers as EthersT } from 'ethers';
import { ProviderError } from 'hardhat/internal/core/providers/errors';
import { ProviderWrapper } from 'hardhat/plugins';
import type { EIP1193Provider, HardhatConfig, RequestArguments } from 'hardhat/types';

import { fhevmContext } from '../EnvironmentExtender';
import { assertHHFhevm } from '../error';
import { mutateErrorInPlace, mutateProviderErrorInPlace } from '../errors/FhevmContractError';

/**
 * Wraps the Hardhat provider.
 *
 * This used to double as the mock FHEVM relayer: it intercepted `fhevm_relayer_metadata`,
 * `fhevm_relayer_v1_*`, `fhevm_get_clear_text` and `evm_revert`, and served them from a JavaScript
 * coprocessor holding cleartexts in a `Map`. All of that is gone — the cleartext stack keeps its
 * values on-chain in `CleartextDB`, and `@fhevm/sdk` in cleartext mode reads them directly. There is
 * no relayer to fake and no JS-side state to keep in sync with `evm_revert`.
 *
 * What remains are the two behaviours that were never about mocking FHE:
 *  - inflating `eth_estimateGas`, to work around under-estimation on some nodes
 *  - decorating `eth_sendTransaction` failures with a decoded FHEVM error
 */
export class FhevmProviderExtender extends ProviderWrapper {
  protected readonly _config: HardhatConfig;
  protected readonly _networkName: string;

  // override estimated gasLimit by 120%, to avoid some edge case with ethermint gas estimation
  private static readonly ESTIMATEGAS_PERCENTAGE: bigint = 120n;

  constructor(_wrappedProvider: EIP1193Provider, _config: HardhatConfig, _network: string) {
    super(_wrappedProvider);
    this._config = _config;
    this._networkName = _network;
  }

  public async request(args: RequestArguments): Promise<unknown> {
    switch (args.method) {
      case 'eth_estimateGas':
        return this._handleEthEstimateGas(args);
      case 'eth_sendTransaction':
        return this._handleEthSendTransaction(args);
      default:
        return this._wrappedProvider.request(args);
    }
  }

  // Try to show a human readable error message
  // Forward to ethers.Signer or window.ethereum
  private async _handleEthSendTransaction(args: RequestArguments): Promise<unknown> {
    // Do not perform any action if we are running in HH node.
    // We could, but we prefer to keep it centralized.
    if (this._networkName === 'hardhat') {
      const fhevmEnv = fhevmContext.get();
      if (fhevmEnv.isRunningInHHNode) {
        return this._wrappedProvider.request(args);
      }
    }

    try {
      return await this._wrappedProvider.request(args);
    } catch (e) {
      if (e instanceof ProviderError || ProviderError.isProviderError(e)) {
        // Debug
        assertHHFhevm(this._networkName !== 'hardhat');
        await mutateProviderErrorInPlace(fhevmContext.get(), e);
      } else if (e instanceof Error) {
        // Debug
        assertHHFhevm(this._networkName === 'hardhat');
        await mutateErrorInPlace(fhevmContext.get(), e, args);
      }

      throw e;
    }
  }

  private async _handleEthEstimateGas(args: RequestArguments): Promise<unknown> {
    try {
      // Call the chained provider first.
      const estimatedGasLimit = (await this._wrappedProvider.request(args)) as bigint;

      // increase by an estimated percentage.
      const increasedGasLimit = EthersT.toBeHex(
        (estimatedGasLimit * FhevmProviderExtender.ESTIMATEGAS_PERCENTAGE) / 100n,
      );

      return increasedGasLimit;
    } catch (e) {
      const fhevmEnv = fhevmContext.get();

      let tx: { from: string; to: string } | undefined = undefined;
      if (args.params !== undefined && Array.isArray(args.params) && args.params.length > 0) {
        const p: unknown = args.params[0];
        if (
          typeof p === 'object' &&
          p !== null &&
          'from' in p &&
          'to' in p &&
          typeof p.from === 'string' &&
          typeof p.to === 'string'
        ) {
          tx = { from: p.from, to: p.to };
        }
      }

      // This is happening when using Metamask + Hardhat node
      // TODO: should display a human readable error
      await mutateProviderErrorInPlace(fhevmEnv, e as ProviderError, tx);
      throw e;
    }
  }
}
