import { JsonRpcProvider } from "ethers";
import { extendEnvironment } from "hardhat/config";
import { lazyObject } from "hardhat/plugins";
import type { HardhatRuntimeEnvironment } from "hardhat/types";

import { FhevmApi } from "./api";
import type { Eip1193Provider } from "./engine/node";
import type { HardhatFhevmRuntimeEnvironment } from "./types";

export { FhevmType } from "./types";
export type {
  HardhatFhevmRuntimeEnvironment,
  RelayerEncryptedInput,
  EncryptedInputResult,
  FhevmTypeEuint,
} from "./types";

/** Chain ids treated as real networks (not the mock). Everything else is a mock (hardhat / anvil). */
const REAL_CHAIN_IDS = new Set<number>([1, 11155111]);

declare module "hardhat/types/runtime" {
  interface HardhatRuntimeEnvironment {
    fhevm: HardhatFhevmRuntimeEnvironment;
  }
}

/**
 * HH2 entry point. All Hardhat coupling is confined to this file; the api and engine are Hardhat-free, so
 * an HH3 port only rewrites this glue (a plugin object + hooks) — not the engine.
 */
extendEnvironment((hre) => {
  hre.fhevm = lazyObject(() => {
    const chainId = hre.network.config.chainId;
    const isMock = chainId === undefined ? true : !REAL_CHAIN_IDS.has(chainId);
    return new FhevmApi({ provider: resolveEngineProvider(hre), isMock });
  });
});

/**
 * The engine drives the chain through cheat codes (`setCode`/`setStorageAt`) and transactions from an
 * impersonated deployer. That requires DIRECT node access:
 *
 *  - External node (anvil / localhost — `network.config.url` set): Hardhat wraps the RPC with an
 *    account-signing provider that rejects `eth_sendTransaction` from unmanaged (impersonated) accounts
 *    (HH103). So connect straight to the URL, bypassing that wrapper. The user's own contract calls still
 *    go through `hre.ethers` (managed accounts) against the same node.
 *  - In-process Hardhat network (no URL): `hre.network.provider` supports impersonation natively.
 */
function resolveEngineProvider(hre: HardhatRuntimeEnvironment): Eip1193Provider {
  const url = (hre.network.config as { url?: string }).url;
  if (typeof url === "string" && url.length > 0) {
    const jsonRpc = new JsonRpcProvider(url);
    return { request: (args) => jsonRpc.send(args.method, (args.params ?? []) as unknown[]) };
  }
  return hre.network.provider as unknown as Eip1193Provider;
}
