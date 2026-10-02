import { createHash } from "node:crypto";
import { arch } from "node:os";

export type Worker = {
  Id: string;
  Name: string;
  Image: string;
  State: { Running: boolean; Paused?: boolean };
  Config: { Image: string; Labels?: Record<string, string> | null };
};
export type ContainerExecution = { backend: string; hardware: string; revision: string; capability?: string };

/** Image labels describe the compiled backend; device observations and the
 * suite's format/compute checks separately establish CUDA availability/use. */
export function containerExecution(workers: Worker[]): ContainerExecution {
  if (!workers.length) throw new Error("no worker containers observed");
  for (const worker of workers) {
    if (!worker.State.Running || worker.State.Paused) throw new Error(`${worker.Name}: worker is not running`);
    if (!/^sha256:[0-9a-f]{64}$/.test(worker.Image)) throw new Error(`${worker.Name}: missing immutable image identity`);
    if (/-cuda[0-9.]+-sm[0-9]+(?:@sha256:[0-9a-f]+)?$/.test(worker.Config.Image) && worker.Config.Labels?.["ai.zama.fhevm.gpu"] !== "true") {
      throw new Error(`${worker.Name}: GPU tag without GPU build labels`);
    }
  }
  const gpu = workers.filter(worker => worker.Config.Labels?.["ai.zama.fhevm.gpu"] === "true");
  if (!gpu.length) return { backend: "cpu", hardware: `cpu-${arch() === "x64" ? "x86_64" : arch() === "arm64" ? "aarch64" : arch()}`, revision: "" };
  if (gpu.length !== workers.length) throw new Error("mixed CPU/GPU worker fleet");
  const classes = workers.map(worker => {
    const labels = worker.Config.Labels!;
    const capability = labels["ai.zama.fhevm.compute-capability"];
    const cuda = labels["ai.zama.fhevm.cuda-version"];
    const revision = labels["org.opencontainers.image.revision"];
    if (!/^[0-9]+$/.test(capability ?? "") || !/^\d+\.\d+(?:\.\d+)?$/.test(cuda ?? "") || (revision !== undefined && !/^[0-9a-f]{40}$/.test(revision))) {
      throw new Error(`${worker.Name}: incomplete GPU build provenance`);
    }
    return { backend: "gpu-cuda", hardware: `cuda-${cuda}-sm${capability}`, revision: revision ?? "", capability };
  });
  if (classes.some(value => JSON.stringify(value) !== JSON.stringify(classes[0]))) throw new Error("GPU worker build classes differ");
  const roleImages = new Map<string, string>();
  for (const worker of workers) {
    const role = /(?:^|-)(tfhe-worker|sns-worker|zkproof-worker)$/.exec(worker.Name)?.[1];
    if (!role) throw new Error(`unknown worker role: ${worker.Name}`);
    if (roleImages.has(role) && roleImages.get(role) !== worker.Image) throw new Error(`different images for ${role}`);
    roleImages.set(role, worker.Image);
  }
  // The published GPU Dockerfile currently has no revision label. Bind the
  // execution class to the observed image set instead of claiming the harness
  // checkout is its source. Source attestation remains a separate release check.
  const revision = classes[0]!.revision || `images-sha256:${createHash("sha256").update(
    JSON.stringify([...roleImages.entries()].sort()),
  ).digest("hex")}`;
  return { ...classes[0]!, revision };
}

export function assertVisibleGpu(execution: ContainerExecution, output: string): void {
  const rows = output.trim().split("\n").filter(Boolean).map(line => line.split(",").map(value => value.trim()));
  if (!rows.length || rows.some(([uuid, capability]) => !/^GPU-[0-9a-f-]+$/.test(uuid ?? "") || capability?.replace(".", "") !== execution.capability)) {
    throw new Error("worker has no visible GPU matching its compiled architecture");
  }
}
