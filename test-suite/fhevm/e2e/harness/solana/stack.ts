// stack — the Solana scenarios' one entry point to the running stack.
//
// `ensureUp(env)` verifies the endpoints every live scenario depends on — validator RPC and
// relayer — and returns a handle bundling the environment with the shared waits.
// It owns readiness, not lifecycle: the stack itself is brought up by `bun run demo up` (CI) or
// by a developer, and `ensureUp` reuses whatever healthy stack it finds rather than starting or
// stopping anything. A scenario that reaches its test body after `ensureUp` resolves can attribute
// any later failure to the arc under test, not to a half-started stack.

import { waitForSnsCommit } from "../../../src/solana/sns";
import { until } from "../../../src/utils/until";
import type { TestEnv } from "../loadEnv";

// Per-attempt request cap, carried over from the `curl -m2`/`-m3` the bash probes used. Without it
// a service that accepts the TCP connection but never answers hangs inside `fetch`, so `until()`
// never gets to re-evaluate its deadline and the scenario dies on bun's test timeout instead of
// naming the probe that stalled.
const PROBE_TIMEOUT_MS = 3_000;

export type SolanaStack = {
  readonly env: TestEnv;
  /** Waits until the SNS worker committed both ciphertext forms for `handle` (0x-hex, 32 bytes). */
  waitForSnsCommit(handle: string): Promise<void>;
};

/**
 * Gates on the running stack's health and returns the scenario-facing handle.
 *
 * The two probes mirror what the suite may race right after a (re)start:
 * - validator RPC answers `getHealth` (a freshly restarted validator refuses connections),
 * - the relayer serves GET /liveness (its own http/server.rs).
 */
export const ensureUp = async (env: TestEnv): Promise<SolanaStack> => {
  await until(
    async () => {
      const response = await fetch(env.rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "getHealth" }),
        signal: AbortSignal.timeout(PROBE_TIMEOUT_MS),
      });
      if (!response.ok) return false;
      const body = (await response.json()) as { result?: string };
      return body.result === "ok";
    },
    { description: "validator RPC health", timeoutMs: 60_000 },
  );
  await until(
    async () => (await fetch(`${env.relayerUrl}/liveness`, { signal: AbortSignal.timeout(PROBE_TIMEOUT_MS) })).ok,
    { description: "relayer liveness", timeoutMs: 60_000 },
  );
  return {
    env,
    // Bound to the environment's container so the `COPROCESSOR_DB_CONTAINER` override reaches the
    // probe; unbound, setting it silently sent every wait at the hardcoded default.
    waitForSnsCommit: (handle: string) => waitForSnsCommit(handle, env.coprocessorDbContainer),
  };
};
