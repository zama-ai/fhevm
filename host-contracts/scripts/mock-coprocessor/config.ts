/**
 * Per-chain configuration for the mock coprocessor daemon. Each chain entry
 * tells the worker:
 *   - which RPC URL to poll (env-overridable, sensible public default)
 *   - which contracts to watch (FHEVMExecutor + ConfidentialBridge), loaded
 *     from the per-chain `addresses/.env.host` snapshot the deploy runbook
 *     produces (sepolia/addresses/, polygonAmoy/addresses/, …).
 *
 * Adding a new chain = adding an entry to `CHAINS` below. No other code path
 * needs to change.
 *
 * Each chain belongs to an environment (`testnet` or `mainnet`). The daemon
 * watches exactly the two chains of the *active* environment, selected by the
 * `BRIDGE_ENV` env var (defaults to `testnet` for backward compatibility).
 */
import { config as dotenvConfig } from 'dotenv';
import { existsSync, readFileSync } from 'fs';
import { resolve } from 'path';

dotenvConfig({ path: resolve(__dirname, '..', '..', '.env') });

export interface ChainConfig {
  /** Human-friendly identifier (matches the hardhat network name). */
  name: string;
  /** EVM chain id. */
  chainId: number;
  /** LayerZero V2 endpoint id (informational, not used for RPC). */
  lzEid: number;
  /** JSON-RPC URL. */
  rpcUrl: string;
  /** Deployed FHEVMExecutor proxy on this chain. */
  fhevmExecutor: string;
  /** Deployed ConfidentialBridge proxy on this chain. */
  confidentialBridge: string;
}

export type BridgeEnv = 'testnet' | 'mainnet';

interface ChainSpec {
  /** Environment this chain belongs to. The daemon runs one environment at a time. */
  env: BridgeEnv;
  name: string;
  chainId: number;
  lzEid: number;
  rpcEnvVar: string;
  rpcDefault: string;
  addressesEnv: string;
}

const CHAINS: ChainSpec[] = [
  // ── testnet ──────────────────────────────────────────────────────────────
  {
    env: 'testnet',
    name: 'sepolia',
    chainId: 11155111,
    lzEid: 40161,
    rpcEnvVar: 'SEPOLIA_RPC_URL',
    rpcDefault: 'https://sepolia.drpc.org',
    addressesEnv: 'addresses-sepolia/.env.host',
  },
  {
    env: 'testnet',
    name: 'polygonAmoy',
    chainId: 80002,
    lzEid: 40267,
    rpcEnvVar: 'POLYGON_AMOY_RPC_URL',
    rpcDefault: 'https://rpc-amoy.polygon.technology',
    addressesEnv: 'addresses-amoy/.env.host',
  },
  // ── mainnet ──────────────────────────────────────────────────────────────
  {
    env: 'mainnet',
    name: 'ethereum',
    chainId: 1,
    lzEid: 30101,
    rpcEnvVar: 'ETHEREUM_MAINNET_RPC_URL',
    rpcDefault: 'https://eth.llamarpc.com',
    addressesEnv: 'addresses-ethereum/.env.host',
  },
  {
    env: 'mainnet',
    name: 'polygon',
    chainId: 137,
    lzEid: 30109,
    rpcEnvVar: 'POLYGON_MAINNET_RPC_URL',
    rpcDefault: 'https://polygon-rpc.com',
    addressesEnv: 'addresses-polygon/.env.host',
  },
];

/**
 * Resolve the active bridge environment from `BRIDGE_ENV` (default `testnet`).
 */
export function activeBridgeEnv(): BridgeEnv {
  const raw = (process.env.BRIDGE_ENV ?? 'testnet').trim().toLowerCase();
  if (raw === 'mainnet' || raw === 'main' || raw === 'prod') return 'mainnet';
  if (raw === 'testnet' || raw === 'test' || raw === '') return 'testnet';
  throw new Error(`Invalid BRIDGE_ENV="${process.env.BRIDGE_ENV}". Use "testnet" (default) or "mainnet".`);
}

function parseEnvFile(path: string): Record<string, string> {
  if (!existsSync(path)) {
    throw new Error(
      `[mock-coprocessor] addresses snapshot not found at ${path}. ` +
        `Run task:deployAllHostContracts on this chain and snapshot the addresses (see addresses/BRIDGE_DEPLOYMENT.md §2.4).`,
    );
  }
  const lines = readFileSync(path, 'utf8').split('\n');
  const env: Record<string, string> = {};
  for (const raw of lines) {
    const line = raw.trim();
    if (line === '' || line.startsWith('#')) continue;
    const eq = line.indexOf('=');
    if (eq < 0) continue;
    const key = line.slice(0, eq).trim();
    let val = line.slice(eq + 1).trim();
    if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) {
      val = val.slice(1, -1);
    }
    env[key] = val;
  }
  return env;
}

function buildConfig(spec: ChainSpec): ChainConfig {
  const addressesPath = resolve(__dirname, '..', '..', spec.addressesEnv);
  const env = parseEnvFile(addressesPath);
  const fhevmExecutor = env.FHEVM_EXECUTOR_CONTRACT_ADDRESS;
  const confidentialBridge = env.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS;
  if (!fhevmExecutor) {
    throw new Error(`[mock-coprocessor] ${spec.addressesEnv} is missing FHEVM_EXECUTOR_CONTRACT_ADDRESS.`);
  }
  if (!confidentialBridge) {
    throw new Error(
      `[mock-coprocessor] ${spec.addressesEnv} is missing CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS — bridge events won't be propagated.`,
    );
  }
  const rpcUrl = (process.env[spec.rpcEnvVar] ?? spec.rpcDefault).trim();
  return {
    name: spec.name,
    chainId: spec.chainId,
    lzEid: spec.lzEid,
    rpcUrl,
    fhevmExecutor,
    confidentialBridge,
  };
}

export function loadChainConfigs(): ChainConfig[] {
  const env = activeBridgeEnv();
  const specs = CHAINS.filter((c) => c.env === env);
  if (specs.length === 0) {
    throw new Error(`[mock-coprocessor] no chains configured for BRIDGE_ENV=${env}.`);
  }
  return specs.map(buildConfig);
}

/**
 * Polling / batching knobs. Override via env to tune to a specific RPC's
 * rate limits without touching the source.
 */
export const RUNTIME = {
  /** Sleep between poll cycles when caught up to head. */
  pollIntervalMs: Number(process.env.MOCK_COPROCESSOR_POLL_INTERVAL_MS ?? 5_000),
  /** Maximum block range per `eth_getLogs` call. Most public RPCs cap at 1k–10k. */
  maxBlockRange: Number(process.env.MOCK_COPROCESSOR_MAX_BLOCK_RANGE ?? 1_000),
  /** Backoff after a failed poll cycle. */
  errorBackoffMs: Number(process.env.MOCK_COPROCESSOR_ERROR_BACKOFF_MS ?? 10_000),
  /** Path to the persistent SQLite DB. */
  dbPath: process.env.MOCK_COPROCESSOR_DB_PATH ?? resolve(__dirname, 'mock-coprocessor.db'),
  /** How many times to retry resolving a pending HandleBridged before giving up. */
  bridgeRetryLimit: Number(process.env.MOCK_COPROCESSOR_BRIDGE_RETRY_LIMIT ?? 20),
};
