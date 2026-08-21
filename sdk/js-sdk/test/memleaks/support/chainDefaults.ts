import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

// Mirrors test/multi-wasm/support/chainDefaults.ts (rpcUrl lookup only — this
// project doesn't need the mnemonic/fheTestAddress fields that file also
// reads, since setup-ethers.ts's own prepareChains() resolves those).

const chainDefaultsPath = resolve(import.meta.dirname, '../../chains/chain-defaults.json');

export type LocalstackChainDefaults = {
  readonly rpcUrl: string;
};

export function loadLocalstackChainDefaults(chainName: string): LocalstackChainDefaults {
  const json = JSON.parse(readFileSync(chainDefaultsPath, 'utf-8')) as Record<string, { readonly rpcUrl?: string }>;
  const entry = json[chainName];
  if (entry === undefined) {
    throw new Error(`Missing "${chainName}" entry in ${chainDefaultsPath}`);
  }
  if (entry.rpcUrl === undefined || entry.rpcUrl === '') {
    throw new Error(`Missing "${chainName}.rpcUrl" in ${chainDefaultsPath}`);
  }
  return Object.freeze({ rpcUrl: entry.rpcUrl });
}
