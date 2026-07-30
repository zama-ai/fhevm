import { clearSolanaEncryptionKeyCache, createFhevmEncryptClient } from "@fhevm/sdk/solana";

import type { DemoConfig } from "./demoSession";

type EncryptionKeyBytes = NonNullable<
  NonNullable<Parameters<typeof createFhevmEncryptClient>[0]["options"]>["fheEncryptionKey"]
>;

type EncryptionKeyResponse = {
  readonly fingerprint: string;
  readonly publicKeyId: string;
  readonly publicKeyBase64: string;
  readonly crsId: string;
  readonly crsBase64: string;
};

let cachedEncryptionKey: { readonly fingerprint: string; readonly key: EncryptionKeyBytes } | undefined;

const decodeBase64 = (value: string): Uint8Array => {
  const binary = atob(value);
  return Uint8Array.from(binary, (character) => character.charCodeAt(0));
};

const parseEncryptionKeyResponse = (value: unknown, config: DemoConfig): EncryptionKeyBytes => {
  if (typeof value !== "object" || value === null) throw new Error("demo encryption key must be an object");
  const response = value as Partial<EncryptionKeyResponse>;
  const string = (candidate: unknown, name: string): string => {
    if (typeof candidate !== "string" || candidate.length === 0) {
      throw new Error(`demo encryption key ${name} must be a non-empty string`);
    }
    return candidate;
  };
  return {
    publicKeyBytes: {
      id: string(response.publicKeyId, "publicKeyId"),
      bytes: decodeBase64(string(response.publicKeyBase64, "publicKeyBase64")),
    },
    crsBytes: {
      id: string(response.crsId, "crsId"),
      capacity: 2048 as EncryptionKeyBytes["crsBytes"]["capacity"],
      bytes: decodeBase64(string(response.crsBase64, "crsBase64")),
    },
    metadata: {
      relayerUrl: config.relayerUrl,
      chainId: BigInt(config.chainId),
    },
  };
};

const getJson = async (path: string): Promise<unknown> => {
  const response = await fetch(path);
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as { readonly error?: string } | null;
    throw new Error(body?.error ?? `${path} failed with HTTP ${response.status}`);
  }
  return response.json();
};

export const loadDemoEncryptionKey = async (config: DemoConfig): Promise<EncryptionKeyBytes> => {
  const metadata = (await getJson("/api/demo-encryption-key-meta")) as { readonly fingerprint?: unknown };
  if (typeof metadata.fingerprint !== "string" || metadata.fingerprint.length === 0) {
    throw new Error("demo encryption key fingerprint must be a non-empty string");
  }
  if (cachedEncryptionKey?.fingerprint === metadata.fingerprint) return cachedEncryptionKey.key;

  const response = await getJson("/api/demo-encryption-key");
  const fingerprint =
    typeof (response as { readonly fingerprint?: unknown }).fingerprint === "string"
      ? (response as { readonly fingerprint: string }).fingerprint
      : "";
  if (fingerprint.length === 0) throw new Error("demo encryption key response has no fingerprint");
  clearSolanaEncryptionKeyCache(config.relayerUrl);
  const key = parseEncryptionKeyResponse(response, config);
  cachedEncryptionKey = { fingerprint, key };
  return key;
};
