// Registry of supported environments for the coprocessor-upgrade hardhat tasks.
//
// To add a new chain to an existing environment: append to that environment's
// `chains` array. To add a new environment: add a top-level key with `chains`
// + `gateway`.
//
// Each entry names an env var; the script reads `process.env[rpcUrlEnv]` at
// run time. If the env var is unset and `defaultRpcUrl` is present, the
// default is used — reserve defaults for PUBLIC, well-known endpoints
// (e.g. testnet faucets). Mainnet endpoints and any private/internal URL
// must omit `defaultRpcUrl` so the operator has to set the secret.

export interface EnvChainEntry {
  chainId: number;
  label: string;
  fallbackBlockTimeSeconds: number;
  rpcUrlEnv: string;
  defaultRpcUrl?: string;
}

export interface EnvGatewayEntry {
  label: string;
  fallbackBlockTimeSeconds: number;
  rpcUrlEnv: string;
  defaultRpcUrl?: string;
}

export interface EnvironmentDef {
  chains: EnvChainEntry[];
  gateway: EnvGatewayEntry;
}

// Public host-chain defaults — same physical networks (Sepolia, Amoy) whether
// called via devnet or testnet, so reused.
const PUBLIC_SEPOLIA_RPC = 'https://ethereum-sepolia-rpc.publicnode.com';
const PUBLIC_AMOY_RPC = 'https://rpc-amoy.polygon.technology';

export const ENVIRONMENTS: Record<string, EnvironmentDef> = {
  devnet: {
    chains: [
      {
        chainId: 11155111,
        label: 'sepolia',
        fallbackBlockTimeSeconds: 12,
        rpcUrlEnv: 'SEPOLIA_ETH_RPC_URL',
        defaultRpcUrl: PUBLIC_SEPOLIA_RPC,
      },
      {
        chainId: 80002,
        label: 'amoy',
        fallbackBlockTimeSeconds: 1.5,
        rpcUrlEnv: 'POLYGON_AMOY_RPC_URL',
        defaultRpcUrl: PUBLIC_AMOY_RPC,
      },
    ],
    // Devnet gateway is a Zama-internal endpoint — no public default; env var required.
    gateway: { label: 'gateway-devnet', fallbackBlockTimeSeconds: 2, rpcUrlEnv: 'GATEWAY_DEVNET_RPC_URL' },
  },
  testnet: {
    chains: [
      {
        chainId: 11155111,
        label: 'sepolia',
        fallbackBlockTimeSeconds: 12,
        rpcUrlEnv: 'SEPOLIA_ETH_RPC_URL',
        defaultRpcUrl: PUBLIC_SEPOLIA_RPC,
      },
      {
        chainId: 80002,
        label: 'amoy',
        fallbackBlockTimeSeconds: 1.5,
        rpcUrlEnv: 'POLYGON_AMOY_RPC_URL',
        defaultRpcUrl: PUBLIC_AMOY_RPC,
      },
    ],
    gateway: {
      label: 'gateway-testnet',
      fallbackBlockTimeSeconds: 2,
      rpcUrlEnv: 'GATEWAY_TESTNET_RPC_URL',
      defaultRpcUrl: 'https://rpc.testnet.zama.org',
    },
  },
  // No defaults — production RPCs are private; env vars required.
  mainnet: {
    chains: [
      { chainId: 1, label: 'ethereum', fallbackBlockTimeSeconds: 12, rpcUrlEnv: 'MAINNET_ETH_RPC_URL' },
      { chainId: 137, label: 'polygon', fallbackBlockTimeSeconds: 2, rpcUrlEnv: 'POLYGON_MAINNET_RPC_URL' },
    ],
    gateway: { label: 'gateway-mainnet', fallbackBlockTimeSeconds: 2, rpcUrlEnv: 'GATEWAY_MAINNET_RPC_URL' },
  },
};

export const SUPPORTED_ENVIRONMENTS = Object.keys(ENVIRONMENTS);

// Module-load invariant: chainIds within an environment must be unique.
for (const [name, def] of Object.entries(ENVIRONMENTS)) {
  const ids = def.chains.map((c) => c.chainId);
  const dup = ids.find((id, i) => ids.indexOf(id) !== i);
  if (dup !== undefined) {
    throw new Error(`ENVIRONMENTS.${name}.chains contains duplicate chainId ${dup}`);
  }
}
