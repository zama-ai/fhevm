// Scenario: deposit arc — BOUNDARY (#1760). The explicit companion to the wrap-phase smoke
// (deposit-arc.scenario.test.ts). Run as `demo:smoke-boundary`. This is NOT a hard gate: it exists
// to DOCUMENT, in executable form, exactly where the live deposit arc stops today and what remains
// to wire — so the gap is a recorded, asserted fact instead of a silent hole in the smoke.
//
// Why this is not a "call it live and watch it fail" test:
//   The next phase after wrap is `joinBatch`, whose signature REQUIRES a real coprocessor input proof
//   (`inputProof: SolanaZkProof` + `inputProofResult: SolanaSubmitInputProofResult` — see
//   sdk/.../vault/joinBatch.ts:49-53). The demo has no path to PRODUCE those artifacts, so the call
//   cannot even be constructed with honest inputs; a "live failure" here could only be manufactured
//   by passing fabricated proof args, which would assert an arbitrary reject reason — the exact
//   anti-pattern to avoid. Likewise `settleBatch` requires a `FhevmRuntime` for the KMS-certificate
//   phase (settleBatch.ts:47). So the boundary is proven structurally, not theatrically:
//     1. every downstream arc action (join → settle → claim → decrypt) EXISTS and is exported/wired;
//     2. the seeded config already carries every ROOT those actions consume (batcher, its per-batch
//        settle lookup table, kmsContext, the proof-service endpoint, both confidential mints);
//     3. the ONLY missing pieces are the two off-chain producers named above.
//   If a future change wires join/settle, this test's console log is the checklist to delete.
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
  test("documents the join -> settle -> claim -> decrypt gap: downstream actions and roots exist; only the coprocessor input-proof and settle FhevmRuntime remain to wire", async () => {
    const { config } = await loadDemoEnv();

    // (1) The downstream arc actions are all present and exported from the vault surface — the demo
    // is one step short of driving them, not missing the code path.
    const vault = await loadVaultModule();
    expect(typeof vault.joinBatch).toBe("function");
    expect(typeof vault.settleBatch).toBe("function");
    expect(typeof vault.buildClaimInstruction).toBe("function");
    expect(typeof vault.decryptPosition).toBe("function");

    // (2) The seeded config already carries every root those actions consume. If any were absent the
    // gap would be "not seeded"; asserting they are present pins the gap to the two producers in (3).
    expect(config.batchers.deposit.batcher).toBeTruthy();
    expect(config.batchers.deposit.lookupTable).toBeTruthy(); // settle's v0-tx address lookup table
    expect(config.kmsContext).toBeTruthy(); // certificate phase's context id
    expect(config.proofServiceUrl).toBeTruthy(); // MMR inclusion-proof source for settle
    expect(config.mints.joinConfidential).toBeTruthy(); // cUSDC (join)
    expect(config.mints.payoutConfidential).toBeTruthy(); // cShares (payout)

    // (3) The remaining, unwired pieces — the checklist that turns this boundary into the full arc.
    console.log(
      [
        "deposit-arc boundary: wrap phase is live; join -> settle -> claim -> decrypt is NOT yet wired.",
        "Remaining to wire (both are off-chain artifact PRODUCERS the demo cannot currently generate):",
        "  1. joinBatch: a real coprocessor input proof (SolanaZkProof + SolanaSubmitInputProofResult)",
        "     for the deposit amount — join takes these as inputs; the demo has no proof source.",
        "  2. settleBatch: an FhevmRuntime for the KMS burn-certificate phase (the MMR inclusion proof",
        "     from proofServiceUrl and the batcher/lookupTable/kmsContext roots are already seeded).",
        "Everything downstream of those (claim, decryptPosition) is exported and root-complete.",
      ].join("\n"),
    );
  });
});
