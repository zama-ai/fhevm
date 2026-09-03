// Renders sdk/fhevm-chains.config.json into a TypeScript face: one `export const` table with every
// deployed host-contract and gateway address, nested by NETWORK GROUP then registry host name, plus
// the registry commit it came from. Nested because a host chain recurs across groups (Sepolia is
// served by the testnet AND the devnet gateway) — the group is what selects the gateway and relayer.
// Consumers (the hardhat plugin first) import the face instead of transcribing addresses — a
// transcription is how a stale gateway chain id once made every input proof recover to a junk
// address. Pure: JSON text in, module text out; the command around it owns files.

import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

import {
  CHAINS_CONFIG_FILE,
  GATEWAY_CONTRACTS,
  HOST_CONTRACTS,
  NETWORK_GROUPS,
  type NetworkGroup,
} from './fhevm-chains.ts';

// Where the face lands: common-vendored/src, like the cleartext-config face — `sync-vendored` fans it
// out to the packages that carry a copy (a published payload cannot import the private helper).
export const CHAIN_CONSTANTS_FACE_PATH = ['common-vendored', 'src', 'fhevm-chains.ts'] as const;

export type GeneratedChainConstantsStatus = {
  readonly path: string;
  readonly status: 'identical' | 'missing' | 'different';
};

/** Writes the face (or, with `check`, compares the committed copy against a fresh render). */
export function generateChainConstants(options: {
  readonly workspaceRoot: string;
  readonly check: boolean;
}): GeneratedChainConstantsStatus {
  const configPath = join(options.workspaceRoot, CHAINS_CONFIG_FILE);
  if (!existsSync(configPath)) {
    throw new Error(`${configPath} not found — run \`fhevm-npm sync-fhevm-chains --latest\` to create it.`);
  }
  const path = join(options.workspaceRoot, ...CHAIN_CONSTANTS_FACE_PATH);
  const content = renderChainConstants(parseChainsConfig(readFileSync(configPath, 'utf8')));

  if (options.check) {
    if (!existsSync(path)) return { path, status: 'missing' };
    return { path, status: readFileSync(path, 'utf8') === content ? 'identical' : 'different' };
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content);
  return { path, status: 'identical' };
}

type AddressEntry = { readonly address: string };
type ChainEntry = { readonly id: number; readonly contracts: Readonly<Record<string, AddressEntry>> };
type GroupEntry = {
  readonly relayerUrl: string;
  readonly gateway: ChainEntry;
  readonly hosts: Readonly<Record<string, ChainEntry>>;
};

export type ChainsConfig = {
  readonly sourceCommit: string;
  readonly groups: Readonly<Record<NetworkGroup, GroupEntry>>;
};

const ADDRESS = /^0x[0-9a-fA-F]{40}$/;
const COMMIT = /^[0-9a-f]{40}$/;
const HOST_FACES = HOST_CONTRACTS.map(([, face]) => face);
const REQUIRED_HOST_FACES = HOST_CONTRACTS.filter(([, , presence]) => presence === 'required').map(([, face]) => face);
const GATEWAY_FACES = GATEWAY_CONTRACTS.map(([, face]) => face);
const REQUIRED_GATEWAY_FACES = GATEWAY_CONTRACTS.filter(([, , presence]) => presence === 'required').map(
  ([, face]) => face,
);

/** Parses and validates the config text; every rejection names the offending path. */
export function parseChainsConfig(text: string, fileName: string = CHAINS_CONFIG_FILE): ChainsConfig {
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch (error) {
    throw new Error(`${fileName}: not valid JSON (${(error as Error).message})`);
  }
  const root = asRecord(parsed, fileName);
  const commit = asRecord(root.source, `${fileName}#source`).commit;
  if (typeof commit !== 'string' || !COMMIT.test(commit)) {
    throw new Error(`${fileName}#source.commit: expected a 40-hex registry commit`);
  }
  const networks = asRecord(root.networks, `${fileName}#networks`);
  for (const name of Object.keys(networks)) {
    if (!NETWORK_GROUPS.includes(name as NetworkGroup)) {
      throw new Error(`${fileName}#networks.${name}: unknown network group (known: ${NETWORK_GROUPS.join(', ')})`);
    }
  }
  const entries = NETWORK_GROUPS.map((group) => {
    const entry = networks[group];
    if (entry === undefined) throw new Error(`${fileName}#networks: missing network group '${group}'`);
    return [group, parseGroup(entry, `${fileName}#networks.${group}`)] as const;
  });
  return { sourceCommit: commit, groups: Object.fromEntries(entries) as Record<NetworkGroup, GroupEntry> };
}

// Chain ids are unique WITHIN a group: the same host chain legitimately appears under several gateways.
function parseGroup(value: unknown, path: string): GroupEntry {
  const record = asRecord(value, path);
  const relayerUrl = record.relayerUrl;
  if (typeof relayerUrl !== 'string' || !/^https?:\/\/\S+$/.test(relayerUrl)) {
    throw new Error(`${path}.relayerUrl: expected an http(s) URL`);
  }
  const gateway = parseChain(record.gateway, `${path}.gateway`, GATEWAY_FACES, REQUIRED_GATEWAY_FACES);
  const hosts = asRecord(record.hosts, `${path}.hosts`);
  const hostNames = Object.keys(hosts);
  if (hostNames.length === 0) throw new Error(`${path}.hosts: expected at least one host chain`);
  const seenChainIds = new Map<number, string>();
  const parsedHosts = hostNames.map((name) => {
    const host = parseChain(hosts[name], `${path}.hosts.${name}`, HOST_FACES, REQUIRED_HOST_FACES);
    const previous = seenChainIds.get(host.id);
    if (previous !== undefined) {
      throw new Error(`${path}.hosts.${name}.id: chain id ${String(host.id)} already used by '${previous}'`);
    }
    seenChainIds.set(host.id, name);
    return [name, host] as const;
  });
  return { relayerUrl, gateway, hosts: Object.fromEntries(parsedHosts) };
}

function parseChain(value: unknown, path: string, known: readonly string[], required: readonly string[]): ChainEntry {
  const record = asRecord(value, path);
  const id = record.id;
  if (typeof id !== 'number' || !Number.isSafeInteger(id) || id <= 0) {
    throw new Error(`${path}.id: expected a positive integer chain id`);
  }
  const contracts = asRecord(record.contracts, `${path}.contracts`);
  for (const name of Object.keys(contracts)) {
    if (!known.includes(name)) throw new Error(`${path}.contracts.${name}: unknown contract`);
  }
  for (const name of required) {
    if (contracts[name] === undefined) throw new Error(`${path}.contracts: missing required contract '${name}'`);
  }
  const faces = Object.fromEntries(
    Object.entries(contracts).map(([name, entry]) => {
      const address = asRecord(entry, `${path}.contracts.${name}`).address;
      if (typeof address !== 'string' || !ADDRESS.test(address)) {
        throw new Error(`${path}.contracts.${name}.address: expected a 20-byte hex address`);
      }
      return [name, { address }] as const;
    }),
  );
  return { id, contracts: faces };
}

function asRecord(value: unknown, path: string): Record<string, unknown> {
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new Error(`${path}: expected an object`);
  }
  return value as Record<string, unknown>;
}

////////////////////////////////////////////////////////////////////////////////
// The face
////////////////////////////////////////////////////////////////////////////////

/** The module text. Hand-formatted to what prettier emits, so the committed copy is prettier-clean. */
export function renderChainConstants(config: ChainsConfig): string {
  const groups = NETWORK_GROUPS.map((group) => renderGroup(group, config.groups[group]));
  return [
    FACE_HEADER,
    `export const FHEVM_CHAINS_SOURCE_COMMIT = '${config.sourceCommit}';`,
    '',
    `export type FhevmNetworkGroup = ${NETWORK_GROUPS.map((group) => `'${group}'`).join(' | ')};`,
    '',
    FACE_TYPES,
    `export const FHEVM_CHAINS = {\n${groups.join('\n')}\n} as const satisfies Record<FhevmNetworkGroup, FhevmNetworkGroupConstants>;`,
    '',
  ].join('\n');
}

function renderGroup(group: NetworkGroup, entry: GroupEntry): string {
  const hosts = Object.entries(entry.hosts).flatMap(([name, host]) => renderHost(name, host));
  return [
    `  ${group}: {`,
    `    relayerUrl: '${entry.relayerUrl}',`,
    `    gateway: {`,
    `      id: ${String(entry.gateway.id)},`,
    `      contracts: {`,
    ...renderContracts(entry.gateway.contracts, GATEWAY_FACES, '        '),
    `      },`,
    `    },`,
    `    hosts: {`,
    ...hosts,
    `    },`,
    `  },`,
  ].join('\n');
}

function renderHost(name: string, host: ChainEntry): string[] {
  return [
    `      ${key(name)}: {`,
    `        name: '${name}',`,
    `        id: ${String(host.id)},`,
    `        fhevm: {`,
    `          contracts: {`,
    ...renderContracts(host.contracts, HOST_FACES, '            '),
    `          },`,
    `        },`,
    `      },`,
  ];
}

// Emitted in the tables' order (alphabetical by face), so two renders of the same data are identical.
// `{ address }` objects, the shape the js-sdk chain definitions use.
function renderContracts(
  contracts: Readonly<Record<string, AddressEntry>>,
  order: readonly string[],
  indent: string,
): string[] {
  return order
    .filter((face) => contracts[face] !== undefined)
    .map((face) => `${indent}${face}: { address: '${contracts[face]?.address ?? ''}' },`);
}

function key(name: string): string {
  return /^[A-Za-z_$][A-Za-z0-9_$]*$/.test(name) ? name : `'${name}'`;
}

const FACE_HEADER = `// AUTO-GENERATED by \`fhevm-npm generate-chain-constants\` from sdk/fhevm-chains.config.json — DO NOT EDIT.
//
// Every deployed fhevm host-contract and gateway address, by NETWORK GROUP (one gateway deployment
// and the host chains it serves; the same host chain may appear under several groups) then by the
// protocol registry's host name. The JSON is itself rendered from Zama's protocol registry at
// \`FHEVM_CHAINS_SOURCE_COMMIT\` and kept current by \`fhevm-npm check-fhevm-chains-origin\`; this module
// is import-free so any package can carry a copy. Regenerate with \`fhevm-npm generate-chain-constants\`;
// \`--check\` fails on drift.
`;

const FACE_TYPES = `export type FhevmAddress = \`0x\${string}\`;

/** A deployed contract, the shape the js-sdk chain definitions use. */
export type FhevmChainContract = { readonly address: FhevmAddress };

/** A host chain's contracts. \`kmsGeneration\` and \`protocolConfig\` exist on v13 hosts only. */
export type FhevmHostContracts = {
  readonly acl: FhevmChainContract;
  readonly fhevmExecutor: FhevmChainContract;
  readonly hcuLimit: FhevmChainContract;
  readonly inputVerifier: FhevmChainContract;
  readonly kmsGeneration?: FhevmChainContract;
  readonly kmsVerifier: FhevmChainContract;
  readonly pauserSet: FhevmChainContract;
  readonly protocolConfig?: FhevmChainContract;
};

/** A host chain in the js-sdk shape (id, fhevm.contracts) plus its registry name. Gateway and relayer live on the group. */
export type FhevmHostChainConstants = {
  /** The registry host name: \`ethereum\`, \`polygon\`, \`ethereum_sepolia\`, ... */
  readonly name: string;
  readonly id: number;
  readonly fhevm: { readonly contracts: FhevmHostContracts };
};

export type FhevmGatewayConstants = {
  readonly id: number;
  readonly contracts: {
    readonly ciphertextCommits: FhevmChainContract;
    readonly decryption: FhevmChainContract;
    readonly gatewayConfig: FhevmChainContract;
    readonly inputVerification: FhevmChainContract;
    /** Absent on the devnet gateway, which carries the _LEGACY predecessors instead. */
    readonly kmsGeneration?: FhevmChainContract;
    readonly multichainAcl?: FhevmChainContract;
    readonly pauserSet: FhevmChainContract;
  };
};

/** One gateway deployment, the relayer that serves it, and the host chains it serves. */
export type FhevmNetworkGroupConstants = {
  readonly relayerUrl: string;
  readonly gateway: FhevmGatewayConstants;
  readonly hosts: Readonly<Record<string, FhevmHostChainConstants>>;
};
`;
