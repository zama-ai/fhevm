import type { Command, OptionValues } from 'commander';
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import * as path from 'path';
import prettier from 'prettier';

import { assertAbsolute, assertRelative } from './utils/paths.js';

export type UserSolidityTestGroup = {
  contractNameTemplate?: string;
  outDir: string;
  parentContract?: ContractConfig;
  imports?: (string | [string, string])[];
};

export type UserTypescriptTestGroup = {
  contractNameTemplate?: string;
  outDir: string;
  parentContract?: ContractConfig;
  imports?: (string | [string, string])[];
};

export type SolidityTestGroup = {
  outDir: string;
  contractNameTemplate?: string;
  parentContract?: ContractConfig;
  imports?: (string | [string, string])[];
};

export type TypescriptTestGroup = {
  outDir: string;
  imports?: (string | [string, string])[];
};

export type TestGroup = {
  outDir: string;
  contractNameTemplate?: string;
  parentContract?: ContractConfig;
  imports?: (string | [string, string])[];
};

export type UserConfig = {
  generateHCULimit?: boolean;
  shuffle?: boolean;
  shuffleWithPseuseRand?: boolean;
  publicDecrypt?: boolean;
  numberOfTestSplits?: number;
  noLib?: boolean;
  noTest?: boolean;
  overloads?: string;
  directories?: DirectoriesUserConfig;
  solidity?: UserSolidityTestGroup;
  typescript?: UserTypescriptTestGroup;
};

export type ResolvedConfig = {
  generateHCULimit: boolean;
  shuffle: boolean;
  shuffleWithPseuseRand: boolean;
  publicDecrypt: boolean;
  numberOfTestSplits: number;
  noLib: boolean;
  noTest: boolean;
  overloads: string;
  directories: DirectoriesConfig;
  solidity?: SolidityTestGroup;
  typescript?: TypescriptTestGroup;
};

export type DirectoriesUserConfig = {
  baseDir?: string;
  fheTypeDir?: string;
  libDir?: string;
  contractsDir?: string;
};

export type ContractConfig = {
  name: string;
  solidityFile: string;
};

export type DirectoriesConfig = {
  baseDir: string;
  fheTypeDir: string; // directory where the FheType.sol file is located
  libDir: string; // directory where the FHE.sol and Impl.sol files are located
  contractsDir: string;
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
    const jsonFile = toAbsoluteJsonFile(p, process.cwd());
    if (!existsSync(jsonFile)) {
      throw new Error(`Codegen config file at ${jsonFile} does not exist.`);
    }
    const json = readFileSync(jsonFile, 'utf8');
    debugLog(`Load config file: ${jsonFile}`);
    try {
      const o = JSON.parse(json);
      return o;
    } catch (e) {
      console.error(`Invalid json file ${p}.\${e}`);
    }
  }
  return undefined;
}

export function isDebug(): boolean {
  return getProgram().opts().verbose === true;
}

export function isDryRun(): boolean {
  return getProgram().opts().dryRun === true;
}

export function getUserOverloadsFile(options: any): string | undefined {
  return options.overloads;
}

export function debugLog(s: string) {
  if (isDebug()) {
    console.log(s);
  }
}

export function debugLogDirectoriesUserConfig(config: DirectoriesUserConfig) {
  debugLog(JSON.stringify(config, null, 2));
}

function assertDirectoriesConfig(resolved: DirectoriesConfig) {
  assertAbsolute(resolved.baseDir);
  assertRelative(resolved.fheTypeDir);
  assertRelative(resolved.libDir);
  assertRelative(resolved.contractsDir);
}

function toAbsoluteDirectory(dir: string, baseDir: string): string {
  if (!path.isAbsolute(baseDir)) {
    throw new Error(`Path ${baseDir} is not absolute.`);
  }
  return path.join(baseDir, dir);
}

export function toAbsoluteJsonFile(jsonFile: string, baseDir: string): string {
  const f = path.parse(jsonFile);
  if (f.ext !== '.json') {
    throw new Error(`Invalid json file name: ${jsonFile}. Missing ".json" extension.`);
  }
  if (path.isAbsolute(jsonFile)) {
    return jsonFile;
  }
  return path.join(baseDir, jsonFile);
}

function toAbsoluteSolidityFile(solidityFile: string, baseDir: string): string {
  const f = path.parse(solidityFile);
  if (f.ext !== '.sol') {
    throw new Error(`Invalid solidity file name: ${solidityFile}. Missing ".sol" extension.`);
  }
  if (path.isAbsolute(solidityFile)) {
    return solidityFile;
  }
  return path.join(baseDir, solidityFile);
}

export function resolveUserConfig(userConfig: UserConfig | undefined): ResolvedConfig {
  return {
    generateHCULimit: userConfig?.generateHCULimit === true,
    shuffle: userConfig?.shuffle === true,
    shuffleWithPseuseRand: userConfig?.shuffleWithPseuseRand === true,
    publicDecrypt: userConfig?.publicDecrypt === true,
    noLib: userConfig?.noLib === true,
    noTest: userConfig?.noTest === true,
    overloads: userConfig?.overloads ?? './overloads.json',
    numberOfTestSplits: userConfig?.numberOfTestSplits ?? 12,
    directories: resolveDirectoriesConfig(userConfig?.directories),
    ...(userConfig?.solidity ? { solidity: resolveTestGroup(userConfig.solidity) } : {}),
    ...(userConfig?.typescript ? { typescript: resolveTestGroup(userConfig.typescript) } : {}),
  };
}

function resolveDirectoriesConfig(userDirs: DirectoriesUserConfig | undefined): DirectoriesConfig {
  let baseDir = userDirs?.baseDir ?? process.cwd();
  if (!path.isAbsolute(baseDir)) {
    baseDir = path.normalize(path.join(process.cwd(), baseDir));
  }
  const libDir = userDirs?.libDir ?? './lib';
  const p: DirectoriesConfig = {
    baseDir,
    fheTypeDir: userDirs?.fheTypeDir ?? libDir,
    libDir,
    contractsDir: userDirs?.contractsDir ?? './contracts',
  };
  assertDirectoriesConfig(p);
  return p;
}

function resolveTestGroup(userTestGroup: UserSolidityTestGroup): TestGroup {
  return {
    outDir: userTestGroup.outDir,
    ...(userTestGroup.parentContract ? { parentContract: userTestGroup.parentContract } : {}),
    ...(userTestGroup.contractNameTemplate ? { contractNameTemplate: userTestGroup.contractNameTemplate } : {}),
    ...(userTestGroup.imports ? { imports: userTestGroup.imports } : {}),
  };
}

export function toAbsulteConfig(resolved: ResolvedConfig): ResolvedConfig {
  const directories = toAbsultePaths(resolved.directories);
  return {
    generateHCULimit: resolved.generateHCULimit,
    shuffle: resolved.shuffle,
    shuffleWithPseuseRand: resolved.shuffleWithPseuseRand,
    publicDecrypt: resolved.publicDecrypt,
    noLib: resolved.noLib,
    noTest: resolved.noTest,
    overloads: toAbsoluteJsonFile(resolved.overloads, directories.baseDir),
    numberOfTestSplits: resolved.numberOfTestSplits,
    directories,
    ...(resolved.solidity ? { solidity: toAbsoluteTestGroup(resolved.solidity, directories) } : {}),
    ...(resolved.typescript ? { typescript: toAbsoluteTestGroup(resolved.typescript, directories) } : {}),
  };
}

function toAbsultePaths(resolved: DirectoriesConfig): DirectoriesConfig {
  return {
    baseDir: resolved.baseDir,
    fheTypeDir: toAbsoluteDirectory(resolved.fheTypeDir, resolved.baseDir),
    libDir: toAbsoluteDirectory(resolved.libDir, resolved.baseDir),
    contractsDir: toAbsoluteDirectory(resolved.contractsDir, resolved.baseDir),
  };
}

function toAbsoluteTestGroup(testGroup: TestGroup, directories: DirectoriesConfig): TestGroup {
  return {
    ...testGroup,
    outDir: toAbsoluteDirectory(testGroup.outDir, directories.baseDir),
    ...(testGroup.parentContract
      ? { parentContract: toAbsoluteContratConfig(testGroup.parentContract, directories) }
      : {}),
  };
}

function toAbsoluteContratConfig(contractConfig: ContractConfig, directories: DirectoriesConfig): ContractConfig {
  const baseDir = directories.baseDir;
  assertAbsolute(baseDir);
  return {
    name: contractConfig.name,
    solidityFile: toAbsoluteSolidityFile(contractConfig.solidityFile, directories.baseDir),
  };
}

export function mkDir(dir: string) {
  if (!path.isAbsolute(dir)) {
    throw new Error(`Path ${dir} is not absolute.`);
  }
  if (!existsSync(dir)) {
    debugLog(`mkdir -p ${dir}`);
    if (!isDryRun()) {
      mkdirSync(dir, { recursive: true });
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

export async function writeFile(file: string, content: string) {
  if (!path.isAbsolute(file)) {
    throw new Error(`Path ${file} is not absolute.`);
  }
  debugLog(`generate file ${file}`);
  const res = await formatFileUsingPrettier(file, content);
  if (!isDryRun()) {
    debugLog(`write file ${file}`);
    const dir = path.dirname(file);
    if (!existsSync(dir)) {
      mkDir(dir);
    }
    writeFileSync(file, res.fromattedCode);
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

async function formatFileUsingPrettier(
  filePath: string,
  content: string,
): Promise<{ result: 'ignored' | 'unchanged' | 'formatted'; fromattedCode: string }> {
  const info = await prettier.getFileInfo(filePath);
  if (info.ignored || info.inferredParser == null) {
    return { result: 'ignored', fromattedCode: content };
  }

  // const input = readFileSync(filePath, "utf8");
  const prettierConfig = await prettier.resolveConfig(filePath);

  const output = await prettier.format(content, { ...prettierConfig, filepath: filePath });

  if (output === content) {
    return { result: 'unchanged', fromattedCode: output };
  }

  return { result: 'formatted', fromattedCode: output };
}
