// Scenario: deposit arc — BOUNDARY (#1760). The explicit companion to the deposit-arc smoke
// (deposit-arc.scenario.test.ts). Run as `demo:smoke-boundary`. This is NOT a hard gate: it exists
// to DOCUMENT, in executable form, exactly where the live deposit arc stops today and what remains
// to wire — so the gap is a recorded, asserted fact instead of a silent hole in the smoke.
//
// Where the arc stops today: wrap, join, dispatch AND settle are live — the smoke drives the full
// batch lifecycle through a real coprocessor input proof (join), the keeper's dispatch, and
// `settleBatch` (MMR inclusion proof + KMS burn certificate + on-chain settle). The remaining
// boundary is the claim -> decrypt continuation: the demo does not yet drive
// `buildClaimInstruction` (alice claiming her cShares payout from the settled batch) or
// `decryptPosition` (reading her confidential position back). That boundary is proven
// structurally, not theatrically:
//   1. every downstream arc action (claim, decryptPosition) EXISTS and is exported/wired;
//   2. the seeded config already carries every ROOT those actions consume (the batcher, the payout
//      confidential mint, the relayer endpoint for user-decrypt);
//   3. the ONLY missing piece is the claim/decrypt orchestration named above.
//   If a future change wires claim/decrypt, this test's console log is the checklist to delete.
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
  test("documents the claim -> decrypt gap: wrap/join/dispatch/settle are live; the claim and decrypt actions and their roots exist; only their orchestration remains to wire", async () => {
    const { config } = await loadDemoEnv();

    // (1) The downstream arc actions are all present and exported from the vault surface — the demo
    // drives joinBatch/settleBatch live already and is missing orchestration, not the code path.
    const vault = await loadVaultModule();
    expect(typeof vault.joinBatch).toBe("function");
    expect(typeof vault.settleBatch).toBe("function");
    expect(typeof vault.buildClaimInstruction).toBe("function");
    expect(typeof vault.decryptPosition).toBe("function");

    // (2) The seeded config already carries every root the claim/decrypt legs consume. If any were
    // absent the gap would be "not seeded"; asserting they are present pins the gap to (3).
    expect(config.batchers.deposit.batcher).toBeTruthy();
    expect(config.mints.payoutConfidential).toBeTruthy(); // cShares — what a claim pays out in
    expect(config.relayerUrl).toBeTruthy(); // user-decrypt endpoint for decryptPosition
    expect(config.userDecryptContextId).toBeTruthy(); // user-decrypt context id

    // (3) The remaining, unwired piece — the checklist that turns this boundary into the full arc.
    console.log(
      [
        "deposit-arc boundary: wrap + join + dispatch + settle legs are live (the smoke settles the",
        "batch with a real KMS burn certificate in deposit-arc.scenario.test.ts); claim -> decrypt is",
        "NOT yet wired. Remaining to wire:",
        "  1. claim orchestration: drive buildClaimInstruction so alice claims her cShares payout",
        "     from the settled batch (the action is exported and root-complete).",
        "  2. decrypt orchestration: drive decryptPosition so alice reads her confidential position",
        "     back through user-decrypt (the action is exported and root-complete).",
      ].join("\n"),
    );
  });
});
