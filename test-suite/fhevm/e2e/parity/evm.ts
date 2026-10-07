// evm — the stack's EVM host chain for the parity cases: viem clients over the stack's mnemonic
// accounts, an `@fhevm/sdk` client, and contract deploys from the test-suite/e2e hardhat artifacts.

import path from "node:path";

import { defineFhevmChain } from "@fhevm/sdk/chains";
import { createFhevmClient, setFhevmRuntimeConfig } from "@fhevm/sdk/viem";
import {
  createPublicClient,
  createWalletClient,
  defineChain,
  http,
  type Abi,
  type Address,
  type Chain,
  type Hex,
  type HttpTransport,
  type WalletClient,
} from "viem";
import { mnemonicToAccount, type HDAccount } from "viem/accounts";

import { envPath, relayerAuth, REPO_ROOT } from "../../src/layout";
import { readEnvFile, withHostReachableFetch } from "../../src/utils/fs";
import type { TestEnv } from "../harness";

/** `npx hardhat compile` in test-suite/e2e writes these. */
const ARTIFACTS_DIR = path.join(REPO_ROOT, "test-suite/e2e/artifacts/contracts");

export type EvmHost = Awaited<ReturnType<typeof openEvmHost>>;
export type EvmAccount = WalletClient<HttpTransport, Chain, HDAccount>;

export const openEvmHost = async (env: TestEnv) => {
  // The env the stack generated for its hardhat suite: the same mnemonic, addresses and chain ids.
  // Its URLs name Docker hosts, so the endpoints come from `env`.
  const stack = await readEnvFile(envPath("test-suite"));
  const required = (name: string): string => {
    const value = stack[name];
    if (!value) throw new Error(`missing ${name} in the stack's test-suite env`);
    return value;
  };
  const chain = defineChain({
    id: Number(required("CHAIN_ID_HOST")),
    name: "fhevm host",
    nativeCurrency: { name: "Ether", symbol: "ETH", decimals: 18 },
    rpcUrls: { default: { http: [env.hostRpcUrl] } },
  });
  const publicClient = createPublicClient({ chain, transport: http() });
  setFhevmRuntimeConfig({ auth: relayerAuth() });
  const fhevm = createFhevmClient({
    // The SDK sources resolve their own viem copy, a different patch release with the same client.
    publicClient: publicClient as unknown as Parameters<typeof createFhevmClient>[0]["publicClient"],
    chain: defineFhevmChain({
      id: chain.id,
      fhevm: {
        contracts: {
          acl: { address: required("ACL_CONTRACT_ADDRESS") as Address },
          inputVerifier: { address: required("INPUT_VERIFIER_CONTRACT_ADDRESS") as Address },
          kmsVerifier: { address: required("KMS_VERIFIER_CONTRACT_ADDRESS") as Address },
          protocolConfig: { address: required("PROTOCOL_CONFIG_CONTRACT_ADDRESS") as Address },
        },
        relayerUrl: env.relayerUrl,
        gateway: {
          id: Number(required("CHAIN_ID_GATEWAY")),
          contracts: {
            decryption: { address: required("DECRYPTION_ADDRESS") as Address },
            inputVerification: { address: required("INPUT_VERIFICATION_ADDRESS") as Address },
          },
        },
      },
    }),
  });
  // The relayer's key-material URLs name the Docker storage host.
  await withHostReachableFetch(() => fhevm.ready);

  /** The stack's mnemonic account at `index`, as a wallet client. */
  const account = (index: number): EvmAccount =>
    createWalletClient({ account: mnemonicToAccount(required("MNEMONIC"), { addressIndex: index }), chain, transport: http() });

  const confirm = async (hash: Hex) => {
    const receipt = await publicClient.waitForTransactionReceipt({ hash });
    if (receipt.status !== "success") throw new Error(`transaction ${hash} reverted`);
    return receipt;
  };

  /** Deploys `contract` from its hardhat artifact (`<file>.sol/<contract>.json`) as `from`. */
  const deploy = async (from: EvmAccount, file: string, contract: string, args: readonly unknown[]) => {
    const artifact = (await Bun.file(path.join(ARTIFACTS_DIR, file, `${contract}.json`)).json()) as { abi: Abi; bytecode: Hex };
    const receipt = await confirm(await from.deployContract({ abi: artifact.abi, bytecode: artifact.bytecode, args }));
    if (!receipt.contractAddress) throw new Error(`${contract} deployment created no contract`);
    return { address: receipt.contractAddress, abi: artifact.abi };
  };

  return { publicClient, fhevm, aclAddress: fhevm.chain.fhevm.contracts.acl.address as Address, account, confirm, deploy };
};
