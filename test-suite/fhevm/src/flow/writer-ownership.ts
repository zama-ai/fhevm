import { existsSync } from "node:fs";
import path from "node:path";

import { PreflightError } from "../errors";
import { RUNTIME_DIR } from "../layout";
import { run } from "../utils/process";
import type { State } from "../types";
import { assertOneWorkerPerQueue } from "./queue-ownership";

/** The global discovery boundary, including writer roles outside worker queues. */
export const assertManagedWriterOwnership = (state: Pick<State, "scenario">, containers: string[], quiescent = false) =>
  assertOneWorkerPerQueue(state, {allowGpu: true, managedWriterContainers: containers, requireQuiescent: quiescent});

/** Refuse unknown owners before mutation and restore the full snapshot on every failure. */
export const withQuiescedWriters = async <T>(
  state: Pick<State, "scenario">, containers: string[], action: () => Promise<T>,
): Promise<T> => {
  await assertManagedWriterOwnership(state, containers);
  const owners = await recordWriterOwnership(containers);
  try {
    await quiesceWriters(owners);
    await assertManagedWriterOwnership(state, containers, true);
    return await action();
  } finally {
    await restoreWriters(owners);
  }
};

/**
 * True while GPU host units are serving the operators' work queues.
 *
 * `gpu-consensus-workers.sh` writes this file when it takes over and clears it
 * on stop, and the shell side of the harness keys the same decision off it.
 */
const gpuSessionActive = () =>
  existsSync(path.join(RUNTIME_DIR, "gpu-consensus-workers", "node-config.env"));

/** The worker roles a GPU session takes over; their containers stay stopped. */
const GPU_OWNED_ROLES = ["tfhe-worker", "zkproof-worker", "sns-worker"] as const;

/** The GPU launcher, which owns the units' invocations. */
const GPU_LAUNCHER = path.join(import.meta.dir, "..", "..", "scripts", "gpu-consensus-workers.sh");

/** systemd --user is addressed through the running uid's runtime directory. */
const userBusEnv = () => {
  const runtime = `/run/user/${process.getuid?.() ?? 0}`;
  return { XDG_RUNTIME_DIR: runtime, DBUS_SESSION_BUS_ADDRESS: `unix:path=${runtime}/bus` };
};

/** `<kind>` and `<index>` of the GPU unit serving a worker container's queue. */
const gpuUnitFor = (container: string) => {
  const role = GPU_OWNED_ROLES.find((candidate) => container.endsWith(`-${candidate}`));
  if (!role) return undefined;
  const kind = role.replace("-worker", "");
  const match = /^coprocessor(\d*)-/.exec(container);
  if (!match) return undefined;
  return { kind, index: match[1] === "" ? "0" : match[1], unit: `fhevm-gpu-consensus-${kind}-${match[1] === "" ? "0" : match[1]}` };
};

const gpuUnitActive = async (unit: string) => {
  const result = await run(["systemctl", "--user", "show", unit, "--property=ActiveState", "--value"], {
    allowFailure: true,
    env: userBusEnv(),
    timeoutMs: 15_000,
  });
  const state = result.stdout.trim();
  if (result.code !== 0 || !["active", "inactive", "failed", "activating", "deactivating"].includes(state)) {
    throw new PreflightError(`Cannot determine GPU unit ${unit}'s state: ${result.stderr.trim() || state}`);
  }
  return state !== "inactive" && state !== "failed";
};

/** Preserve a deliberately suspended owner by refusing before any mutation. */
const recordGpuRunning = async (unit: string) => {
  const result = await run(["systemctl", "--user", "show", unit, "--property=ActiveState", "--property=MainPID"], {
    allowFailure: true, env: userBusEnv(), timeoutMs: 15_000,
  });
  const active = /^ActiveState=(.+)$/m.exec(result.stdout)?.[1];
  const pid = Number(/^MainPID=(\d+)$/m.exec(result.stdout)?.[1]);
  if (result.code !== 0 || !active || !Number.isSafeInteger(pid)) throw new PreflightError(`Cannot snapshot GPU owner ${unit}`);
  if (["inactive", "failed"].includes(active) && pid === 0) return false;
  if (active !== "active" || pid < 1) throw new PreflightError(`Cannot take ownership of GPU unit ${unit} while it is ${active}`);
  const stat = await run(["cat", `/proc/${pid}/stat`], {allowFailure: true, timeoutMs: 15_000});
  const processState = /\) ([A-Zt]) /.exec(stat.stdout)?.[1];
  if (stat.code !== 0 || !processState) throw new PreflightError(`Cannot snapshot GPU process ${unit}`);
  if (["T", "t"].includes(processState)) throw new PreflightError(`Cannot take ownership of GPU unit ${unit} while it is paused`);
  if (["Z", "X"].includes(processState)) throw new PreflightError(`Cannot take ownership of GPU unit ${unit} while its process is exiting`);
  return true;
};

const containerState = async (container: string) => {
  const result = await run(["docker", "inspect", "-f", "{{.State.Status}}", container], {
    allowFailure: true, timeoutMs: 15_000,
  });
  if (result.code !== 0) {
    if (/No such (object|container)/i.test(result.stderr)) return "missing";
    throw new PreflightError(`Cannot determine container ${container}'s state: ${result.stderr.trim()}`);
  }
  const state = result.stdout.trim();
  if (!["created", "running", "paused", "restarting", "removing", "exited", "dead"].includes(state)) {
    throw new PreflightError(`Unrecognized container ${container} state: ${state}`);
  }
  return state;
};

/** One writer, and how it is currently served. */
export type WriterOwnership = {
  container: string;
  /** Set when a GPU host unit serves this container's queue. */
  gpu?: { kind: string; index: string; unit: string };
  wasRunning: boolean;
};

/**
 * Records how every writer is served, and whether it was running, before
 * anything is displaced.
 *
 * "Which of these were running" has to be answered before the first stop, not
 * reconstructed afterwards: a restore that starts everything revives a worker
 * an operator had deliberately left down, and a restore that starts nothing
 * leaves the stack an operator short for whatever runs next.
 */
export const recordWriterOwnership = async (containers: string[]): Promise<WriterOwnership[]> => {
  const owners: WriterOwnership[] = [];
  for (const container of containers) {
    const gpu = gpuSessionActive() ? gpuUnitFor(container) : undefined;
    if (gpu) {
      const displaced = await containerState(container);
      if (["running", "paused", "restarting", "removing"].includes(displaced)) {
        throw new PreflightError(`GPU-owned queue ${container} also has a live Docker writer (${displaced}); refusing DB revert`);
      }
      owners.push({ container, gpu, wasRunning: await recordGpuRunning(gpu.unit) });
      continue;
    }
    const state = await containerState(container);
    if (state === "paused" || state === "removing") {
      throw new PreflightError(`Cannot take ownership of ${container} while it is ${state}`);
    }
    owners.push({ container, wasRunning: state === "running" || state === "restarting" });
  }
  return owners;
};

/**
 * Stops every writer and PROVES it stopped, on whichever backend serves it.
 *
 * Starting a displaced container beside a live CUDA unit creates two writers
 * on one queue. Leaving the CUDA unit running during database recovery is also
 * unsafe: `revert_coprocessor_db_state` requires every coprocessor service to be
 * stopped before it deletes rows. Ownership must therefore cover both backends.
 *
 * So the units are stopped through the launcher, which keeps their recorded
 * invocation configuration so they can be restored exactly, and a writer that
 * cannot be quiesced aborts the operation instead of being worked around.
 */
export const quiesceWriters = async (owners: WriterOwnership[]) => {
  for (const owner of owners) {
    if (!owner.wasRunning) continue;
    if (owner.gpu) {
      const stopped = await run([GPU_LAUNCHER, "stop-unit", owner.gpu.kind, owner.gpu.index], {
        allowFailure: true,
        env: userBusEnv(),
        timeoutMs: 120_000,
      });
      if (stopped.code !== 0) {
        throw new PreflightError(
          `Could not stop GPU unit ${owner.gpu.unit}, which serves ${owner.container}'s queue: ` +
            `${(stopped.stderr ?? stopped.stdout ?? "").trim().slice(0, 300)}. Refusing to run SQL against a ` +
            "database a worker is still writing to",
        );
      }
      continue;
    }
    await run(["docker", "stop", owner.container], { allowFailure: true, timeoutMs: 120_000 });
  }

  // Independent verification, not the stop commands' exit statuses: a container
  // that ignored SIGTERM, or a unit systemd restarted underneath us, is still a
  // live writer.
  const stillLive: string[] = [];
  for (const owner of owners) {
    if (owner.gpu) {
      if (await gpuUnitActive(owner.gpu.unit)) stillLive.push(`${owner.container} (unit ${owner.gpu.unit})`);
      const displaced = await containerState(owner.container);
      if (["running", "paused", "restarting", "removing"].includes(displaced)) stillLive.push(`${owner.container} (${displaced})`);
      continue;
    }
    const state = await containerState(owner.container);
    if (state === "running" || state === "paused" || state === "restarting" || state === "removing") stillLive.push(`${owner.container} (${state})`);
  }
  if (stillLive.length > 0) {
    throw new PreflightError(
      `db-state-revert requires every coprocessor writer stopped, but ${stillLive.length} is still live: ` +
        `${stillLive.join(", ")}. The revert deletes rows those processes are writing`,
    );
  }
  console.log(`[revert] quiesced ${owners.filter((owner) => owner.wasRunning).length} writer(s), verified stopped`);
};

/**
 * Restores exactly what was running, and verifies one worker per queue.
 *
 * A GPU unit is recreated through the launcher from its RECORDED configuration:
 * the units are transient, so `systemctl start` cannot bring one back, and
 * re-resolving the tuning from this process's environment would quietly revert
 * a heterogeneous operator to fleet defaults.
 */
export const restoreWriters = async (owners: WriterOwnership[]) => {
  const failures: string[] = [];
  for (const owner of owners) {
    if (!owner.wasRunning) continue;
    // An unknown postcondition fails the operation, but must not prevent the
    // remaining stopped writers from receiving their own restoration attempt.
    try {
      if (owner.gpu) {
        const restarted = await run([GPU_LAUNCHER, "restart-unit", owner.gpu.kind, owner.gpu.index], {
          allowFailure: true,
          env: userBusEnv(),
          timeoutMs: 180_000,
        });
        if (restarted.code !== 0 || !(await gpuUnitActive(owner.gpu.unit))) {
          failures.push(`${owner.gpu.unit} (${(restarted.stderr ?? "").trim().slice(0, 160)})`);
        }
        continue;
      }
      const started = await run(["docker", "start", owner.container], { allowFailure: true, timeoutMs: 120_000 });
      if (started.code !== 0 || await containerState(owner.container) !== "running") failures.push(owner.container);
    } catch (error) {
      failures.push(`${owner.gpu?.unit ?? owner.container}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  // Check every queue even when another owner's state cannot be inspected.
  const doublyServed: string[] = [];
  for (const owner of owners) {
    if (!owner.gpu) continue;
    try {
      if (["running", "restarting", "paused"].includes(await containerState(owner.container)) && (await gpuUnitActive(owner.gpu.unit))) {
        doublyServed.push(`${owner.container} + ${owner.gpu.unit}`);
      }
    } catch (error) {
      failures.push(`${owner.container} ownership verification: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  if (doublyServed.length > 0) failures.push(`Queues served twice: ${doublyServed.join(", ")}`);
  if (failures.length > 0) {
    throw new PreflightError(
      `Could not restore writers: ${failures.join(", ")}. ` +
        "Later results on this stack cannot be trusted",
    );
  }
  console.log(`[revert] restored ${owners.filter((owner) => owner.wasRunning).length} writer(s), one per queue`);
};
