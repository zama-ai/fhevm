// Caller-supplied host chain set for the coprocessor-upgrade tasks.
//
// A proposal must cover the same chains as the coprocessors' `host_chains` table. This repo cannot
// read that table, so the set is an input, not a derived value. Entries name an RPC env var, so the
// file itself needs no secret.
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

import type { EnvChainEntry } from './environments';
import { formatError } from './formatError';

// A pod gets config through `envFrom`, where a path does not exist. The prefix keeps the choice at
// the call site, so a stray variable cannot replace the chain set.
export const ENV_SOURCE_PREFIX = 'env:';

export interface ParsedChainSet {
  chains: EnvChainEntry[];
  // From `"canonical": true` on one entry. Undefined keeps the environment's own choice.
  canonicalChainId?: number;
}

export interface ChainSetOverride extends ParsedChainSet {
  // The definitions behind the calldata, with a digest that separates two sets.
  source: string;
}

const ALLOWED_KEYS = new Set([
  'chainId',
  'label',
  'minWindowSeconds',
  'fallbackBlockTimeSeconds',
  'rpcUrlEnv',
  'defaultRpcUrl',
  'canonical',
]);

function fail(origin: string, detail: string): never {
  throw new Error(`${origin}: ${detail}`);
}

function requirePositiveInteger(value: unknown, field: string, origin: string, where: string): number {
  if (typeof value !== 'number' || !Number.isInteger(value) || value <= 0) {
    fail(origin, `${where}.${field} must be an integer greater than 0`);
  }
  return value;
}

function requireNonNegativeNumber(value: unknown, field: string, origin: string, where: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    fail(origin, `${where}.${field} must be a number of seconds that is 0 or more`);
  }
  return value;
}

function requirePositiveNumber(value: unknown, field: string, origin: string, where: string): number {
  if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
    fail(origin, `${where}.${field} must be a number of seconds greater than 0`);
  }
  return value;
}

function requireNonEmptyString(value: unknown, field: string, origin: string, where: string): string {
  if (typeof value !== 'string' || value.trim().length === 0) {
    fail(origin, `${where}.${field} must be a non-empty string`);
  }
  return value;
}

// This set decides the block windows in DAO calldata. A bad field must stop here, not become a
// window that looks correct.
export function parseChainsJson(raw: string, origin: string, environment?: string): ParsedChainSet {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (err) {
    fail(origin, `is not valid JSON (${formatError(err)})`);
  }
  if (!Array.isArray(parsed)) {
    fail(origin, 'must be a JSON array of chain entries');
  }
  if (parsed.length === 0) {
    fail(origin, 'is an empty array; a proposal needs at least one host chain');
  }

  let canonicalChainId: number | undefined;
  const chains = parsed.map((entry, index): EnvChainEntry => {
    const where = `chains[${index}]`;
    if (typeof entry !== 'object' || entry === null || Array.isArray(entry)) {
      fail(origin, `${where} must be an object`);
    }
    const record = entry as Record<string, unknown>;
    // Without this check, a typo in an optional key stays silent.
    const unknownKey = Object.keys(record).find((key) => !ALLOWED_KEYS.has(key));
    if (unknownKey !== undefined) {
      fail(origin, `${where} has unknown key "${unknownKey}"; allowed: ${[...ALLOWED_KEYS].join(', ')}`);
    }
    const chainId = requirePositiveInteger(record.chainId, 'chainId', origin, where);

    if (record.defaultRpcUrl !== undefined) {
      requireNonEmptyString(record.defaultRpcUrl, 'defaultRpcUrl', origin, where);
      // environments.ts keeps defaults off mainnet, so the operator must supply the secret.
      if (environment === 'mainnet') {
        fail(origin, `${where}.defaultRpcUrl is not allowed on mainnet; set ${String(record.rpcUrlEnv)} instead`);
      }
    }
    if (record.canonical !== undefined) {
      if (typeof record.canonical !== 'boolean') {
        fail(origin, `${where}.canonical must be true or false`);
      }
      if (record.canonical) {
        if (canonicalChainId !== undefined) {
          fail(origin, `marks both chain ${canonicalChainId} and chain ${chainId} as canonical; mark exactly one`);
        }
        canonicalChainId = chainId;
      }
    }

    return {
      chainId,
      label: requireNonEmptyString(record.label, 'label', origin, where),
      minWindowSeconds: requireNonNegativeNumber(record.minWindowSeconds, 'minWindowSeconds', origin, where),
      fallbackBlockTimeSeconds: requirePositiveNumber(
        record.fallbackBlockTimeSeconds,
        'fallbackBlockTimeSeconds',
        origin,
        where,
      ),
      rpcUrlEnv: requireNonEmptyString(record.rpcUrlEnv, 'rpcUrlEnv', origin, where),
      defaultRpcUrl: record.defaultRpcUrl as string | undefined,
    };
  });

  const ids = chains.map((c) => c.chainId);
  const duplicate = ids.find((id, index) => ids.indexOf(id) !== index);
  if (duplicate !== undefined) {
    fail(origin, `lists chain ${duplicate} twice (the contract rejects duplicate chain ids)`);
  }
  return { chains, canonicalChainId };
}

export function digest(raw: string): string {
  return createHash('sha256').update(raw).digest('hex').slice(0, 12);
}

// No flag means the environment default.
export function loadChainSetOverride(chainsFile?: string, environment?: string): ChainSetOverride | undefined {
  const arg = chainsFile?.trim();
  if (!arg) {
    return undefined;
  }

  if (arg.startsWith(ENV_SOURCE_PREFIX)) {
    const name = arg.slice(ENV_SOURCE_PREFIX.length).trim();
    if (name.length === 0) {
      throw new Error(
        `--chains-file ${ENV_SOURCE_PREFIX}<NAME> needs an env var name, e.g. ${ENV_SOURCE_PREFIX}HOST_CHAINS`,
      );
    }
    const raw = process.env[name];
    if (!raw || raw.trim().length === 0) {
      throw new Error(`--chains-file names env var ${name}, which is unset or empty`);
    }
    return {
      ...parseChainsJson(raw, `${name} env var`, environment),
      source: `${name} env var (sha256:${digest(raw)})`,
    };
  }

  const path = resolve(arg);
  let raw: string;
  try {
    raw = readFileSync(path, 'utf8');
  } catch (err) {
    throw new Error(`--chains-file ${path} could not be read (${formatError(err)})`);
  }
  return { ...parseChainsJson(raw, `--chains-file ${path}`, environment), source: `${path} (sha256:${digest(raw)})` };
}
