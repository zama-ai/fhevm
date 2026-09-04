import { execFileSync } from 'node:child_process';

/**
 * The one place this repo asks Foundry anything.
 *
 * Every answer comes from `forge config --json` and nothing else. No reading a `foundry.toml`, no
 * regexing it, no assuming a default. Forge is the only thing that resolves `extends`, profile
 * selection and its own built-in defaults, and a hand-rolled parser disagrees with it silently — which
 * is the expensive kind of wrong. `hardhat/v2/e2e` is the standing example: its cache is `cache-forge`,
 * not `cache`, so anything that assumed the default name was already wrong about one of three projects.
 */
export type ForgeConfig = Readonly<Record<string, unknown>>;

/** Injectable so tests never shell out. */
export type ForgeConfigReader = (directory: string, configFile?: string) => ForgeConfig;

export function readForgeConfig(directory: string, configFile?: string): ForgeConfig {
  const output = execFileSync('forge', ['config', '--json'], {
    cwd: directory,
    encoding: 'utf8',
    // FOUNDRY_CONFIG points forge at a specific file, which is how the shared `foundry.base.toml`
    // gets resolved despite not being a project root of its own.
    env: configFile === undefined ? process.env : { ...process.env, FOUNDRY_CONFIG: configFile },
  });
  const parsed: unknown = JSON.parse(output);
  if (!isRecord(parsed)) throw new Error(`forge config --json did not return an object for '${directory}'`);
  return parsed;
}

/** Memoized: a single `check-vendored-origin` run would otherwise spawn forge dozens of times. */
export function memoizedForgeConfigReader(read: ForgeConfigReader = readForgeConfig): ForgeConfigReader {
  const cache = new Map<string, ForgeConfig>();
  return (directory, configFile) => {
    const key = `${directory}\0${configFile ?? ''}`;
    const hit = cache.get(key);
    if (hit !== undefined) return hit;
    const value = read(directory, configFile);
    cache.set(key, value);
    return value;
  };
}

/**
 * The directories a Forge project writes and a `clean` must therefore remove, resolved rather than
 * guessed. `broadcast` is included: a script run leaves receipts there, and they are stale the moment
 * the contracts change.
 */
export function forgeArtifactDirectories(config: ForgeConfig): readonly string[] {
  const directories = ['cache_path', 'out', 'broadcast']
    .map((key) => config[key])
    .filter((value): value is string => typeof value === 'string' && value.length > 0);
  return [...new Set(directories)];
}

/** The `[fmt]` block as forge resolved it. */
export function forgeFmtSettings(config: ForgeConfig): ReadonlyMap<string, string> {
  const fmt = config.fmt;
  if (!isRecord(fmt)) return new Map();
  return new Map(Object.entries(fmt).map(([key, value]) => [key, JSON.stringify(value)]));
}

export function forgeDependencyVersions(config: ForgeConfig): Readonly<Record<string, string>> | undefined {
  if (!isRecord(config.dependencies)) return undefined;
  const dependencies: Record<string, string> = {};
  for (const [name, version] of Object.entries(config.dependencies)) {
    if (typeof version !== 'string') return undefined;
    dependencies[name] = version;
  }
  return dependencies;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
