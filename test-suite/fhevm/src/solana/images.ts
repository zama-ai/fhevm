// Loads the Solana PoC image tags from the single source of truth (../../solana-images.env)
// so TS call sites and the shell bring-up (clean-e2e.sh) cannot drift. See FI#1546.
import { readFileSync } from "node:fs";

function parseEnvFile(text: string): Record<string, string> {
  const out: Record<string, string> = {};
  for (const line of text.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const eq = trimmed.indexOf("=");
    if (eq === -1) continue;
    out[trimmed.slice(0, eq).trim()] = trimmed.slice(eq + 1).trim();
  }
  return out;
}

const raw = parseEnvFile(readFileSync(new URL("../../solana-images.env", import.meta.url), "utf8"));

function required(key: string): string {
  const v = raw[key];
  if (!v) throw new Error(`solana-images.env is missing ${key}`);
  return v;
}

/** Solana PoC image tags — the only place these are defined. */
export const solanaImages = {
  CORE_VERSION: required("CORE_VERSION"),
  CONNECTOR_GW_LISTENER_VERSION: required("CONNECTOR_GW_LISTENER_VERSION"),
  CONNECTOR_KMS_WORKER_VERSION: required("CONNECTOR_KMS_WORKER_VERSION"),
  RELAYER_IMAGE_REPOSITORY: required("RELAYER_IMAGE_REPOSITORY"),
  RELAYER_VERSION: required("RELAYER_VERSION"),
} as const;
