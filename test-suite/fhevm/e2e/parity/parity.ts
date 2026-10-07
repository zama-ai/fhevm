// parity — runs one case on both host chains and compares what each reader decrypted with the
// expected outcome and across chains. Each chain reaches the outcome its own way; a leg only
// reports what every reader saw.

import { expect, test } from "bun:test";

import { until } from "../harness";

/** What one reader got: a cleartext, or a refusal. */
export type Outcome = bigint | "denied";

/** One expected read, keyed `<reader>:<value>` in a leg's results. */
export type Expectation = { readonly step: string; readonly is: Outcome };

type StepOutcome =
  | { readonly kind: "value"; readonly value: bigint }
  | { readonly kind: "denied"; readonly reason: string }
  | { readonly kind: "error"; readonly message: string };
type StepResult = StepOutcome & { readonly ms: number };

/** Names a recognized refusal, or returns undefined for any other error. */
export type RefusalReason = (error: unknown) => string | undefined;

/** How a leg records its reads. A read that throws is recorded and does not stop the leg. */
export type Reads = {
  /** Retries `decrypt` until it answers: the ciphertext or a grant may still be propagating. */
  value(step: string, decrypt: () => Promise<bigint>): Promise<void>;
  /** One attempt, which must be refused. Run it after the same reader decrypted a value it may read. */
  denied(step: string, decrypt: () => Promise<bigint>, refusal: RefusalReason): Promise<void>;
};

export type Leg<Values> = (values: Values, reads: Reads) => Promise<void>;

const DECRYPT_TIMEOUT_MS = 240_000;
const DECRYPT_INTERVAL_MS = 5_000;

const message = (error: unknown) => (error instanceof Error ? error.message : String(error));

const runLeg = async <Values>(leg: Leg<Values>, values: Values) => {
  const results = new Map<string, StepResult>();
  const timedStep = async (step: string, body: () => Promise<StepOutcome>) => {
    const start = performance.now();
    const outcome = await body().catch((error: unknown): StepOutcome => ({ kind: "error", message: message(error) }));
    results.set(step, { ...outcome, ms: Math.round(performance.now() - start) });
  };
  const reads: Reads = {
    value: (step, decrypt) =>
      timedStep(step, async () => ({
        kind: "value",
        value: await until(decrypt, { timeoutMs: DECRYPT_TIMEOUT_MS, intervalMs: DECRYPT_INTERVAL_MS, description: step }),
      })),
    denied: (step, decrypt, refusal) =>
      timedStep(step, async () => {
        try {
          return { kind: "value", value: await decrypt() };
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
  return { results, failure, ms: Math.round(performance.now() - start) };
};

const show = (result: StepResult | undefined): string => {
  if (result === undefined) return "not run";
  const seconds = `${(result.ms / 1000).toFixed(1)}s`;
  if (result.kind === "value") return `${result.value} (${seconds})`;
  if (result.kind === "denied") return `denied: ${result.reason} (${seconds})`;
  return `error: ${result.message} (${seconds})`;
};

const matches = (result: StepResult | undefined, expected: Outcome): boolean =>
  expected === "denied" ? result?.kind === "denied" : result?.kind === "value" && result.value === expected;

/**
 * Registers one bun test per value set. Each test runs the EVM leg, then the Solana leg, prints
 * every step of both, and fails on any step that misses its expected outcome. The legs run one after
 * the other: both swap the global `fetch` while they reach the relayer's key material.
 */
export const parityCase = <Values extends Record<string, bigint>>(spec: {
  readonly name: string;
  readonly cases: readonly Values[];
  readonly expect: (values: Values) => readonly Expectation[];
  readonly evm: Leg<Values>;
  readonly solana: Leg<Values>;
  readonly timeoutMs: number;
}) => {
  for (const values of spec.cases) {
    const label = Object.entries(values).map(([key, value]) => `${key}=${value}`).join(", ");
    test(
      `${spec.name} (${label})`,
      async () => {
        const evm = await runLeg(spec.evm, values);
        const solana = await runLeg(spec.solana, values);
        const expectations = spec.expect(values);
        const rows = expectations.map(({ step, is }) => [step, String(is), show(evm.results.get(step)), show(solana.results.get(step))]);
        const misses = expectations.flatMap(({ step, is }) => [
          ...(matches(evm.results.get(step), is) ? [] : [`evm ${step}: expected ${is}, got ${show(evm.results.get(step))}`]),
          ...(matches(solana.results.get(step), is) ? [] : [`solana ${step}: expected ${is}, got ${show(solana.results.get(step))}`]),
        ]);
        const failures = [
          ...(evm.failure === undefined ? [] : [`evm leg stopped: ${evm.failure}`]),
          ...(solana.failure === undefined ? [] : [`solana leg stopped: ${solana.failure}`]),
        ];
        console.log(
          [
            `[parity] ${spec.name} (${label})  evm ${(evm.ms / 1000).toFixed(1)}s  solana ${(solana.ms / 1000).toFixed(1)}s`,
            ...[["step", "expected", "evm", "solana"], ...rows].map((row) => row.join("  |  ")),
            ...failures,
            `parity: ${misses.length === 0 && failures.length === 0 ? "match" : "differ"}`,
          ].join("\n"),
        );
        expect([...failures, ...misses]).toEqual([]);
      },
      spec.timeoutMs,
    );
  }
};
