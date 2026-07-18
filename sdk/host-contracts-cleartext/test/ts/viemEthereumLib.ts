import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  DeployParameters,
  DeployReturnType,
} from '@fhevm/host-contracts-cleartext/ts';
import { createPublicClient, createTestClient, createWalletClient, http, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { foundry } from 'viem/chains';
import { createViemEthereumUtils } from './ethUtils.ts';

export type ViemEthereumAdapters = {
  provider: AbstractEthereumProvider;
  signer: AbstractEthereumSigner;
  utils: AbstractEthereumUtils;
};

export function createViemEthereumAdapters(args: {
  readonly rpcUrl: string;
  readonly privateKey: `0x${string}`;
}): ViemEthereumAdapters {
  const account = privateKeyToAccount(args.privateKey);
  const transport = http(args.rpcUrl);
  const publicClient = createPublicClient({ chain: foundry, transport });
  const testClient = createTestClient({ chain: foundry, mode: 'anvil', transport });
  const walletClient = createWalletClient({ account, chain: foundry, transport });

  return {
    utils: createViemEthereumUtils(),

    provider: {
      async setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void> {
        await testClient.setCode({ address: parameters.address as Address, bytecode: parameters.bytecode as Hex });
      },

      async setStorageAt(parameters: {
        readonly address: string;
        readonly slot: string;
        readonly value: string;
      }): Promise<void> {
        await testClient.setStorageAt({
          address: parameters.address as Address,
          index: parameters.slot as Hex,
          value: parameters.value as Hex,
        });
      },

      async getCodeAt(parameters: { readonly address: string }): Promise<string> {
        return (await publicClient.getCode({ address: parameters.address as Address })) ?? '0x';
      },

      readContract(parameters: {
        readonly address: string;
        readonly abi: readonly unknown[];
        readonly functionName: string;
        readonly args?: readonly unknown[];
      }): Promise<unknown> {
        return publicClient.readContract(parameters as Parameters<typeof publicClient.readContract>[0]);
      },

      getTransactionCount(parameters: { readonly address: string }): Promise<number> {
        return publicClient.getTransactionCount({ address: parameters.address as Address });
      },
    },

    signer: {
      getAddress(): Promise<string> {
        return Promise.resolve(account.address);
      },

      async deploy(parameters: DeployParameters): Promise<DeployReturnType> {
        const hash = await walletClient.deployContract({
          abi: parameters.abi ?? [],
          bytecode: parameters.bytecode as Hex,
          args: parameters.args,
        });
        const receipt = await publicClient.waitForTransactionReceipt({ hash });
        if (receipt.contractAddress === null || receipt.contractAddress === undefined) {
          throw new Error('Contract deployment did not return a contract address');
        }

        return { contractAddress: receipt.contractAddress };
      },

      async writeContract(parameters: unknown): Promise<unknown> {
        // Await the receipt (like `deploy` above) so the tx is mined — and its effects observable —
        // by the time the call resolves. Without this, a caller that reads state right after (e.g.
        // `updateV13ToV14` then a `getCurrentKmsContextId`) races the block inclusion.
        const hash = await walletClient.writeContract(parameters as Parameters<typeof walletClient.writeContract>[0]);
        const receipt = await publicClient.waitForTransactionReceipt({ hash });
        if (receipt.status !== 'success') {
          throw new Error(`writeContract transaction reverted: ${hash}`);
        }

        return receipt;
      },
    },
  };
}
