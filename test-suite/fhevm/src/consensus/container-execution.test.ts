import { expect, test } from "bun:test";
import { assertVisibleGpu, containerExecution } from "./container-execution";

const revision = "a".repeat(40);
const worker = (gpu = false) => ({
  Id: "fixture-id", Name: "/coprocessor-tfhe-worker", Image: `sha256:${"b".repeat(64)}`,
  State: { Running: true, Paused: false },
  Config: { Image: gpu ? "worker:release-cuda12.8-sm90" : "worker:release", Labels: gpu ? {
    "ai.zama.fhevm.gpu": "true", "ai.zama.fhevm.compute-capability": "90",
    "ai.zama.fhevm.cuda-version": "12.8", "org.opencontainers.image.revision": revision,
  } : {} as Record<string, string> },
});
test("ordinary CPU containers remain CPU regardless of caller backend labels", () => {
  expect(containerExecution([worker(), worker()]).backend).toBe("cpu");
});
test("published CUDA containers carry observed runtime revision and hardware", () => {
  expect(containerExecution([worker(true), worker(true)])).toEqual({ backend: "gpu-cuda", hardware: "cuda-12.8-sm90", revision, capability: "90" });
});
test("a single CPU writer cannot be hidden in a GPU fleet", () => {
  expect(() => containerExecution([worker(true), worker()])).toThrow("mixed CPU/GPU");
});
test("absent, stopped and paused workers cannot establish a backend", () => {
  expect(() => containerExecution([])).toThrow();
  for (const state of [{ Running: false, Paused: false }, { Running: true, Paused: true }]) {
    expect(() => containerExecution([{ ...worker(true), State: state }])).toThrow("not running");
  }
});
test("GPU tag alone cannot stand in for build provenance", () => {
  const value = worker(true); value.Config.Labels = {};
  expect(() => containerExecution([value])).toThrow("without GPU build labels");
});
test("GPU revision, architecture and CUDA version must be complete and agree", () => {
  for (const key of ["ai.zama.fhevm.compute-capability", "ai.zama.fhevm.cuda-version"]) {
    const missing = worker(true); delete missing.Config.Labels[key];
    expect(() => containerExecution([missing])).toThrow("incomplete");
    const other = worker(true); other.Config.Labels[key] = key.includes("revision") ? "c".repeat(40) : key.includes("capability") ? "89" : "12.9";
    expect(() => containerExecution([worker(true), other])).toThrow("classes differ");
  }
});
test("visible device architecture must match the binary, including every exposed device", () => {
  const execution = containerExecution([worker(true)]);
  assertVisibleGpu(execution, "GPU-01234567-89ab, 9.0\n");
  for (const output of ["", "GPU-01234567-89ab, 8.9", "GPU-01234567-89ab, 9.0\nGPU-12345678-abcd, 8.9", "NVIDIA-SMI has failed"]) {
    expect(() => assertVisibleGpu(execution, output)).toThrow("matching its compiled architecture");
  }
});

test("unlabelled source revisions use the immutable image set, never the checkout", () => {
  const a = worker(true); delete a.Config.Labels["org.opencontainers.image.revision"];
  expect(containerExecution([a]).revision).toMatch(/^images-sha256:[0-9a-f]{64}$/);
  const b = { ...a, Image: `sha256:${"d".repeat(64)}` };
  expect(containerExecution([b]).revision).not.toBe(containerExecution([a]).revision);
  expect(() => containerExecution([a, b])).toThrow("different images");
  expect(() => containerExecution([a, worker(true)])).toThrow("classes differ");
});
test("different or malformed source revision labels are rejected", () => {
  const a = worker(true); a.Config.Labels["org.opencontainers.image.revision"] = "c".repeat(40);
  expect(() => containerExecution([worker(true), a])).toThrow("classes differ");
  a.Config.Labels["org.opencontainers.image.revision"] = "unknown";
  expect(() => containerExecution([a])).toThrow("incomplete");
});
