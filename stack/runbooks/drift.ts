// EXEMPLAR — ciphertext-drift chaos runbook.
// Injects a one-bit flip into a single coprocessor's ciphertext_digest table,
// asserts the on-chain divergence is detectable, then verifies the stack
// self-recovers after cleanup and restart.  Mirrors the "ciphertext-drift-auto-recovery"
// profile used in the heavy rollout test mode (test-suite/fhevm/rollouts/v0.12-to-v0.13/run.ts).
import type { Stack } from "../lib/stack";

// SQL that arms the one-shot drift trigger on coprocessor instance 0.
// Full DDL lives in test-suite/fhevm/src/drift.ts (DRIFT_INSTALL_SQL /
// DRIFT_CLEANUP_SQL); this runbook uses the same schema.
const DRIFT_INSTALL_SQL = `
CREATE TABLE IF NOT EXISTS drift_injection_state (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE,
  enabled BOOLEAN NOT NULL,
  consumed BOOLEAN NOT NULL DEFAULT FALSE,
  injected_handle BYTEA
);
INSERT INTO drift_injection_state (id, enabled, consumed, injected_handle)
  VALUES (TRUE, TRUE, FALSE, NULL)
  ON CONFLICT (id) DO UPDATE
    SET enabled = EXCLUDED.enabled,
        consumed = EXCLUDED.consumed,
        injected_handle = EXCLUDED.injected_handle;

CREATE OR REPLACE FUNCTION inject_ciphertext_drift_once()
RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE should_inject BOOLEAN;
BEGIN
  SELECT enabled AND NOT consumed INTO should_inject
    FROM drift_injection_state WHERE id = TRUE;
  IF NOT COALESCE(should_inject, FALSE) THEN RETURN NEW; END IF;
  IF NEW.txn_is_sent = FALSE
     AND NEW.ciphertext IS NOT NULL
     AND NEW.ciphertext128 IS NOT NULL
     AND (OLD.ciphertext IS NULL OR OLD.ciphertext128 IS NULL)
     AND EXISTS (SELECT 1 FROM computations WHERE output_handle = NEW.handle)
  THEN
    NEW.ciphertext := set_byte(NEW.ciphertext, 0, get_byte(NEW.ciphertext, 0) # 1);
    UPDATE drift_injection_state
       SET consumed = TRUE, injected_handle = NEW.handle
     WHERE id = TRUE;
  END IF;
  RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
CREATE TRIGGER ciphertext_drift_injector
  BEFORE UPDATE ON ciphertext_digest
  FOR EACH ROW EXECUTE FUNCTION inject_ciphertext_drift_once();
`;

const DRIFT_CLEANUP_SQL = `
DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
DROP FUNCTION IF EXISTS inject_ciphertext_drift_once();
DROP TABLE IF EXISTS drift_injection_state;
`;

// Database name for coprocessor instance 0 (instances >0 are "coprocessor_N").
const COPROCESSOR_DB = "coprocessor";

export default async (s: Stack): Promise<void> => {
  // ── healthy baseline ──────────────────────────────────────────────────────
  // Boot a two-of-three stack.  Three instances allow a single faulty node
  // to be outvoted by the honest majority.
  await s.up({ scenario: "two-of-three" });
  await s.test("rollout-standard");

  // ── inject drift ─────────────────────────────────────────────────────────
  // Arm the one-shot trigger on instance 0's DB.  The trigger fires exactly
  // once on the next ciphertext_digest row that gets a fresh ciphertext pair,
  // flipping bit 0 of the stored ciphertext bytes.
  await s.sql(COPROCESSOR_DB, DRIFT_INSTALL_SQL);

  // ── wait for drift warning ────────────────────────────────────────────────
  // The coprocessor consensus watchdog logs a warning when it detects that a
  // local ciphertext diverges from the majority view.  Wait up to 90 s; the
  // trigger fires on the first eligible computation after injection.
  await s.waitForLog("coprocessor", /ciphertext.?drift|consensus.?mismatch/i, {
    timeoutMs: 90_000,
  });

  // ── assert on-chain divergence ────────────────────────────────────────────
  // The SNS worker should NOT have posted the drifted ciphertext on-chain
  // because the threshold was not met (2-of-3 honest nodes disagree with
  // instance 0).  eth_getLogs for any SNS publication with the injected
  // handle should return an empty array.
  const driftedHandleLogs = await s.chain("eth_getLogs", [
    {
      // ACL_CONTRACT_ADDRESS is populated from discovery; the Stack
      // implementation resolves endpoint indirection at call time.
      address: "$ACL_CONTRACT_ADDRESS",
      fromBlock: "0x0",
      toBlock: "latest",
      // The injected handle is not known statically; any SNS-level "Verified"
      // event would indicate the drifted ciphertext leaked through consensus.
      topics: [
        "0x" + Buffer.from("CiphertextVerified(bytes32,address)").toString("hex").padStart(64, "0"),
      ],
    },
  ]);
  if ((driftedHandleLogs as unknown[]).length > 0) {
    throw new Error(
      `drift runbook: drifted ciphertext appeared on-chain — ` +
        `consensus did not reject it (${(driftedHandleLogs as unknown[]).length} log(s))`,
    );
  }

  // ── cleanup + restart ─────────────────────────────────────────────────────
  // Remove the trigger so normal operation can resume, then restart the
  // affected coprocessor instance so it re-reads the DB without the drifted row.
  await s.sql(COPROCESSOR_DB, DRIFT_CLEANUP_SQL);
  await s.restart("coprocessor");

  // ── wait for recovery ─────────────────────────────────────────────────────
  // Poll until the stack reports healthy again (all readiness probes pass and
  // the test harness can connect).  Timeout of 120 s covers the restart cycle
  // plus consensus re-sync.
  await s.until(
    async () => {
      const st = await s.state();
      return st.healthy;
    },
    { timeoutMs: 120_000, pollMs: 3_000 },
  );

  // ── final test gate ───────────────────────────────────────────────────────
  await s.test("rollout-standard");
};
