import type { Command, OptionValues } from 'commander';
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import * as path from 'path';
import * as prettier from 'prettier';

import { assertRelative } from './utils/paths';

export type UserSolidityTestGroup = {
  outDir: string;
  parentContractName?: string;
  imports?: (string | [string, string])[];
};

export type UserTypescriptTestGroup = {
  outDir: string;
  imports?: (string | [string, string])[];
};

export type TestGroup = {
  outDir: string;
  imports?: (string | [string, string])[];
};

export type SolidityTestGroup = TestGroup & {
  parentContractName?: string;
};

export type TypescriptTestGroup = TestGroup;

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
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (globalThis as any).program! as Command;
}

export function getOptions(): OptionValues {
  return getProgram().opts();
}

export function getUserConfig(): UserConfig | undefined {
  const p = getProgram().opts().config;
  if (p) {
    const jsonFile = toAbsoluteFileWithExtension(p, '.json', process.cwd());
    if (!existsSync(jsonFile)) {
      errorLog(`Codegen config file at ${jsonFile} does not exist.`);
      process.exit(1);
    }
    const json = readFileSync(jsonFile, 'utf8');
    debugLog(`Load config file: ${jsonFile}`);
    try {
      const o = JSON.parse(json);
      // If baseDir is not specified, use config file containing directory as baseDir
      if (!o.baseDir) {
        o.baseDir = path.dirname(jsonFile);
      }
      return o;
    } catch (e) {
      console.error(`Invalid json file ${p}.\${e}`);
    }
  }
  return undefined;
}

export function isVerbose(): boolean {
  return getProgram().opts().verbose === true || isDryRun();
}

export function isDryRun(): boolean {
  return getProgram().opts().dryRun === true;
}

export function getUserOverloadsFile(options: any): string | undefined {
  return options.overloads;
}

export function debugLog(s: string) {
  if (isVerbose()) {
    console.log(s);
  }
}

export function errorLog(s: string) {
  const RED_COLOR = '\x1b[31m';
  const RESET_COLOR = '\x1b[0m';
  console.error(RED_COLOR + s + RESET_COLOR);
}

export function debugLogDirectoriesUserConfig(config: HostContractsUserConfig) {
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
    ...(testsUserConfig?.solidity ? { solidity: resolveSolidityTestGroup(testsUserConfig?.solidity) } : {}),
    ...(testsUserConfig?.typescript ? { typescript: resolveTypescriptTestGroup(testsUserConfig?.typescript) } : {}),
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
  };
  assertRelative(p.fheTypeDir);
  assertRelative(p.outDir);
  return p;
}

function resolveSolidityTestGroup(userTestGroup: UserSolidityTestGroup): SolidityTestGroup {
  return {
    outDir: userTestGroup.outDir,
    ...(userTestGroup.parentContractName ? { parentContractName: userTestGroup.parentContractName } : {}),
    ...(userTestGroup.imports ? { imports: userTestGroup.imports } : {}),
  };
}

function resolveTypescriptTestGroup(userTestGroup: UserTypescriptTestGroup): TypescriptTestGroup {
  return {
    outDir: userTestGroup.outDir,
    ...(userTestGroup.imports ? { imports: userTestGroup.imports } : {}),
  };
}

function toAbsoluteTestsConfig(resolvedTestsConfig: TestsConfig, baseDir: string): TestsConfig {
  return {
    overloads: toAbsoluteFileWithExtension(resolvedTestsConfig.overloads, '.json', baseDir),
    publicDecrypt: resolvedTestsConfig.publicDecrypt,
    numberOfTestSplits: resolvedTestsConfig.numberOfTestSplits,
    shuffle: resolvedTestsConfig.shuffle,
    shuffleWithPseuseRand: resolvedTestsConfig.shuffleWithPseuseRand,
    ...(resolvedTestsConfig?.solidity ? { solidity: toAbsoluteTestGroup(resolvedTestsConfig?.solidity, baseDir) } : {}),
    ...(resolvedTestsConfig?.typescript
      ? { typescript: toAbsoluteTestGroup(resolvedTestsConfig?.typescript, baseDir) }
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
  };
}

function toAbsoluteTestGroup(testGroup: SolidityTestGroup | TypescriptTestGroup, baseDir: string): TestGroup {
  return {
    ...testGroup,
    outDir: toAbsoluteDirectory(testGroup.outDir, baseDir),
    ...((testGroup as SolidityTestGroup).parentContractName
      ? { parentContractName: (testGroup as SolidityTestGroup).parentContractName }
      : {}),
  };
}

export function mkDir(dir: string) {
  if (!path.isAbsolute(dir)) {
    throw new Error(`Path ${dir} is not absolute.`);
  }
  if (!existsSync(dir)) {
    if (!isDryRun()) {
      debugLog(`mkdir -p ${dir}`);
      mkdirSync(dir, { recursive: true });
    } else {
      debugLog(`Skip create directory ${dir} (dry-run)`);
    }
  }
}

function prettierConfigExists(baseDir: string): boolean {
  return existsSync(path.join(baseDir, '.prettierrc.yml')) || existsSync(path.join(baseDir, '.prettierrc.json'));
}

export function generatePrettierConfig(baseDir: string) {
  if (!isDryRun()) {
    if (!prettierConfigExists(baseDir)) {
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

export async function formatAndWriteFile(file: string, content: string) {
  if (!path.isAbsolute(file)) {
    throw new Error(`Path ${file} is not absolute.`);
  }

  const existingCode = readFileSync(file, 'utf8');

  debugLog(`format file ${file}`);
  const res = await formatFileUsingPrettier(file, content);

  const dir = path.dirname(file);
  if (!existsSync(dir)) {
    mkDir(dir);
  }

  if (res.fromattedCode === existingCode) {
    debugLog(`✅ file ${file} unchanged`);
    return;
  }

  if (!isDryRun()) {
    debugLog(`🚚 write file ${file}`);
    writeFileSync(file, res.fromattedCode);
  } else {
    debugLog(`Skip write file ${file} (dry-run)`);
  }
}

export function toImportsCode(imports: (string | [string, string])[]) {
  let importsCode: string[] = [];
  if (imports) {
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

function _isPromise(value: any) {
  return typeof value === 'object' && value !== null && typeof value.then === 'function';
}

async function _resolveValue(value: any): Promise<string> {
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

async function formatFileUsingPrettier(
  filePath: string,
  content: string,
): Promise<{ result: 'ignored' | 'unchanged' | 'formatted'; fromattedCode: string }> {
  const info = await prettier.getFileInfo(filePath);
  if (info.ignored || info.inferredParser == null) {
    return { result: 'ignored', fromattedCode: content };
  }

  const prettierConfig = await prettier.resolveConfig(filePath);

  const output = await _resolveValue(
    // Warning! prettier.format can return a Promise!
    prettier.format(content, {
      ...prettierConfig,
      filepath: filePath,
    }),
  );

  if (output === content) {
    return { result: 'unchanged', fromattedCode: output };
  }

  return { result: 'formatted', fromattedCode: output };
}
