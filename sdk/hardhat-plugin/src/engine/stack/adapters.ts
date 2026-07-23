import { Interface, type InterfaceAbi } from "ethers";

import { FhevmNode } from "../node";
import { ACL_OWNER } from "./config";

/**
 * The three adapters `@fhevm/host-contracts-cleartext` deploys through, implemented over a dev node's
 * EIP-1193 provider.
 *
 * This is all the plugin owes the package. Placing the contracts, patching their cross-references, seeding
 * the proxy state and running the initializers are the PACKAGE's job — see `deployAt`. The plugin only
 * says how to talk to this particular node.
 */

/** Structurally matches the package's `AbstractEthereumProvider` (dev-node cheat codes). */
export interface EthProvider {
  setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void>;
  setStorageAt(parameters: { readonly address: string; readonly slot: string; readonly value: string }): Promise<void>;
  getCodeAt(parameters: { readonly address: string }): Promise<string>;
}

/** Structurally matches the package's `AbstractEthereumSigner`. */
export interface EthSigner {
  getAddress(): Promise<string>;
  deploy(parameters: { readonly bytecode: string }): Promise<{ contractAddress: string }>;
  writeContract(parameters: unknown): Promise<unknown>;
}

interface WriteContractCall {
  readonly address: string;
  readonly abi: InterfaceAbi;
  readonly functionName: string;
  readonly args?: readonly unknown[];
}

export function createEthProvider(node: FhevmNode): EthProvider {
  return {
    setCodeAt: ({ address, bytecode }) => node.setCode(address, bytecode),
    setStorageAt: ({ address, slot, value }) => node.setStorageAt(address, slot, value),
    getCodeAt: ({ address }) => node.getCode(address),
  };
}

/**
 * The admin signer: ACL's owner, and the sender of every initializer.
 *
 * It is an IMPERSONATED account rather than one of the user's, so the mock never consumes a nonce the test
 * might be counting on. The node signs for it, so no key is involved.
 */
export async function createAdminSigner(node: FhevmNode): Promise<EthSigner> {
  await node.setBalance(ACL_OWNER, 10n ** 20n);
  await node.impersonate(ACL_OWNER);

  return {
    getAddress: () => Promise.resolve(ACL_OWNER),

    deploy: () => {
      // `deployAt` never deploys — it places code. Only the package's CREATE-based `deploy` needs this, and
      // that path cannot reach the pinned addresses, so the plugin never takes it.
      throw new Error("The cleartext mock places code with setCodeAt; it does not deploy via CREATE.");
    },

    writeContract: async (parameters: unknown): Promise<unknown> => {
      const call = parameters as WriteContractCall;
      const data = new Interface(call.abi).encodeFunctionData(call.functionName, [...(call.args ?? [])]);
      await node.sendTransaction({ from: ACL_OWNER, to: call.address, data });
      return undefined;
    },
  };
}
