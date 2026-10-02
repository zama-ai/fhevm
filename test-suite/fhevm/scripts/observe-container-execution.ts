#!/usr/bin/env bun
import { assertVisibleGpu, containerExecution, type Worker } from "../src/consensus/container-execution";

const count = Number(process.argv[2]);
if (!Number.isInteger(count) || count < 1 || count > 32) throw new Error("operator count must be 1..32");
const names = Array.from({ length: count }, (_, index) => ["tfhe-worker", "sns-worker", "zkproof-worker"].map(role =>
  `coprocessor${index || ""}-${role}`)).flat();
function command(args: string[]): string {
  const result = Bun.spawnSync(args, { stdout: "pipe", stderr: "pipe", timeout: 30_000 });
  if (result.exitCode !== 0) throw new Error(`${args.slice(0, 2).join(" ")} failed while observing worker execution`);
  return result.stdout.toString();
}
const before: Worker[] = JSON.parse(command(["docker", "inspect", ...names]));
if (before.length !== names.length) throw new Error("incomplete worker observation");
const execution = containerExecution(before);
if (execution.backend === "gpu-cuda") {
  for (const worker of before) assertVisibleGpu(execution, command(["docker", "exec", worker.Id,
    "nvidia-smi", "--query-gpu=uuid,compute_cap", "--format=csv,noheader"]));
}
const after: Worker[] = JSON.parse(command(["docker", "inspect", ...names]));
if (after.length !== before.length || after.some((worker, index) =>
  worker.Id !== before[index].Id || worker.Image !== before[index].Image || !worker.State.Running || worker.State.Paused)) {
  throw new Error("worker ownership changed during execution observation");
}
console.log(`${execution.backend}|${execution.hardware}|${execution.revision}`);
