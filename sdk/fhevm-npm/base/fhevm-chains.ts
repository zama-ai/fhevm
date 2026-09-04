// Renders sdk/fhevm-chains.config.json from Zama's protocol registry — the single source of truth for
// every deployed address (see its DEVELOPERS.md). The file lists EVERY fhevm host-contract and gateway
// address on mainnet and testnet, and follows the vendored paradigm: it records the registry commit it
// was rendered from (`source.commit`), `sync-fhevm-chains` writes it at a pin, and
// `check-fhevm-chains-origin` re-fetches that pin, re-renders, and fails on any difference.
//
// Host chains are DISCOVERED, not listed: every registry key matching `[PREFIX_]ACL_HOST` declares one
// (ethereum, polygon, ethereum_sepolia, polygon_amoy, ...), so a new host chain lands here on the next
// sync without touching this module.

import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { format, resolveConfig } from 'prettier';

export const REGISTRY_REPOSITORY = 'https://github.com/zama-ai/protocol-registry-internal';
export const REGISTRY_GITHUB_PATH = 'zama-ai/protocol-registry-internal';
export const CHAINS_CONFIG_FILE = 'fhevm-chains.config.json';

const NETWORKS = ['mainnet', 'testnet'] as const;
type NetworkName = (typeof NETWORKS)[number];

const REGISTRY_FILES: Readonly<Record<NetworkName, string>> = {
  mainnet: 'dist/mainnet.json',
  testnet: 'dist/testnet.json',
};

// Not in the registry: the relayer is Zama infrastructure, not an on-chain address. Decided here.
const RELAYER_URLS: Readonly<Record<NetworkName, string>> = {
  mainnet: 'https://relayer.mainnet.zama.org',
  testnet: 'https://relayer.testnet.zama.org',
};

// Registry name (suffix for hosts) → face key. Alphabetical by face key; emission preserves this order.
const HOST_CONTRACTS = [
  ['ACL_HOST', 'acl', 'required'],
  ['FHEVM_EXECUTOR', 'fhevmExecutor', 'required'],
  ['HCU_LIMIT', 'hcuLimit', 'required'],
  ['INPUT_VERIFIER', 'inputVerifier', 'required'],
  // v13 hosts only — polygon/amoy do not carry it (yet), so absent means omitted, not failed.
  ['KMS_GENERATION_HOST', 'kmsGeneration', 'optional'],
  ['KMS_VERIFIER', 'kmsVerifier', 'required'],
  ['PAUSER_SET_HOST', 'pauserSet', 'required'],
  ['PROTOCOL_CONFIG', 'protocolConfig', 'optional'],
] as const;

const GATEWAY_CONTRACTS = [
  ['CIPHERTEXT_COMMITS', 'ciphertextCommits'],
  ['DECRYPTION', 'decryption'],
  ['GATEWAY_CONFIG', 'gatewayConfig'],
  ['INPUT_VERIFICATION', 'inputVerification'],
  ['KMS_GENERATION', 'kmsGeneration'],
  ['MULTICHAIN_ACL', 'multichainAcl'],
  ['PAUSER_SET_GATEWAY', 'pauserSet'],
] as const;

type RegistryContract = { readonly address?: string; readonly chain?: string };
type Registry = {
  readonly chains?: Record<string, { readonly chain_id?: number }>;
  readonly contracts?: Record<string, RegistryContract>;
};

/** Fetches one registry file's raw text at a commit; `resolveHead` names the default branch's HEAD. */
export type RegistryReader = {
  readonly fetchFile: (path: string, ref: string) => string;
  readonly resolveHead: () => string;
};

export function githubRegistryReader(): RegistryReader {
  const gh = (args: readonly string[]): string => {
    try {
      return execFileSync('gh', [...args], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
    } catch (error) {
      throw new Error(
        `cannot read ${REGISTRY_GITHUB_PATH} via \`gh\` — it is a private repository, so an authenticated ` +
          `GitHub CLI is required (\`gh auth status\`). Underlying error: ${(error as Error).message}`,
      );
    }
  };
  return {
    fetchFile: (path, ref) =>
      gh([
        'api',
        '-H',
        'Accept: application/vnd.github.raw',
        `repos/${REGISTRY_GITHUB_PATH}/contents/${path}?ref=${ref}`,
      ]),
    resolveHead: () => gh(['api', `repos/${REGISTRY_GITHUB_PATH}/commits/HEAD`, '--jq', '.sha']).trim(),
  };
}

export function chainsConfigPath(workspaceRoot: string): string {
  return join(workspaceRoot, CHAINS_CONFIG_FILE);
}

/** The pin recorded in the committed file, or undefined when the file does not exist yet. */
export function pinnedCommit(workspaceRoot: string): string | undefined {
  const path = chainsConfigPath(workspaceRoot);
  if (!existsSync(path)) return undefined;
  const parsed = JSON.parse(readFileSync(path, 'utf8')) as { source?: { commit?: string } };
  const commit = parsed.source?.commit;
  if (typeof commit !== 'string' || !/^[0-9a-f]{40}$/.test(commit)) {
    throw new Error(`${CHAINS_CONFIG_FILE} has no valid source.commit pin — regenerate with --latest`);
  }
  return commit;
}

/**
 * The full file text, prettier-formatted with the destination's own config. Registry files are fetched
 * at `fetchRef`; `sourceCommit` (default: `fetchRef`) is what the `source.commit` field records — the
 * check passes the committed pin there so that comparing against a fresh HEAD render flags ADDRESS
 * drift only, not every unrelated registry commit.
 */
export async function renderChainsConfig(
  workspaceRoot: string,
  reader: RegistryReader,
  fetchRef: string,
  sourceCommit: string = fetchRef,
): Promise<string> {
  const networks = Object.fromEntries(
    NETWORKS.map((network) => {
      const registry = JSON.parse(reader.fetchFile(REGISTRY_FILES[network], fetchRef)) as Registry;
      return [network, renderNetwork(network, registry)];
    }),
  );

  const config = {
    _readme: [
      "AUTO-GENERATED by `fhevm-npm sync-fhevm-chains` from Zama's protocol registry — DO NOT EDIT.",
      '',
      'Every fhevm host-contract and gateway address on mainnet and testnet, rendered from the',
      "registry's dist/{mainnet,testnet}.json. `source.commit` records the registry revision of the",
      'last sync; `fhevm-npm check-fhevm-chains-origin` always checks against the CURRENT head of the',
      "registry's main and fails when the addresses here have fallen behind it — `fhevm-npm",
      'sync-fhevm-chains --latest` catches up. Host chains are discovered from the registry',
      '([PREFIX_]ACL_HOST), so a new host chain appears on the next sync.',
      '',
      'relayerUrl is deliberately NOT from the registry (the relayer is infrastructure, not an',
      'on-chain address); it is decided by the generator. js-sdk chain definitions',
      '(src/core/chains/definitions/*.ts) are to be generated from this file.',
    ],
    source: {
      repository: REGISTRY_REPOSITORY,
      commit: sourceCommit,
      files: Object.values(REGISTRY_FILES),
    },
    networks,
  };
  const path = chainsConfigPath(workspaceRoot);
  const prettierConfig = (await resolveConfig(path)) ?? {};
  return format(JSON.stringify(config), { ...prettierConfig, filepath: path });
}

type AddressFace = { readonly address: string };
type ChainFace = { readonly id: number; readonly contracts: Record<string, AddressFace> };

function renderNetwork(
  network: NetworkName,
  registry: Registry,
): { relayerUrl: string; gateway: ChainFace; hosts: Record<string, ChainFace> } {
  const contracts = registry.contracts ?? {};
  return {
    relayerUrl: RELAYER_URLS[network],
    gateway: renderGateway(network, registry),
    hosts: Object.fromEntries(hostPrefixes(contracts).map((prefix) => renderHost(network, registry, prefix))),
  };
}

function renderGateway(network: NetworkName, registry: Registry): ChainFace {
  const decryption = entry(network, registry, 'DECRYPTION');
  const chainKey = decryption.chain ?? '';
  const faces: Record<string, AddressFace> = {};
  for (const [name, face] of GATEWAY_CONTRACTS) {
    const contract = entry(network, registry, name);
    if (contract.chain !== chainKey) {
      throw new Error(`${network}: ${name} is on chain '${contract.chain ?? ''}', expected gateway '${chainKey}'`);
    }
    faces[face] = { address: address(network, name, contract) };
  }
  return { id: chainId(network, registry, chainKey), contracts: faces };
}

/** '' for the unprefixed host, else the 'POLYGON'/'AMOY'-style prefix, sorted with '' first. */
function hostPrefixes(contracts: Record<string, RegistryContract>): readonly string[] {
  const prefixes = Object.keys(contracts)
    .map((key) => /^(?:([A-Z0-9]+)_)?ACL_HOST$/.exec(key))
    .filter((match) => match !== null)
    .map((match) => match[1] ?? '');
  return [...prefixes].sort();
}

function renderHost(network: NetworkName, registry: Registry, prefix: string): [string, ChainFace] {
  const registryName = (suffix: string): string => (prefix === '' ? suffix : `${prefix}_${suffix}`);
  const chainKey = entry(network, registry, registryName('ACL_HOST')).chain ?? '';
  const faces: Record<string, AddressFace> = {};
  for (const [suffix, face, presence] of HOST_CONTRACTS) {
    const name = registryName(suffix);
    const contract = (registry.contracts ?? {})[name];
    if (contract === undefined) {
      if (presence === 'optional') continue;
      throw new Error(`${network}: registry has no ${name} — required for host chain '${chainKey}'`);
    }
    if (contract.chain !== chainKey) {
      throw new Error(`${network}: ${name} is on chain '${contract.chain ?? ''}', expected '${chainKey}'`);
    }
    faces[face] = { address: address(network, name, contract) };
  }
  return [chainKey, { id: chainId(network, registry, chainKey), contracts: faces }];
}

function entry(network: NetworkName, registry: Registry, name: string): RegistryContract {
  const contract = (registry.contracts ?? {})[name];
  if (contract === undefined) throw new Error(`${network}: registry declares no contract ${name}`);
  return contract;
}

function address(network: NetworkName, name: string, contract: RegistryContract): string {
  const value = contract.address ?? '';
  if (!/^0x[0-9a-fA-F]{40}$/.test(value)) {
    throw new Error(`${network}: ${name} has no plausible EVM address (got '${value}')`);
  }
  return value;
}

function chainId(network: NetworkName, registry: Registry, chainKey: string): number {
  const id = (registry.chains ?? {})[chainKey]?.chain_id;
  if (typeof id !== 'number') throw new Error(`${network}: registry declares no chain_id for '${chainKey}'`);
  return id;
}
