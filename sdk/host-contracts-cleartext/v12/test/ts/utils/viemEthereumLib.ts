// viem adapter for `@fhevm/host-contracts-cleartext/ts`.
//
// ## Why there is no nonce management here
//
// `AbstractEthereumSigner` requires that every send occupy the signer's next nonce, with no gaps and
// no reuse — every host address is CREATE(deployer, startNonce + k). The ethers adapter next door
// keeps its own counter to guarantee that. This one does not, and does not need to:
//
//   - viem's `prepareTransactionRequest` fetches `eth_getTransactionCount` with `blockTag: 'pending'`
//     on every send, when the account carries no `nonceManager`.
//   - that request opts OUT of dedupe for block tags — `getTransactionCount` passes
//     `dedupe: typeof blockNumber === 'bigint' || blockHash !== undefined`, which is false for a tag.
//     So the value is read from the node each time, never served from a cache.
//   - `pending` counts queued transactions as well as mined ones, so the count is right even before
//     inclusion.
//
// ethers behaves differently: `AbstractProvider` caches that same response for `cacheTimeout` (250 ms
// of wall clock, which mining does not invalidate), which is why an ethers adapter must supply nonces
// itself. Verified against viem 2.55.19; it is current behaviour rather than a guarantee, so if these
// tests ever fail with `nonce has already been used`, port the counter from ethersEthereumLib.ts.
//
// Both adapters do await inclusion, which the interface requires separately — see the receipt waits
// below.
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
        // `updateV12ToV13` then a `getCurrentKmsContextId`) races the block inclusion.
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
