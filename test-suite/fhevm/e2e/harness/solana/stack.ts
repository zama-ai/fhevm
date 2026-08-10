// stack — the Solana scenarios' one entry point to the running stack.
//
// `ensureUp(env)` verifies the endpoints every live scenario depends on — validator RPC, relayer,
// solana-proof-service — and returns a handle bundling the environment with the shared waits.
// It owns readiness, not lifecycle: the stack itself is brought up by `bun run demo up` (CI) or
// by a developer, and `ensureUp` reuses whatever healthy stack it finds rather than starting or
// stopping anything. A scenario that reaches its test body after `ensureUp` resolves can attribute
// any later failure to the arc under test, not to a half-started stack.

import { waitForSnsCommit, type SnsCommitOptions } from "../../../src/solana/sns";
import { until } from "../until";
import type { TestEnv } from "../loadEnv";

export type SolanaStack = {
  readonly env: TestEnv;
  /** Waits until the SNS worker committed both ciphertext forms for `handle` (0x-hex, 32 bytes). */
  waitForSnsCommit(handle: string, options?: SnsCommitOptions): Promise<void>;
};

/**
 * Gates on the running stack's health and returns the scenario-facing handle.
 *
 * The three probes mirror what the suite may race right after a (re)start:
 * - validator RPC answers `getHealth` (a freshly restarted validator refuses connections),
 * - the relayer serves GET /liveness (its own http/server.rs),
 * - the solana-proof-service reports `"ready": true` (it must have caught up with geyser).
 */
export const ensureUp = async (env: TestEnv): Promise<SolanaStack> => {
  await until(
    async () => {
      const response = await fetch(env.rpcUrl, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "getHealth" }),
      });
      return response.ok && /"ok"/.test(await response.text());
    },
    { description: "validator RPC health", timeoutMs: 60_000 },
  );
  await until(
    async () => (await fetch(`${env.relayerUrl}/liveness`)).ok,
    { description: "relayer liveness", timeoutMs: 60_000 },
  );
  await until(
    async () => {
      const body = await (await fetch(`${env.proofServiceUrl}/health/readiness`)).text();
      return /"ready"\s*:\s*true/.test(body);
    },
    { description: "solana-proof-service readiness", timeoutMs: 120_000 },
  );
  return { env, waitForSnsCommit };
};
