import { Mnemonic, isAddress } from 'ethers';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';

import { PROFILE_SCHEMA_PATH } from '../../infrastructure/paths.js';
import { MIN_SECRET_LENGTH, type SecretValue } from '../../infrastructure/secrets/redactor.js';
import { createYamlValidator } from '../../infrastructure/yaml/yaml-document.js';
import { deriveAccounts } from './accounts.js';
import { parseEnvFile } from './env-file.js';
import {
  BUILTIN_PROFILE,
  type ChainSectionDefinition,
  type ContractsSectionDefinition,
  type ProfileDefinition,
  ProfileError,
  type RelayerSectionDefinition,
  type ResolvedChainSection,
  type ResolvedContractsSection,
  type ResolvedProfile,
  type ResolvedRelayerSection,
  type ResolvedSetting,
  type ResolvedValue,
  type ValueSource,
} from './types.js';

const PROFILE_FILE_SUFFIX = '.yaml';
const validateProfile = createYamlValidator<ProfileDefinition>(PROFILE_SCHEMA_PATH);

export interface ProfileOptions {
  profilesDir: string;
  /** Environment used to resolve `env:` sources (default: `process.env`). */
  env?: NodeJS.ProcessEnv;
}

const BUILTIN: ResolvedProfile = {
  name: BUILTIN_PROFILE,
  description: 'Built-in profile: no environment, no settings.',
  requiresConfirmation: false,
  settings: {},
};

function assertProfilesDir(profilesDir: string): void {
  if (!existsSync(profilesDir)) throw new Error(`Profiles directory not found: ${profilesDir}`);
}

/** Every profile name usable with `--profile` or in `spec.targets`: `none` plus the profile files. */
export function listProfileNames(profilesDir: string): string[] {
  assertProfilesDir(profilesDir);
  const fileNames = readdirSync(profilesDir)
    .filter((file) => file.endsWith(PROFILE_FILE_SUFFIX))
    .map((file) => file.slice(0, -PROFILE_FILE_SUFFIX.length));
  if (fileNames.includes(BUILTIN_PROFILE)) {
    throw new ProfileError(path.join(profilesDir, `${BUILTIN_PROFILE}${PROFILE_FILE_SUFFIX}`), [
      `'${BUILTIN_PROFILE}' is reserved for the built-in profile; remove this file`,
    ]);
  }
  return [BUILTIN_PROFILE, ...fileNames.sort()];
}

/** The value sources of the `chain` section, in display order. */
const CHAIN_SOURCES = ['rpcUrl', 'chainId', 'mnemonic'] as const;
/** The gateway value sources of the `relayer` section, in display order. */
const GATEWAY_SOURCES = ['chainId', 'decryption', 'inputVerification'] as const;
/** The value sources of the `contracts` section, in display order. */
const CONTRACT_SOURCES = ['acl', 'executor', 'inputVerifier', 'kmsVerifier', 'protocolConfig'] as const;

function checkValueSource(where: string, source: ValueSource): string[] {
  const problems: string[] = [];
  const hasValue = source.value !== undefined;
  const hasEnv = source.env !== undefined;
  if (hasValue === hasEnv) problems.push(`${where}: exactly one of 'value' or 'env' is required`);
  if (source.default !== undefined && !hasEnv) problems.push(`${where}: 'default' is only allowed with 'env'`);
  if (source.secret) {
    if (hasValue)
      problems.push(`${where}: a secret cannot have a literal 'value' (secrets never live in the repository)`);
    // An empty default only means "not set": it puts no secret in the repository.
    if (source.default !== undefined && source.default !== '') {
      problems.push(`${where}: a secret cannot have a 'default' other than the empty string`);
    }
  }
  return problems;
}

/**
 * Reads and validates one profile file, without resolving any value.
 *
 * @throws ProfileError listing every problem found.
 */
const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null && !Array.isArray(value);

/**
 * Checks the schema cannot express (file name, value-source combinations). Runs on the
 * parsed document even when the schema failed, guarding every access, so that all problems are
 * reported at once.
 */
function semanticProblems(profilePath: string, document: unknown): string[] {
  const problems: string[] = [];
  const fileName = path.basename(profilePath, PROFILE_FILE_SUFFIX);
  const metadata = isRecord(document) && isRecord(document.metadata) ? document.metadata : {};
  const spec = isRecord(document) && isRecord(document.spec) ? document.spec : {};
  if (fileName === BUILTIN_PROFILE) {
    problems.push(`'${BUILTIN_PROFILE}' is reserved for the built-in profile`);
  } else if (typeof metadata.name === 'string' && metadata.name !== fileName) {
    problems.push(`metadata.name '${metadata.name}' must match the file name '${fileName}'`);
  }
  if (isRecord(spec.chain)) problems.push(...chainProblems(spec.chain));
  if (isRecord(spec.relayer)) problems.push(...relayerProblems(spec.relayer));
  if (isRecord(spec.contracts)) {
    for (const field of CONTRACT_SOURCES) {
      const source = spec.contracts[field];
      if (isRecord(source)) problems.push(...checkValueSource(`spec.contracts.${field}`, source as ValueSource));
    }
  }
  if (isRecord(spec.settings)) {
    for (const [key, source] of Object.entries(spec.settings)) {
      if (isRecord(source)) problems.push(...checkValueSource(`spec.settings.${key}`, source as ValueSource));
    }
  }
  return problems;
}

function chainProblems(chain: Record<string, unknown>): string[] {
  const problems: string[] = [];
  for (const field of CHAIN_SOURCES) {
    const source = chain[field];
    if (isRecord(source)) problems.push(...checkValueSource(`spec.chain.${field}`, source as ValueSource));
  }
  if (isRecord(chain.mnemonic) && chain.mnemonic.secret !== true) {
    problems.push(`spec.chain.mnemonic: must declare 'secret: true'`);
  }
  if (isRecord(chain.chainId) && chain.chainId.secret === true) {
    problems.push(`spec.chain.chainId: cannot be a secret`);
  }
  if (isRecord(chain.wallets)) {
    const roleByIndex = new Map<unknown, string>();
    for (const [role, index] of Object.entries(chain.wallets)) {
      const other = roleByIndex.get(index);
      if (other !== undefined)
        problems.push(`spec.chain.wallets: '${other}' and '${role}' use the same index ${index}`);
      else roleByIndex.set(index, role);
    }
  }
  return problems;
}

function relayerProblems(relayer: Record<string, unknown>): string[] {
  const problems: string[] = [];
  for (const field of ['url', 'apiKey'] as const) {
    const source = relayer[field];
    if (isRecord(source)) problems.push(...checkValueSource(`spec.relayer.${field}`, source as ValueSource));
  }
  if (isRecord(relayer.apiKey) && relayer.apiKey.secret !== true) {
    problems.push(`spec.relayer.apiKey: must declare 'secret: true'`);
  }
  if (isRecord(relayer.gateway)) {
    const gateway = relayer.gateway;
    for (const field of GATEWAY_SOURCES) {
      const source = gateway[field];
      if (isRecord(source)) problems.push(...checkValueSource(`spec.relayer.gateway.${field}`, source as ValueSource));
    }
    if (isRecord(gateway.chainId) && gateway.chainId.secret === true) {
      problems.push(`spec.relayer.gateway.chainId: cannot be a secret`);
    }
  }
  return problems;
}

/**
 * Reads and validates one profile file, without resolving any value.
 *
 * @throws ProfileError listing every problem found.
 */
export function readProfileFile(profilePath: string): ProfileDefinition {
  const result = validateProfile(readFileSync(profilePath, 'utf8'));
  const problems = [...(result.ok ? [] : result.problems), ...semanticProblems(profilePath, result.document)];
  if (problems.length > 0 || !result.ok) throw new ProfileError(profilePath, [...new Set(problems)]);
  return result.document;
}

export interface ProfileListing {
  profiles: ProfileDefinition[];
  errors: ProfileError[];
}

/** Validates every profile file of the directory (no value is resolved). */
export function listProfiles(profilesDir: string): ProfileListing {
  assertProfilesDir(profilesDir);
  const profiles: ProfileDefinition[] = [];
  const errors: ProfileError[] = [];
  const files = readdirSync(profilesDir)
    .filter((file) => file.endsWith(PROFILE_FILE_SUFFIX))
    .sort();
  for (const file of files) {
    try {
      profiles.push(readProfileFile(path.join(profilesDir, file)));
    } catch (error) {
      if (!(error instanceof ProfileError)) throw error;
      errors.push(error);
    }
  }
  return { profiles, errors };
}

interface EnvFileValues {
  path: string;
  values: Map<string, string>;
}

function readEnvFiles(definition: ProfileDefinition, profilePath: string, problems: string[]): EnvFileValues[] {
  const files: EnvFileValues[] = [];
  for (const entry of definition.spec.envFiles) {
    const relative = typeof entry === 'string' ? entry : entry.path;
    const optional = typeof entry === 'string' ? false : entry.optional === true;
    const envPath = path.resolve(path.dirname(profilePath), relative);
    if (!existsSync(envPath)) {
      if (!optional) problems.push(`spec.envFiles: file not found: ${relative}`);
      continue;
    }
    try {
      files.push({ path: envPath, values: parseEnvFile(readFileSync(envPath, 'utf8'), envPath) });
    } catch (error) {
      problems.push(`spec.envFiles: ${(error as Error).message}`);
    }
  }
  return files;
}

function resolveSetting(
  where: string,
  source: ValueSource,
  env: NodeJS.ProcessEnv,
  envFiles: EnvFileValues[],
  problems: string[],
): ResolvedSetting | undefined {
  const secret = source.secret === true;
  if (source.value !== undefined) return { value: source.value, secret, source: 'literal' };

  const name = source.env!;
  let resolved: ResolvedSetting | undefined;
  if (env[name] !== undefined) {
    resolved = { value: env[name], secret, source: `env:${name}` };
  } else {
    const file = envFiles.find((candidate) => candidate.values.has(name));
    if (file) resolved = { value: file.values.get(name)!, secret, source: `env-file:${file.path}:${name}` };
    else if (source.default !== undefined) resolved = { value: source.default, secret, source: 'default' };
  }

  if (!resolved) {
    problems.push(`${where}: environment variable '${name}' is not set`);
    return undefined;
  }
  // An empty secret means "not set" (for example an optional API key): there is nothing to redact.
  const length = String(resolved.value).length;
  if (secret && length > 0 && length < MIN_SECRET_LENGTH) {
    problems.push(`${where}: secret value must be at least ${MIN_SECRET_LENGTH} characters long`);
    return undefined;
  }
  return resolved;
}

type ResolveValue = (where: string, source: ValueSource) => ResolvedSetting | undefined;

/**
 * Resolves the `chain` section and checks the types of its values: a value read from the
 * environment is a string, so types can only be checked once resolved.
 */
function resolveChain(
  chain: ChainSectionDefinition,
  resolve: ResolveValue,
  problems: string[],
): ResolvedChainSection | undefined {
  const before = problems.length;
  const rpcUrl = resolve('spec.chain.rpcUrl', chain.rpcUrl);
  const chainId = resolve('spec.chain.chainId', chain.chainId);
  const mnemonic = resolve('spec.chain.mnemonic', chain.mnemonic);

  if (rpcUrl && !isHttpUrl(String(rpcUrl.value))) {
    problems.push(`spec.chain.rpcUrl: must be an http(s) URL`);
  }
  const chainIdValue = chainId ? toChainId(chainId.value) : undefined;
  if (chainId && chainIdValue === undefined) {
    problems.push(`spec.chain.chainId: must be a positive integer, got '${String(chainId.value)}'`);
  }
  // The message never quotes the mnemonic.
  if (mnemonic && !Mnemonic.isValidMnemonic(String(mnemonic.value))) {
    problems.push(`spec.chain.mnemonic: not a valid BIP-39 mnemonic`);
  }
  if (problems.length > before || !rpcUrl || !chainId || !mnemonic || chainIdValue === undefined) return undefined;

  return {
    rpcUrl: { ...rpcUrl, value: String(rpcUrl.value) },
    chainId: { ...chainId, value: chainIdValue },
    mnemonic: { ...mnemonic, value: String(mnemonic.value) },
    hdPath: chain.hdPath,
    wallets: { ...chain.wallets },
    gas: { ...chain.gas },
  };
}

/** Resolves the `relayer` section and checks the types of its values. */
function resolveRelayer(
  relayer: RelayerSectionDefinition,
  resolve: ResolveValue,
  problems: string[],
): ResolvedRelayerSection | undefined {
  const before = problems.length;
  const url = resolve('spec.relayer.url', relayer.url);
  const apiKey = relayer.apiKey ? resolve('spec.relayer.apiKey', relayer.apiKey) : undefined;
  const chainId = resolve('spec.relayer.gateway.chainId', relayer.gateway.chainId);
  const decryption = resolveAddress('spec.relayer.gateway.decryption', relayer.gateway.decryption, resolve, problems);
  const inputVerification = resolveAddress(
    'spec.relayer.gateway.inputVerification',
    relayer.gateway.inputVerification,
    resolve,
    problems,
  );

  if (url) problems.push(...relayerUrlProblems(String(url.value), String(apiKey?.value ?? '') !== ''));
  const chainIdValue = chainId ? toChainId(chainId.value) : undefined;
  if (chainId && chainIdValue === undefined) {
    problems.push(`spec.relayer.gateway.chainId: must be a positive integer, got '${String(chainId.value)}'`);
  }
  if (problems.length > before || !url || !chainId || chainIdValue === undefined || !decryption || !inputVerification) {
    return undefined;
  }
  return {
    url: { ...url, value: String(url.value) },
    ...(apiKey ? { apiKey: { ...apiKey, value: String(apiKey.value) } } : {}),
    gateway: { chainId: { ...chainId, value: chainIdValue }, decryption, inputVerification },
  };
}

/**
 * The URL rules of the SDK, checked here so that a bad URL stops the run before any scenario
 * instead of failing the first FHE step. The messages never quote the URL (it may be secret).
 */
function relayerUrlProblems(url: string, withApiKey: boolean): string[] {
  let parsed: URL;
  try {
    parsed = new URL(url);
  } catch {
    return [`spec.relayer.url: must be an http(s) URL`];
  }
  if (parsed.protocol !== 'http:' && parsed.protocol !== 'https:') return [`spec.relayer.url: must be an http(s) URL`];
  const problems: string[] = [];
  if (parsed.username !== '' || parsed.password !== '') {
    problems.push(`spec.relayer.url: must not contain credentials (user:pass@host); use relayer.apiKey`);
  }
  if (withApiKey && parsed.protocol === 'http:' && !LOCAL_HOSTS.includes(parsed.hostname)) {
    problems.push(
      `spec.relayer.url: must use https when relayer.apiKey is set (the SDK refuses to send a key over http)`,
    );
  }
  return problems;
}

/** Hosts the SDK accepts over http with an API key. */
const LOCAL_HOSTS = ['localhost', '127.0.0.1', '[::1]'];

/**
 * The relayer base URL the SDK takes: profiles give the URL with the API version (`/v1`, `/v2`), as
 * `test-suite.env` and the e2e suite do.
 */
export const relayerBaseUrl = (url: string): string => url.replace(/\/v[12]\/?$/, '');

/** Resolves the `contracts` section; every value must be an address. */
function resolveContracts(
  contracts: ContractsSectionDefinition,
  resolve: ResolveValue,
  problems: string[],
): ResolvedContractsSection | undefined {
  const before = problems.length;
  const [acl, executor, inputVerifier, kmsVerifier, protocolConfig] = CONTRACT_SOURCES.map((field) =>
    resolveAddress(`spec.contracts.${field}`, contracts[field], resolve, problems),
  );
  if (problems.length > before || !acl || !executor || !inputVerifier || !kmsVerifier || !protocolConfig)
    return undefined;
  return { acl, executor, inputVerifier, kmsVerifier, protocolConfig };
}

/**
 * Resolves a value that must be a 20-byte hex address. A mixed-case address must have a valid
 * checksum: a wrong one usually means a typo.
 */
function resolveAddress(
  where: string,
  source: ValueSource,
  resolve: ResolveValue,
  problems: string[],
): ResolvedValue<string> | undefined {
  const resolved = resolve(where, source);
  if (!resolved) return undefined;
  const value = String(resolved.value);
  if (!isAddress(value)) {
    // A secret value is never quoted (the profile is resolved before any redactor exists).
    const got = resolved.secret ? '' : `, got '${value}'`;
    problems.push(`${where}: must be a 20-byte hex address (with a valid checksum if mixed-case)${got}`);
    return undefined;
  }
  return { ...resolved, value };
}

function isHttpUrl(value: string): boolean {
  try {
    const { protocol } = new URL(value);
    return protocol === 'http:' || protocol === 'https:';
  } catch {
    return false;
  }
}

function toChainId(value: unknown): number | undefined {
  const text = String(value).trim();
  if (typeof value === 'boolean' || !/^[0-9]+$/.test(text)) return undefined;
  const chainId = Number(text);
  return Number.isSafeInteger(chainId) && chainId > 0 ? chainId : undefined;
}

/**
 * Loads, validates and resolves a profile. `none` is built in. Resolution happens once, before
 * any scenario runs, so that a missing variable fails fast. Env files are read in memory only.
 *
 * @throws ProfileError listing every problem found.
 */
export function loadProfile(name: string, options: ProfileOptions): ResolvedProfile {
  if (name === BUILTIN_PROFILE) return structuredClone(BUILTIN);

  const available = listProfileNames(options.profilesDir);
  if (!available.includes(name)) {
    throw new ProfileError(name, [`unknown profile '${name}'. Available: ${available.join(', ')}`]);
  }
  const profilePath = path.join(options.profilesDir, `${name}${PROFILE_FILE_SUFFIX}`);
  const definition = readProfileFile(profilePath);

  const problems: string[] = [];
  const envFiles = readEnvFiles(definition, profilePath, problems);
  const resolve: ResolveValue = (where, source) =>
    resolveSetting(where, source, options.env ?? process.env, envFiles, problems);
  const { chain: chainDefinition, relayer: relayerDefinition, contracts: contractsDefinition } = definition.spec;
  const chain = chainDefinition ? resolveChain(chainDefinition, resolve, problems) : undefined;
  const relayer = relayerDefinition ? resolveRelayer(relayerDefinition, resolve, problems) : undefined;
  const contracts = contractsDefinition ? resolveContracts(contractsDefinition, resolve, problems) : undefined;
  const settings: Record<string, ResolvedSetting> = {};
  for (const [key, source] of Object.entries(definition.spec.settings)) {
    const resolved = resolve(`spec.settings.${key}`, source);
    if (resolved) settings[key] = resolved;
  }
  if (problems.length > 0) throw new ProfileError(profilePath, problems);

  return {
    name: definition.metadata.name,
    description: definition.metadata.description,
    requiresConfirmation: definition.spec.requiresConfirmation,
    ...(chain ? { chain } : {}),
    ...(relayer ? { relayer } : {}),
    ...(contracts ? { contracts } : {}),
    settings,
  };
}

/** The resolved values of the `relayer` section under their display key, in display order. */
function relayerValues(relayer: ResolvedRelayerSection): [string, ResolvedSetting][] {
  return [
    ['relayer.url', relayer.url],
    ...(relayer.apiKey ? ([['relayer.apiKey', relayer.apiKey]] as [string, ResolvedSetting][]) : []),
    ...GATEWAY_SOURCES.map((field): [string, ResolvedSetting] => [`relayer.gateway.${field}`, relayer.gateway[field]]),
  ];
}

/**
 * Every resolved value of a profile under its display key (`chain.<field>`, `relayer.<field>`,
 * `contracts.<field>`, then setting keys).
 */
function valuesOf(profile: ResolvedProfile): [string, ResolvedSetting][] {
  const { chain, relayer, contracts } = profile;
  return [
    ...(chain ? CHAIN_SOURCES.map((field): [string, ResolvedSetting] => [`chain.${field}`, chain[field]]) : []),
    ...(relayer ? relayerValues(relayer) : []),
    ...(contracts
      ? CONTRACT_SOURCES.map((field): [string, ResolvedSetting] => [`contracts.${field}`, contracts[field]])
      : []),
    ...Object.entries(profile.settings),
  ];
}

/**
 * The secret values of a resolved profile (typed sections and settings), for building a redactor.
 * With a `chain` section, the private keys derived for the wallet roles are secrets too
 * (`chain.wallets.<role>`, matched without the `0x` prefix so both forms are redacted).
 */
export function secretsOf(profile: ResolvedProfile): SecretValue[] {
  const secrets = valuesOf(profile)
    // An empty secret means "not set": there is nothing to redact.
    .filter(([, setting]) => setting.secret && String(setting.value) !== '')
    .map(([key, setting]) => ({ key, value: String(setting.value) }));
  // The SDK is given (and logs) the relayer URL without its API version suffix.
  const relayerUrl = profile.relayer?.url;
  if (relayerUrl?.secret) {
    const base = relayerBaseUrl(relayerUrl.value);
    if (base !== relayerUrl.value && base.length >= MIN_SECRET_LENGTH)
      secrets.push({ key: 'relayer.url', value: base });
  }
  if (profile.chain) {
    const { mnemonic, hdPath, wallets } = profile.chain;
    for (const [role, account] of Object.entries(deriveAccounts(mnemonic.value, hdPath, wallets))) {
      secrets.push({ key: `chain.wallets.${role}`, value: account.privateKey.slice(2) });
    }
  }
  return secrets;
}

const redact = <T extends ResolvedSetting>(key: string, setting: T): T =>
  setting.secret && String(setting.value) !== '' ? { ...setting, value: `[REDACTED:${key}]` } : setting;

/** A copy of the `chain` section safe to print or store: secret values replaced by a placeholder. */
export function redactedChain(profile: ResolvedProfile): ResolvedChainSection | undefined {
  if (!profile.chain) return undefined;
  // chainId cannot be a secret (checked when the file is read).
  const { rpcUrl, mnemonic } = profile.chain;
  return {
    ...profile.chain,
    rpcUrl: redact('chain.rpcUrl', rpcUrl),
    mnemonic: redact('chain.mnemonic', mnemonic),
  };
}

/** A copy of the `relayer` section safe to print or store: secret values replaced by a placeholder. */
export function redactedRelayer(profile: ResolvedProfile): ResolvedRelayerSection | undefined {
  if (!profile.relayer) return undefined;
  // gateway.chainId cannot be a secret (checked when the file is read).
  const { url, apiKey, gateway } = profile.relayer;
  return {
    url: redact('relayer.url', url),
    ...(apiKey ? { apiKey: redact('relayer.apiKey', apiKey) } : {}),
    gateway: {
      chainId: gateway.chainId,
      decryption: redact('relayer.gateway.decryption', gateway.decryption),
      inputVerification: redact('relayer.gateway.inputVerification', gateway.inputVerification),
    },
  };
}

/** A copy of the `contracts` section safe to print or store: secret values replaced by a placeholder. */
export function redactedContracts(profile: ResolvedProfile): ResolvedContractsSection | undefined {
  const contracts = profile.contracts;
  if (!contracts) return undefined;
  const [acl, executor, inputVerifier, kmsVerifier, protocolConfig] = CONTRACT_SOURCES.map((field) =>
    redact(`contracts.${field}`, contracts[field]),
  );
  return {
    acl: acl!,
    executor: executor!,
    inputVerifier: inputVerifier!,
    kmsVerifier: kmsVerifier!,
    protocolConfig: protocolConfig!,
  };
}

/** A copy of the settings safe to print or store: secret values replaced by a placeholder. */
export function redactedSettings(profile: ResolvedProfile): Record<string, ResolvedSetting> {
  return Object.fromEntries(Object.entries(profile.settings).map(([key, setting]) => [key, redact(key, setting)]));
}
