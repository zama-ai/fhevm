import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, mkdtemp, readFile, rm } from "fs/promises";
import { tmpdir } from "os";
import { join } from "path";

import {
  __internal,
  downloadBucket,
  getObject,
  listBucketObjects,
  putObject,
  uploadBucket,
} from "./minio-s3";

afterEach(() => {
  __internal.resetOpsForTests();
});

function makeXml(keys: string[], options: { truncated?: boolean; nextMarker?: string } = {}): string {
  const contents = keys.map((key) => `<Contents><Key>${key}</Key></Contents>`).join("");
  return `<ListBucketResult>${contents}<IsTruncated>${options.truncated ? "true" : "false"}</IsTruncated>${options.nextMarker ? `<NextMarker>${options.nextMarker}</NextMarker>` : ""}</ListBucketResult>`;
}

describe("minio s3", () => {
  test("listBucketObjects parses S3 XML", async () => {
    __internal.setOpsForTests({
      fetch: (async () => new Response(makeXml(["PUB/key1", "PUB/nested/key2"]))) as unknown as typeof fetch,
    });

    const keys = await listBucketObjects("http://127.0.0.1:9000", "kms-public");
    expect(keys).toEqual(["PUB/key1", "PUB/nested/key2"]);
  });

  test("listBucketObjects handles empty bucket", async () => {
    __internal.setOpsForTests({
      fetch: (async () => new Response(makeXml([]))) as unknown as typeof fetch,
    });

    expect(await listBucketObjects("http://127.0.0.1:9000", "kms-public")).toEqual([]);
  });

  test("listBucketObjects handles pagination", async () => {
    const urls: string[] = [];
    __internal.setOpsForTests({
      fetch: (async (input: RequestInfo | URL) => {
        const url = String(input);
        urls.push(url);
        if (!url.includes("marker=")) {
          return new Response(makeXml(["PUB/key1"], { truncated: true, nextMarker: "PUB/key1" }));
        }
        return new Response(makeXml(["PUB/key2"]));
      }) as unknown as typeof fetch,
    });

    const keys = await listBucketObjects("http://127.0.0.1:9000", "kms-public");
    expect(keys).toEqual(["PUB/key1", "PUB/key2"]);
    expect(urls[1]).toContain("marker=PUB%2Fkey1");
  });

  test("getObject returns bytes", async () => {
    __internal.setOpsForTests({
      fetch: (async () => new Response(new Uint8Array([1, 2, 3]))) as unknown as typeof fetch,
    });

    expect(Array.from(await getObject("http://127.0.0.1:9000", "kms-public", "PUB/key"))).toEqual([1, 2, 3]);
  });

  test("putObject sends PUT request", async () => {
    const requests: Array<{ url: string; method: string; size: number }> = [];
    __internal.setOpsForTests({
      fetch: (async (input: RequestInfo | URL, init?: RequestInit) => {
        requests.push({
          url: String(input),
          method: init?.method ?? "GET",
          size:
            init?.body instanceof ArrayBuffer
              ? init.body.byteLength
              : init?.body instanceof Blob
                ? init.body.size
              : (init?.body as Uint8Array | undefined)?.byteLength ?? 0,
        });
        return new Response(null, { status: 200 });
      }) as unknown as typeof fetch,
    });

    await putObject("http://127.0.0.1:9000", "kms-public", "PUB/key", new Uint8Array([9, 8]));

    expect(requests).toEqual([
      {
        url: "http://127.0.0.1:9000/kms-public/PUB/key",
        method: "PUT",
        size: 2,
      },
    ]);
  });

  test("downloadBucket writes files preserving paths", async () => {
    const tmp = await mkdtemp(join(tmpdir(), "fhevm-minio-download-"));
    try {
      __internal.setOpsForTests({
        fetch: (async (input: RequestInfo | URL) => {
          const url = String(input);
          if (url.endsWith("/kms-public")) {
            return new Response(makeXml(["PUB/key1", "PUB/nested/key2"]));
          }
          if (url.endsWith("/kms-public/PUB/key1")) {
            return new Response("alpha");
          }
          return new Response("beta");
        }) as unknown as typeof fetch,
      });

      const count = await downloadBucket("http://127.0.0.1:9000", "kms-public", tmp);
      expect(count).toBe(2);
      expect(await readFile(join(tmp, "PUB", "key1"), "utf8")).toBe("alpha");
      expect(await readFile(join(tmp, "PUB", "nested", "key2"), "utf8")).toBe("beta");
    } finally {
      await rm(tmp, { recursive: true, force: true });
    }
  });

  test("uploadBucket uploads all local files", async () => {
    const tmp = await mkdtemp(join(tmpdir(), "fhevm-minio-upload-"));
    const uploads: string[] = [];

    try {
      await mkdir(join(tmp, "PUB", "nested"), { recursive: true });
      await Bun.write(join(tmp, "PUB", "key1"), "alpha");
      await Bun.write(join(tmp, "PUB", "nested", "key2"), "beta");

      __internal.setOpsForTests({
        fetch: (async (input: RequestInfo | URL, init?: RequestInit) => {
          if (init?.method === "PUT") {
            uploads.push(String(input));
          }
          return new Response(null, { status: 200 });
        }) as unknown as typeof fetch,
      });

      const count = await uploadBucket("http://127.0.0.1:9000", "kms-public", tmp);
      expect(count).toBe(2);
      expect(uploads.sort()).toEqual([
        "http://127.0.0.1:9000/kms-public/PUB/key1",
        "http://127.0.0.1:9000/kms-public/PUB/nested/key2",
      ]);
    } finally {
      await rm(tmp, { recursive: true, force: true });
    }
  });
});
