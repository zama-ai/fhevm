import { execSync } from "child_process";
import { existsSync, readFileSync } from "fs";
import { join } from "path";

import {
  EXCLUDED_CONTRACT_FUNCTION_PATTERNS,
  EXCLUDED_FUNCTION_PATTERNS,
  EXCLUDED_MODIFIERS,
  PACKAGE_CONFIG,
  type PackageName,
} from "./config";
import { ABI_COMPAT_EXCEPTIONS } from "./exceptions";

type AbiParam = {
  type: string;
  indexed?: boolean;
  components?: AbiParam[];
};

type AbiEntry = {
  type: string;
  name?: string;
  inputs?: AbiParam[];
  outputs?: AbiParam[];
  anonymous?: boolean;
};

export type AbiCompatResult = {
  name: string;
  baselineExists: boolean;
  baselineStableCount: number;
  targetStableCount: number;
  missing: string[];
  allowedMissing: string[];
  added: string[];
  errors: string[];
};

function runJson(command: string) {
  return execSync(command, {
    encoding: "utf-8",
    stdio: ["pipe", "pipe", "pipe"],
    env: { ...process.env, NO_COLOR: "1" },
  });
}

function forgeInspectAbi(contract: string, root: string): AbiEntry[] | null {
  try {
    const raw = runJson(`forge inspect "contracts/${contract}.sol:${contract}" abi --root "${root}" --json --force`);
    // Forge may prepend compilation progress to stdout on the first invocation in a
    // clean directory.  Extract the JSON array instead of parsing the whole output.
    const jsonStart = raw.indexOf("[");
    if (jsonStart === -1) {
      return null;
    }
    return JSON.parse(raw.slice(jsonStart));
  } catch (error: any) {
    if (error.stderr) {
      console.error(String(error.stderr));
    }
    return null;
  }
}

function canonicalType(param: AbiParam): string {
  const suffix = param.type.match(/(\[[^\]]*\])+$/)?.[0] ?? "";
  if (!param.type.startsWith("tuple")) {
    return param.type;
  }
  const components = (param.components ?? []).map(canonicalType).join(",");
  return `(${components})${suffix}`;
}

function canonicalSignature(entry: AbiEntry): string | null {
  if (!entry.name) {
    return null;
  }

  const inputs = (entry.inputs ?? []).map((input) => {
    const indexed = entry.type === "event" && input.indexed ? " indexed" : "";
    return `${canonicalType(input)}${indexed}`;
  });

  if (entry.type === "function") {
    const outputs = (entry.outputs ?? []).map(canonicalType).join(",");
    return `function ${entry.name}(${inputs.join(",")}) returns (${outputs})`;
  }

  if (entry.type === "event") {
    const anonymous = entry.anonymous ? " anonymous" : "";
    return `event ${entry.name}(${inputs.join(",")})${anonymous}`;
  }

  if (entry.type === "error") {
    return `error ${entry.name}(${inputs.join(",")})`;
  }

  return null;
}

function stripComments(source: string) {
  return source.replace(/\/\*[\s\S]*?\*\//g, "").replace(/\/\/.*$/gm, "");
}

function excludedFunctionNames(contract: string, source: string) {
  const names = new Set<string>();
  const sanitized = stripComments(source);
  const matcher = /function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([\s\S]*?)\)\s*([^{;]*)[;{]/g;
  const contractPatterns = EXCLUDED_CONTRACT_FUNCTION_PATTERNS[contract] ?? [];

  for (const match of sanitized.matchAll(matcher)) {
    const name = match[1];
    if (
      EXCLUDED_FUNCTION_PATTERNS.some((pattern) => pattern.test(name)) ||
      contractPatterns.some((pattern) => pattern.test(name))
    ) {
      names.add(name);
      continue;
    }

    const declarationSuffix = match[3].split(/\breturns\b/)[0] ?? "";
    const tokens = declarationSuffix.match(/\b[A-Za-z_][A-Za-z0-9_]*\b/g) ?? [];
    if (tokens.some((token) => EXCLUDED_MODIFIERS.has(token))) {
      names.add(name);
    }
  }

  return names;
}

function collectStableSignatures(contract: string, root: string, abi: AbiEntry[]) {
  const sourcePath = join(root, "contracts", `${contract}.sol`);
  if (!existsSync(sourcePath)) {
    return { signatures: new Set<string>(), count: 0, error: `Missing source file for ${contract}: ${sourcePath}` };
  }

  const excludedNames = excludedFunctionNames(contract, readFileSync(sourcePath, "utf-8"));
  const contractPatterns = EXCLUDED_CONTRACT_FUNCTION_PATTERNS[contract] ?? [];
  const signatures = new Set<string>();

  for (const entry of abi) {
    if (!["function", "event", "error"].includes(entry.type)) {
      continue;
    }
    if (
      entry.name &&
      (EXCLUDED_FUNCTION_PATTERNS.some((pattern) => pattern.test(entry.name)) ||
        contractPatterns.some((pattern) => pattern.test(entry.name)))
    ) {
      continue;
    }
    if (entry.type === "function" && entry.name && excludedNames.has(entry.name)) {
      continue;
    }
    const signature = canonicalSignature(entry);
    if (signature) {
      signatures.add(signature);
    }
  }

  return { signatures, count: signatures.size };
}

export function collectAbiCompatResults(baselineDir: string, targetDir: string, pkg: PackageName): AbiCompatResult[] {
  return PACKAGE_CONFIG[pkg].contracts.map((name) => {
    const baselineSource = join(baselineDir, "contracts", `${name}.sol`);
    if (!existsSync(baselineSource)) {
      return {
        name,
        baselineExists: false,
        baselineStableCount: 0,
        targetStableCount: 0,
        missing: [],
        allowedMissing: [],
        added: [],
        errors: [],
      };
    }

    const baselineAbi = forgeInspectAbi(name, baselineDir);
    const targetAbi = forgeInspectAbi(name, targetDir);
    const errors: string[] = [];
    if (baselineAbi == null) {
      errors.push(`Failed to inspect baseline ABI for ${name}`);
    }
    if (targetAbi == null) {
      errors.push(`Failed to inspect target ABI for ${name}`);
    }
    if (errors.length > 0 || baselineAbi == null || targetAbi == null) {
      return {
        name,
        baselineExists: true,
        baselineStableCount: 0,
        targetStableCount: 0,
        missing: [],
        allowedMissing: [],
        added: [],
        errors,
      };
    }

    const baselineStable = collectStableSignatures(name, baselineDir, baselineAbi);
    const targetStable = collectStableSignatures(name, targetDir, targetAbi);
    if (baselineStable.error) {
      errors.push(baselineStable.error);
    }
    if (targetStable.error) {
      errors.push(targetStable.error);
    }

    const allowedMissingSet = new Set(ABI_COMPAT_EXCEPTIONS[pkg]?.[name] ?? []);
    const missing = [...baselineStable.signatures]
      .filter((signature) => !targetStable.signatures.has(signature) && !allowedMissingSet.has(signature))
      .sort();
    const allowedMissing = [...baselineStable.signatures]
      .filter((signature) => !targetStable.signatures.has(signature) && allowedMissingSet.has(signature))
      .sort();
    const added = [...targetStable.signatures].filter((signature) => !baselineStable.signatures.has(signature)).sort();

    return {
      name,
      baselineExists: true,
      baselineStableCount: baselineStable.count,
      targetStableCount: targetStable.count,
      missing,
      allowedMissing,
      added,
      errors,
    };
  });
}
