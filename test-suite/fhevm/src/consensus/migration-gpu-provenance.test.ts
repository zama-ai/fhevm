import { describe, expect, test } from "bun:test";
import { migrationGpuRoles, validateMigrationGpuImage, validateMigrationGpuReceipt, validateMigrationGpuSource, type MigrationGpuReceipt } from "./migration-gpu-provenance";

const revision = "a".repeat(40);
const checkout = "b".repeat(40);
const id = `sha256:${"c".repeat(64)}`;
function receipt(published = true): MigrationGpuReceipt {
  return {
    mode: published ? "published" : "checkout", revision,
    releaseTag: "v0.15.0-0", device: "GPU-1234-abcd", capability: "90",
    images: Object.fromEntries(migrationGpuRoles.map(role => [role, id])) as MigrationGpuReceipt["images"],
    references: Object.fromEntries(migrationGpuRoles.map(role => [role, `ghcr.io/zama-ai/fhevm/coprocessor/${role}:v0.15.0-0-cuda12.8-sm90@${id}`])) as MigrationGpuReceipt["references"],
  };
}
function image(source?: string) {
  const labels: Record<string, string> = { "ai.zama.fhevm.gpu": "true", "ai.zama.fhevm.compute-capability": "90", "ai.zama.fhevm.cuda-version": "12.8" };
  if (source) labels["org.opencontainers.image.revision"] = source;
  return { Id: id, Config: { Labels: labels } };
}
describe("GPU migration source and artifact provenance", () => {
  test("checkout builds require exact source and a source label", () => {
    const r = receipt(false);
    validateMigrationGpuReceipt(r);
    validateMigrationGpuSource(r, revision);
    validateMigrationGpuImage(r, "tfhe-worker", image(revision));
    expect(() => validateMigrationGpuSource(r, checkout)).toThrow("differs");
    expect(() => validateMigrationGpuImage(r, "tfhe-worker", image())).toThrow("source revision");
    delete r.mode;
    expect(() => validateMigrationGpuImage(r, "tfhe-worker", image())).toThrow("source revision");
  });
  test("published artifacts allow absent source labels without manufacturing them", () => {
    const r = receipt();
    validateMigrationGpuReceipt(r);
    validateMigrationGpuSource(r, checkout, revision, ["test-suite/fhevm/scripts/hold-migration-gpu.sh"]);
    const i = image();
    for (const role of migrationGpuRoles) validateMigrationGpuImage(r, role, i);
    expect(i.Config.Labels["org.opencontainers.image.revision"]).toBeUndefined();
    expect(() => validateMigrationGpuImage(r, "sns-worker", image(checkout))).toThrow("conflicting");
  });
  test("source equivalence excludes production changes and incorrect tag resolutions", () => {
    const r = receipt();
    expect(() => validateMigrationGpuSource(r, checkout, checkout)).toThrow("release tag");
    for (const path of ["coprocessor/fhevm-engine/Cargo.lock", "shared/src/lib.rs", "Dockerfile.workspace"]) {
      expect(() => validateMigrationGpuSource(r, checkout, revision, [path])).toThrow("production source");
    }
    expect(() => validateMigrationGpuSource(r, `${checkout}-dirty`, revision)).toThrow("clean checkout");
  });
  test("every role needs a pinned reference to its release and architecture", () => {
    const ref = receipt().references!["tfhe-worker"];
    for (const bad of [ref.replace("zama-ai", "other"), ref.replace("tfhe-worker", "sns-worker"), ref.replace("v0.15.0-0", "v0.14.2"), ref.replace("sm90", "sm80"), ref.split("@")[0]!]) {
      const r = receipt(); r.references!["tfhe-worker"] = bad;
      expect(() => validateMigrationGpuReceipt(r)).toThrow();
    }
    for (const role of migrationGpuRoles) {
      const r = receipt(); delete (r.images as Partial<MigrationGpuReceipt["images"]>)[role];
      expect(() => validateMigrationGpuReceipt(r)).toThrow("all three");
    }
  });
  test("observed identity and GPU labels must match the receipt", () => {
    for (const [label, value] of [["ai.zama.fhevm.gpu", "false"], ["ai.zama.fhevm.compute-capability", "80"], ["ai.zama.fhevm.cuda-version", "12.6"]]) {
      const i = image(); i.Config.Labels[label!] = value!;
      expect(() => validateMigrationGpuImage(receipt(), "tfhe-worker", i)).toThrow();
    }
    const i = image(); i.Id = `sha256:${"d".repeat(64)}`;
    expect(() => validateMigrationGpuImage(receipt(), "tfhe-worker", i)).toThrow("identity");
  });
  test("unknown receipt modes and dirty revisions fail closed", () => {
    const r = receipt(); r.mode = "unknown" as MigrationGpuReceipt["mode"];
    expect(() => validateMigrationGpuReceipt(r)).toThrow("unknown");
    const dirty = receipt(); dirty.revision += "-dirty";
    expect(() => validateMigrationGpuReceipt(dirty)).toThrow("clean source");
  });
});
