import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const chainDefaultsPath = resolve(import.meta.dirname, '../../chains/chain-defaults.json');
const envPath = resolve(import.meta.dirname, '../../.env');

export type LocalstackChainDefaults = {
  readonly rpcUrl: string;
  readonly mnemonic: string;
  readonly fheTestAddress: string;
};

export function loadLocalstackChainDefaults(chainName: string): LocalstackChainDefaults {
  const json = JSON.parse(readFileSync(chainDefaultsPath, 'utf-8')) as Record<
    string,
    { readonly rpcUrl?: string; readonly mnemonic?: string; readonly fheTestAddress?: string }
  >;
  const entry = json[chainName];
  if (entry === undefined) {
    throw new Error(`Missing "${chainName}" entry in ${chainDefaultsPath}`);
  }
  if (entry.rpcUrl === undefined || entry.rpcUrl === '') {
    throw new Error(`Missing "${chainName}.rpcUrl" in ${chainDefaultsPath}`);
  }

  const mnemonic =
    entry.mnemonic !== undefined && entry.mnemonic !== ''
      ? entry.mnemonic
      : (parseEnvFile(envPath).MNEMONIC ?? process.env.MNEMONIC);
  if (mnemonic === undefined || mnemonic === '') {
    throw new Error(
      `Missing mnemonic for "${chainName}" — set "${chainName}.mnemonic" in ${chainDefaultsPath}, MNEMONIC in ${envPath}, or the MNEMONIC env var.`,
    );
  }

  if (entry.fheTestAddress === undefined || entry.fheTestAddress === '') {
    throw new Error(`Missing "${chainName}.fheTestAddress" in ${chainDefaultsPath}`);
  }
  return Object.freeze({
    rpcUrl: entry.rpcUrl,
    mnemonic,
    fheTestAddress: entry.fheTestAddress,
  });
}

function parseEnvFile(filePath: string): Record<string, string> {
  if (!existsSync(filePath)) {
    return {};
  }
  const content = readFileSync(filePath, 'utf-8');
  const result: Record<string, string> = {};
  for (const line of content.split('\n')) {
    const trimmed = line.trim();
    if (trimmed === '' || trimmed.startsWith('#')) {
      continue;
    }
    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) {
      continue;
    }
    const key = trimmed.slice(0, eqIndex).trim();
    let value = trimmed.slice(eqIndex + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    result[key] = value;
  }
  return result;
}
