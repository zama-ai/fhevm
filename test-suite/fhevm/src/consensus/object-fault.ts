import { createHash } from "node:crypto";

export const OBJECT_FAULTS = ["missing", "truncated", "wrong-handle", "wrong-key", "wrong-format"] as const;
export type ObjectFault = typeof OBJECT_FAULTS[number];
export type StoredObject = { body: Uint8Array; metadata: Record<string, string> };
export type ObjectBackup = { path: string; body: string; metadata: Record<string, string>; sha256: string };
export interface ObjectStore { read(path: string): Promise<StoredObject | null>; write(path: string, value: StoredObject): Promise<void>; remove(path: string): Promise<void> }
export const digest = (body: Uint8Array) => createHash("sha256").update(body).digest("hex");
export const objectPath = (operator: number, handle: string) => {
  if (!Number.isInteger(operator) || operator < 0 || operator > 2 || !/^0x[0-9a-f]{64}$/.test(handle)) throw new Error("invalid isolated object target");
  return `coproc-${operator}/ct128/${handle.slice(2)}/1`;
};
export function mutatedObject(mode: Exclude<ObjectFault, "missing">, original: StoredObject, alternate?: StoredObject): StoredObject {
  if (mode === "wrong-handle") {
    if (!alternate || digest(alternate.body) === digest(original.body)) throw new Error("wrong-handle control requires distinct real material");
    return structuredClone(alternate);
  }
  const changed = structuredClone(original);
  if (mode === "truncated") {
    if (original.body.length < 2) throw new Error("original object too short");
    changed.body = original.body.slice(0, Math.floor(original.body.length / 2));
  } else {
    const attestation = JSON.parse(changed.metadata["ct-attestation"] ?? "null");
    if (!attestation || !attestation.signature || !attestation.key_id || !attestation.format) throw new Error("original attestation missing required fields");
    if (mode === "wrong-key") attestation.key_id = attestation.key_id === "0x0" ? "0x1" : "0x0";
    else if (mode === "wrong-format") attestation.format = attestation.format === "compressed_on_cpu" ? "compressed_on_gpu" : "compressed_on_cpu";
    else throw new Error("unknown object fault");
    changed.metadata["ct-attestation"] = JSON.stringify(attestation);
  }
  return changed;
}

export async function snapshotObjects(store: ObjectStore, handle: string): Promise<ObjectBackup[]> {
  const snapshots: ObjectBackup[] = [];
  for (let operator = 0; operator < 3; operator++) {
    const path = objectPath(operator, handle);
    const value = await store.read(path);
    if (!value?.body.length || !value.metadata["ct-attestation"]) throw new Error(`no restorable signed object at ${path}`);
    snapshots.push({ path, body: Buffer.from(value.body).toString("base64"), metadata: value.metadata, sha256: digest(value.body) });
  }
  return snapshots;
}
export async function restoreObjects(store: ObjectStore, backups: ObjectBackup[]): Promise<void> {
  if (backups.length !== 3) throw new Error("incomplete object recovery journal");
  const failures: string[] = [];
  for (const [operator, backup] of backups.entries()) {
    try {
      const match = /^coproc-([0-2])\/ct128\/([0-9a-f]{64})\/1$/.exec(backup.path);
      if (!match || Number(match[1]) !== operator) throw new Error("unscoped recovery path");
      const body = Buffer.from(backup.body, "base64");
      if (digest(body) !== backup.sha256 || !backup.metadata["ct-attestation"]) throw new Error("damaged recovery journal");
      await store.write(backup.path, { body, metadata: backup.metadata });
      const restored = await store.read(backup.path);
      if (!restored || digest(restored.body) !== backup.sha256 || Object.entries(backup.metadata).some(([key, value]) => restored.metadata[key] !== value)) throw new Error("restored bytes/metadata differ");
    } catch (error) { failures.push(`${backup.path}: ${error}`); }
  }
  if (failures.length) throw new Error(`object restoration incomplete: ${failures.join("; ")}`);
}
