import { mkdir, readdir } from "fs/promises";
import { dirname, join, relative } from "path";

interface MinioOps {
  fetch: typeof fetch;
}

const DEFAULT_OPS: MinioOps = { fetch };
let ops: MinioOps = DEFAULT_OPS;
const S3_REQUEST_TIMEOUT_MS = 15_000;

function normalizeEndpoint(endpoint: string): string {
  return endpoint.replace(/\/+$/, "");
}

function encodeS3Key(key: string): string {
  return key
    .split("/")
    .map((part) => encodeURIComponent(part))
    .join("/");
}

function decodeXml(value: string): string {
  return value
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, '"')
    .replace(/&apos;/g, "'");
}

function parseTag(xml: string, tag: string): string | undefined {
  const match = xml.match(new RegExp(`<${tag}>([^<]+)</${tag}>`, "i"));
  return match?.[1] ? decodeXml(match[1]) : undefined;
}

function parseKeys(xml: string): string[] {
  return [...xml.matchAll(/<Key>([^<]+)<\/Key>/g)].map((match) => decodeXml(match[1] ?? "")).filter(Boolean);
}

async function fetchBucketListPage(endpoint: string, bucket: string, marker?: string): Promise<string> {
  const params = new URLSearchParams();
  if (marker) {
    params.set("marker", marker);
  }

  const url = `${normalizeEndpoint(endpoint)}/${bucket}${params.size ? `?${params.toString()}` : ""}`;
  const response = await ops.fetch(url, { signal: AbortSignal.timeout(S3_REQUEST_TIMEOUT_MS) });
  if (!response.ok) {
    throw new Error(`failed to list bucket objects (status ${response.status})`);
  }

  return response.text();
}

async function listFilesRecursive(root: string, current = root): Promise<string[]> {
  const entries = await readdir(current, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    const fullPath = join(current, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await listFilesRecursive(root, fullPath)));
      continue;
    }
    if (entry.isFile()) {
      files.push(fullPath);
    }
  }

  return files;
}

export async function listBucketObjects(endpoint: string, bucket: string): Promise<string[]> {
  const objects: string[] = [];
  let marker: string | undefined;

  while (true) {
    const xml = await fetchBucketListPage(endpoint, bucket, marker);
    const keys = parseKeys(xml);
    objects.push(...keys);

    const isTruncated = parseTag(xml, "IsTruncated")?.toLowerCase() === "true";
    if (!isTruncated) {
      return objects;
    }

    const nextMarker = parseTag(xml, "NextMarker") ?? keys.at(-1);
    if (!nextMarker) {
      return objects;
    }
    marker = nextMarker;
  }
}

export async function getObject(endpoint: string, bucket: string, key: string): Promise<Uint8Array> {
  const url = `${normalizeEndpoint(endpoint)}/${bucket}/${encodeS3Key(key)}`;
  const response = await ops.fetch(url, { signal: AbortSignal.timeout(S3_REQUEST_TIMEOUT_MS) });
  if (!response.ok) {
    throw new Error(`failed to fetch object ${key} (status ${response.status})`);
  }

  return new Uint8Array(await response.arrayBuffer());
}

export async function putObject(endpoint: string, bucket: string, key: string, data: Uint8Array): Promise<void> {
  const url = `${normalizeEndpoint(endpoint)}/${bucket}/${encodeS3Key(key)}`;
  const response = await ops.fetch(url, {
    method: "PUT",
    body: new Blob([data as unknown as BlobPart]),
    signal: AbortSignal.timeout(S3_REQUEST_TIMEOUT_MS),
  });
  if (!response.ok) {
    throw new Error(`failed to upload object ${key} (status ${response.status})`);
  }
}

export async function downloadBucket(endpoint: string, bucket: string, destDir: string): Promise<number> {
  await mkdir(destDir, { recursive: true });
  const keys = await listBucketObjects(endpoint, bucket);

  for (const key of keys) {
    const data = await getObject(endpoint, bucket, key);
    const targetPath = join(destDir, ...key.split("/"));
    await mkdir(dirname(targetPath), { recursive: true });
    await Bun.write(targetPath, data);
  }

  return keys.length;
}

export async function uploadBucket(endpoint: string, bucket: string, sourceDir: string): Promise<number> {
  let files: string[] = [];
  try {
    files = await listFilesRecursive(sourceDir);
  } catch {
    return 0;
  }

  for (const filePath of files) {
    const key = relative(sourceDir, filePath).split("\\").join("/");
    const data = new Uint8Array(await Bun.file(filePath).arrayBuffer());
    await putObject(endpoint, bucket, key, data);
  }

  return files.length;
}

export const __internal = {
  parseKeys,
  resetOpsForTests(): void {
    ops = DEFAULT_OPS;
  },
  setOpsForTests(overrides: Partial<MinioOps>): void {
    ops = { ...DEFAULT_OPS, ...overrides };
  },
};
