// parity — runs one case on both host chains and compares what each reader decrypted with the
// expected outcome and across chains. Each chain reaches the outcome its own way; a leg only
// reports what every reader saw.
//
// Each chain's leg runs in its own child process (`leg.ts`): an `@fhevm/sdk` entry point owns the
// encrypt and decrypt WASM modules of its process, so the viem and Solana entry points cannot both
// encrypt or decrypt in one process (zama-ai/fhevm-internal#2135). The parent starts both children
// at once, reads the result each writes, and compares them.

import { beforeAll, describe, expect, test } from "bun:test";
import { mkdtemp, readFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { until } from "../harness";

/** What one reader got: a cleartext, or a refusal. */
export type Outcome = bigint | "denied";

/** One expected read, keyed `<reader>:<value>` in a leg's results. */
export type Expectation = { readonly step: string; readonly is: Outcome };

/** Names a recognized refusal, or returns undefined for any other error. */
export type RefusalReason = (error: unknown) => string | undefined;

/** How a leg records its reads. A read that throws is recorded and does not stop the leg. */
export type Reads = {
  /** Retries `decrypt` until it answers: the ciphertext or a grant may still be propagating. */
  value(step: string, decrypt: () => Promise<bigint>): Promise<void>;
  /**
   * One attempt, which must be refused. Run it after the same reader decrypted a value it may read.
   * Recorded as an error without decrypting when an earlier read in the leg failed.
   */
  denied(step: string, decrypt: () => Promise<bigint>, refusal: RefusalReason): Promise<void>;
};

type Values = Record<string, bigint>;
export type Leg<V extends Values> = (values: V, reads: Reads) => Promise<void>;
export const CHAINS = ["evm", "solana"] as const;
export type Chain = (typeof CHAINS)[number];

/** A parity case: the value sets, the expected reads, and each chain's way of reaching them. */
export type ParityCase<V extends Values> = {
  readonly name: string;
  readonly cases: readonly V[];
  readonly expect: (values: V) => readonly Expectation[];
} & Record<Chain, Leg<V>>;

type StepOutcome =
  | { readonly kind: "value"; readonly value: string }
  | { readonly kind: "denied"; readonly reason: string }
  | { readonly kind: "error"; readonly message: string };
type StepResult = StepOutcome & { readonly ms: number; readonly attempts: number };

/** One value set's run on one chain, as the child writes it (bigints as decimal strings). */
export type LegRun = {
  readonly values: Record<string, string>;
  readonly steps: Record<string, StepResult>;
  readonly failure: string | undefined;
  readonly ms: number;
};

const DECRYPT_TIMEOUT_MS = 240_000;
const DECRYPT_INTERVAL_MS = 5_000;
/** A child that has not written its result by then is killed, and its leg fails. */
const LEG_DEADLINE_MS = 25 * 60_000;
const STDERR_TAIL_LINES = 40;

const message = (error: unknown) => (error instanceof Error ? error.message : String(error));
const encodeValues = (values: Values) => Object.fromEntries(Object.entries(values).map(([key, value]) => [key, String(value)]));

/** Runs one leg on one value set, recording every read. Used by the child process. */
export const runLeg = async <V extends Values>(leg: Leg<V>, values: V): Promise<LegRun> => {
  const steps: Record<string, StepResult> = {};
  const record = async (step: string, body: (attempt: () => number) => Promise<StepOutcome>) => {
    let attempts = 0;
    const start = performance.now();
    const outcome = await body(() => ++attempts).catch((error: unknown): StepOutcome => ({ kind: "error", message: message(error) }));
    steps[step] = { ...outcome, ms: Math.round(performance.now() - start), attempts };
  };
  const reads: Reads = {
    value: (step, decrypt) =>
      record(step, async (attempt) => {
        const value = await until(
          () => {
            attempt();
            return decrypt();
          },
          { timeoutMs: DECRYPT_TIMEOUT_MS, intervalMs: DECRYPT_INTERVAL_MS, description: step },
        );
        return { kind: "value", value: String(value) };
      }),
    denied: (step, decrypt, refusal) =>
      record(step, async (attempt) => {
        // A refusal means nothing when the stack failed a permitted read in this leg.
        const failed = Object.keys(steps).find((earlier) => steps[earlier]?.kind === "error");
        if (failed !== undefined) return { kind: "error", message: `not run: an earlier read in this leg failed (${failed})` };
        attempt();
        try {
          return { kind: "value", value: String(await decrypt()) };
        } catch (error) {
          const reason = refusal(error);
          if (reason === undefined) throw error;
          return { kind: "denied", reason };
        }
      }),
  };
  const start = performance.now();
  let failure: string | undefined;
  await leg(values, reads).catch((error: unknown) => {
    failure = message(error);
  });
  return { values: encodeValues(values), steps, failure, ms: Math.round(performance.now() - start) };
};

/** One child's outcome: its runs, one per value set, or why there are none. */
type LegReport = { readonly runs: readonly LegRun[] } | { readonly stopped: string };

const LEG_SCRIPT = path.join(import.meta.dir, "leg.ts");

const runChild = async (caseFile: string, chain: Chain, expectedRuns: number): Promise<LegReport> => {
  const output = path.join(await mkdtemp(path.join(os.tmpdir(), "parity-")), `${chain}.json`);
  const child = Bun.spawn(["bun", LEG_SCRIPT, caseFile, chain, output], {
    stdout: "inherit",
    stderr: "pipe",
    timeout: LEG_DEADLINE_MS,
    killSignal: "SIGKILL",
  });
  const [stderr, exitCode] = await Promise.all([new Response(child.stderr).text(), child.exited]);
  if (stderr) process.stderr.write(stderr.replace(/^/gm, `[${chain} leg] `));
  const tail = stderr.trimEnd().split("\n").slice(-STDERR_TAIL_LINES).join("\n");
  const stopped = (why: string): LegReport => ({ stopped: `${chain} leg ${why}${tail ? `\n--- ${chain} leg stderr (tail) ---\n${tail}` : ""}` });
  if (child.signalCode !== null) return stopped(`killed by ${child.signalCode} (deadline ${LEG_DEADLINE_MS / 60_000} min)`);
  if (exitCode !== 0) return stopped(`exited ${exitCode}`);
  const runs = await readFile(output, "utf8").then((text) => JSON.parse(text) as LegRun[]).catch(() => undefined);
  if (runs === undefined) return stopped("wrote no readable result");
  if (runs.length !== expectedRuns) return stopped(`wrote ${runs.length} of ${expectedRuns} value sets`);
  return { runs };
};

const show = (result: StepResult | undefined): string => {
  if (result === undefined) return "not run";
  const timing = `${(result.ms / 1000).toFixed(1)}s, ${result.attempts} attempt${result.attempts === 1 ? "" : "s"}`;
  if (result.kind === "value") return `${result.value} (${timing})`;
  if (result.kind === "denied") return `denied: ${result.reason} (${timing})`;
  return `error: ${result.message} (${timing})`;
};

const matches = (result: StepResult | undefined, expected: Outcome): boolean =>
  expected === "denied" ? result?.kind === "denied" : result?.kind === "value" && result.value === String(expected);

/**
 * Registers the case's tests: one child per chain runs every value set, both at once, and one test
 * per value set compares their reads with the expected outcomes. A chain whose child crashed, timed
 * out or wrote a partial result fails every test.
 */
export const parityTests = async (caseFile: string) => {
  const spec = (await import(caseFile)).parityCase as ParityCase<Values>;
  const reports = {} as Record<Chain, LegReport>;
  describe(spec.name, () => {
    beforeAll(async () => {
      const results = await Promise.all(CHAINS.map((chain) => runChild(caseFile, chain, spec.cases.length)));
      CHAINS.forEach((chain, index) => {
        reports[chain] = results[index]!;
      });
    }, LEG_DEADLINE_MS + 60_000);

    spec.cases.forEach((values, index) => {
      const label = Object.entries(values).map(([key, value]) => `${key}=${value}`).join(", ");
      test(label, () => {
        const legs = CHAINS.map((chain): { chain: Chain; run?: LegRun; stopped?: string } => {
          const report = reports[chain];
          if ("stopped" in report) return { chain, stopped: report.stopped };
          const run = report.runs[index]!;
          if (JSON.stringify(run.values) !== JSON.stringify(encodeValues(values))) {
            return { chain, stopped: `${chain} leg reported value set ${JSON.stringify(run.values)} at position ${index}` };
          }
          return { chain, run, stopped: run.failure === undefined ? undefined : `${chain} leg stopped: ${run.failure}` };
        });
        const expectations = spec.expect(values);
        const rows = expectations.map(({ step, is }) => [step, String(is), ...legs.map((leg) => show(leg.run?.steps[step]))]);
        const failures = legs.flatMap((leg) => (leg.stopped === undefined ? [] : [leg.stopped]));
        const misses = expectations.flatMap(({ step, is }) =>
          legs
            .filter((leg) => !matches(leg.run?.steps[step], is))
            .map((leg) => `${leg.chain} ${step}: expected ${is}, got ${show(leg.run?.steps[step])}`),
        );
        const durations = legs.map((leg) => `${leg.chain} ${leg.run === undefined ? "-" : `${(leg.run.ms / 1000).toFixed(1)}s`}`).join("  ");
        console.log(
          [
            `[parity] ${spec.name} (${label})  ${durations}`,
            ...[["step", "expected", ...CHAINS], ...rows].map((row) => row.join("  |  ")),
            ...failures,
            `parity: ${misses.length === 0 && failures.length === 0 ? "match" : "differ"}`,
          ].join("\n"),
        );
        expect([...failures, ...misses]).toEqual([]);
      });
    });
  });
};
