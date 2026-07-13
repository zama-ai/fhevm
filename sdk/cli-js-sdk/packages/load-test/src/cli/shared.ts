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

export const parseValueTypes = (value: string): string[] => {
  const types = value.split(",").map((entry) => entry.trim());
  for (const type of types) {
    if (!(FHE_VALUE_TYPES as readonly string[]).includes(type)) {
      throw new Error(`Unknown value type "${type}". Expected: ${FHE_VALUE_TYPES.join(", ")}.`);
    }
  }
  return types;
};

export const readReport = async (path: string): Promise<Report> => {
  const candidate = path.endsWith(".json") ? path : join(path, "report.json");
  return readReportFile(candidate);
};
