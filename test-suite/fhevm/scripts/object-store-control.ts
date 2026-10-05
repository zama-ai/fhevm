#!/usr/bin/env bun
import { rename } from "node:fs/promises";
import { OBJECT_FAULTS, objectPath, mutatedObject, snapshotObjects, restoreObjects, digest, type ObjectStore, type ObjectBackup, type ObjectFault } from "../src/consensus/object-fault";

const [command, journal, mode, handle, alternateHandle] = process.argv.slice(2);
if (!journal || !["arm", "restore"].includes(command ?? "")) throw new Error("usage: object-store-control.ts arm|restore JOURNAL [MODE HANDLE ALTERNATE]");
// The coprocessor buckets accept anonymous writes and deletes (see object-store-docker-compose.yml).
async function mutate(method: "PUT" | "DELETE", path: string, init: { headers?: Record<string, string>; body?: BodyInit } = {}) {
  const response = await fetch(`http://127.0.0.1:9000/${path}`, { method, ...init, signal: AbortSignal.timeout(60_000) });
  if (!response.ok) throw new Error(`Object store ${method} failed (${response.status}); original material is retained in the recovery journal`);
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
    // Raw headers preserve each original metadata value byte for byte, as signed attestations require.
    await mutate("PUT", path, { headers: Object.fromEntries(entries.map(([key, val]) => [`x-amz-meta-${key}`, val])), body: new Uint8Array(value.body) });
  },
  async remove(path) { await mutate("DELETE", path); },
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
