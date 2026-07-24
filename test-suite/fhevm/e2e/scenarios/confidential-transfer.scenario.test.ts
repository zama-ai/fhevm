// Scenario: confidential-transfer arc — the "portable now" token-flow phase ported from
// `solana/scripts/e2e/full-vertical.sh` (the `==> [sdk-transfer]` phase).
//
// It drives the product arc entirely through `@fhevm/sdk` Solana actions:
//   encrypt input -> submitInputProof -> confidentialTransfer -> userDecrypt (current handle).
// The SDK-driven arc + its assertions live in `runSolanaTwoHolderTransfer`; this scenario supplies
// the environment (via loadEnv), the readiness preconditions (via until), and the actor funding
// (via personas), then runs the arc and lets its assertions stand.
//
// Assertion map — bash `[sdk-transfer]` phase  ->  this scenario (all in runSolanaTwoHolderTransfer):
//   bash: "Alice 1000 -> confidentialTransfer(400) -> Alice 600, Bob 400"
//     - Alice initial balance == 1000  (userDecrypt of Alice's current handle)
//     - Bob   initial balance == 0      (userDecrypt of Bob's current handle)
//     - transfer rotates BOTH current balance handles (else "did not rotate both …" throws)
//     - Alice final balance   == 600    (userDecrypt of Alice's rotated handle)
//     - Bob   final balance   == 400    (userDecrypt of Bob's rotated handle)
// Every assertion is a live SDK current user-decrypt of a real on-chain balance — nothing is
// hard-coded, so the phase cannot pass on a trivial value. Provisioning (confidential mint, wrap,
// balance-state reads) still goes through the Rust live-client: those are SDK gaps (see the PR
// report), not part of this phase's assertions.

import { describe, test } from "bun:test";

import {
  createRealTwoHolderDependencies,
  runSolanaTwoHolderTransfer,
  solanaUserDecryptContext,
} from "../../src/solana/two-holder-transfer";
import { loadEnv, loadPersonas, until } from "../harness";

// A live stack takes minutes: SNS-commit waits alone poll up to ~2min per handle, times four.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

describe("solana confidential-transfer scenario", () => {
  test(
    "two holders: Alice 1000 -> transfer(400) -> Alice 600, Bob 400 (SDK actions, current decrypt)",
    async () => {
      const env = loadEnv();
      const personas = await loadPersonas(env);

      // Precondition (the suite may run right after a relayer/proof-service (re)start): gate on both
      // health endpoints before submitting. The relayer exposes GET /liveness (relayer's own
      // http/server.rs); a 200 means it is up.
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

      // The transfer arc pays its own tx fees from the deployer wallet; top it up where a faucet
      // exists (local). This genuinely exercises the faucet capability + persona funding.
      if (env.capabilities.faucet) await personas.fund(personas.deployer);

      // Run the SDK-driven arc against the injected environment. Its internal assertions (mapped
      // above) throw on any mismatch; reaching the end means the phase is green.
      await runSolanaTwoHolderTransfer(
        createRealTwoHolderDependencies({
          rpcUrl: env.rpcUrl,
          wsUrl: env.wsUrl,
          relayerUrl: env.relayerUrl,
          aclProgram: env.aclProgram,
          userDecryptContext: solanaUserDecryptContext(env.userDecryptContextId),
        }),
      );
    },
    SCENARIO_TIMEOUT_MS,
  );
});
