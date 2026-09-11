import { PreflightError } from "../errors";

export type PollerCatchupTarget = { name: string; block: bigint };

/** Wait for every poller to ingest the head captured before starting test traffic. */
export async function waitForPollerCatchup(
  targets: PollerCatchupTarget[],
  operations: {
    running(name: string): Promise<boolean>;
    progress(name: string): Promise<bigint | null>;
    sleep(ms: number): Promise<void>;
    now(): number;
  },
  timeoutMs = 15 * 60_000,
): Promise<void> {
  if (!targets.length) throw new PreflightError("No pollers selected for catchup readiness");
  const deadline = operations.now() + timeoutMs;
  for (;;) {
    const pending = [];
    for (const { name, block } of targets) {
      if (!(await operations.running(name))) throw new PreflightError(`Poller stopped during catchup: ${name}`);
      const progress = await operations.progress(name);
      if (progress === null || progress < block) pending.push(`${name}=${progress ?? "uninitialized"}/${block}`);
    }
    if (!pending.length) {
      console.log("[legacy-poller] All operators and chains caught up; starting test traffic");
      return;
    }
    const status = pending.join(", ");
    console.log(`[legacy-poller] Waiting for catchup: ${status}`);
    if (operations.now() >= deadline) throw new PreflightError(`Poller catchup timed out: ${status}`);
    await operations.sleep(5_000);
  }
}
