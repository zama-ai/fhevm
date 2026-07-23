// Scenario: deposit arc — the confidential-vault demo's forward leg (#1760), the FIRST live-cluster
// exercise of the batcher + vault via `@fhevm/sdk/solana/vault`. Run as `demo:smoke`.
//
// STATUS: live-only, UNVERIFIED here. It requires a running demo stack with the two demo programs
// deployed and `demo:seed` completed (both classifier-gated / blocked in this environment — see
// solana/scripts/demo/demo-keypairs/README and demo/seed.ts). It is exercised only by the
// `solana-demo-acceptance` workflow (manual dispatch), which runs demo-up.sh (deploy + seed) and
// starts the faucet before invoking this. The SDK is reached through the runtime dynamic-import seam
// (string module specifier), so the vault module is untyped here by construction (same reason as
// `src/solana/current-user-decrypt.ts`): the SDK's generated `_types` are not built at tsc time.
//
// Assertion map — the deposit arc (join = mock USDC → cUSDC, payout = cShares):
//   1. alice funded with mock USDC through the demo faucet (env.capabilities.faucet).
//   2. wrap USDC → cUSDC confidential balance                [live-client gap, see below].
//   3. joinBatch(deposit batcher current batch) with an SDK input proof for the wrapped amount.
//   4. dispatchBatch once the batch is old enough (min_batch_age_slots).
//   5. settleBatch (keeper/operator action) against the batch's settle lookup table.
//   6. claim alice's proportional cShares payout.
//   7. decryptPosition asserts alice's claimed cShares > 0 (live user-decrypt of on-chain state).
//
// WRAP GAP (step 2): wrapping an SPL underlying into a confidential balance is not on the stable
// `@fhevm/sdk/solana/vault` surface; the existing two-holder scenario performs the confidential-mint
// wrap through the Rust live-client (`poc-live-client`), documented there as an SDK gap. This
// scenario keeps that boundary explicit rather than inventing a wrap call whose behavior cannot be
// verified here. Wiring it (and seed provisioning) is the remaining live task, tracked in the PR
// report; the workflow surfaces the real state end-to-end.

import { describe, test } from "bun:test";

import { loadPersonas, until } from "../harness";
import { DEMO_KEYPAIRS, loadDemoEnv } from "../../demo/loadDemoEnv";
import { depositRoots } from "../../demo/config";

// A live batcher arc waits on slot age + SNS commit + settle certificate: allow well beyond the
// transfer scenario's budget.
const SCENARIO_TIMEOUT_MS = 20 * 60_000;

/** The vault batch-lifecycle actions (untyped: reached through the runtime dynamic-import seam). */
const loadVaultModule = async (): Promise<Record<string, (...args: unknown[]) => unknown>> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as Record<string, (...args: unknown[]) => unknown>;
};

describe("solana deposit-arc scenario", () => {
  test(
    "deposit: alice wraps mock USDC -> joins -> dispatch -> settle (keeper) -> claim -> decrypt cShares > 0",
    async () => {
      const { env, config } = await loadDemoEnv();

      // Personas: the keeper is the operator that plays dispatch + settle; alice is the depositing
      // end-user. Both load from committed demo keypairs (pubkeys cross-checked against the config).
      const personas = await loadPersonas(env, {
        keeper: DEMO_KEYPAIRS.keeper,
        alice: DEMO_KEYPAIRS.alice,
      });

      // Preconditions: the suite may run right after a relayer / proof-service (re)start. Gate on both
      // health endpoints before submitting (same gates as the confidential-transfer scenario).
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

      // Provisioning + endpoints are ready; project the config onto the deposit-direction roots the
      // SDK derivation helpers consume, and prove the vault subpath resolves at runtime.
      const roots = depositRoots(config);
      const vault = await loadVaultModule();
      void vault;
      void roots;
      void personas;

      // The remaining arc (steps 1–7 above) requires the WRAP GAP resolved (confidential-token wrap
      // via the live-client) and seed provisioning wired. Until then this fails loudly rather than
      // yielding a false green; the solana-demo-acceptance workflow is the verification vehicle.
      throw new Error(
        "deposit-arc smoke is not runnable until demo:seed provisioning and the confidential-token " +
          "wrap (poc-live-client) are wired; verifiable end-to-end only via the solana-demo-acceptance workflow.",
      );
    },
    SCENARIO_TIMEOUT_MS,
  );
});
