// Single source of truth for the test topology: which slots exist, the foundry
// profile / anvil port / chain id behind each, and the key file each one serves.
//
// Everything downstream derives from this — anvil specs (orchestration), gateway
// config (relayer + rpc proxy), and (later) the per-slot viem chain defs — so the
// matrix stays consistent and adding a slot is a one-line change.
//
// foundry profile -> on-chain ACL version -> resolved TFHE/key version:
//   v12 -> protocol 0.12 -> tfhe 1.5.4
//   v13 -> protocol 0.13 -> tfhe 1.6.1
//
// The two anvils deploy from DIFFERENT deployer mnemonics so their FHEVM stacks
// land at DIFFERENT addresses (the SDK resolves/caches the protocol version per
// contract address — shared addresses across chains collide). v12 uses
// FIRST_ANVIL_MNEMONIC; v13 uses the deploy's default mnemonic (so its committed
// addresses file is regenerated identically and stays clean). Each slot's
// addresses are derived here and served at `/<slot>/config` so the browser never
// hardcodes them.

import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { AnvilSpec } from './anvil/anvils.js';
import type { GatewayConfig, GatewaySlot, SlotChainConfig } from './gateway/gateway.js';
import { deriveFhevmHostAddresses } from './anvil/fhevmAddresses.js';
import {
  CURRENT_SLOT,
  DEFAULT_DEPLOYER_MNEMONIC,
  GATEWAY_CHAIN_ID,
  GATEWAY_CHEAT_ADDRESSES,
  GATEWAY_MOUNT_PREFIX,
  LEGACY_SLOT,
  OLD_MODULE_NEW_KEY_SLOT,
  SLOT_INFO,
  type FoundryProfile,
  type SlotId,
} from './config.js';

/**
 * Absolute path to `test/keys`. Prefer the `FHEVM_TEST_KEYS_DIR` env var (set by
 * the test runner) so bundled consumers (e.g. a Turbopack-built Next route) never
 * evaluate `import.meta.url`, which the bundler rewrites. The `import.meta.url`
 * fallback is for un-bundled Node contexts (CLI, globalSetup) and is computed
 * lazily so it is only touched when the env var is absent.
 */
function keysDir(): string {
  const fromEnv = process.env.FHEVM_TEST_KEYS_DIR;
  if (fromEnv !== undefined && fromEnv.length > 0) {
    return fromEnv;
  }
  // topology.ts lives in test/infra, so keys is one level up.
  return resolve(dirname(fileURLToPath(import.meta.url)), '../keys');
}

/**
 * SDK wasm dir whose raw assets the gateway serves at `/gw/asset/<filename>` (for the
 * URL-based wasm-load modes). Prefer `FHEVM_TEST_WASM_DIR`; otherwise default to
 * browser-next's installed SDK (resolved relative to this file, so it works from both
 * the in-process globalSetup gateway and the standalone server).
 */
function wasmAssetDir(): string {
  const fromEnv = process.env.FHEVM_TEST_WASM_DIR;
  if (fromEnv !== undefined && fromEnv.length > 0) {
    return fromEnv;
  }
  return resolve(dirname(fileURLToPath(import.meta.url)), '../browser-next/node_modules/@fhevm/sdk/_esm/wasm');
}

export type SlotTopology = {
  readonly slot: string;
  readonly foundryProfile: FoundryProfile;
  readonly port: number;
  readonly chainId: number;
  readonly tfheVersion: string;
  /** Basename of the key file under `test/keys`. */
  readonly keyFile: string;
  /**
   * Deployer mnemonic for this slot's anvil. Omit to use the deploy's built-in
   * default (the committed addresses; its FHEVMHostAddresses.sol stays clean). Set
   * a custom one to land the stack at a distinct address set — anvils.ts then
   * passes it as `DEPLOYER_MNEMONIC` and restores the regenerated address file.
   */
  readonly deployerMnemonic?: string | undefined;
};

// Derived from config's SLOT_INFO — adding/rolling a slot is a one-line change
// there. Each slot uses a distinct chain id (Foundry broadcast/cache is keyed by
// chain id; concurrent same-id deploys collide as "nonce too low") AND a distinct
// deployer (so addresses differ — see config.ts).
export const TOPOLOGY: readonly SlotTopology[] = (Object.keys(SLOT_INFO) as SlotId[]).map((slot) => {
  const info = SLOT_INFO[slot];
  return {
    slot,
    foundryProfile: info.foundryProfile,
    port: info.port,
    chainId: info.chainId,
    tfheVersion: info.tfheVersion,
    keyFile: info.keyFile,
    deployerMnemonic: info.deployerMnemonic,
  };
});

/** Resolved deployer mnemonic for a slot (its own, or the deploy default). */
function slotMnemonic(t: SlotTopology): string {
  return t.deployerMnemonic ?? DEFAULT_DEPLOYER_MNEMONIC;
}

/** Anvil specs for `startAnvils`. Carries the per-slot deployer + its ACL address
 *  (for the reuse check) so each anvil deploys to — and is recognised at — its own
 *  distinct addresses. */
export function anvilSpecs(): AnvilSpec[] {
  return TOPOLOGY.map((t) => ({
    slot: t.slot,
    port: t.port,
    foundryProfile: t.foundryProfile,
    chainId: t.chainId,
    deployerMnemonic: t.deployerMnemonic,
    aclAddress: deriveFhevmHostAddresses(slotMnemonic(t)).acl,
  }));
}

/** Per-slot chain config (addresses) served at `/<slot>/config`. */
function slotChainConfig(chainId: number, mnemonic: string): SlotChainConfig {
  const h = deriveFhevmHostAddresses(mnemonic);
  return {
    chainId,
    contracts: {
      acl: h.acl,
      inputVerifier: h.inputVerifier,
      kmsVerifier: h.kmsVerifier,
      protocolConfig: h.protocolConfig,
    },
    gateway: {
      id: GATEWAY_CHAIN_ID,
      contracts: {
        decryption: GATEWAY_CHEAT_ADDRESSES.decryption,
        inputVerification: GATEWAY_CHEAT_ADDRESSES.inputVerification,
      },
    },
  };
}

/** Gateway config: each slot proxies its anvil RPC, serves its key, and serves its
 *  (per-deployer) chain config at `/<slot>/config`. */
export function gatewayConfig(): GatewayConfig {
  const dir = keysDir();
  const slots: Record<string, GatewaySlot> = {};
  for (const t of TOPOLOGY) {
    slots[t.slot] = {
      keyFilePath: resolve(dir, t.keyFile),
      rpcUrl: `http://127.0.0.1:${String(t.port)}`,
      chainConfig: slotChainConfig(t.chainId, slotMnemonic(t)),
    };
  }

  // Alias slot for the expected-fail leg (see OLD_MODULE_NEW_KEY_SLOT in config):
  // the current key over the legacy anvil. Its chain config is the legacy slot's (it
  // proxies the legacy RPC, so init reads the older ACL → older module), letting that
  // older module fail to deserialize the newer key. Derived from TOPOLOGY so it
  // tracks any port/key change.
  const legacy = TOPOLOGY.find((t) => t.slot === LEGACY_SLOT);
  const current = TOPOLOGY.find((t) => t.slot === CURRENT_SLOT);
  if (legacy !== undefined && current !== undefined) {
    slots[OLD_MODULE_NEW_KEY_SLOT] = {
      keyFilePath: resolve(dir, current.keyFile),
      rpcUrl: `http://127.0.0.1:${String(legacy.port)}`,
      chainConfig: slotChainConfig(legacy.chainId, slotMnemonic(legacy)),
    };
  }

  return { mountPrefix: GATEWAY_MOUNT_PREFIX, slots, assetDir: wasmAssetDir() };
}
