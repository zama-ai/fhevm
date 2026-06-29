// Centralized configuration for the test infra + browser tests.
//
// CONSTANTS ONLY — no functions, no `node:` imports — so the Node infra
// (topology, anvils, gateway), the browser app bundle, and the specs can all
// import this same file. Anything that needs to read env or the filesystem lives
// in topology.ts, not here.
//
// ─── VERSION ROLL ───────────────────────────────────────────────────────────
// The protocol rolls forward roughly every release: vX retires, vX+1 arrives.
// When that happens, edit ONLY this file — rename LEGACY_SLOT / CURRENT_SLOT and
// bump the matching SLOT_INFO entry (profile, dir, port, chain id, tfhe version,
// key file). Every downstream consumer (gateway URLs, anvil specs, app pages,
// specs) derives from these, so nothing else needs touching.

// ─── Gateway ────────────────────────────────────────────────────────────────
// `gw` has no leading underscore on purpose: Next.js treats `_`-prefixed app
// folders as private (excluded from routing), so the mount segment cannot start
// with `_`. Keep GATEWAY_PORT in sync with next.config.mjs.
export const GATEWAY_MOUNT_PREFIX = '/gw';
export const GATEWAY_PORT = 8590;

// The on-chain gateway chain id baked into the deploy (chain_id_gateway).
export const GATEWAY_CHAIN_ID = 654_321;

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

// Distinct deployer for the LEGACY slot so its FHEVM stack lands at a DIFFERENT
// address set than the current slot — mandatory, because the SDK resolves/caches
// the protocol version per contract address, so two chains sharing addresses
// collide. Any valid BIP-39 phrase works; fixed here for reproducibility. The
// current slot keeps the default mnemonic so its committed FHEVMHostAddresses.sol
// regenerates identically (stays clean); the legacy slot's file is restored after
// deploy (see anvils.ts).
export const FIRST_ANVIL_MNEMONIC = 'voyage gaze harbor anchor library snow trash old possible outdoor ability note';

// ─── Slots ──────────────────────────────────────────────────────────────────
// The two protocol slots exercised by the matrix. These ids are also the gateway
// URL segments (`/gw/<slot>/...`) and the Foundry profile names.
export const LEGACY_SLOT = 'v12';
export const CURRENT_SLOT = 'v13';

export type SlotId = typeof LEGACY_SLOT | typeof CURRENT_SLOT;
/** Foundry profile name — currently identical to the slot id. */
export type FoundryProfile = SlotId;

// Gateway-only alias slot: serves the CURRENT key (newer TFHE) but proxies the
// LEGACY anvil (older ACL → older module). It exists as a distinct id/URL so the
// SDK's relayer-URL-keyed key cache does not collide with the real current slot —
// letting old-module + new-key fail for the right reason (deserialization), not a
// cache clash. No anvil backs it; it reuses the legacy anvil.
export const OLD_MODULE_NEW_KEY_SLOT = 'oldmod-newkey';

// Per-slot constants. Each slot MUST use a distinct chainId (Foundry broadcast
// cache is keyed by chain id; concurrent same-id deploys collide as "nonce too
// low") AND a distinct deployer (so addresses differ — see FIRST_ANVIL_MNEMONIC).
// `deployerMnemonic: undefined` ⇒ use the deploy's built-in default (committed
// addresses; FHEVMHostAddresses.sol stays clean).
//
//   profileVersionDir: the contracts source dir whose generated
//   FHEVMHostAddresses.sol the deploy (re)writes — restored post-deploy.
export const SLOT_INFO = {
  [LEGACY_SLOT]: {
    foundryProfile: 'v12',
    profileVersionDir: 'v0.12.0',
    port: 8544,
    chainId: 31337,
    tfheVersion: '1.5.4',
    keyFile: 'key.1.5.4.json',
    deployerMnemonic: FIRST_ANVIL_MNEMONIC,
  },
  [CURRENT_SLOT]: {
    foundryProfile: 'v13',
    profileVersionDir: 'v0.13.0',
    port: 8546,
    chainId: 31338,
    tfheVersion: '1.6.1',
    keyFile: 'key.1.6.1.json',
    deployerMnemonic: undefined,
  },
} as const;
