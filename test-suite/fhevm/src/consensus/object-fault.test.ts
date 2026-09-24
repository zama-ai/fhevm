import { expect, test } from "bun:test";
import { digest, mutatedObject, objectPath, restoreObjects, snapshotObjects, type ObjectStore, type StoredObject } from "./object-fault";
const handle = `0x${"a".repeat(64)}`;
const value: StoredObject = { body: new Uint8Array([1, 2, 3, 4]), metadata: { "ct-attestation": JSON.stringify({ signature: "signed", key_id: "0x1234", format: "compressed_on_cpu" }), "custom-tag": "retained" } };
function memory() {
  const objects = new Map(Array.from({ length: 3 }, (_, i) => [objectPath(i, handle), structuredClone(value)]));
  const writes: string[] = [];
  const store: ObjectStore = { read: async (path) => objects.get(path) ?? null, write: async (path, item) => { writes.push(path); objects.set(path, structuredClone(item)); }, remove: async (path) => { objects.delete(path); } };
  return { objects, store, writes };
}
test("fault arms change only their intended material and preserve a restorable original", () => {
  expect([...mutatedObject("truncated", value).body]).toEqual([1, 2]);
  expect(mutatedObject("truncated", value).metadata).toEqual(value.metadata);
  for (const mode of ["wrong-key", "wrong-format"] as const) {
    const changed = mutatedObject(mode, value);
    expect(changed.body).toEqual(value.body);
    const before = JSON.parse(value.metadata["ct-attestation"]), after = JSON.parse(changed.metadata["ct-attestation"]);
    const field = mode === "wrong-key" ? "key_id" : "format";
    expect(after[field]).not.toEqual(before[field]);
    expect({ ...after, [field]: before[field] }).toEqual(before);
  }
  expect(() => mutatedObject("wrong-handle", value, value)).toThrow("distinct");
  const alternate = { ...value, body: new Uint8Array([9, 8]) };
  expect(mutatedObject("wrong-handle", value, alternate)).toEqual(alternate);
  expect([...value.body]).toEqual([1, 2, 3, 4]);
});
test("partial mutation is restored byte for byte, including all metadata", async () => {
  const { store, objects } = memory();
  const journal = await snapshotObjects(store, handle);
  await store.remove(journal[0].path);
  await store.write(journal[1].path, mutatedObject("wrong-format", value));
  await restoreObjects(store, journal);
  for (const item of objects.values()) expect(item).toEqual(value);
});
test("one restore failure cannot prevent recovery of later operators", async () => {
  const { store, writes } = memory();
  const journal = await snapshotObjects(store, handle);
  const write = store.write;
  store.write = async (path, item) => { if (path === journal[0].path) throw new Error("unavailable"); await write(path, item); };
  await expect(restoreObjects(store, journal)).rejects.toThrow("incomplete");
  expect(writes).toEqual(journal.slice(1).map((x) => x.path));
});
test("unscoped paths and damaged journals cannot overwrite unrelated objects", async () => {
  const { store, writes } = memory();
  const journal = await snapshotObjects(store, handle);
  journal[0].path = "unrelated/keys";
  journal[1].body = Buffer.from("damaged").toString("base64");
  await expect(restoreObjects(store, journal)).rejects.toThrow("incomplete");
  expect(writes).toEqual([journal[2].path]);
  expect(() => objectPath(3, handle)).toThrow();
  expect(() => objectPath(0, "../keys")).toThrow();
});
test("a successful write without matching read-back cannot claim cleanup", async () => {
  const { store } = memory();
  const journal = await snapshotObjects(store, handle);
  store.read = async () => ({ ...value, body: new Uint8Array([0]) });
  await expect(restoreObjects(store, journal)).rejects.toThrow("restored bytes/metadata differ");
  expect(digest(value.body)).toEqual(journal[0].sha256);
});
