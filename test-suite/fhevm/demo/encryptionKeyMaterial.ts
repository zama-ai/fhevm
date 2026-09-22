// encryptionKeyMaterial — the FHE public key and CRS the page encrypts with, fetched by the operator.
//
// The relayer names the current key and CRS (`/v2/keyurl`) with the URLs they are served from. The
// material is public, but the fetch runs from this process, so the URL policy is explicit: the local
// stack serves keys from MinIO on loopback, a preview namespace from S3 over TLS. The material is
// cached per fingerprint (ids plus the objects' ETags), so a key rotation is picked up without a
// restart and an unchanged key costs one HEAD per check.

import type { DemoEncryptionKey } from "./operator";
import { MINIO_PORT } from "../src/layout";

type Descriptor = {
  readonly fingerprint: string;
  readonly publicKeyId: string;
  readonly publicKeyUrl: string;
  readonly crsId: string;
  readonly crsUrl: string;
};

export type EncryptionKeyMaterial = {
  readonly fingerprint: () => Promise<string>;
  readonly key: () => Promise<DemoEncryptionKey>;
};

const requiredString = (value: unknown, name: string): string => {
  if (typeof value !== "string" || value.length === 0) throw new Error(`${name} must be a non-empty string`);
  return value;
};

const objectUrl = (value: unknown, name: string, network: "localnet" | "devnet"): string => {
  const url = new URL(requiredString(value, name));
  if (network === "devnet") {
    if (url.protocol !== "https:") throw new Error(`${name} must use https on devnet`);
    return url.toString();
  }
  if (url.protocol !== "http:" || url.port !== String(MINIO_PORT)) {
    throw new Error(`${name} must use the local MinIO HTTP endpoint`);
  }
  // The relayer names MinIO by its compose service name; this process reaches it on loopback.
  if (url.hostname === "minio") url.hostname = "127.0.0.1";
  if (url.hostname !== "127.0.0.1") throw new Error(`${name} must use the local MinIO host`);
  return url.toString();
};

export const createEncryptionKeyMaterial = (options: {
  readonly relayerUrl: string;
  readonly apiKey: string;
  readonly network: "localnet" | "devnet";
}): EncryptionKeyMaterial => {
  let cached: { readonly fingerprint: string; readonly key: Promise<DemoEncryptionKey> } | undefined;

  const describe = async (): Promise<Descriptor> => {
    const keyUrlResponse = await fetch(`${options.relayerUrl}/v2/keyurl`, {
      headers: { accept: "application/json", "x-api-key": options.apiKey },
    });
    if (!keyUrlResponse.ok) throw new Error(`relayer key URL failed with HTTP ${keyUrlResponse.status}`);
    const body = (await keyUrlResponse.json()) as {
      readonly response?: {
        readonly fheKeyInfo?: readonly [{ readonly fhePublicKey?: { readonly dataId?: unknown; readonly urls?: unknown } }];
        readonly crs?: Record<string, { readonly dataId?: unknown; readonly urls?: unknown }>;
      };
    };
    const publicKey = body.response?.fheKeyInfo?.[0]?.fhePublicKey;
    const crs = body.response?.crs?.["2048"];
    const publicKeyUrl = objectUrl(Array.isArray(publicKey?.urls) ? publicKey.urls[0] : undefined, "public key URL", options.network);
    const crsUrl = objectUrl(Array.isArray(crs?.urls) ? crs.urls[0] : undefined, "CRS URL", options.network);
    const [publicKeyHead, crsHead] = await Promise.all([fetch(publicKeyUrl, { method: "HEAD" }), fetch(crsUrl, { method: "HEAD" })]);
    if (!publicKeyHead.ok) throw new Error(`public key metadata failed with HTTP ${publicKeyHead.status}`);
    if (!crsHead.ok) throw new Error(`CRS metadata failed with HTTP ${crsHead.status}`);
    const publicKeyId = requiredString(publicKey?.dataId, "public key dataId");
    const crsId = requiredString(crs?.dataId, "CRS dataId");
    const tag = (response: Response): string =>
      response.headers.get("etag") ?? response.headers.get("last-modified") ?? "unknown";
    return {
      fingerprint: `${publicKeyId}:${tag(publicKeyHead)}:${crsId}:${tag(crsHead)}`,
      publicKeyId,
      publicKeyUrl,
      crsId,
      crsUrl,
    };
  };

  const download = async (descriptor: Descriptor): Promise<DemoEncryptionKey> => {
    const [publicKeyResponse, crsResponse] = await Promise.all([fetch(descriptor.publicKeyUrl), fetch(descriptor.crsUrl)]);
    if (!publicKeyResponse.ok) throw new Error(`public key fetch failed with HTTP ${publicKeyResponse.status}`);
    if (!crsResponse.ok) throw new Error(`CRS fetch failed with HTTP ${crsResponse.status}`);
    const [publicKeyBytes, crsBytes] = await Promise.all([publicKeyResponse.arrayBuffer(), crsResponse.arrayBuffer()]);
    return {
      fingerprint: descriptor.fingerprint,
      publicKeyId: descriptor.publicKeyId,
      publicKeyBase64: Buffer.from(publicKeyBytes).toString("base64"),
      crsId: descriptor.crsId,
      crsBase64: Buffer.from(crsBytes).toString("base64"),
    };
  };

  return {
    fingerprint: async () => (await describe()).fingerprint,
    key: async () => {
      const descriptor = await describe();
      if (cached?.fingerprint !== descriptor.fingerprint) {
        const pending = download(descriptor).catch((error) => {
          if (cached?.key === pending) cached = undefined;
          throw error;
        });
        cached = { fingerprint: descriptor.fingerprint, key: pending };
      }
      return cached.key;
    },
  };
};
