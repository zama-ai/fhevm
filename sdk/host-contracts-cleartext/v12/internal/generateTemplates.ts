// Turns the forge artifacts in ./out into the committed payload templates under pkg/.

import { existsSync, mkdirSync, readFileSync, rmSync } from 'node:fs';
import { basename, join, relative } from 'node:path';
import { ADDRESS_NAMES, PACKAGE_ROOT_ABS_PATH, PKG_DIR_ABS_PATH, type ContractName } from './constants.ts';
import { normalizeHex, readJson, toJsonLiteral, writeJson, writeTypeScript } from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

export const PATCH_SITES_PATH = join(PACKAGE_ROOT_ABS_PATH, 'internal', 'placeholders', 'patch-sites.json');
export type HexString = `0x${string}`;

export type Artifact = {
  abi: unknown[];
  bytecode: { object: HexString };
  deployedBytecode: { object: HexString };
};

type TargetContract = {
  contractName: ContractName;
  kind: 'proxy' | 'non-proxy';
  sourcePath: string;
};

export type AddressReference = {
  placeholder: HexString;
  bytecodeOffsets: number[];
  deployedBytecodeOffsets: number[];
};

// forge writes to ./out at the harness root; the generated artifacts belong to the payload (./pkg).

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

////////////////////////////////////////////////////////////////////////////////

function _findByteOffsets(bytecode: HexString, placeholder: HexString): number[] {
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

////////////////////////////////////////////////////////////////////////////////

function _parseAddressConfig(): Record<(typeof ADDRESS_NAMES)[number], HexString> {
  // parse <root>/config/addresses.sol
  const configPath = join(PACKAGE_ROOT_ABS_PATH, 'internal', 'placeholders', 'addresses.sol');
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

////////////////////////////////////////////////////////////////////////////////

/** Where `writeTemplates` puts a contract's template, and where consumers read it back from. */
export function templatePathFor(contractName: string): string {
  return join(PKG_DIR_ABS_PATH, 'templates', `${contractName}.json`);
}

////////////////////////////////////////////////////////////////////////////////

export function artifactPathFor(target: TargetContract): string {
  return join(PACKAGE_ROOT_ABS_PATH, 'out', basename(target.sourcePath), `${target.contractName}.json`);
}

////////////////////////////////////////////////////////////////////////////////

function _loadArtifact(target: TargetContract): { artifact: Artifact; artifactPath: string } {
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

////////////////////////////////////////////////////////////////////////////////

function _addressReferencesFor(
  artifact: Artifact,
  addresses: Record<(typeof ADDRESS_NAMES)[number], HexString>,
): Record<string, AddressReference> {
  return Object.fromEntries(
    ADDRESS_NAMES.map((name) => [
      name,
      {
        placeholder: addresses[name],
        bytecodeOffsets: _findByteOffsets(artifact.bytecode.object, addresses[name]),
        deployedBytecodeOffsets: _findByteOffsets(artifact.deployedBytecode.object, addresses[name]),
      },
    ]),
  );
}

////////////////////////////////////////////////////////////////////////////////

async function _writeArtifactTypes(artifactDir: string): Promise<void> {
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

////////////////////////////////////////////////////////////////////////////////

async function _writeArtifactModule(parameters: {
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

////////////////////////////////////////////////////////////////////////////////

/**
 * Committed baseline of how many bytecode sites each placeholder is patched at, per contract.
 *
 * This is a review tripwire, not a semantic check: nothing here knows whether a placeholder *should*
 * be patched (that needs per-contract AST reference resolution). What it does guarantee is that any
 * change to the numbers — an upstream `src/contracts/` sync altering how an address is used, a solc
 * bump, a different optimizer setting — shows up as a diff someone has to look at, instead of silently
 * changing what gets patched at deploy time.
 *
 * A count dropping to 0 for an address the contracts still use is the dangerous case; the deploy-time
 * post-condition in `pkg/ts/utils.ts` (`assertNoPlaceholdersRemain`) is what actually blocks it.
 *
 * Deliberately NOT written by this generator: a baseline that regenerates itself asserts nothing. It is
 * committed, and `test/templates.test.ts` fails when the live counts drift from it.
 */

export function patchSiteCounts(references: Record<string, AddressReference>): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const name of ADDRESS_NAMES) {
    const reference = references[name];
    counts[name] =
      reference === undefined ? 0 : reference.bytecodeOffsets.length + reference.deployedBytecodeOffsets.length;
  }
  return counts;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Rebuilds pkg/abi, pkg/templates and pkg/ts/artifacts from the forge artifacts in ./out.
 *
 * Requires `forge build` to have run — _loadArtifact says so explicitly if it has not.
 */
export async function writeTemplates(): Promise<void> {
  const addresses = _parseAddressConfig();
  const abiDir = join(PKG_DIR_ABS_PATH, 'abi');
  const templateDir = join(PKG_DIR_ABS_PATH, 'templates');
  const tsArtifactDir = join(PKG_DIR_ABS_PATH, 'ts', 'artifacts');

  rmSync(abiDir, { recursive: true, force: true });
  rmSync(templateDir, { recursive: true, force: true });
  rmSync(tsArtifactDir, { recursive: true, force: true });
  mkdirSync(abiDir, { recursive: true });
  mkdirSync(templateDir, { recursive: true });
  mkdirSync(tsArtifactDir, { recursive: true });

  await _writeArtifactTypes(tsArtifactDir);

  for (const target of TARGET_CONTRACTS) {
    const { artifact, artifactPath } = _loadArtifact(target);
    const addressReferences = _addressReferencesFor(artifact, addresses);
    const template = {
      contractName: target.contractName,
      kind: target.kind,
      sourcePath: target.sourcePath,
      artifactPath: relative(PACKAGE_ROOT_ABS_PATH, artifactPath),
      bytecode: artifact.bytecode.object,
      deployedBytecode: artifact.deployedBytecode.object,
      addressReferences,
    };

    writeJson(join(abiDir, `${target.contractName}.json`), artifact.abi);
    writeJson(templatePathFor(target.contractName), template);
    await _writeArtifactModule({ artifactDir: tsArtifactDir, target, artifact, template });
  }
}
