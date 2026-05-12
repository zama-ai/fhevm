import { execFileSync } from "child_process";
import { existsSync, readFileSync } from "fs";
import { join } from "path";

const VERSION_RE = /(?<name>REINITIALIZER_VERSION|MAJOR_VERSION|MINOR_VERSION|PATCH_VERSION)\s*=\s*(?<value>\d+)/g;
const REINITIALIZE_FUNCTION_PREFIX = "reinitializeV";

export interface ContractVersions {
  REINITIALIZER_VERSION: number;
  MAJOR_VERSION: number;
  MINOR_VERSION: number;
  PATCH_VERSION: number;
}

export interface ReinitializerInfo {
  name: string;
  signature: string;
  inputs: string[];
}

export interface ContractCheckResult {
  name: string;
  baselineExists: boolean;
  bytecodeChanged: boolean;
  semanticVersionChanged: boolean;
  reinitializerVersionChanged: boolean;
  baselineVersions?: ContractVersions;
  targetVersions?: ContractVersions;
  reinitializer?: ReinitializerInfo;
  errors: string[];
}

function execForgeInspect(contract: string, root: string, field: string): string | null {
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(contract)) {
    throw new Error(`Invalid Solidity contract name: ${contract}`);
  }

  try {
    return execFileSync("forge", ["inspect", `contracts/${contract}.sol:${contract}`, "--root", root, field], {
      encoding: "utf-8",
      stdio: ["pipe", "pipe", "pipe"],
      env: { ...process.env, NO_COLOR: "1" },
    });
  } catch (error: any) {
    if (error.stderr) {
      console.error(String(error.stderr));
    }
    return null;
  }
}

function forgeInspectBytecode(contract: string, root: string): string | null {
  const raw = execForgeInspect(contract, root, "deployedBytecode");
  if (raw == null) return null;
  const match = raw.match(/0x[0-9a-fA-F]+/);
  return match ? match[0] : null;
}

function extractVersions(filePath: string): { versions: Partial<ContractVersions>; source: string } {
  const source = readFileSync(filePath, "utf-8");
  const versions: Partial<ContractVersions> = {};
  for (const { groups } of source.matchAll(VERSION_RE)) {
    versions[groups!.name as keyof ContractVersions] = Number(groups!.value);
  }
  return { versions, source };
}

function getReinitializer(source: string): ReinitializerInfo | undefined {
  const uncommented = source.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
  const match = uncommented.match(/function\s+(reinitializeV\d+)\s*\(([\s\S]*?)\)\s*(public|external|internal|private)/);
  if (!match) return undefined;
  const [, name, rawParams] = match;
  const normalizedParams = rawParams.replace(/\s+/g, " ").trim();
  const inputs = normalizedParams.length > 0 ? normalizedParams.split(",").map((input) => input.trim()) : [];
  return {
    name,
    signature: `${name}(${normalizedParams})`,
    inputs,
  };
}

export function collectUpgradeVersionResults(baselineDir: string, targetDir: string): ContractCheckResult[] {
  const manifestPath = join(targetDir, "upgrade-manifest.json");
  if (!existsSync(manifestPath)) {
    throw new Error(`upgrade-manifest.json not found in ${targetDir}`);
  }

  const contracts: string[] = JSON.parse(readFileSync(manifestPath, "utf-8"));

  return contracts.map((name) => {
    const baseSol = join(baselineDir, "contracts", `${name}.sol`);
    const targetSol = join(targetDir, "contracts", `${name}.sol`);
    const errors: string[] = [];

    if (!existsSync(baseSol)) {
      return {
        name,
        baselineExists: false,
        bytecodeChanged: false,
        semanticVersionChanged: false,
        reinitializerVersionChanged: false,
        errors,
      };
    }

    if (!existsSync(targetSol)) {
      return {
        name,
        baselineExists: true,
        bytecodeChanged: false,
        semanticVersionChanged: false,
        reinitializerVersionChanged: false,
        errors: [`${name} listed in upgrade-manifest.json but missing in target`],
      };
    }

    const { versions: baseVersions } = extractVersions(baseSol);
    const { versions: targetVersions, source: targetSource } = extractVersions(targetSol);

    for (const key of ["REINITIALIZER_VERSION", "MAJOR_VERSION", "MINOR_VERSION", "PATCH_VERSION"] as const) {
      if (baseVersions[key] == null || targetVersions[key] == null) {
        errors.push(`Failed to parse ${key}`);
      }
    }

    const baselineBytecode = errors.length === 0 ? forgeInspectBytecode(name, baselineDir) : null;
    const targetBytecode = errors.length === 0 ? forgeInspectBytecode(name, targetDir) : null;

    if (errors.length === 0 && baselineBytecode == null) {
      errors.push("Failed to compile baseline bytecode");
    }
    if (errors.length === 0 && targetBytecode == null) {
      errors.push("Failed to compile target bytecode");
    }

    const bytecodeChanged = baselineBytecode != null && targetBytecode != null && baselineBytecode !== targetBytecode;
    const reinitializerVersionChanged =
      baseVersions.REINITIALIZER_VERSION != null &&
      targetVersions.REINITIALIZER_VERSION != null &&
      baseVersions.REINITIALIZER_VERSION !== targetVersions.REINITIALIZER_VERSION;
    const semanticVersionChanged =
      baseVersions.MAJOR_VERSION != null &&
      targetVersions.MAJOR_VERSION != null &&
      (baseVersions.MAJOR_VERSION !== targetVersions.MAJOR_VERSION ||
        baseVersions.MINOR_VERSION !== targetVersions.MINOR_VERSION ||
        baseVersions.PATCH_VERSION !== targetVersions.PATCH_VERSION);

    if (bytecodeChanged && !reinitializerVersionChanged) {
      errors.push(
        `${name} bytecode changed but REINITIALIZER_VERSION was not bumped (still ${targetVersions.REINITIALIZER_VERSION})`,
      );
    }

    if (bytecodeChanged && !semanticVersionChanged) {
      errors.push(
        `${name} bytecode changed but semantic version was not bumped (still v${targetVersions.MAJOR_VERSION}.${targetVersions.MINOR_VERSION}.${targetVersions.PATCH_VERSION})`,
      );
    }

    if (!bytecodeChanged && reinitializerVersionChanged) {
      errors.push(
        `${name} REINITIALIZER_VERSION bumped (${baseVersions.REINITIALIZER_VERSION} -> ${targetVersions.REINITIALIZER_VERSION}) but bytecode is unchanged`,
      );
    }

    const reinitializer = getReinitializer(targetSource);
    if (bytecodeChanged && reinitializerVersionChanged) {
      const expectedFn = `${REINITIALIZE_FUNCTION_PREFIX}${targetVersions.REINITIALIZER_VERSION! - 1}`;
      if (reinitializer?.name !== expectedFn) {
        errors.push(
          `${name} has REINITIALIZER_VERSION=${targetVersions.REINITIALIZER_VERSION} but expected ${expectedFn}()`,
        );
      }
    }

    return {
      name,
      baselineExists: true,
      bytecodeChanged,
      semanticVersionChanged,
      reinitializerVersionChanged,
      baselineVersions: baseVersions as ContractVersions,
      targetVersions: targetVersions as ContractVersions,
      reinitializer,
      errors,
    };
  });
}
