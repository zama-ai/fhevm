import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import { join } from "node:path";

import type { EnvOverrides, LoadTestEnv } from "../env";
import type { Report } from "../report/schema";
import { readReportFile } from "../report/runtime";

type GlobalOptions = Readonly<{
  network?: string;
  relayerUrl?: string;
  relayerApiPrefix?: string;
  relayerB?: string;
  relayerBApiPrefix?: string;
  rpcUrl?: string;
  dataDir?: string;
  relayerConfig?: string;
  relayerBConfig?: string;
}>;

export const envFromCommand = async (command: {
  optsWithGlobals(): unknown;
}): Promise<LoadTestEnv> => {
  const globals = command.optsWithGlobals() as GlobalOptions;
  const overrides: EnvOverrides = {
    network: globals.network,
    relayerUrl: globals.relayerUrl,
    relayerApiPrefix: globals.relayerApiPrefix,
    relayerBUrl: globals.relayerB,
    relayerBApiPrefix: globals.relayerBApiPrefix,
    rpcUrl: globals.rpcUrl,
    dataDir: globals.dataDir,
    relayerConfigPath: globals.relayerConfig,
    relayerBConfigPath: globals.relayerBConfig,
  };
  const { resolveEnv } = await import("../env");
  return resolveEnv(overrides);
};

export const parsePositiveInt = (value: string): number => {
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`Expected a positive integer, got "${value}".`);
  }
  return parsed;
};

export const parsePositiveIntOrAuto = (value: string): number | "auto" =>
  value === "auto" ? "auto" : parsePositiveInt(value);

/** Positive-integer parser bounded by a reasonable ceiling for a resource flag. */
export const parseBoundedInt = (
  label: string,
  max: number,
) => (value: string): number => {
  const parsed = parsePositiveInt(value);
  if (parsed > max) {
    throw new Error(`${label} must be at most ${max.toString()}, got "${value}".`);
  }
  return parsed;
};

/** `parseBoundedInt`, also accepting the literal `"auto"`. */
export const parseBoundedIntOrAuto = (
  label: string,
  max: number,
) => (value: string): number | "auto" =>
  value === "auto" ? "auto" : parseBoundedInt(label, max)(value);

export const parsePositiveNumber = (value: string): number => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`Expected a positive number, got "${value}".`);
  }
  return parsed;
};

export const parseNonNegativeInt = (value: string): number => {
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed < 0) {
    throw new Error(`Expected a non-negative integer, got "${value}".`);
  }
  return parsed;
};

export const parseNonNegativeNumber = (value: string): number => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new Error(`Expected a non-negative number, got "${value}".`);
  }
  return parsed;
};

/** `parseNonNegativeNumber`, additionally bounded to an inclusive `[0, max]` range. */
export const parseBoundedNonNegativeNumber = (
  label: string,
  max: number,
) => (value: string): number => {
  const parsed = parseNonNegativeNumber(value);
  if (parsed > max) {
    throw new Error(`${label} must be between 0 and ${max.toString()}, got "${value}".`);
  }
  return parsed;
};

export const parseValueTypes = (value: string): string[] => {
  const types = value.split(",").map((entry) => entry.trim());
  const seen = new Set<string>();
  for (const type of types) {
    if (!(FHE_VALUE_TYPES as readonly string[]).includes(type)) {
      throw new Error(`Unknown value type "${type}". Expected: ${FHE_VALUE_TYPES.join(", ")}.`);
    }
    if (seen.has(type)) {
      throw new Error(
        `Duplicate value type "${type}" in --types. Each type must appear at most once, ` +
          "since it silently weights round-robin generation.",
      );
    }
    seen.add(type);
  }
  return types;
};

export const readReport = async (path: string): Promise<Report> => {
  const candidate = path.endsWith(".json") ? path : join(path, "report.json");
  return readReportFile(candidate);
};
