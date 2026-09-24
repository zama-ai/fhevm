import path from "node:path";
import { mkdir, mkdtemp } from "node:fs/promises";
import { REPO_ROOT } from "../layout";

/** Child stdin owns the fault lifetime even if its parent is killed. */
export async function withRolloutSupervisor<T>(stateDir: string, script: string, args: string[], env: Record<string, string>, task: () => Promise<T>): Promise<T> {
  if (!["hold-retired-writers.sh", "hold-key-download.sh", "hold-migration-gpu.sh", "hold-rollout-services.sh", "interrupt-upgrade-controller.sh", "interrupt-key-application.sh", "hold-host-report.sh"].includes(script)) throw new Error("unknown rollout fault supervisor");
  const root = path.join(stateDir, "rollout");
  await mkdir(root, { recursive: true });
  const directory = await mkdtemp(path.join(root, "fault-owner-"));
  const child = Bun.spawn(["bash", path.join(REPO_ROOT, "test-suite/fhevm/scripts", script), ...args], {
    stdin: "pipe", stdout: "pipe", stderr: "inherit",
    env: { ...process.env, ...env, SC_RESTORE_LOG: path.join(directory, "restore.log") },
  });
  const reader = child.stdout.getReader();
  let ready = "";
  let timer: ReturnType<typeof setTimeout> | undefined;
  let output: Promise<void> | undefined;
  let taskCompleted = false;
  try {
    await Promise.race([
      (async () => {
        while (!ready.includes("ROLLOUT_HOLD_READY\n")) {
          const next = await reader.read();
          if (next.done) throw new Error("fault supervisor exited before establishing ownership");
          ready += new TextDecoder().decode(next.value);
        }
      })(),
      new Promise<never>((_, reject) => { timer = setTimeout(() => reject(new Error("fault supervisor setup timed out")), 180_000); }),
    ]);
    if (timer) clearTimeout(timer);
    console.log(`[rollout fault] supervisor=${script}; recovery ledger ${directory}`);
    output = (async () => {
      for (;;) {
        const next = await reader.read();
        if (next.done) return;
        process.stdout.write(next.value);
      }
    })();
    // Observe failures immediately to avoid an unhandled rejection while task runs.
    output.catch(() => undefined);
    const result = await task();
    taskCompleted = true;
    return result;
  } finally {
    if (timer) clearTimeout(timer);
    // A completed workload asks the supervisor to finish its fault and recovery.
    // Failure closes stdin without that request, cancelling an unobserved fault.
    try {
      if (taskCompleted) child.stdin.write("release\n");
      await child.stdin.end();
    } catch { /* Exit status remains authoritative. */ }
    const status = await child.exited;
    if (output) await output;
    else await reader.cancel();
    reader.releaseLock();
    if (status !== 0) throw new Error(`fault/recovery failed (${status}); retain ${directory} and discard the rollout stack until restored`);
  }
}
