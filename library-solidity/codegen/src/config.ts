import type { Command, OptionValues } from 'commander';
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { createRequire } from 'module';
import * as path from 'path';
import * as prettier from 'prettier';

import { assertRelative } from './paths.js';

export type UserSolidityTestGroup = {
  outDir: string;
  parentContractName?: string;
  imports?: Array<string | [string, string]>;
};

/**
 * Hardhat version the generated TypeScript tests target. 2 (the default) keeps the HH2 output unchanged; 3 generates
 * ESM tests for HH3 (see `generateTypeScriptTestCode`).
 */
export type HardhatVersion = 2 | 3;

export type UserTypescriptTestGroup = {
  outDir: string;
  imports?: Array<string | [string, string]>;
  hardhatVersion?: HardhatVersion;
};

export type TestGroup = {
  outDir: string;
  imports?: Array<string | [string, string]>;
};

export type SolidityTestGroup = TestGroup & {
  parentContractName?: string;
};

export type TypescriptTestGroup = TestGroup & {
  hardhatVersion: HardhatVersion;
};

export type UserConfig = {
  baseDir?: string;
  noLib?: boolean;
  noHostContracts?: boolean;
  noTest?: boolean;
  lib?: LibUserConfig;
  hostContracts?: HostContractsUserConfig;
  tests: TestsUserConfig;
};

export type ResolvedConfig = {
  baseDir: string;
  noLib: boolean;
  noHostContracts: boolean;
  noTest: boolean;
  lib: LibConfig;
  hostContracts: HostContractsConfig;
  tests: TestsConfig;
};

export type HostContractsUserConfig = {
  outDir?: string;
  generateHCULimit?: boolean;
};

export type LibUserConfig = {
  outDir?: string;
  fheTypeDir?: string;
  // whether to generate the confidential-bridge OApp abstracts under `lib/bridge/`
  // (ConfidentialOAppCore/Sender/Receiver/ConfidentialOApp + IDstApp). Library-only;
  // host-contracts leaves this off so it does not emit the OApp abstracts.
  bridge?: boolean;
};

export type TestsUserConfig = {
  overloads?: string;
  publicDecrypt?: boolean;
  numberOfTestSplits?: number;
  shuffle?: boolean;
  shuffleWithPseuseRand?: boolean;
  solidity?: UserSolidityTestGroup;
  typescript?: UserTypescriptTestGroup;
};

export type HostContractsConfig = {
  outDir: string;
};

export type LibConfig = {
  // directory where the FHE.sol and Impl.sol files are located
  outDir: string;
  // directory where the FheType.sol file is located
  fheTypeDir: string;
  // whether to generate the confidential-bridge OApp abstracts under `lib/bridge/`
  bridge: boolean;
};

export type TestsConfig = {
  overloads: string;
  publicDecrypt: boolean;
  numberOfTestSplits: number;
  shuffle: boolean;
  shuffleWithPseuseRand: boolean;
  solidity?: SolidityTestGroup;
  typescript?: TypescriptTestGroup;
};

export function getProgram(): Command {
  return (globalThis as unknown as { program: Command }).program;
}

export function getOptions(): OptionValues {
  return getProgram().opts();
}

export function getUserConfig(): UserConfig | undefined {
  // commander types option values as `any`: `--config <path>` gives a string when set.
  const p: unknown = getProgram().opts().config;
  if (typeof p === 'string' && p !== '') {
    const jsonFile = toAbsoluteFileWithExtension(p, '.json', process.cwd());
    if (!existsSync(jsonFile)) {
      errorLog(`Codegen config file at ${jsonFile} does not exist.`);
      process.exit(1);
    }
    const json = readFileSync(jsonFile, 'utf8');
    debugLog(`Load config file: ${jsonFile}`);

    // An invalid config stops the run: continuing with default settings could generate files in the wrong place.
    let o: unknown;
    try {
      o = JSON.parse(json);
    } catch (e) {
      errorLog(`Invalid codegen config ${jsonFile}: not valid JSON. ${String(e)}`);
      process.exit(1);
    }
    if (typeof o !== 'object' || o === null || Array.isArray(o)) {
      errorLog(`Invalid codegen config ${jsonFile}: expected a JSON object.`);
      process.exit(1);
    }
    const userConfig = o as UserConfig;

    // Runtime check: the config is hand-written JSON, so `baseDir` may not match its static type.
    const baseDir: unknown = userConfig.baseDir;
    if (baseDir === undefined) {
      // If baseDir is not specified, use config file containing directory as baseDir
      userConfig.baseDir = path.dirname(jsonFile);
    } else if (typeof baseDir !== 'string' || baseDir.trim() === '') {
      errorLog(
        `Invalid codegen config ${jsonFile}: "baseDir" must be a non-empty string, got ${JSON.stringify(baseDir)}. ` +
          `Remove it to use the config file's directory.`,
      );
      process.exit(1);
    } else {
      // Like the paths in a tsconfig.json, a relative baseDir is resolved against the directory of the config file
      // that declares it, not against the current working directory: the result does not depend on where codegen
      // is run from. An absolute baseDir is kept (normalized).
      userConfig.baseDir = path.resolve(path.dirname(jsonFile), baseDir);
    }
    return userConfig;
  }
  return undefined;
}

export function isVerbose(): boolean {
  return getProgram().opts().verbose === true || isDryRun();
}

export function isDryRun(): boolean {
  return getProgram().opts().dryRun === true;
}

// Options of the `lib` and `overloads` commands, as declared in codegen.mjs. commander does not type them (its
// option values are `any`), so keep these types in sync with codegen.mjs by hand.

/** `lib` command: `--overloads <path>`, `--no-test`, `--no-lib` (a `--no-x` flag makes `x` default to `true`). */
export type LibCommandOptions = {
  overloads?: string;
  test: boolean;
  lib: boolean;
};

/** `overloads` command: `--force`, `--update`. */
export type OverloadsCommandOptions = {
  force?: boolean;
  update?: boolean;
};

export function getUserOverloadsFile(options: LibCommandOptions): string | undefined {
  return options.overloads;
}

export function debugLog(s: string): void {
  if (isVerbose()) {
    console.log(s);
  }
}

export function errorLog(s: string): void {
  const RED_COLOR = '\x1b[31m';
  const RESET_COLOR = '\x1b[0m';
  console.error(RED_COLOR + s + RESET_COLOR);
}

export function debugLogDirectoriesUserConfig(config: HostContractsUserConfig): void {
  debugLog(JSON.stringify(config, null, 2));
}

function toAbsoluteDirectory(dir: string, baseDir: string): string {
  if (!path.isAbsolute(baseDir)) {
    throw new Error(`Path ${baseDir} is not absolute.`);
  }
  return path.join(baseDir, dir);
}

export function toAbsoluteFileWithExtension(filePath: string, expectedFileExt: string, baseDir: string): string {
  const f = path.parse(filePath);
  if (f.ext !== expectedFileExt) {
    throw new Error(`Invalid ${expectedFileExt} file name: ${filePath}. Missing "${expectedFileExt}" extension.`);
  }
  if (path.isAbsolute(filePath)) {
    return filePath;
  }
  return path.join(baseDir, filePath);
}

function resolveTestsUserConfig(testsUserConfig: TestsUserConfig | undefined): TestsConfig {
  return {
    overloads: testsUserConfig?.overloads ?? './overloads.json',
    numberOfTestSplits: testsUserConfig?.numberOfTestSplits ?? 12,
    publicDecrypt: testsUserConfig?.publicDecrypt === true,
    shuffle: testsUserConfig?.shuffle === true,
    shuffleWithPseuseRand: testsUserConfig?.shuffleWithPseuseRand === true,
    ...(testsUserConfig?.solidity ? { solidity: resolveSolidityTestGroup(testsUserConfig.solidity) } : {}),
    ...(testsUserConfig?.typescript ? { typescript: resolveTypescriptTestGroup(testsUserConfig.typescript) } : {}),
  };
}

export function resolveUserConfig(userConfig: UserConfig | undefined): ResolvedConfig {
  return {
    baseDir: resolveBaseDirectory(userConfig?.baseDir),
    noLib: userConfig?.noLib === true,
    noHostContracts: userConfig?.noHostContracts === true,
    noTest: userConfig?.noTest === true,
    hostContracts: resolveHostContractsConfig(userConfig?.hostContracts),
    tests: resolveTestsUserConfig(userConfig?.tests),
    lib: resolveLibConfig(userConfig?.lib),
  };
}

function resolveBaseDirectory(baseDir: string | undefined): string {
  baseDir = baseDir ?? process.cwd();
  if (!path.isAbsolute(baseDir)) {
    baseDir = path.normalize(path.join(process.cwd(), baseDir));
  }
  return baseDir;
}

function resolveHostContractsConfig(userConfig: HostContractsUserConfig | undefined): HostContractsConfig {
  const p: HostContractsConfig = {
    outDir: userConfig?.outDir ?? './contracts',
  };
  assertRelative(p.outDir);
  return p;
}

function resolveLibConfig(userLibConfig: LibUserConfig | undefined): LibConfig {
  const outDir = userLibConfig?.outDir ?? './lib';
  const p: LibConfig = {
    fheTypeDir: userLibConfig?.fheTypeDir ?? outDir,
    outDir,
    bridge: userLibConfig?.bridge ?? false,
  };
  assertRelative(p.fheTypeDir);
  assertRelative(p.outDir);
  return p;
}

function resolveSolidityTestGroup(userTestGroup: UserSolidityTestGroup): SolidityTestGroup {
  // Runtime check: the config is hand-written JSON, so the value may not match its static type. When present,
  // `parentContractName` must be a non-empty string (the Solidity contract the generated test contracts inherit from).
  const parentContractName: unknown = userTestGroup.parentContractName;
  if (
    parentContractName !== undefined &&
    (typeof parentContractName !== 'string' || parentContractName.trim() === '')
  ) {
    throw new Error(
      `Invalid codegen config: "tests.solidity.parentContractName" must be a non-empty string, got ` +
        `${JSON.stringify(parentContractName)}. Remove it to generate test contracts without a parent contract.`,
    );
  }
  return {
    outDir: userTestGroup.outDir,
    ...(typeof parentContractName === 'string' ? { parentContractName } : {}),
    ...(userTestGroup.imports ? { imports: userTestGroup.imports } : {}),
  };
}

function resolveTypescriptTestGroup(userTestGroup: UserTypescriptTestGroup): TypescriptTestGroup {
  // Runtime check: the config is hand-written JSON, so the value may not match its static type.
  const hardhatVersion: unknown = userTestGroup.hardhatVersion ?? 2;
  if (hardhatVersion !== 2 && hardhatVersion !== 3) {
    throw new Error(
      `Invalid codegen config: "tests.typescript.hardhatVersion" must be 2 or 3, got ${JSON.stringify(hardhatVersion)}.`,
    );
  }
  return {
    outDir: userTestGroup.outDir,
    ...(userTestGroup.imports ? { imports: userTestGroup.imports } : {}),
    hardhatVersion,
  };
}

function toAbsoluteTestsConfig(resolvedTestsConfig: TestsConfig, baseDir: string): TestsConfig {
  return {
    overloads: toAbsoluteFileWithExtension(resolvedTestsConfig.overloads, '.json', baseDir),
    publicDecrypt: resolvedTestsConfig.publicDecrypt,
    numberOfTestSplits: resolvedTestsConfig.numberOfTestSplits,
    shuffle: resolvedTestsConfig.shuffle,
    shuffleWithPseuseRand: resolvedTestsConfig.shuffleWithPseuseRand,
    ...(resolvedTestsConfig.solidity ? { solidity: toAbsoluteTestGroup(resolvedTestsConfig.solidity, baseDir) } : {}),
    ...(resolvedTestsConfig.typescript
      ? { typescript: toAbsoluteTestGroup(resolvedTestsConfig.typescript, baseDir) }
      : {}),
  };
}

export function toAbsulteConfig(resolved: ResolvedConfig): ResolvedConfig {
  const hostContracts = toAbsoluteHostContractsConfig(resolved.hostContracts, resolved.baseDir);
  const tests = toAbsoluteTestsConfig(resolved.tests, resolved.baseDir);
  const lib = toAbsoluteLibConfig(resolved.lib, resolved.baseDir);

  return {
    baseDir: resolved.baseDir,
    noLib: resolved.noLib,
    noHostContracts: resolved.noHostContracts,
    noTest: resolved.noTest,
    hostContracts,
    tests,
    lib,
  };
}

function toAbsoluteHostContractsConfig(resolved: HostContractsConfig, baseDir: string): HostContractsConfig {
  return {
    outDir: toAbsoluteDirectory(resolved.outDir, baseDir),
  };
}

function toAbsoluteLibConfig(resolved: LibConfig, baseDir: string): LibConfig {
  return {
    outDir: toAbsoluteDirectory(resolved.outDir, baseDir),
    fheTypeDir: toAbsoluteDirectory(resolved.fheTypeDir, baseDir),
    bridge: resolved.bridge,
  };
}

// Returns a copy of `testGroup` with an absolute `outDir`. All other fields (e.g. a Solidity group's
// `parentContractName`) are kept as they are.
function toAbsoluteTestGroup<T extends TestGroup>(testGroup: T, baseDir: string): T {
  return {
    ...testGroup,
    outDir: toAbsoluteDirectory(testGroup.outDir, baseDir),
  };
}

export function mkDir(dir: string): void {
  if (!path.isAbsolute(dir)) {
    throw new Error(`Path ${dir} is not absolute.`);
  }
  if (!existsSync(dir)) {
    if (!isDryRun()) {
      debugLog(`mkdir -p ${dir}`);
      mkdirSync(dir, { recursive: true });
    } else {
      debugLog(`[dry-run] Skip create directory ${dir}`);
    }
  }
}

function prettierConfigExists(baseDir: string): boolean {
  return existsSync(path.join(baseDir, '.prettierrc.yml')) || existsSync(path.join(baseDir, '.prettierrc.json'));
}

// Upper bound on the number of directories walked by `prettierConfigExistsInTree`. Reaching the
// filesystem root normally stops the walk long before this; the bound only guards against an
// endless loop.
const MAX_PRETTIER_CONFIG_SEARCH_DEPTH = 64;

// Prettier resolves its config by walking up from the formatted file, so a config in any
// ancestor directory (e.g. the repo root `.prettierrc.yml`) applies to `baseDir` as well.
function prettierConfigExistsInTree(baseDir: string): boolean {
  let dir = path.resolve(baseDir);
  for (let depth = 0; depth < MAX_PRETTIER_CONFIG_SEARCH_DEPTH; ++depth) {
    if (prettierConfigExists(dir)) {
      return true;
    }
    const parent = path.dirname(dir);
    if (parent === dir) {
      // Reached the filesystem root without finding a config.
      return false;
    }
    dir = parent;
  }
  throw new Error(
    `Prettier config search from ${baseDir} did not reach the filesystem root within ${MAX_PRETTIER_CONFIG_SEARCH_DEPTH} directories.`,
  );
}

export function generatePrettierConfig(baseDir: string): void {
  if (!isDryRun()) {
    if (!prettierConfigExistsInTree(baseDir)) {
      const prettierConfigFile = path.resolve('./.prettierrc.json');

      if (!existsSync(prettierConfigFile)) {
        throw new Error(`Prettier config file ${prettierConfigFile} does not exist`);
      }

      if (!existsSync(baseDir)) {
        mkDir(baseDir);
      }

      const newPrettierConfigFile = path.join(baseDir, path.basename(prettierConfigFile));
      copyFileSync(prettierConfigFile, newPrettierConfigFile);

      if (!prettierConfigExists(baseDir)) {
        throw new Error(`No prettier config file in ${baseDir}`);
      }
    }
  }
}

export async function formatAndWriteFile(file: string, content: string): Promise<void> {
  if (!path.isAbsolute(file)) {
    throw new Error(`Path ${file} is not absolute.`);
  }

  let existingCode: string | undefined = undefined;
  if (existsSync(file)) {
    existingCode = readFileSync(file, 'utf8');
  }

  debugLog(`format file ${file}`);
  const res = await formatFileUsingPrettier(file, content);

  const dir = path.dirname(file);
  if (!existsSync(dir)) {
    mkDir(dir);
  }

  if (existingCode !== undefined && res.fromattedCode === existingCode) {
    debugLog(`✅ Unchanged file ${file}`);
    return;
  }

  if (!isDryRun()) {
    debugLog(`🚚 write file ${file}`);
    writeFileSync(file, res.fromattedCode);
  } else {
    debugLog(`[dry-run] Skip write file ${file}`);
  }
}

export function toImportsCode(imports: Array<string | [string, string]>): string[] {
  let importsCode: string[] = [];
  if ((imports as unknown) !== undefined && imports.length > 0) {
    importsCode = imports.map((i) => {
      if (typeof i === 'string') {
        return `import "${i}"`;
      }
      return `import {${i[0]}} from "${i[1]}"`;
    });

    importsCode.forEach(debugLog);
  } else {
    debugLog(`No imports in solidity FHEVMTestSuiteXX.sol files.`);
  }
  return importsCode;
}

function _isPromise(value: unknown): value is PromiseLike<unknown> {
  return typeof value === 'object' && value !== null && 'then' in value && typeof value.then === 'function';
}

async function _resolveValue(value: unknown): Promise<string> {
  if (_isPromise(value)) {
    const res = await value;
    if (typeof res !== 'string') {
      throw new Error(`Unexpected type`);
    }
    return res;
  }
  if (typeof value !== 'string') {
    throw new Error(`Unexpected type`);
  }
  return value;
}

/**
 * Directory of the package that owns `file`: the nearest ancestor directory that has a `package.json`.
 * Generated files are formatted the way this package formats its own code (see `formatFileUsingPrettier`).
 */
function findOwningPackageDir(file: string): string {
  let dir = path.dirname(path.resolve(file));
  for (let depth = 0; depth < MAX_PRETTIER_CONFIG_SEARCH_DEPTH; ++depth) {
    if (existsSync(path.join(dir, 'package.json'))) {
      return dir;
    }
    const parent = path.dirname(dir);
    if (parent === dir) {
      break;
    }
    dir = parent;
  }
  throw new Error(`No package.json found above ${file}: cannot tell which package's prettier setup to use.`);
}

/** `true` for a package name (e.g. `prettier-plugin-solidity`, `@scope/name`), `false` for a file path. */
function isPackageName(specifier: string): boolean {
  return !specifier.startsWith('.') && !path.isAbsolute(specifier);
}

/**
 * The part of the prettier 3 API used by `formatFileUsingPrettier`, declared here rather than taken from the
 * `prettier` typings: the prettier used is loaded at runtime from the package that owns the generated file, and the
 * typings TypeScript finds for `prettier` depend on the install layout (e.g. `@types/prettier` 2.x, brought in by
 * typechain, can shadow prettier 3's own types).
 */
type PrettierApi = {
  resolveConfig(filePath: string): Promise<Record<string, unknown> | null>;
  getFileInfo(
    filePath: string,
    options: { ignorePath: string[]; resolveConfig: boolean; plugins?: unknown[] },
  ): Promise<{ ignored: boolean; inferredParser: string | null }>;
  format(source: string, options: Record<string, unknown>): string | Promise<string>;
};

/**
 * Formats a generated file exactly as running `prettier` inside the package that owns it would: with that package's
 * prettier, its plugins and its ignore files, whatever the directory codegen is run from.
 *
 * On its own, prettier resolves plugin names and ignore files from the current working directory, so the result
 * depended on where codegen was run (another plugin version from the repo root, a crash elsewhere).
 */
async function formatFileUsingPrettier(
  filePath: string,
  content: string,
): Promise<{ result: 'ignored' | 'unchanged' | 'formatted'; fromattedCode: string }> {
  const packageDir = findOwningPackageDir(filePath);
  const packageRequire = createRequire(path.join(packageDir, 'package.json'));

  // The package's own prettier. A package without one has no formatting of its own: use codegen's.
  let packagePrettier = prettier as unknown as PrettierApi;
  try {
    packagePrettier = packageRequire('prettier') as PrettierApi;
  } catch {
    debugLog(`No prettier installed for ${packageDir}: formatting ${filePath} with codegen's prettier.`);
  }

  const prettierConfig = (await packagePrettier.resolveConfig(filePath)) ?? {};

  // Plugin package names are resolved from the owning package, not from the current working directory.
  const configPlugins: unknown = prettierConfig.plugins;
  if (configPlugins !== undefined && !Array.isArray(configPlugins)) {
    throw new Error(`Invalid prettier config for ${filePath}: "plugins" must be an array.`);
  }
  const plugins = configPlugins?.map((plugin: unknown) =>
    typeof plugin === 'string' && isPackageName(plugin) ? packageRequire.resolve(plugin) : plugin,
  );

  // `getFileInfo` also loads the config's plugins (to infer the parser): give it the resolved ones instead.
  const info = await packagePrettier.getFileInfo(filePath, {
    ignorePath: [path.join(packageDir, '.gitignore'), path.join(packageDir, '.prettierignore')],
    resolveConfig: false,
    ...(plugins !== undefined ? { plugins } : {}),
  });
  if (info.ignored || info.inferredParser == null) {
    return { result: 'ignored', fromattedCode: content };
  }

  const output = await _resolveValue(
    // Warning! prettier.format can return a Promise!
    packagePrettier.format(content, {
      ...prettierConfig,
      ...(plugins !== undefined ? { plugins } : {}),
      filepath: filePath,
    }),
  );

  if (output === content) {
    return { result: 'unchanged', fromattedCode: output };
  }

  return { result: 'formatted', fromattedCode: output };
}
