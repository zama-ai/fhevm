// Resolves the environment for `run-tests.sh --chain <name>`, so one pod holding the env vars of
// several chains can run the tests against any of them.
//
// The chain name selects the Hardhat network (fixed table below), and the chain-prefixed vars
// (e.g. BNB_ACL_CONTRACT_ADDRESS, BNB_RPC_URL) provide that chain's addresses and RPC for the
// current cluster. Every required var is validated first: on any problem the script lists them
// all on stderr and exits 1. On success it prints `export`/`unset` lines on stdout for run-tests.sh
// to eval (run-tests.sh checks every line has that shape first).
//
// Usage: npx ts-node --transpile-only scripts/resolve-chain-env.ts --chain <name> [--network <name>]
import dotenv from 'dotenv';
import { resolve } from 'path';

import { readAddress } from './env-validation';

// Same dotenv resolution as hardhat.config.ts. Values already in the environment win.
dotenv.config({ path: resolve(__dirname, '..', process.env.DOTENV_CONFIG_PATH || './.env') });

type Chain = {
  prefix: string;
  network: string;
  chainId: number;
  // Network-specific RPC var read first by hardhat.config.ts for this network.
  rpcEnv: string;
};

// DevNet and Testnet run the testnet chains; mainnet needs the explicit `-mainnet` suffix.
const CHAINS: Record<string, Chain> = {
  eth: { prefix: 'ETH', network: 'sepolia', chainId: 11155111, rpcEnv: 'SEPOLIA_ETH_RPC_URL' },
  polygon: { prefix: 'POLYGON', network: 'polygonAmoy', chainId: 80002, rpcEnv: 'POLYGON_AMOY_RPC_URL' },
  bnb: { prefix: 'BNB', network: 'bnbTestnet', chainId: 97, rpcEnv: 'BNB_TESTNET_RPC_URL' },
  hoodi: { prefix: 'HOODI', network: 'hoodi', chainId: 560048, rpcEnv: 'HOODI_RPC_URL' },
  'eth-mainnet': { prefix: 'ETH', network: 'mainnet', chainId: 1, rpcEnv: 'MAINNET_ETH_RPC_URL' },
  'polygon-mainnet': { prefix: 'POLYGON', network: 'polygon', chainId: 137, rpcEnv: 'POLYGON_RPC_URL' },
  'bnb-mainnet': { prefix: 'BNB', network: 'bnb', chainId: 56, rpcEnv: 'BNB_RPC_URL' },
};

// Chain-specific vars, read as `<PREFIX>_<NAME>` and exported as `<NAME>`.
const CHAIN_ADDRESS_VARS = [
  'ACL_CONTRACT_ADDRESS',
  'FHEVM_EXECUTOR_CONTRACT_ADDRESS',
  'KMS_VERIFIER_CONTRACT_ADDRESS',
  'INPUT_VERIFIER_CONTRACT_ADDRESS',
  'PROTOCOL_CONFIG_CONTRACT_ADDRESS',
];
// Only the HCU block cap scenario needs it.
const OPTIONAL_CHAIN_ADDRESS_VARS = ['HCU_LIMIT_CONTRACT_ADDRESS'];

// Shared by every chain of the cluster, read unprefixed.
const SHARED_ADDRESS_VARS = ['DECRYPTION_ADDRESS', 'INPUT_VERIFICATION_ADDRESS'];
const SHARED_VARS = ['RELAYER_URL', 'MNEMONIC'];
const SHARED_INTEGER_VARS = ['CHAIN_ID_GATEWAY'];

const parseArgs = (): { chainName?: string; network?: string } => {
  const args = process.argv.slice(2);
  const valueOf = (flag: string) => {
    const idx = args.indexOf(flag);
    return idx === -1 ? undefined : args[idx + 1];
  };
  return { chainName: valueOf('--chain'), network: valueOf('--network') };
};

const shellQuote = (value: string) => `'${value.replace(/'/g, `'\\''`)}'`;

const fail = (chainName: string | undefined, errors: string[]): never => {
  console.error(`Error: --chain ${chainName ?? ''}: ${errors.length} problem(s)`);
  for (const error of errors) console.error(`  - ${error}`);
  process.exit(1);
};

const main = () => {
  const { chainName, network } = parseArgs();
  const chain = chainName ? CHAINS[chainName] : undefined;
  if (!chain) {
    fail(chainName, [`unknown chain '${chainName ?? ''}'; expected one of: ${Object.keys(CHAINS).join(', ')}`]);
    return;
  }
  if (network && network !== chain.network) {
    fail(chainName, [`--network ${network} conflicts with --chain ${chainName}, which selects '${chain.network}'`]);
  }

  const errors: string[] = [];
  const exports: Record<string, string> = {};

  for (const name of CHAIN_ADDRESS_VARS) {
    const address = readAddress(`${chain.prefix}_${name}`, errors);
    if (address) exports[name] = address;
  }
  // Unset optional vars the chain doesn't define, so another chain's value can't leak through.
  const unsets: string[] = [];
  for (const name of OPTIONAL_CHAIN_ADDRESS_VARS) {
    if (!process.env[`${chain.prefix}_${name}`]?.trim()) {
      unsets.push(name);
      continue;
    }
    const address = readAddress(`${chain.prefix}_${name}`, errors);
    if (address) exports[name] = address;
  }
  const rpcName = `${chain.prefix}_RPC_URL`;
  const rpcUrl = process.env[rpcName]?.trim();
  if (!rpcUrl) errors.push(`${rpcName} is not set`);

  for (const name of SHARED_ADDRESS_VARS) readAddress(name, errors);
  for (const name of SHARED_VARS) {
    if (!process.env[name]?.trim()) errors.push(`${name} is not set`);
  }
  for (const name of SHARED_INTEGER_VARS) {
    const value = process.env[name]?.trim();
    if (!value) errors.push(`${name} is not set`);
    else if (!/^[1-9][0-9]*$/.test(value)) errors.push(`${name} is not a positive integer: ${value}`);
  }

  if (errors.length) fail(chainName, errors);

  // The chain's RPC is exported both as RPC_URL and as the network-specific var that
  // hardhat.config.ts reads first, so a value stored with `npx hardhat vars` can't take precedence.
  Object.assign(exports, {
    RPC_URL: rpcUrl!,
    [chain.rpcEnv]: rpcUrl!,
    CHAIN_ID_HOST: String(chain.chainId),
    HARDHAT_NETWORK: chain.network,
    NETWORK: chain.network,
    E2E_COPROCESSOR_CONFIG_FROM_ENV: 'true',
  });

  for (const [name, value] of Object.entries(exports)) {
    const current = process.env[name];
    if (current !== undefined && current !== value && name !== 'NETWORK' && name !== 'HARDHAT_NETWORK') {
      // Values aren't printed: RPC URLs may embed API keys.
      console.error(`Note: --chain ${chainName} overrides the existing ${name}`);
    }
    console.log(`export ${name}=${shellQuote(value)}`);
  }
  for (const name of unsets) {
    if (process.env[name] !== undefined) {
      console.error(`Note: --chain ${chainName} unsets ${name} (${chain.prefix}_${name} is not set)`);
    }
    console.log(`unset ${name}`);
  }
};

main();
