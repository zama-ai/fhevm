import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';

import { PROFILE_SCHEMA_PATH } from '../paths.js';
import { createYamlValidator } from '../schema/yaml-document.js';
import { MIN_SECRET_LENGTH, type SecretValue } from '../secrets/redactor.js';
import { parseEnvFile } from './env-file.js';
import {
  BUILTIN_PROFILE,
  type ProfileDefinition,
  ProfileError,
  type ResolvedProfile,
  type ResolvedSetting,
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

function checkValueSource(key: string, source: ValueSource): string[] {
  const problems: string[] = [];
  const where = `spec.settings.${key}`;
  const hasValue = source.value !== undefined;
  const hasEnv = source.env !== undefined;
  if (hasValue === hasEnv) problems.push(`${where}: exactly one of 'value' or 'env' is required`);
  if (source.default !== undefined && !hasEnv) problems.push(`${where}: 'default' is only allowed with 'env'`);
  if (source.secret) {
    if (hasValue)
      problems.push(`${where}: a secret cannot have a literal 'value' (secrets never live in the repository)`);
    if (source.default !== undefined) problems.push(`${where}: a secret cannot have a 'default'`);
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
  if (isRecord(spec.settings)) {
    for (const [key, source] of Object.entries(spec.settings)) {
      if (isRecord(source)) problems.push(...checkValueSource(key, source as ValueSource));
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
  key: string,
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
    problems.push(`spec.settings.${key}: environment variable '${name}' is not set`);
    return undefined;
  }
  if (secret && String(resolved.value).length < MIN_SECRET_LENGTH) {
    problems.push(`spec.settings.${key}: secret value must be at least ${MIN_SECRET_LENGTH} characters long`);
    return undefined;
  }
  return resolved;
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
  const settings: Record<string, ResolvedSetting> = {};
  for (const [key, source] of Object.entries(definition.spec.settings)) {
    const resolved = resolveSetting(key, source, options.env ?? process.env, envFiles, problems);
    if (resolved) settings[key] = resolved;
  }
  if (problems.length > 0) throw new ProfileError(profilePath, problems);

  return {
    name: definition.metadata.name,
    description: definition.metadata.description,
    requiresConfirmation: definition.spec.requiresConfirmation,
    settings,
  };
}

/** The secret values of a resolved profile, for building a redactor. */
export function secretsOf(profile: ResolvedProfile): SecretValue[] {
  return Object.entries(profile.settings)
    .filter(([, setting]) => setting.secret)
    .map(([key, setting]) => ({ key, value: String(setting.value) }));
}

/** A copy of the settings safe to print or store: secret values replaced by a placeholder. */
export function redactedSettings(profile: ResolvedProfile): Record<string, ResolvedSetting> {
  return Object.fromEntries(
    Object.entries(profile.settings).map(([key, setting]) => [
      key,
      setting.secret ? { ...setting, value: `[REDACTED:${key}]` } : setting,
    ]),
  );
}
