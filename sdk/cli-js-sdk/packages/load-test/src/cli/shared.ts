import { DEFAULT_NETWORK, FHE_VALUE_TYPES, NETWORKS } from "@cli-fhevm-sdk/toolkit/types";
import { Option, type CommandUnknownOpts } from "@commander-js/extra-typings";
import { join } from "node:path";

import type { EnvOverrides, LoadTestEnv } from "../env";
import type { Report } from "../report/schema";
import { readReportFile } from "../report/runtime";

type EnvCommandOptions = Readonly<{
  network?: string;
  relayerUrl?: string;
  relayerApiPrefix?: string;
  relayerBUrl?: string;
  relayerBApiPrefix?: string;
  rpcUrl?: string;
  dataDir?: string;
  relayerConfig?: string;
  relayerBConfig?: string;
}>;

/**
 * Attaches the relayer/network/data-dir options to exactly the commands that
 * resolve a {@link LoadTestEnv}. Read-only commands (report, baseline, scenario
 * and suite show/list) never call {@link envFromCommand}, so they omit these
 * flags to keep their help output focused.
 */
export const withEnvOptions = <T extends CommandUnknownOpts>(command: T): T => {
  command
    .addOption(
      new Option("-n, --network <network>", `network to target (default: ${DEFAULT_NETWORK})`).choices(NETWORKS),
    )
    .option("--relayer-url <url>", "relayer base URL override")
    .option("--relayer-api-prefix <prefix>", "primary relayer API route prefix (raw flows only)")
    .option("--relayer-b-url <url>", "candidate relayer base URL for paired dispatch")
    .option("--relayer-b-api-prefix <prefix>", "candidate API route prefix (raw flows only)")
    .option("--rpc-url <url>", "host chain RPC URL override")
    .option("--data-dir <dir>", "pools and run artifacts root (default .load-test)")
    .option("--relayer-config <path>", "primary relayer config file to snapshot")
    .option("--relayer-b-config <path>", "candidate relayer config file to snapshot");
  return command;
};

/** Adds a `--format text|json` option to a read-only command. */
export const withFormatOption = <T extends CommandUnknownOpts>(command: T): T => {
  command.addOption(
    new Option("--format <format>", "output format").choices(["text", "json"]).default("text"),
  );
  return command;
};

/**
 * In JSON mode, silences info/warn/success logger lines — consola sends those to
 * stdout — so a single JSON document is the only thing on stdout; errors still
 * reach stderr. Returns whether JSON output was requested.
 */
export const useJsonOutput = async (
  options: { readonly format?: string } & Record<string, unknown>,
): Promise<boolean> => {
  if (options.format !== "json") return false;
  const { logger } = await import("../shared/logger");
  logger.level = 0;
  return true;
};

/** Writes exactly one pretty-printed JSON document to stdout. */
export const emitJson = (value: unknown): void => {
  process.stdout.write(`${JSON.stringify(value, null, 2)}\n`);
};

export const envFromCommand = async (command: {
  optsWithGlobals(): unknown;
}): Promise<LoadTestEnv> => {
  const options = command.optsWithGlobals() as EnvCommandOptions;
  const overrides: EnvOverrides = {
    network: options.network,
    relayerUrl: options.relayerUrl,
    relayerApiPrefix: options.relayerApiPrefix,
    relayerBUrl: options.relayerBUrl,
    relayerBApiPrefix: options.relayerBApiPrefix,
    rpcUrl: options.rpcUrl,
    dataDir: options.dataDir,
    relayerConfigPath: options.relayerConfig,
    relayerBConfigPath: options.relayerBConfig,
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
