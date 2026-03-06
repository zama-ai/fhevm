import { mkdir, rename } from "fs/promises";
import { ENV_GENERATORS } from "./env-mapping";
import type { FhevmConfig } from "./model";
import type { EnvFileName } from "./service-map";
import { dirnameOf } from "../utils/path";

export interface EnvGenerationConfigSet {
  defaultConfig: FhevmConfig;
  overrides?: Partial<Record<EnvFileName, FhevmConfig>>;
}

/**
 * Compose files reference env_file: ../env/staging/.env.<name>.local
 * relative to the docker-compose/ directory. The CLI EnvFileName "kms-core"
 * maps to the compose convention "core".
 */
const COMPOSE_ENV_STEM: Record<EnvFileName, string> = {
  coprocessor: "coprocessor",
  "kms-connector": "kms-connector",
  "kms-core": "core",
  database: "database",
  minio: "minio",
  "gateway-sc": "gateway-sc",
  "host-sc": "host-sc",
  relayer: "relayer",
  "gateway-node": "gateway-node",
  "host-node": "host-node",
  "test-suite": "test-suite",
  "gateway-mocked-payment": "gateway-mocked-payment",
};

/** Returns the file path compose expects for a given EnvFileName, relative to envDir. */
export function composeEnvFilePath(envDir: string, name: EnvFileName): string {
  const stem = COMPOSE_ENV_STEM[name];
  return `${envDir}/.env.${stem}.local`;
}

function needsQuotes(value: string): boolean {
  return /[\s#"\n\r]/.test(value);
}

function escapeValue(value: string): string {
  return value.replaceAll("\\", "\\\\").replaceAll('"', '\\"');
}

export function formatEnvContent(vars: Record<string, string>): string {
  const lines = Object.keys(vars)
    .sort((a, b) => a.localeCompare(b))
    .map((key) => {
      const value = vars[key] ?? "";
      if (value.length === 0) {
        return `${key}=`;
      }
      return needsQuotes(value) ? `${key}="${escapeValue(value)}"` : `${key}=${value}`;
    });

  return `${lines.join("\n")}\n`;
}

export async function writeEnvFile(filePath: string, vars: Record<string, string>): Promise<void> {
  await mkdir(dirnameOf(filePath), { recursive: true });
  const content = formatEnvContent(vars);
  const tmpPath = `${filePath}.tmp.${Date.now()}`;
  await Bun.write(tmpPath, content);
  await rename(tmpPath, filePath);
}

export async function generateEnvFile(
  configOrSet: FhevmConfig | EnvGenerationConfigSet,
  envDir: string,
  name: EnvFileName,
): Promise<string> {
  const generator = ENV_GENERATORS[name];
  const filePath = composeEnvFilePath(envDir, name);
  const config = "defaultConfig" in configOrSet ? configOrSet.overrides?.[name] ?? configOrSet.defaultConfig : configOrSet;
  await writeEnvFile(filePath, generator(config));
  return filePath;
}

export async function generateAllEnvFiles(
  configOrSet: FhevmConfig | EnvGenerationConfigSet,
  envDir: string,
): Promise<Map<string, string>> {
  const outputs = new Map<string, string>();
  const names = Object.keys(ENV_GENERATORS) as EnvFileName[];
  for (const name of names) {
    const filePath = await generateEnvFile(configOrSet, envDir, name);
    outputs.set(name, filePath);
  }
  return outputs;
}
