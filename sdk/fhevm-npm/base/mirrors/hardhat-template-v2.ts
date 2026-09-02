export const hardhatTemplateV2PackageKey = './hardhat/v2/fhevm-hardhat-template/pkg';

const identity: Readonly<Record<string, string>> = {
  name: 'fhevm-hardhat-template-v2',
  version: '0.4.2',
  description: 'Hardhat v2 based template for developing FHEVM Solidity smart contracts',
};

const removedDependencies = ['@fhevm/mock-utils', '@zama-fhe/relayer-sdk'] as const;

const addedDependencies: Readonly<Record<string, string>> = {
  '@fhevm/solidity': '^0.13.3',
};

const addedDevDependencies: Readonly<Record<string, string>> = {
  '@fhevm/hardhat-plugin': 'file:../../plugin/pkg',
  '@fhevm/host-contracts-cleartext': 'file:../../../../host-contracts-cleartext/v13/pkg',
  '@fhevm/sdk': '^0.13.3',
};

export type JsonObject = Record<string, unknown>;

export function patchHardhatTemplateV2Manifest(
  source: JsonObject,
  log: (message: string) => void = () => undefined,
): JsonObject {
  const manifest = structuredClone(source);
  for (const [field, value] of Object.entries(identity)) {
    log(`~ ${field.padEnd(33)} ${JSON.stringify(manifest[field])} → ${JSON.stringify(value)}`);
    manifest[field] = value;
  }

  const dependencies = dependencyMap(manifest, 'dependencies');
  const devDependencies = dependencyMap(manifest, 'devDependencies');
  for (const name of removedDependencies) {
    const maps = [dependencies, devDependencies].filter((map) => name in map);
    for (const map of maps) delete map[name];
    log(`${maps.length === 0 ? '·' : '−'} ${name}${maps.length === 0 ? ' (not declared)' : ''}`);
  }
  for (const [target, additions] of [
    [dependencies, addedDependencies],
    [devDependencies, addedDevDependencies],
  ] as const) {
    for (const [name, spec] of Object.entries(additions)) {
      log(`${name in target ? '~' : '+'} ${name.padEnd(33)} ${spec}`);
      target[name] = spec;
    }
  }

  manifest.dependencies = sortKeys(dependencies);
  manifest.devDependencies = sortKeys(devDependencies);
  const scripts = dependencyMap(manifest, 'scripts');
  scripts['check:mirror'] = 'node ../../../fhevm-npm/fhevm-npm.ts check-mirror ./hardhat/v2/fhevm-hardhat-template';
  manifest.scripts = sortKeys(scripts);
  return manifest;
}

function dependencyMap(manifest: JsonObject, field: string): Record<string, string> {
  const value = manifest[field];
  if (value === undefined) return {};
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new Error(`package.json: "${field}" is not an object`);
  }
  const entries = Object.entries(value);
  if (entries.some(([, item]) => typeof item !== 'string')) {
    throw new Error(`package.json: "${field}" contains a non-string value`);
  }
  return Object.fromEntries(entries) as Record<string, string>;
}

function sortKeys(entries: Readonly<Record<string, string>>): Record<string, string> {
  return Object.fromEntries(Object.entries(entries).sort(([left], [right]) => left.localeCompare(right)));
}
