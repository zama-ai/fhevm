/**
 * Centralized configuration for the test infra + browser tests.
 *
 * CONSTANTS ONLY — no functions, no `node:` imports — so the Node infra
 * (topology, anvils, gateway), the browser app bundle, and the specs can all
 * import this same file. Anything that needs to read env or the filesystem lives
 * in topology.ts, not here.
 *
 * ─── VERSION ROLL ───────────────────────────────────────────────────────────
 * The protocol rolls forward roughly every release. When that happens, edit ONLY
 * this file — rename CURRENT_SLOT and bump its SLOT_INFO entry (profile, port,
 * chain id, tfhe version, key file) and GATEWAY_CHAIN_ID. Every downstream
 * consumer (gateway URLs, anvil specs, app pages, specs) derives from these, so
 * nothing else needs touching.
 */

// ─── Gateway ────────────────────────────────────────────────────────────────
// `gw` has no leading underscore on purpose: Next.js treats `_`-prefixed app
// folders as private (excluded from routing), so the mount segment cannot start
// with `_`. Keep GATEWAY_PORT in sync with next.config.mjs.
export const GATEWAY_MOUNT_PREFIX = '/gw';
export const GATEWAY_PORT = 8590;

// The on-chain gateway chain id baked into the deploy: the v1.0 stack's
// LocalHostBootstrap.GATEWAY_CHAIN_ID, uint48(keccak256("fhevm.cheat.chainId cleartext gateway")).
export const GATEWAY_CHAIN_ID = 100_733_346_448_153;

// Gateway verifying-contract addresses: cheat addresses (keccak of a fixed
// string), constant across every slot/deploy — NOT deployer-derived.
export const GATEWAY_CHEAT_ADDRESSES = {
  inputVerification: '0x6189F6c0c3E40B4a3c72ec86262295D78d845297',
  decryption: '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721',
} as const;

// ─── Deployers ──────────────────────────────────────────────────────────────
// Default deployer mnemonic baked into contracts/scripts/fhevm-deploy.sh, and the
// account index it uses for the main deployer.
export const DEFAULT_DEPLOYER_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
export const DEFAULT_DEPLOYER_INDEX = 5;

// ─── Slots ──────────────────────────────────────────────────────────────────
// The protocol slot exercised by the matrix. Its id is also the gateway URL
// segment (`/gw/<slot>/...`) and the Foundry profile name.
export const CURRENT_SLOT = 'v1.0';

export type SlotId = typeof CURRENT_SLOT;
/** Foundry profile name — currently identical to the slot id. */
export type FoundryProfile = SlotId;

// Per-slot constants. The slot deploys with the deploy's built-in default
// mnemonic: the v1.0 deploy checks the committed src/v1.0.0/config/addresses.sol,
// so it only succeeds at that canonical address set.
export const SLOT_INFO = {
  [CURRENT_SLOT]: {
    foundryProfile: 'v1.0',
    port: 8546,
    chainId: 31338,
    tfheVersion: '1.6.2',
    keyFile: 'key.1.6.1.json',
  },
} as const;
