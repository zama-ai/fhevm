/**
 * Single source of truth for the test topology: which slots exist, the foundry
 * anvil port / chain id behind each, and the key file each one serves.
 *
 * Everything downstream derives from this — anvil specs (orchestration), gateway
 * config (relayer + rpc proxy), and (later) the per-slot viem chain defs — so the
 * matrix stays consistent and adding a slot is a one-line change.
 *
 * There is a single slot today: the current cleartext stack (protocol 1.0). It deploys
 * from the default deployer mnemonic (the only one the v1.0 deploy accepts), and
 * its addresses are derived here and served at `/<slot>/config` so the browser
 * never hardcodes them.
 */

import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { AnvilSpec } from './anvil/anvils.js';
import type { GatewayConfig, GatewaySlot, SlotChainConfig } from './gateway/gateway.js';
import { deriveFhevmHostAddresses } from './anvil/fhevmAddresses.js';
import {
  DEFAULT_DEPLOYER_MNEMONIC,
  GATEWAY_CHAIN_ID,
  GATEWAY_CHEAT_ADDRESSES,
  GATEWAY_MOUNT_PREFIX,
  SLOT_INFO,
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
  readonly port: number;
  readonly chainId: number;
  readonly tfheVersion: string;
  /** Basename of the key file under `test/keys`. */
  readonly keyFile: string;
};

// Derived from config's SLOT_INFO — adding/rolling a slot is a one-line change there.
export const TOPOLOGY: readonly SlotTopology[] = (Object.keys(SLOT_INFO) as SlotId[]).map((slot) => {
  const info = SLOT_INFO[slot];
  return {
    slot,
    port: info.port,
    chainId: info.chainId,
    tfheVersion: info.tfheVersion,
    keyFile: info.keyFile,
  };
});

/** Anvil specs for `startAnvils`. Carries the slot's ACL address (for the reuse
 *  check) so a reused anvil is recognized only when its stack is deployed, and the
 *  KMSGeneration to seed with the slot key's digests (key.<ver>.digests.json). */
export function anvilSpecs(): AnvilSpec[] {
  const dir = keysDir();
  const h = deriveFhevmHostAddresses(DEFAULT_DEPLOYER_MNEMONIC);
  return TOPOLOGY.map((t) => ({
    slot: t.slot,
    port: t.port,
    chainId: t.chainId,
    aclAddress: h.acl,
    kmsGeneration: {
      address: h.kmsGeneration,
      protocolConfigAddress: h.protocolConfig,
      aclAddress: h.acl,
      digestsFilePath: resolve(dir, t.keyFile.replace(/\.json$/, '.digests.json')),
    },
  }));
}

/** Per-slot chain config (addresses) served at `/<slot>/config`. */
function slotChainConfig(chainId: number): SlotChainConfig {
  const h = deriveFhevmHostAddresses(DEFAULT_DEPLOYER_MNEMONIC);
  return {
    chainId,
    contracts: {
      acl: h.acl,
      inputVerifier: h.inputVerifier,
      kmsVerifier: h.kmsVerifier,
      protocolConfig: h.protocolConfig,
      kmsGeneration: h.kmsGeneration,
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
 *  chain config at `/<slot>/config`. */
export function gatewayConfig(): GatewayConfig {
  const dir = keysDir();
  const slots: Record<string, GatewaySlot> = {};
  for (const t of TOPOLOGY) {
    slots[t.slot] = {
      keyFilePath: resolve(dir, t.keyFile),
      rpcUrl: `http://127.0.0.1:${String(t.port)}`,
      chainConfig: slotChainConfig(t.chainId),
    };
  }

  return { mountPrefix: GATEWAY_MOUNT_PREFIX, slots, assetDir: wasmAssetDir() };
}
