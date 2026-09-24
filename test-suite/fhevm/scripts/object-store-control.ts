#!/usr/bin/env bun
import { rename } from "node:fs/promises";
import { envPath } from "../src/layout";
import { readEnvFile } from "../src/utils/fs";
import { OBJECT_FAULTS, objectPath, mutatedObject, snapshotObjects, restoreObjects, digest, type ObjectStore, type ObjectBackup, type ObjectFault } from "../src/consensus/object-fault";

const [command, journal, mode, handle, alternateHandle] = process.argv.slice(2);
if (!journal || !["arm", "restore"].includes(command ?? "")) throw new Error("usage: object-store-control.ts arm|restore JOURNAL [MODE HANDLE ALTERNATE]");
const env = await readEnvFile(envPath("minio"));
if (!env.MINIO_ROOT_USER || !env.MINIO_ROOT_PASSWORD) throw new Error("managed MinIO credentials missing");
const alias = new URL("http://127.0.0.1:9000");
alias.username = env.MINIO_ROOT_USER; alias.password = env.MINIO_ROOT_PASSWORD;
async function mc(args: string[], input?: Uint8Array) {
  const child = Bun.spawn(["docker", "exec", "-i", "-e", "MC_HOST_consensus", "fhevm-minio", "mc", "--quiet", ...args], {
    stdin: "pipe", stdout: "pipe", stderr: "pipe", timeout: 60_000, env: { ...process.env, MC_HOST_consensus: alias.href },
  });
  if (input) child.stdin.write(input);
  await child.stdin.end();
  const [status] = await Promise.all([child.exited, new Response(child.stderr).text(), new Response(child.stdout).text()]);
  if (status !== 0) throw new Error(`MinIO control failed (${status}); original material is retained in the recovery journal`);
}
const store: ObjectStore = {
  async read(path) {
    const response = await fetch(`http://127.0.0.1:9000/${path}`, { signal: AbortSignal.timeout(30_000) });
    if (response.status === 404) return null;
    if (!response.ok) throw new Error(`object GET ${response.status}`);
    const body = new Uint8Array(await response.arrayBuffer());
    if (!body.length || body.length > 32 * 1024 * 1024) throw new Error("invalid test object size");
    const metadata: Record<string, string> = {};
    for (const [key, value] of response.headers) if (key.startsWith("x-amz-meta-")) metadata[key.slice(11)] = value;
    return { body, metadata };
  },
  async write(path, value) {
    const entries = Object.entries(value.metadata);
    if (entries.some(([key, val]) => !/^[a-z0-9-]+$/.test(key) || /[\r\n]/.test(val))) throw new Error("invalid object metadata header");
    // mc --attr removes JSON quotes, corrupting signed attestations. Custom
    // headers preserve each original metadata value byte for byte.
    await mc([...entries.flatMap(([key, val]) => ["--custom-header", `x-amz-meta-${key}:${val}`]), "pipe", `consensus/${path}`], value.body);
  },
  async remove(path) { await mc(["rm", `consensus/${path}`]); },
};
if (command === "restore") {
  const file = Bun.file(journal);
  if (await file.exists()) {
    const saved = await file.json() as { originals: ObjectBackup[]; restored?: boolean };
    await restoreObjects(store, saved.originals);
    await Bun.write(journal, JSON.stringify({ ...saved, restored: true }));
  }
} else {
  if (!OBJECT_FAULTS.includes(mode as ObjectFault)) throw new Error("unknown object fault");
  if (await Bun.file(journal).exists()) throw new Error("refusing to overwrite an existing recovery journal");
  const originals = await snapshotObjects(store, handle!);
  await Bun.write(`${journal}.new`, JSON.stringify({ mode, handle, originals, restored: false }));
  await rename(`${journal}.new`, journal); // Persist every original before the first mutation.
  for (const [operator, original] of originals.entries()) {
    if (mode === "missing") {
      await store.remove(original.path);
      if (await store.read(original.path)) throw new Error("missing-object fault not observed");
    } else {
      const alternate = mode === "wrong-handle" ? await store.read(objectPath(operator, alternateHandle!)) : undefined;
      const changed = mutatedObject(mode as Exclude<ObjectFault, "missing">,
        { body: Buffer.from(original.body, "base64"), metadata: original.metadata }, alternate ?? undefined);
      await store.write(original.path, changed);
      const observed = await store.read(original.path);
      if (!observed || digest(observed.body) !== digest(changed.body) || observed.metadata["ct-attestation"] !== changed.metadata["ct-attestation"]) throw new Error("object fault was not observed at its public path");
    }
  }
  console.log(`observed ${mode} on all three buckets for ${handle}`);
}
