import { randomUUID } from "node:crypto";
import { loadMergedComposeDoc } from "../generate/compose";
import { dockerArgs, PROJECT } from "../layout";
import { mergeArgs } from "../utils/fs";
import { composeEnv, run } from "../utils/process";

const PRIMARY = "coprocessor-sns-worker";
const COUNT = 640;
const BATCH = 20;
type Phase = (phase: "issue" | "verify", address?: string) => Promise<string>;

function cohort(output: string): { address: string; handles: string[] } {
  const line = output.split("\n").find((entry) => entry.startsWith("SNS_BATCH_COHORT="));
  if (!line) throw new Error("The host test did not report its handle cohort");
  const { address, handles } = JSON.parse(line.slice("SNS_BATCH_COHORT=".length));
  if (
    !/^0x[0-9a-fA-F]{40}$/.test(address) ||
    !Array.isArray(handles) ||
    handles.length !== COUNT ||
    new Set(handles).size !== COUNT ||
    handles.some((h) => typeof h !== "string" || !/^0x[0-9a-fA-F]{64}$/.test(h))
  ) {
    throw new Error("Invalid SNS publication cohort");
  }
  return { address, handles };
}

async function waitForCohort(query: () => Promise<string>, state: string, signal: AbortSignal) {
  const deadline = Date.now() + 360_000;
  let count = 0;
  do {
    signal.throwIfAborted();
    count = Number(await query());
    if (count === COUNT) return;
    await Bun.sleep(1000);
  } while (Date.now() < deadline);
  throw new Error(`Timed out with ${count}/${COUNT} test handles ${state}`);
}

interface Interval {
  start: number;
  end: number;
}
export function snsBatchEvidence(lines: string) {
  const intervals: Interval[] = [];
  let start: number | undefined;
  let migrationDisabled = false,
    idle = false,
    delayed = 0,
    immediate = 0;
  // Docker returns stdout and stderr separately. Only consume structured worker events.
  const events = lines
    .split("\n")
    .filter((line) => line.startsWith("{"))
    .map((line) => JSON.parse(line))
    .sort((a, b) => Date.parse(a.timestamp) - Date.parse(b.timestamp));
  for (const event of events) {
    const f = event.fields;
    if (!f) continue;
    const time = Date.parse(event.timestamp);
    if (f.message === "S3 migration is disabled") migrationDisabled = true;
    if (f.message === "Fetched SnS tasks") {
      if (f.count === 0) idle = true;
      start = f.count === BATCH ? time : undefined;
    }
    const victim = event.span?.target === "sns" && /deadlock detected/i.test(f.error ?? "");
    if (victim && f.message === "Service error; retrying...") delayed++;
    if (victim && f.message === "Database deadlock; retrying immediately") immediate++;
    if (start !== undefined && (victim || f.message === "more tasks to process, continuing")) {
      intervals.push({ start, end: time });
      start = undefined;
    }
  }
  return { intervals, migrationDisabled, idle, delayed, immediate };
}

export function hasCompetingBatches(workers: { intervals: Interval[] }[]): boolean {
  return workers.some(({ intervals }, i) =>
    workers
      .slice(i + 1)
      .some((other) => intervals.some((a) => other.intervals.some((b) => a.start < b.end && b.start < a.end))),
  );
}

/** Runs only against the primary operator of a disposable local compose stack. */
export async function runSnsBatchPublication(phase: Phase, query: (sql: string) => Promise<string>) {
  const [primaryId, status, project, started, primaryImage] = (
    await run([
      "docker",
      "inspect",
      "--format",
      '{{.Id}} {{.State.Status}} {{index .Config.Labels "com.docker.compose.project"}} {{.State.StartedAt}} {{.Image}}',
      PRIMARY,
    ])
  ).stdout
    .trim()
    .split(" ");
  if (status !== "running" || project !== PROJECT) throw new Error("Expected the running local Compose SNS worker");
  const logs = async (id: string, since: string) => {
    const result = await run(["docker", "logs", "--since", since, id]);
    return result.stdout + "\n" + result.stderr;
  };
  if (!snsBatchEvidence(await logs(primaryId, started)).migrationDisabled) {
    throw new Error("Run this profile with S3 migration disabled on the primary SNS worker");
  }

  const mirror = await query(`SELECT EXISTS (
    SELECT 1 FROM pg_trigger t JOIN pg_proc f ON f.oid = t.tgfoid
    WHERE t.tgrelid = 'public.ciphertext_digest'::regclass AND (t.tgtype::int & 4) = 4
      AND f.proname = 'mirror_ciphertext_digest_branchless' AND t.tgenabled IN ('O', 'A')
  )`);
  if (mirror !== "t") throw new Error("The v0.14 digest insert mirror must be enabled for this profile");

  const env = await composeEnv("coprocessor");
  const runId = randomUUID();
  const service = (await loadMergedComposeDoc("coprocessor")).services[PRIMARY];
  if (!Array.isArray(service?.command)) throw new Error("Missing local SNS Compose command");
  // The normal Compose profile allows 100 uploads on a 10-connection pool.
  // Direct uploads can exhaust that pool at cold start before SNS retries run.
  const command = mergeArgs(service.command as string[], [
    "--work-items-batch-size=20",
    "--s3-migration=no",
    "--pg-pool-connections=24",
    "--s3-max-concurrent-uploads=4",
    "--pg-polling-interval=2",
    "--s3-regular-recheck-duration=5s",
  ]);
  const override = JSON.stringify({ services: { [PRIMARY]: { command } } });
  const replicas = [1, 2, 3, 4].map((i) => `sns-publication-${runId}-${i}`);
  const created: string[] = [];
  const controller = new AbortController();
  const interrupt = () => controller.abort(new Error("SNS batch publication interrupted"));
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", interrupt);
  try {
    // Stage through the host contract while only this operator's SNS service is
    // stopped. Other end-to-end profiles must not run against this stack concurrently.
    await run(["docker", "stop", primaryId]);
    const since = new Date().toISOString();
    const launches = await Promise.allSettled(
      replicas.map(async (name) => {
        // Same Compose service, database, keys and image as the primary. Each
        // process gets its own container namespace and worker connection pool.
        await run([...dockerArgs("coprocessor"), "-f", "-", "run", "-d", "--no-deps", "--name", name, PRIMARY], {
          env,
          input: override,
        });
        created.push(name);
        const image = (await run(["docker", "inspect", "--format", "{{.Image}}", name])).stdout.trim();
        if (image !== primaryImage) throw new Error("SNS replica image differs from the primary");
      }),
    );
    for (const launch of launches) if (launch.status === "rejected") throw launch.reason;

    // Warm the keys on empty queues before making the normal host workload.
    // All replicas must be able to work the backlog when it becomes available.
    const readyBy = Date.now() + 120_000;
    for (const name of replicas) {
      while (!snsBatchEvidence(await logs(name, since)).idle) {
        controller.signal.throwIfAborted();
        if (Date.now() >= readyBy) throw new Error(`SNS replica ${name} never polled an empty queue`);
        await Bun.sleep(500);
      }
    }
    controller.signal.throwIfAborted();
    await run(["docker", "pause", ...replicas]);
    const { address, handles } = cohort(await phase("issue"));
    const ids = handles.map((h) => `decode('${h.slice(2)}','hex')`).join(",");
    const filter = `p.handle IN (${ids})`;
    await waitForCohort(
      () =>
        query(`SELECT count(DISTINCT p.handle) FROM pbs_computations p
      JOIN ciphertexts c USING (handle) WHERE ${filter} AND NOT p.is_completed AND c.ciphertext IS NOT NULL`),
      "ready for SNS",
      controller.signal,
    );

    const deadlocks = () =>
      query("SELECT deadlocks FROM pg_stat_database WHERE datname = current_database()").then(BigInt);
    const before = await deadlocks();
    await run(["docker", "unpause", ...replicas]);

    await waitForCohort(
      () =>
        query(`SELECT count(DISTINCT p.handle) FROM pbs_computations p
      JOIN ciphertext_digest d USING (handle) WHERE ${filter} AND p.is_completed
      AND octet_length(d.ciphertext) = 32 AND octet_length(d.ciphertext128) = 32`),
      "published by the primary operator",
      controller.signal,
    );
    const evidence = await Promise.all(created.map(async (id) => snsBatchEvidence(await logs(id, since))));
    console.log(
      "[sns-batch]",
      evidence.map(({ intervals, migrationDisabled, delayed, immediate }) => ({
        fullBatches: intervals.length,
        migrationDisabled,
        delayed,
        immediate,
      })),
    );
    if (evidence.some((e) => !e.migrationDisabled)) throw new Error("Missing migration-disabled startup evidence");
    if (!hasCompetingBatches(evidence))
      throw new Error("Inconclusive run: no overlapping full batches on different SNS workers");

    await phase("verify", address);
    controller.signal.throwIfAborted();
    const dbVictims = (await deadlocks()) - before;
    const victims = evidence.reduce((total, e) => total + e.delayed + e.immediate, 0);
    console.log(
      `[sns-batch] PostgreSQL deadlocks on the isolated operator database: ${dbVictims}, SNS compute-loop retries: ${victims}`,
    );
    if (victims || dbVictims !== 0n) throw new Error("Observed a database deadlock while publishing the cohort");
    console.log(`[sns-batch] ${COUNT} handles published, competing batches observed, zero SNS deadlock retries`);
  } finally {
    const cleanup = await Promise.allSettled([
      run(["docker", "start", primaryId]),
      ...created.map((name) => run(["docker", "rm", "-f", name])),
    ]);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", interrupt);
    const failed = cleanup.filter((result) => result.status === "rejected");
    if (failed.length)
      throw new AggregateError(
        failed.map((result) => result.reason),
        "Could not restore the SNS stack",
      );
  }
}
