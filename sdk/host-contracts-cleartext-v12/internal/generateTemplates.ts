import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { format, resolveConfig } from 'prettier';

export type HexString = `0x${string}`;

export type Artifact = {
  abi: unknown[];
  bytecode: { object: HexString };
  deployedBytecode: { object: HexString };
};

export type TargetContract = {
  contractName: string;
  kind: 'proxy' | 'non-proxy';
  sourcePath: string;
};

export type AddressReference = {
  placeholder: HexString;
  bytecodeOffsets: number[];
  deployedBytecodeOffsets: number[];
};

export const PACKAGE_ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');

export const ADDRESS_NAMES = [
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'INPUT_VERIFIER_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'PAUSER_SET_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
  'CLEARTEXT_DB_ADDRESS',
] as const;

export const TARGET_CONTRACTS: TargetContract[] = [
  {
    contractName: 'EmptyUUPSProxyACL',
    kind: 'non-proxy',
    sourcePath: 'src/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol',
  },
  {
    contractName: 'ERC1967Proxy',
    kind: 'non-proxy',
    sourcePath: 'src/erc1967/ERC1967Proxy.sol',
  },
  {
    contractName: 'EmptyUUPSProxy',
    kind: 'non-proxy',
    sourcePath: 'src/contracts/emptyProxy/EmptyUUPSProxy.sol',
  },
  // Proxies
  // v0.12.0
  { contractName: 'ACL', kind: 'proxy', sourcePath: 'src/contracts/ACL.sol' },
  {
    contractName: 'CleartextFHEVMExecutor',
    kind: 'proxy',
    sourcePath: 'src/cleartext/CleartextFHEVMExecutor.sol',
  },
  {
    contractName: 'CleartextKMSVerifier',
    kind: 'proxy',
    sourcePath: 'src/cleartext/CleartextKMSVerifier.sol',
  },
  {
    contractName: 'CleartextInputVerifier',
    kind: 'proxy',
    sourcePath: 'src/cleartext/CleartextInputVerifier.sol',
  },
  { contractName: 'HCULimit', kind: 'proxy', sourcePath: 'src/contracts/HCULimit.sol' },
  // Cleartext infrastructure (test-stack only)
  { contractName: 'CleartextArithmetic', kind: 'proxy', sourcePath: 'src/cleartext/CleartextArithmetic.sol' },
  { contractName: 'CleartextDB', kind: 'proxy', sourcePath: 'src/cleartext/CleartextDB.sol' },
  // Others
  { contractName: 'PauserSet', kind: 'non-proxy', sourcePath: 'src/contracts/immutable/PauserSet.sol' },
  { contractName: 'ACLOwner', kind: 'non-proxy', sourcePath: 'src/upgrade/ACLOwner.sol' },
];

export function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, 'utf8')) as T;
}

function writeJson(path: string, value: unknown): void {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function toJsonLiteral(value: unknown): string {
  return JSON.stringify(value, null, 2);
}

async function writeTypeScript(path: string, source: string): Promise<void> {
  writeFileSync(
    path,
    await format(source, { ...((await resolveConfig(path)) ?? {}), filepath: path, parser: 'typescript' }),
  );
}

export function normalizeHex(value: string, label: string): string {
  if (!/^0x[0-9a-fA-F]*$/.test(value)) {
    throw new Error(`${label} is not a hex string`);
  }

  const hex = value.slice(2).toLowerCase();
  if (hex.length % 2 !== 0) {
    throw new Error(`${label} has an odd hex length`);
  }

  return hex;
}

export function findByteOffsets(bytecode: HexString, placeholder: HexString): number[] {
  const haystack = normalizeHex(bytecode, 'bytecode');
  const needle = normalizeHex(placeholder, 'placeholder address');
  const offsets: number[] = [];

  for (let index = haystack.indexOf(needle); index !== -1; index = haystack.indexOf(needle, index + needle.length)) {
    if (index % 2 !== 0) {
      throw new Error(`Address placeholder ${placeholder} was found at a non-byte-aligned offset`);
    }
    offsets.push(index / 2);
  }

  return offsets;
}

export function parseAddressConfig(): Record<(typeof ADDRESS_NAMES)[number], HexString> {
  // parse <root>/config/addresses.sol
  const configPath = join(PACKAGE_ROOT, 'config', 'addresses.sol');
  const source = readFileSync(configPath, 'utf8');
  const found = new Map<string, HexString>();
  const addressConstant = /address\s+constant\s+([A-Z0-9_]+)\s*=\s*address\(0x([0-9a-fA-F]{40})\);/g;

  for (const match of source.matchAll(addressConstant)) {
    const [, name, rawAddress] = match;
    if (name === undefined || rawAddress === undefined) {
      throw new Error(`Invalid address constant match in ${relative(process.cwd(), configPath)}`);
    }

    found.set(name, `0x${rawAddress.toLowerCase()}`);
  }

  const addresses = {} as Record<(typeof ADDRESS_NAMES)[number], HexString>;
  const seen = new Map<string, string>();

  for (const name of ADDRESS_NAMES) {
    const address = found.get(name);
    if (address === undefined) {
      throw new Error(`Missing dummy address constant ${name} in ${relative(process.cwd(), configPath)}`);
    }

    const existingName = seen.get(address);
    if (existingName !== undefined) {
      throw new Error(`Dummy address ${address} is reused by ${existingName} and ${name}`);
    }

    seen.set(address, name);
    addresses[name] = address;
  }

  return addresses;
}

export function artifactPathFor(target: TargetContract): string {
  return join(PACKAGE_ROOT, 'out', basename(target.sourcePath), `${target.contractName}.json`);
}

export function loadArtifact(target: TargetContract): { artifact: Artifact; artifactPath: string } {
  const artifactPath = artifactPathFor(target);
  if (!existsSync(artifactPath)) {
    throw new Error(`Missing artifact ${relative(process.cwd(), artifactPath)}. Run forge build first.`);
  }

  const artifact = readJson<Artifact>(artifactPath);
  if (!Array.isArray(artifact.abi)) {
    throw new Error(`${relative(process.cwd(), artifactPath)} does not contain an ABI array`);
  }
  normalizeHex(artifact.bytecode.object, `${target.contractName}.bytecode.object`);
  normalizeHex(artifact.deployedBytecode.object, `${target.contractName}.deployedBytecode.object`);

  return { artifact, artifactPath };
}

export function addressReferencesFor(
  artifact: Artifact,
  addresses: Record<(typeof ADDRESS_NAMES)[number], HexString>,
): Record<string, AddressReference> {
  return Object.fromEntries(
    ADDRESS_NAMES.map((name) => [
      name,
      {
        placeholder: addresses[name],
        bytecodeOffsets: findByteOffsets(artifact.bytecode.object, addresses[name]),
        deployedBytecodeOffsets: findByteOffsets(artifact.deployedBytecode.object, addresses[name]),
      },
    ]),
  );
}

async function writeArtifactTypes(artifactDir: string): Promise<void> {
  await writeTypeScript(
    join(artifactDir, 'types.ts'),
    `// This file is generated by internal/generateTemplates.ts. Do not edit manually.

export type HexString = \`0x\${string}\`;

export type ContractKind = 'proxy' | 'non-proxy';

export type AddressReference = {
  readonly placeholder: HexString;
  readonly bytecodeOffsets: readonly number[];
  readonly deployedBytecodeOffsets: readonly number[];
};

export type ContractTemplate = {
  readonly contractName: string;
  readonly kind: ContractKind;
  readonly sourcePath: string;
  readonly artifactPath: string;
  readonly bytecode: HexString;
  readonly deployedBytecode: HexString;
  readonly addressReferences: Readonly<Record<string, AddressReference>>;
};
`,
  );
}

async function writeArtifactModule(parameters: {
  artifactDir: string;
  target: TargetContract;
  artifact: Artifact;
  template: unknown;
}): Promise<void> {
  await writeTypeScript(
    join(parameters.artifactDir, `${parameters.target.contractName}.ts`),
    `// This file is generated by internal/generateTemplates.ts. Do not edit manually.

import type { ContractTemplate } from './types.js';

export const abi: readonly unknown[] = ${toJsonLiteral(parameters.artifact.abi)};

export const template: ContractTemplate = ${toJsonLiteral(parameters.template)};
`,
  );
}

export async function main(): Promise<void> {
  const addresses = parseAddressConfig();
  const abiDir = join(PACKAGE_ROOT, 'abi');
  const templateDir = join(PACKAGE_ROOT, 'templates');
  const tsArtifactDir = join(PACKAGE_ROOT, 'ts', 'artifacts');

  rmSync(abiDir, { recursive: true, force: true });
  rmSync(templateDir, { recursive: true, force: true });
  rmSync(tsArtifactDir, { recursive: true, force: true });
  mkdirSync(abiDir, { recursive: true });
  mkdirSync(templateDir, { recursive: true });
  mkdirSync(tsArtifactDir, { recursive: true });

  await writeArtifactTypes(tsArtifactDir);

  for (const target of TARGET_CONTRACTS) {
    const { artifact, artifactPath } = loadArtifact(target);
    const addressReferences = addressReferencesFor(artifact, addresses);
    const template = {
      contractName: target.contractName,
      kind: target.kind,
      sourcePath: target.sourcePath,
      artifactPath: relative(PACKAGE_ROOT, artifactPath),
      bytecode: artifact.bytecode.object,
      deployedBytecode: artifact.deployedBytecode.object,
      addressReferences,
    };

    writeJson(join(abiDir, `${target.contractName}.json`), artifact.abi);
    writeJson(join(templateDir, `${target.contractName}.json`), template);
    await writeArtifactModule({ artifactDir: tsArtifactDir, target, artifact, template });
  }
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  await main();
}
