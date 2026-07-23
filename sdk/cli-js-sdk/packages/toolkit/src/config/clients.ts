import {
  createFhevmClient,
  createFhevmDecryptClient,
} from "@fhevm/sdk/viem";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Hex,
} from "viem";
import type { Account } from "viem/accounts";

import { loadAccount } from "./account";
import { resolveNetworkConfig } from "./networks";
import {
  resolveChain,
  resolveContractAddress,
  resolveRpcUrl,
} from "./resolve";
import { configureFhevmRuntime } from "./runtime";
import type { ClientOptions } from "./types";

/**
 * Creates read-only viem and FHEVM SDK clients for the selected network.
 *
 * The viem host chain can differ from the FHEVM gateway chain; use the returned
 * `chain` for SDK calls and `publicClient` for host-chain contract reads.
 */
export const createClients = (options: ClientOptions) => {
  const networkConfig = resolveNetworkConfig(options.network);
  configureFhevmRuntime();

  const chain = resolveChain(options);
  const rpcUrl = resolveRpcUrl(options);
  const transport = http(rpcUrl);
  const publicClient = createPublicClient({
    chain: networkConfig.hostChain,
    transport,
  });
  const fhevm = createFhevmClient({ chain, publicClient });

  return { chain, fhevm, publicClient, rpcUrl, transport };
};

const DECRYPT_CLIENT_TKMS_VERSIONS = ["0.13.10", "0.13.20-0"] as const;

export type DecryptClientTkmsVersion =
  (typeof DECRYPT_CLIENT_TKMS_VERSIONS)[number];

/**
 * Narrows the SDK's open `tkmsVersion: string` to a supported literal, throwing
 * when the generated transport key pair reports a version this CLI cannot pin.
 */
export const asDecryptClientTkmsVersion = (
  tkmsVersion: string,
): DecryptClientTkmsVersion => {
  if (
    (DECRYPT_CLIENT_TKMS_VERSIONS as readonly string[]).includes(tkmsVersion)
  ) {
    return tkmsVersion as DecryptClientTkmsVersion;
  }
  throw new Error(
    `Unsupported TKMS version ${tkmsVersion}; expected one of ${DECRYPT_CLIENT_TKMS_VERSIONS.join(", ")}.`,
  );
};

/** Creates read-only host-chain and version-pinned decrypt-only SDK clients. */
export const createDecryptClients = (
  options: ClientOptions,
  tkmsVersion: DecryptClientTkmsVersion,
) => {
  const networkConfig = resolveNetworkConfig(options.network);
  configureFhevmRuntime();

  const chain = resolveChain(options);
  const rpcUrl = resolveRpcUrl(options);
  const transport = http(rpcUrl);
  const publicClient = createPublicClient({
    chain: networkConfig.hostChain,
    transport,
  });
  const fhevm = createFhevmDecryptClient({
    chain,
    publicClient,
    options: { moduleVersions: { kms: tkmsVersion } },
  });

  return { chain, fhevm, publicClient, rpcUrl, transport };
};

/** Read-only client context plus the resolved FHETest contract address. */
export type ClientContext = ReturnType<typeof createClients> &
  Readonly<{
    contractAddress: Hex;
  }>;

/** Creates a read-only client context for the resolved FHETest contract. */
export const createClientContext = (options: ClientOptions): ClientContext => ({
  ...createClients(options),
  contractAddress: resolveContractAddress(options),
});

/** Decrypt-only client context plus the resolved FHETest contract address. */
export type DecryptClientContext = ReturnType<typeof createDecryptClients> &
  Readonly<{
    contractAddress: Hex;
  }>;

/** Creates a decrypt-only context without initializing TFHE encryption. */
export const createDecryptClientContext = (
  options: ClientOptions,
  tkmsVersion: DecryptClientTkmsVersion,
): DecryptClientContext => ({
  ...createDecryptClients(options, tkmsVersion),
  contractAddress: resolveContractAddress(options),
});

/** Creates a wallet client from default wallet credentials. */
export const createWallet = (
  options: ClientOptions & { privateKey?: Hex; mnemonic?: string },
) => {
  const account = loadAccount(options.privateKey, options.mnemonic);
  return createWalletForAccount(options, account);
};

/** Creates a wallet client for an already resolved account. */
export const createWalletForAccount = (
  options: ClientOptions,
  account: Account,
) => {
  const { transport, publicClient, fhevm, chain, rpcUrl } =
    createClients(options);
  const walletClient = createWalletClient({
    account,
    chain: resolveNetworkConfig(options.network).hostChain,
    transport,
  });

  return { account, chain, fhevm, publicClient, rpcUrl, walletClient };
};

/** Wallet client context plus the resolved FHETest contract address. */
export type WalletContext = ReturnType<typeof createWallet> &
  Readonly<{
    contractAddress: Hex;
  }>;

/** Creates a wallet context from default wallet credentials. */
export const createWalletContext = (
  options: ClientOptions & { privateKey?: Hex; mnemonic?: string },
): WalletContext => ({
  ...createWallet(options),
  contractAddress: resolveContractAddress(options),
});

/** Creates a wallet context for a caller-supplied account. */
export const createWalletContextForAccount = (
  options: ClientOptions,
  account: Account,
): WalletContext => ({
  ...createWalletForAccount(options, account),
  contractAddress: resolveContractAddress(options),
});
