// Scenario: confidential-transfer arc, driven entirely through `@fhevm/sdk` Solana actions:
//   encrypt input -> submitInputProof -> confidentialTransfer -> userDecrypt (current handle).
// The arc and its assertions live in `runSolanaTwoHolderTransfer`; this scenario supplies the
// environment (via loadEnv), the readiness preconditions (via until), and the actor funding (via
// personas), then runs the arc and lets its assertions stand. Alice sends Bob 400 of her 1000; the
// transfer must rotate both balance handles, and each balance is read by a live SDK user-decrypt.

import { describe, test } from "bun:test";

import { createRealTwoHolderDependencies, runSolanaTwoHolderTransfer } from "../../src/solana/two-holder-transfer";
import { loadEnv, loadPersonas } from "../harness";
import { ensureUp } from "../harness/solana/stack";

// A live stack takes minutes: SNS-commit waits alone poll up to ~2min per handle, times four.
const SCENARIO_TIMEOUT_MS = 15 * 60_000;

describe("solana confidential-transfer scenario", () => {
  test(
    "two holders: Alice 1000 -> transfer(400) -> Alice 600, Bob 400 (SDK actions, current decrypt)",
    async () => {
      const env = loadEnv();
      const personas = await loadPersonas(env);

      // Precondition (the suite may run right after a relayer (re)start): gate on
      // the stack's health before submitting.
      const stack = await ensureUp(env);

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
          decryptionRelayerUrl: env.decryptionRelayerUrl,
          aclProgram: env.aclProgram,
          funding: env.funding,
          funderKeypairPath: env.capabilities.faucet ? undefined : env.roots.deployerKeypairPath,
          waitForHandle: stack.waitForSnsCommit,
        }),
      );
    },
    SCENARIO_TIMEOUT_MS,
  );
});
