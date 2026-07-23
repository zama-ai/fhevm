// Scenario: deposit arc — BOUNDARY (#1760). The explicit companion to the wrap+join smoke
// (deposit-arc.scenario.test.ts). Run as `demo:smoke-boundary`. This is NOT a hard gate: it exists
// to DOCUMENT, in executable form, exactly where the live deposit arc stops today and what remains
// to wire — so the gap is a recorded, asserted fact instead of a silent hole in the smoke.
//
// Where the arc stops today: wrap AND join are live — the smoke builds a real coprocessor input
// proof (SDK local TFHE prover, verified by the relayer) and drives `joinBatch` on-chain. The
// remaining boundary is the dispatch -> settle -> claim -> decrypt continuation:
//   `settleBatch` requires an `FhevmRuntime` for the KMS burn-certificate leg (settleBatch.ts:47),
//   and the dispatch/claim/decrypt orchestration is not yet driven by the demo. That boundary is
//   proven structurally, not theatrically:
//     1. every downstream arc action (join → settle → claim → decrypt) EXISTS and is exported/wired;
//     2. the seeded config already carries every ROOT those actions consume (batcher, its per-batch
//        settle lookup table, kmsContext, the proof-service endpoint, both confidential mints);
//     3. the ONLY missing pieces are the settle FhevmRuntime plus the dispatch/claim/decrypt
//        orchestration named above.
//   If a future change wires dispatch/settle, this test's console log is the checklist to delete.
//
// STATUS: reads the seeded demo-config (produced by `demo:seed`); the SDK surface is reached through
// the runtime dynamic-import seam (untyped by construction). No transactions are submitted.

import { describe, expect, test } from "bun:test";

import { loadDemoEnv } from "../../demo/loadDemoEnv";

/** The downstream arc surface this boundary asserts is present and wired (untyped: dynamic-import seam). */
type VaultArcSurface = {
  joinBatch: unknown;
  settleBatch: unknown;
  buildClaimInstruction: unknown;
  decryptPosition: unknown;
};

const loadVaultModule = async (): Promise<VaultArcSurface> => {
  const vaultModule = "@fhevm/sdk/solana/vault";
  return (await import(vaultModule)) as unknown as VaultArcSurface;
};

// Demo-lane gate: `test:e2e` sweeps this directory on a stack that never ran `demo:seed`, so the
// seeded demo-config cannot exist there. The `demo:smoke-boundary` script sets RUN_DEMO_SCENARIOS=1;
// under it the test runs unconditionally, so a missing config still fails loudly in the demo lane.
const runsDemoScenarios = process.env.RUN_DEMO_SCENARIOS === "1";

describe.skipIf(!runsDemoScenarios)("solana deposit-arc boundary", () => {
  test("documents the dispatch -> settle -> claim -> decrypt gap: join is live; downstream actions and roots exist; only the settle FhevmRuntime and dispatch/claim/decrypt orchestration remain to wire", async () => {
    const { config } = await loadDemoEnv();

    // (1) The downstream arc actions are all present and exported from the vault surface — the demo
    // drives joinBatch live already and is missing producers/orchestration, not the code path.
    const vault = await loadVaultModule();
    expect(typeof vault.joinBatch).toBe("function");
    expect(typeof vault.settleBatch).toBe("function");
    expect(typeof vault.buildClaimInstruction).toBe("function");
    expect(typeof vault.decryptPosition).toBe("function");

    // (2) The seeded config already carries every root those actions consume. If any were absent the
    // gap would be "not seeded"; asserting they are present pins the gap to the two producers in (3).
    expect(config.batchers.deposit.batcher).toBeTruthy();
    expect(config.batchers.deposit.lookupTable).toBeTruthy(); // settle's v0-tx address lookup table
    expect(config.kmsContext).toBeTruthy(); // certificate leg's context id
    expect(config.proofServiceUrl).toBeTruthy(); // MMR inclusion-proof source for settle
    expect(config.mints.joinConfidential).toBeTruthy(); // cUSDC (join)
    expect(config.mints.payoutConfidential).toBeTruthy(); // cShares (payout)

    // (3) The remaining, unwired pieces — the checklist that turns this boundary into the full arc.
    console.log(
      [
        "deposit-arc boundary: wrap + join legs are live (join runs with a real coprocessor input",
        "proof in deposit-arc.scenario.test.ts); dispatch -> settle -> claim -> decrypt is NOT yet wired.",
        "Remaining to wire:",
        "  1. settleBatch: an FhevmRuntime for the KMS burn-certificate leg (the MMR inclusion proof",
        "     from proofServiceUrl and the batcher/lookupTable/kmsContext roots are already seeded).",
        "  2. dispatch/claim/decrypt orchestration: the actions (buildClaimInstruction, decryptPosition)",
        "     are exported and root-complete; the demo does not drive them yet.",
      ].join("\n"),
    );
  });
});
