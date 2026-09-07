import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";

import { createKeyPairSignerFromBytes, type TransactionSigner } from "@solana/kit";

/** Parses a Solana CLI keypair JSON (64-byte seed+pubkey array). */
export const parseKeypairBytes = (json: string): Uint8Array => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch {
    throw new Error("invalid Solana keypair JSON");
  }
  if (
    !Array.isArray(parsed) ||
    parsed.length !== 64 ||
    parsed.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)
  ) {
    throw new Error("invalid Solana keypair JSON (expected 64 bytes 0-255)");
  }
  return Uint8Array.from(parsed);
};

/** Writes a Solana CLI keypair JSON to `file` (mode 0600) and returns the path. */
export const writeKeypairJson = async (file: string, json: string): Promise<string> => {
  parseKeypairBytes(json);
  await mkdir(path.dirname(file), { recursive: true });
  await writeFile(file, json, { mode: 0o600 });
  return file;
};

export const loadKeypairSigner = async (keypairPath: string): Promise<TransactionSigner> => {
  const { readFile } = await import("node:fs/promises");
  return createKeyPairSignerFromBytes(parseKeypairBytes(await readFile(keypairPath, "utf8")));
};

/**
 * Resolves a keypair file path from either an explicit path env or an inline JSON env.
 * Inline JSON is the Helm path (`secretKeyRef` into an env var); path is the local CLI path.
 */
export const resolveKeypairPath = async (parameters: {
  readonly pathEnv: string | undefined;
  readonly jsonEnv: string | undefined;
  readonly fallbackPath: string;
  readonly writePath: string;
}): Promise<string> => {
  if (parameters.jsonEnv) {
    return writeKeypairJson(parameters.writePath, parameters.jsonEnv);
  }
  if (parameters.pathEnv) return parameters.pathEnv;
  return parameters.fallbackPath;
};
